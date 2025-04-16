#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! anyhow = "1.0"
//! clap = { version = "4.4", features = ["derive"] }
//! flate2 = "1.0"
//! reqwest = { version = "0.11", features = ["json", "stream"] }
//! rustdoc-types = "0.39"
//! semver = "1.0"
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! tar = "0.4"
//! tempfile = "3.8"
//! tokio = { version = "1.34", features = ["full"] }
//! tracing = "0.1"
//! tracing-subscriber = { version = "0.3", features = ["env-filter"] }
//! rustdoc-json = "*"
//! rustup-toolchain = "0.1"
//! ```
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::cognitive_complexity)] // Allow complex functions for now

use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use flate2::read::GzDecoder;
use rustdoc_json::Builder;
use rustdoc_types::{
    Abi, Constant, Crate, Discriminant, Enum, Function, GenericArg, GenericArgs, GenericBound,
    GenericParamDef, Generics, Id, Impl, Item, ItemEnum, ItemKind, Module, Path, PolyTrait,
    Primitive, Struct, StructKind, Term, Trait, Type, Variant, VariantKind, WherePredicate,
};
use semver::{Version, VersionReq};
use serde::Deserialize;
// Removed unused imports
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Write;
use std::fs::File;
// Removed unused Hash import
use std::io::{BufReader, Cursor};
use std::path::{Path as FilePath, PathBuf};
use tar::Archive;
use tracing::{debug, info, warn};

const NIGHTLY_RUST_VERSION: &str = "nightly-2025-03-24";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the crate on crates.io
    crate_name: String,

    /// Optional version requirement (e.g., "1.0", "1", "~1.2.3", "*")
    #[arg(default_value = "*")]
    crate_version: String,

    /// Include prerelease versions when selecting the latest
    #[arg(long)]
    include_prerelease: bool,

    /// Output directory for crate documentation build artifacts
    #[arg(long, default_value = ".ai/docs/rust/build")]
    output_dir: String,

    /// Filter documented items by module path (e.g., "::style", "widgets::Button"). Can be specified multiple times.
    /// Paths starting with '::' imply the root of the current crate.
    /// Matches are prefix-based (e.g., "::style" matches "::style::TextStyle").
    #[arg(long = "path")]
    paths: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct CratesApiResponse {
    versions: Vec<CrateVersion>,
}

#[derive(Deserialize, Debug, Clone)]
struct CrateVersion {
    #[serde(rename = "crate")]
    crate_name: String,
    num: String, // Version number string
    yanked: bool,
    #[serde(skip)]
    semver: Option<Version>, // Parsed version, populated later
}

async fn find_best_version(
    client: &reqwest::Client,
    crate_name: &str,
    version_req_str: &str,
    include_prerelease: bool,
) -> Result<CrateVersion> {
    info!(
        "Fetching versions for crate '{}' from crates.io...",
        crate_name
    );
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
    let response = client.get(&url).send().await?.error_for_status()?;
    let mut api_data: CratesApiResponse = response
        .json()
        .await
        .context("Failed to parse JSON response from crates.io API")?;

    if api_data.versions.is_empty() {
        bail!("No versions found for crate '{}'", crate_name);
    }

    // Parse semver and filter out yanked versions
    api_data.versions.retain_mut(|v| {
        if v.yanked {
            debug!("Ignoring yanked version: {}", v.num);
            return false;
        }
        match Version::parse(&v.num) {
            Ok(sv) => {
                v.semver = Some(sv);
                true
            }
            Err(e) => {
                warn!("Failed to parse version '{}': {}", v.num, e);
                false // Ignore versions we can't parse
            }
        }
    });

    // Filter based on prerelease flag
    if !include_prerelease {
        api_data
            .versions
            .retain(|v| v.semver.as_ref().map_or(false, |sv| sv.pre.is_empty()));
    }

    // Sort remaining versions (highest first)
    api_data
        .versions
        .sort_unstable_by(|a, b| b.semver.cmp(&a.semver)); // descending

    if api_data.versions.is_empty() {
        bail!(
            "No suitable non-yanked{} versions found for crate '{}'",
            if include_prerelease { "" } else { " stable" },
            crate_name
        );
    }

    match version_req_str {
        "*" => {
            // Find the latest non-prerelease (unless include_prerelease is true)
            info!("No version specified, selecting latest suitable version...");
            api_data.versions.into_iter().next().ok_or_else(|| {
                anyhow!(
                    "Could not determine the latest{} version for crate '{}'",
                    if include_prerelease { "" } else { " stable" },
                    crate_name
                )
            })
        }
        req_str => {
            info!(
                "Finding best match for version requirement '{}'...",
                req_str
            );
            let req = VersionReq::parse(req_str)
                .with_context(|| format!("Invalid version requirement string: '{}'", req_str))?;

            api_data
                .versions
                .into_iter()
                .find(|v| v.semver.as_ref().map_or(false, |sv| req.matches(sv)))
                .ok_or_else(|| {
                    anyhow!(
                        "No version found matching requirement '{}' for crate '{}'",
                        req_str,
                        crate_name
                    )
                })
        }
    }
}

async fn download_and_unpack_crate(
    client: &reqwest::Client,
    krate: &CrateVersion,
    output_path: &FilePath,
) -> Result<PathBuf> {
    let crate_dir_name = format!("{}-{}", krate.crate_name, krate.num);
    let target_dir = output_path.join(crate_dir_name);

    if target_dir.exists() {
        info!(
            "Crate already downloaded and unpacked at: {}",
            target_dir.display()
        );
        return Ok(target_dir);
    }

    info!("Downloading {} version {}...", krate.crate_name, krate.num);
    let url = format!(
        "https://crates.io/api/v1/crates/{}/{}/download",
        krate.crate_name, krate.num
    );
    let response = client.get(&url).send().await?.error_for_status()?;

    let content = response.bytes().await?;
    let reader = Cursor::new(content);

    info!("Unpacking crate to: {}", target_dir.display());
    std::fs::create_dir_all(&target_dir)
        .with_context(|| format!("Failed to create directory: {}", target_dir.display()))?;

    let tar = GzDecoder::new(reader);
    let mut archive = Archive::new(tar);

    // Crate files are usually inside a directory like "crate_name-version/"
    let crate_dir_prefix = format!("{}-{}/", krate.crate_name, krate.num);

    for entry_result in archive.entries()? {
        let mut entry = entry_result?;
        let path = entry.path()?;

        // Ensure we extract only files within the expected subdirectory
        if path.starts_with(&crate_dir_prefix) {
            let relative_path = path.strip_prefix(&crate_dir_prefix)?;
            let dest_path = target_dir.join(relative_path);

            if entry.header().entry_type().is_dir() {
                std::fs::create_dir_all(&dest_path)?;
            } else {
                if let Some(parent) = dest_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                entry.unpack(&dest_path)?;
            }
        } else {
            debug!("Skipping entry outside expected crate dir: {:?}", path);
        }
    }

    info!("Unpacked to: {}", target_dir.display());
    Ok(target_dir)
}

fn run_rustdoc(crate_dir: &FilePath, crate_name: &str) -> Result<PathBuf> {
    let manifest_path = crate_dir.join("Cargo.toml");
    if !manifest_path.exists() {
        bail!(
            "Cargo.toml not found in unpacked crate at {}",
            manifest_path.display()
        );
    }

    info!("Generating rustdoc JSON using rustdoc-json crate...");

    let json_path = crate_dir
        .join("target/doc")
        .join(format!("{}.json", crate_name));

    // Avoid regenerating if exists
    if json_path.exists() {
        info!("rustdoc JSON already exists at: {}", json_path.display());
        return Ok(json_path);
    }

    let builder = Builder::default()
        .manifest_path(manifest_path)
        .toolchain(NIGHTLY_RUST_VERSION) // Specify the nightly toolchain
        .target_dir(crate_dir.join("target/doc")) // Set the output directory
        .package(crate_name) // Specify the package
        .all_features(true); // Enable all features to get max docs

    // Generate the JSON file
    match builder.build() {
        Ok(s) => {
            info!("Generated rustdoc JSON at: {}", s.display());
            Ok(s)
        }
        Err(e) => {
            // Attempt to read stderr if possible (rustdoc-json might not expose it easily)
            eprintln!("--- rustdoc-json build failed ---");
            eprintln!("{:?}", e); // Print the error itself

            // Try to read potential rustdoc output if the file exists but is invalid
            if json_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&json_path) {
                    eprintln!(
                        "\n--- Potential content of {}: ---\n{}",
                        json_path.display(),
                        content
                    );
                }
            }

            bail!("rustdoc-json failed: {}", e);
        }
    }
}

fn normalize_path(user_path: &str, _crate_name: &str, normalized_crate_name: &str) -> Vec<String> {
    let path = if user_path.starts_with("::") {
        format!("{}{}", normalized_crate_name, user_path)
    } else if !user_path.contains("::") && !user_path.is_empty() {
        // Assume single segment refers to top-level item in the crate
        format!("{}::{}", normalized_crate_name, user_path)
    } else {
        user_path.to_string() // Use as is if it contains '::' but doesn't start with it (e.g., external crate path)
    };
    path.split("::").map(|s| s.to_string()).collect()
}

fn path_matches(item_path: &[String], filter_path: &[String]) -> bool {
    if filter_path.len() > item_path.len() {
        return false; // Filter path is more specific than item path
    }
    item_path.starts_with(filter_path)
}

/// Gets the `Id` associated with a type, if it's a path-based type.
fn get_type_id(ty: &Type) -> Option<Id> {
    match ty {
        Type::ResolvedPath(p) => Some(p.id),
        Type::Generic(_) => None, // Generic types don't have a direct ID in this context
        Type::Primitive(_) => None,
        Type::FunctionPointer(_) => None, // Function pointers don't have an ID
        Type::Tuple(_) => None,
        Type::Slice(inner) => get_type_id(inner), // Look inside
        Type::Array { type_, .. } => get_type_id(type_), // Look inside
        Type::Pat { type_, .. } => get_type_id(type_), // Look inside
        Type::Infer => None,
        Type::RawPointer { type_, .. } => get_type_id(type_), // Look inside
        Type::BorrowedRef { type_, .. } => get_type_id(type_), // Look inside
        Type::QualifiedPath { trait_, .. } => trait_.as_ref().map(|p| p.id), // ID of the trait? Or self_type? Context dependent. Let's prioritize trait for now.
        Type::ImplTrait(_) => None,
        Type::DynTrait(_) => None,
    }
}

/// Finds all reachable `Id`s referenced within a `Type`.
fn find_type_dependencies(ty: &Type, krate: &Crate, dependencies: &mut HashSet<Id>) {
    // Add the direct ID if the type itself resolves to one
    if let Some(id) = get_type_id(ty) {
        // Check if the ID is part of the current crate before adding
        if krate.index.contains_key(&id) {
            dependencies.insert(id);
        }
    }

    // Recursively check inner types and generic arguments
    match ty {
        Type::ResolvedPath(Path { args, id, .. }) => {
            // Add the path's own ID
            if krate.index.contains_key(id) {
                dependencies.insert(*id);
            }
            // Check generic arguments
            if let Some(args) = args.as_ref() {
                // args is &Box<GenericArgs>, need to get &GenericArgs
                find_generic_args_dependencies(args, krate, dependencies);
            }
        }
        Type::Tuple(inner_types) => {
            for inner_ty in inner_types {
                find_type_dependencies(inner_ty, krate, dependencies);
            }
        }
        Type::Slice(inner_ty) => {
            find_type_dependencies(inner_ty, krate, dependencies);
        }
        Type::Array { type_, .. } => {
            find_type_dependencies(type_, krate, dependencies);
        }
        Type::Pat { type_, .. } => {
            find_type_dependencies(type_, krate, dependencies);
        }
        Type::RawPointer { type_, .. } => {
            find_type_dependencies(type_, krate, dependencies);
        }
        Type::BorrowedRef { type_, .. } => {
            find_type_dependencies(type_, krate, dependencies);
        }
        Type::QualifiedPath {
            args,
            self_type,
            trait_,
            ..
        } => {
            find_type_dependencies(self_type, krate, dependencies);
            if let Some(trait_path) = trait_ {
                if krate.index.contains_key(&trait_path.id) {
                    dependencies.insert(trait_path.id);
                }
            }
            match &**args {
                GenericArgs::AngleBracketed { args, constraints } => {
                    for arg in args {
                        match arg {
                            GenericArg::Type(t) => find_type_dependencies(t, krate, dependencies),
                            // Constant expr/value are stringly typed
                            GenericArg::Const(_) | GenericArg::Lifetime(_) | GenericArg::Infer => {}
                        }
                    }
                    for constraint in constraints {
                        // AssocItemConstraint { name: String, kind: AssocItemConstraintKind }
                        match constraint {
                            // Use tuple variant matching
                            rustdoc_types::AssocItemConstraint {
                                name: _,
                                args: _,
                                binding: rustdoc_types::AssocItemConstraintKind::Equality(term),
                            } => match term {
                                Term::Type(t) => find_type_dependencies(t, krate, dependencies),
                                // Constant expr/value are stringly typed
                                Term::Constant(_) => {}
                            },
                            rustdoc_types::AssocItemConstraint {
                                name: _,
                                args: _,
                                binding: rustdoc_types::AssocItemConstraintKind::Constraint(bounds),
                            } => {
                                for bound in bounds {
                                    find_generic_bound_dependencies(bound, krate, dependencies);
                                }
                            }
                        }
                    }
                }
                GenericArgs::Parenthesized { inputs, output } => {
                    for input in inputs {
                        find_type_dependencies(input, krate, dependencies);
                    }
                    if let Some(out) = output {
                        find_type_dependencies(out, krate, dependencies);
                    }
                }
                GenericArgs::ReturnTypeNotation => {}
            }
        }
        Type::DynTrait(dyn_trait) => {
            for poly_trait in &dyn_trait.traits {
                if krate.index.contains_key(&poly_trait.trait_.id) {
                    dependencies.insert(poly_trait.trait_.id);
                }
                // Check generic param defs within the poly trait
                for param_def in &poly_trait.generic_params {
                    find_generic_param_def_dependencies(param_def, krate, dependencies);
                }
            }
        }
        Type::ImplTrait(bounds) => {
            for bound in bounds {
                find_generic_bound_dependencies(bound, krate, dependencies);
            }
        }
        Type::FunctionPointer(fp) => {
            // generic_params are HRTBs for the pointer itself
            for param_def in &fp.generic_params {
                find_generic_param_def_dependencies(param_def, krate, dependencies);
            }
            // sig contains input/output types
            for (_name, input_type) in &fp.sig.inputs {
                find_type_dependencies(input_type, krate, dependencies);
            }
            if let Some(output) = &fp.sig.output {
                find_type_dependencies(output, krate, dependencies);
            }
        }
        // Types without complex inner structures or IDs
        Type::Generic(_) | Type::Primitive(_) | Type::Infer => {}
    }
}

fn find_generic_args_dependencies(
    args: &GenericArgs,
    krate: &Crate,
    dependencies: &mut HashSet<Id>,
) {
    match args {
        GenericArgs::AngleBracketed {
            args, constraints, ..
        } => {
            for arg in args {
                match arg {
                    GenericArg::Type(t) => find_type_dependencies(t, krate, dependencies),
                    GenericArg::Const(_) => {}
                    GenericArg::Lifetime(_) | GenericArg::Infer => {}
                }
            }
            for constraint in constraints {
                // AssocItemConstraint { name: String, kind: AssocItemConstraintKind }
                match constraint {
                    // Use tuple variant matching
                    rustdoc_types::AssocItemConstraint {
                        name: _,
                        args: _,
                        binding: rustdoc_types::AssocItemConstraintKind::Equality(term),
                    } => match term {
                        Term::Type(t) => find_type_dependencies(t, krate, dependencies),
                        Term::Constant(_) => {}
                    },
                    rustdoc_types::AssocItemConstraint {
                        name: _,
                        args: _,
                        binding: rustdoc_types::AssocItemConstraintKind::Constraint(bounds),
                    } => {
                        for bound in bounds {
                            find_generic_bound_dependencies(bound, krate, dependencies);
                        }
                    }
                }
            }
        }
        GenericArgs::Parenthesized { inputs, output, .. } => {
            for input in inputs {
                find_type_dependencies(input, krate, dependencies);
            }
            if let Some(out) = output {
                find_type_dependencies(out, krate, dependencies);
            }
        }
        GenericArgs::ReturnTypeNotation { .. } => {} // TODO: Handle this? T::method(..) - maybe the T part?
    }
}

fn find_generic_bound_dependencies(
    bound: &GenericBound,
    krate: &Crate,
    dependencies: &mut HashSet<Id>,
) {
    match bound {
        GenericBound::TraitBound {
            trait_, // This is a Path struct
            generic_params,
            ..
        } => {
            if krate.index.contains_key(&trait_.id) {
                dependencies.insert(trait_.id);
            }
            // Trait path itself might have generic args
            if let Some(args) = trait_.args.as_ref() {
                find_generic_args_dependencies(args, krate, dependencies);
            }
            // Check HRTBs (generic_params)
            for param_def in generic_params {
                find_generic_param_def_dependencies(param_def, krate, dependencies);
            }
        }
        GenericBound::Outlives(_) | GenericBound::Use(_) => {}
    }
}

fn find_generics_dependencies(generics: &Generics, krate: &Crate, dependencies: &mut HashSet<Id>) {
    for param in &generics.params {
        find_generic_param_def_dependencies(param, krate, dependencies);
    }
    for predicate in &generics.where_predicates {
        match predicate {
            WherePredicate::BoundPredicate {
                type_,
                bounds,
                generic_params, // HRTBs for the predicate
                ..
            } => {
                find_type_dependencies(type_, krate, dependencies);
                for bound in bounds {
                    find_generic_bound_dependencies(bound, krate, dependencies);
                }
                // Check HRTBs (generic_params)
                for param_def in generic_params {
                    find_generic_param_def_dependencies(param_def, krate, dependencies);
                }
            }
            WherePredicate::LifetimePredicate { .. } => {} // Lifetimes don't have IDs
            WherePredicate::EqPredicate { lhs, rhs, .. } => {
                find_type_dependencies(lhs, krate, dependencies);
                // rhs is Term
                match rhs {
                    Term::Type(t) => find_type_dependencies(t, krate, dependencies),
                    Term::Constant(_) => {} // Constant expr/value are stringly typed
                }
            }
        }
    }
}

fn find_generic_param_def_dependencies(
    param_def: &GenericParamDef,
    krate: &Crate,
    dependencies: &mut HashSet<Id>,
) {
    match &param_def.kind {
        rustdoc_types::GenericParamDefKind::Lifetime { .. } => {}
        rustdoc_types::GenericParamDefKind::Type {
            bounds, default, ..
        } => {
            for bound in bounds {
                find_generic_bound_dependencies(bound, krate, dependencies);
            }
            if let Some(ty) = default {
                find_type_dependencies(ty, krate, dependencies);
            }
        }
        rustdoc_types::GenericParamDefKind::Const { type_, .. } => {
            // Ignore default string
            find_type_dependencies(type_, krate, dependencies);
        }
    }
}

/// Selects items based on path filters and recursively includes their dependencies.
fn select_items(krate: &Crate, user_paths: &[String]) -> Result<HashSet<Id>> {
    let mut selected_ids: HashSet<Id> = HashSet::new();

    if user_paths.is_empty() {
        info!("No path filters specified, selecting all items.");
        selected_ids.extend(krate.index.keys().cloned());
        return Ok(selected_ids);
    }

    let root_item = krate
        .index
        .get(&krate.root)
        .ok_or_else(|| anyhow!("Crate root item not found in index"))?;
    let crate_name = root_item
        .name
        .as_ref()
        .ok_or_else(|| anyhow!("Crate root item has no name"))?;
    let normalized_crate_name = crate_name.replace('-', "_");

    let normalized_filters: Vec<Vec<String>> = user_paths
        .iter()
        .map(|p| normalize_path(p, crate_name, &normalized_crate_name))
        .collect();

    info!("Normalized path filters: {:?}", normalized_filters);

    // Initial selection based on paths
    for (id, item_summary) in &krate.paths {
        // We only care about items from the local crate for initial selection (crate_id 0)
        if item_summary.crate_id == 0 {
            let mut qualified_item_path = item_summary.path.clone();
            // Ensure the path starts with the crate name if it doesn't already
            if !qualified_item_path.is_empty() && qualified_item_path[0] != normalized_crate_name {
                qualified_item_path.insert(0, normalized_crate_name.clone());
            }

            for filter in &normalized_filters {
                if path_matches(&qualified_item_path, filter) {
                    debug!(
                        "Path filter {:?} matched item {:?} ({:?})",
                        filter, qualified_item_path, id
                    );
                    selected_ids.insert(*id);
                    break; // No need to check other filters for this item
                }
            }
        }
    }

    if selected_ids.is_empty() {
        warn!(
            "No items matched the provided path filters: {:?}",
            user_paths
        );
        return Ok(selected_ids); // Return empty set
    }

    info!(
        "Initially selected {} items based on path filters.",
        selected_ids.len()
    );

    // Iterative dependency selection
    let mut queue: VecDeque<Id> = selected_ids.iter().cloned().collect();
    let mut processed_for_deps = HashSet::new();

    while let Some(id) = queue.pop_front() {
        if !processed_for_deps.insert(id) {
            continue; // Already processed this item for dependencies
        }

        if let Some(item) = krate.index.get(&id) {
            let mut item_deps: HashSet<Id> = HashSet::new();

            // 1. Direct Links (value is Id)
            for (_link_text, link_id_val) in &item.links {
                // Check if link_id_val exists in krate.index before adding
                if krate.index.contains_key(link_id_val) {
                    item_deps.insert(*link_id_val);
                }
            }

            // 2. Item Kind Specific Dependencies
            match &item.inner {
                ItemEnum::Module(m) => {
                    item_deps.extend(
                        m.items
                            .iter()
                            .filter(|dep_id| krate.index.contains_key(dep_id))
                            .cloned(),
                    );
                }
                ItemEnum::Struct(s) => {
                    item_deps.extend(
                        s.impls
                            .iter()
                            .filter(|dep_id| krate.index.contains_key(dep_id))
                            .cloned(),
                    );
                    find_generics_dependencies(&s.generics, krate, &mut item_deps);
                    // Find deps in fields (StructKind can be Plain, Tuple, Unit)
                    match &s.kind {
                        rustdoc_types::StructKind::Plain { fields, .. } => {
                            // fields_stripped ignored
                            for field_id in fields {
                                if krate.index.contains_key(field_id) {
                                    item_deps.insert(*field_id);
                                    // Also get dependencies of the field's type
                                    if let Some(field_item) = krate.index.get(field_id) {
                                        if let ItemEnum::StructField(field_type) = &field_item.inner
                                        {
                                            find_type_dependencies(
                                                field_type,
                                                krate,
                                                &mut item_deps,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        rustdoc_types::StructKind::Tuple(fields) => {
                            // fields_stripped ignored here
                            // fields is Vec<Option<Id>>
                            for field_id in fields.iter().filter_map(|opt_id| opt_id.as_ref()) {
                                if krate.index.contains_key(field_id) {
                                    item_deps.insert(*field_id);
                                    // Also get dependencies of the field's type
                                    if let Some(field_item) = krate.index.get(field_id) {
                                        if let ItemEnum::StructField(field_type) = &field_item.inner
                                        {
                                            find_type_dependencies(
                                                field_type,
                                                krate,
                                                &mut item_deps,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        rustdoc_types::StructKind::Unit => {}
                    }
                }
                ItemEnum::Enum(e) => {
                    item_deps.extend(
                        e.variants
                            .iter()
                            .filter(|dep_id| krate.index.contains_key(dep_id))
                            .cloned(),
                    );
                    item_deps.extend(
                        e.impls
                            .iter()
                            .filter(|dep_id| krate.index.contains_key(dep_id))
                            .cloned(),
                    );
                    find_generics_dependencies(&e.generics, krate, &mut item_deps);
                }
                ItemEnum::Variant(v) => {
                    match &v.kind {
                        rustdoc_types::VariantKind::Plain => {}
                        rustdoc_types::VariantKind::Tuple(fields) => {
                            // fields is Vec<Option<Id>>
                            for field_id in fields.iter().filter_map(|opt_id| opt_id.as_ref()) {
                                if krate.index.contains_key(field_id) {
                                    item_deps.insert(*field_id);
                                    if let Some(field_item) = krate.index.get(field_id) {
                                        if let ItemEnum::StructField(field_type) = &field_item.inner
                                        {
                                            find_type_dependencies(
                                                field_type,
                                                krate,
                                                &mut item_deps,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        rustdoc_types::VariantKind::Struct { fields, .. } => {
                            // fields_stripped ignored
                            // fields is Vec<Id>
                            for field_id in fields {
                                if krate.index.contains_key(field_id) {
                                    item_deps.insert(*field_id);
                                    if let Some(field_item) = krate.index.get(field_id) {
                                        if let ItemEnum::StructField(field_type) = &field_item.inner
                                        {
                                            find_type_dependencies(
                                                field_type,
                                                krate,
                                                &mut item_deps,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if let Some(disr) = &v.discriminant {
                        // Discriminant has expr and value (strings), no direct type dependency ID
                        let _ = disr; // Avoid unused warning
                    }
                }
                ItemEnum::Function(f) => {
                    find_generics_dependencies(&f.generics, krate, &mut item_deps);
                    for (_name, param_type) in &f.sig.inputs {
                        find_type_dependencies(param_type, krate, &mut item_deps);
                    }
                    if let Some(output) = &f.sig.output {
                        find_type_dependencies(output, krate, &mut item_deps);
                    }
                }
                ItemEnum::Trait(t) => {
                    item_deps.extend(
                        t.items
                            .iter()
                            .filter(|dep_id| krate.index.contains_key(dep_id))
                            .cloned(),
                    );
                    find_generics_dependencies(&t.generics, krate, &mut item_deps);
                    // Also consider supertraits (t.bounds -> Vec<GenericBound>)
                    for bound in &t.bounds {
                        find_generic_bound_dependencies(bound, krate, &mut item_deps);
                    }
                    // Also consider t.implementations Vec<Id> ? Usually external... filter?
                    item_deps.extend(
                        t.implementations
                            .iter()
                            .filter(|dep_id| krate.index.contains_key(dep_id))
                            .cloned(),
                    );
                }
                ItemEnum::Impl(imp) => {
                    item_deps.extend(
                        imp.items
                            .iter()
                            .filter(|dep_id| krate.index.contains_key(dep_id))
                            .cloned(),
                    );
                    if let Some(trait_path) = &imp.trait_ {
                        // trait_path is Path
                        if krate.index.contains_key(&trait_path.id) {
                            item_deps.insert(trait_path.id);
                        }
                        // Check trait generics too
                        if let Some(args) = trait_path.args.as_ref() {
                            find_generic_args_dependencies(args, krate, &mut item_deps);
                        }
                    }
                    find_type_dependencies(&imp.for_, krate, &mut item_deps);
                    find_generics_dependencies(&imp.generics, krate, &mut item_deps);
                }
                ItemEnum::TypeAlias(ta) => {
                    find_type_dependencies(&ta.type_, krate, &mut item_deps);
                    find_generics_dependencies(&ta.generics, krate, &mut item_deps);
                }
                // Use struct pattern matching for Constant
                ItemEnum::Constant { type_, .. } => {
                    find_type_dependencies(type_, krate, &mut item_deps);
                    // Maybe parse expr/value for IDs? Complex.
                }
                ItemEnum::Static(s) => {
                    find_type_dependencies(&s.type_, krate, &mut item_deps);
                }
                ItemEnum::ExternType => {}   // No inner types
                ItemEnum::Macro(_) => {}     // Source string, hard to parse reliably
                ItemEnum::ProcMacro(_) => {} // No direct code dependencies representable by ID
                ItemEnum::Primitive(_) => {} // No dependencies
                // Use correct fields for AssocConst { type_, value }
                ItemEnum::AssocConst { type_, value: _ } => {
                    // Ignore default string (value)
                    find_type_dependencies(type_, krate, &mut item_deps);
                }
                // Use renamed field type_ (was default)
                ItemEnum::AssocType {
                    generics,
                    bounds,
                    type_, // Renamed from default
                    ..
                } => {
                    find_generics_dependencies(generics, krate, &mut item_deps);
                    for bound in bounds {
                        find_generic_bound_dependencies(bound, krate, &mut item_deps);
                    }
                    if let Some(def_type) = type_ {
                        find_type_dependencies(def_type, krate, &mut item_deps);
                    }
                }
                ItemEnum::Union(u) => {
                    find_generics_dependencies(&u.generics, krate, &mut item_deps);
                    item_deps.extend(
                        u.fields
                            .iter()
                            .filter(|dep_id| krate.index.contains_key(dep_id))
                            .cloned(),
                    );
                    item_deps.extend(
                        u.impls
                            .iter()
                            .filter(|dep_id| krate.index.contains_key(dep_id))
                            .cloned(),
                    );
                    for field_id in &u.fields {
                        if krate.index.contains_key(field_id) {
                            if let Some(field_item) = krate.index.get(field_id) {
                                if let ItemEnum::StructField(field_type) = &field_item.inner {
                                    find_type_dependencies(field_type, krate, &mut item_deps);
                                }
                            }
                        }
                    }
                }
                ItemEnum::TraitAlias(ta) => {
                    find_generics_dependencies(&ta.generics, krate, &mut item_deps);
                    for bound in &ta.params {
                        find_generic_bound_dependencies(bound, krate, &mut item_deps);
                    }
                }
                ItemEnum::StructField(ty) => find_type_dependencies(ty, krate, &mut item_deps),
                // Use renamed variant Use (was Import)
                ItemEnum::ExternCrate { .. } | ItemEnum::Use { .. } => {} // Ignore these for dep finding
            }

            // Add newly found dependencies to the queue if they aren't already selected
            for dep_id in item_deps {
                // Check if dep_id exists in krate.index before adding
                if krate.index.contains_key(&dep_id) && selected_ids.insert(dep_id) {
                    debug!("Adding dependency {:?} from item {:?}", dep_id, id);
                    queue.push_back(dep_id);
                }
            }
        }
    }

    info!(
        "Selected {} items after including dependencies.",
        selected_ids.len()
    );
    Ok(selected_ids)
}

// --- Formatting Helpers ---

fn format_id_path(id: &Id, krate: &Crate) -> String {
    krate
        .paths
        .get(id)
        .map(|p| p.path.join("::"))
        .unwrap_or_else(|| format!("{{unknown_id:{:?}}}", id))
}

fn format_path(path: &Path, krate: &Crate) -> String {
    let base_path = format_id_path(&path.id, krate);
    // Use as_ref() to get Option<&GenericArgs> from Option<Box<GenericArgs>>
    if let Some(args) = path.args.as_ref() {
        let args_str = format_generic_args(args, krate, true); // Angle brackets only
        if !args_str.is_empty() {
            format!("{}<{}>", base_path, args_str)
        } else {
            base_path
        }
    } else {
        base_path
    }
}

fn format_poly_trait(poly_trait: &PolyTrait, krate: &Crate) -> String {
    let hrtb = if poly_trait.generic_params.is_empty() {
        "".to_string()
    } else {
        format!(
            "for<{}> ",
            poly_trait
                .generic_params
                .iter()
                .map(|p| format_generic_param_def(p, krate)) // Format full param def
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    format!("{}{}", hrtb, format_path(&poly_trait.trait_, krate)) // Use format_path for the Path struct
}

fn format_type(ty: &Type, krate: &Crate) -> String {
    match ty {
        Type::ResolvedPath(p) => format_path(p, krate),
        Type::DynTrait(dt) => {
            let lifetime_bound = dt
                .lifetime
                .as_ref()
                .map(|lt| format!(" + '{}'", lt)) // Add quote for lifetime
                .unwrap_or_default();
            format!(
                "dyn {}{}",
                dt.traits
                    .iter()
                    .map(|pt| format_poly_trait(pt, krate))
                    .collect::<Vec<_>>()
                    .join(" + "),
                lifetime_bound
            )
        }
        Type::Generic(name) => name.clone(),
        Type::Primitive(name) => name.clone(),
        Type::FunctionPointer(fp) => {
            let hrtb = if fp.generic_params.is_empty() {
                "".to_string()
            } else {
                format!(
                    "for<{}> ",
                    fp.generic_params
                        .iter()
                        .map(|p| format_generic_param_def(p, krate))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            let abi = if !matches!(fp.header.abi, Abi::Rust) {
                format!("extern \"{:?}\" ", fp.header.abi) // Use Debug for Abi for now
            } else {
                "".to_string()
            };
            let unsafe_kw = if fp.header.is_unsafe { "unsafe " } else { "" };
            format!(
                "{}{}{}fn({}){}",
                hrtb,
                unsafe_kw,
                abi,
                fp.sig
                    .inputs
                    .iter()
                    .map(|(_name, type_)| format_type(type_, krate)) // Ignore name pattern for now
                    .collect::<Vec<_>>()
                    .join(", "),
                fp.sig
                    .output
                    .as_ref()
                    .map(|t| format!(" -> {}", format_type(t, krate)))
                    .unwrap_or_default()
            )
        }
        Type::Tuple(types) => {
            // Special case for empty tuple
            if types.is_empty() {
                "()".to_string()
            } else {
                format!(
                    "({})",
                    types
                        .iter()
                        .map(|t| format_type(t, krate))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
        Type::Slice(inner) => format!("[{}]", format_type(inner, krate)),
        Type::Array { type_, len } => format!("[{}; {}]", format_type(type_, krate), len),
        Type::Pat { type_, .. } => format!("pat {}", format_type(type_, krate)), // Placeholder
        Type::ImplTrait(bounds) => {
            format!(
                "impl {}",
                bounds
                    .iter()
                    .map(|b| format_generic_bound(b, krate))
                    .collect::<Vec<_>>()
                    .join(" + ")
            )
        }
        Type::Infer => "_".to_string(),
        Type::RawPointer { is_mutable, type_ } => {
            format!(
                "*{}{}",
                if *is_mutable { "mut " } else { "const " },
                format_type(type_, krate)
            )
        }
        Type::BorrowedRef {
            lifetime,
            is_mutable,
            type_,
        } => format!(
            "&{}{}{}",
            lifetime
                .as_ref()
                .map(|lt| format!("'{} ", lt)) // Add quote
                .unwrap_or_default(),
            if *is_mutable { "mut " } else { "" },
            format_type(type_, krate)
        ),
        Type::QualifiedPath {
            name,
            args,
            self_type,
            trait_,
        } => {
            let self_type_str = format_type(self_type, krate);
            let trait_str = trait_
                .as_ref()
                .map(|t| format_path(t, krate)) // Use format_path
                .unwrap_or("_".to_string());
            // Args here are for the associated type, not the trait bound
            let args_str = format_generic_args(args, krate, true); // Angle brackets only

            format!(
                "<{} as {}>::{}{}",
                self_type_str,
                trait_str,
                name,
                if args_str.is_empty() {
                    "".to_string()
                } else {
                    format!("<{}>", args_str)
                }
            )
        }
    }
}

fn format_generic_args(args: &GenericArgs, krate: &Crate, angle_brackets_only: bool) -> String {
    match args {
        // Use renamed field constraints
        GenericArgs::AngleBracketed {
            args, constraints, ..
        } => {
            let arg_strs: Vec<String> = args.iter().map(|a| format_generic_arg(a, krate)).collect();
            let constraint_strs: Vec<String> = constraints
                .iter()
                .map(|c| match c {
                    // Use tuple variant matching
                    rustdoc_types::AssocItemConstraint {
                        name,
                        args: assoc_args, // these are args for the assoc item constraint itself
                        binding: rustdoc_types::AssocItemConstraintKind::Equality(term),
                    } => {
                        let assoc_args_str = format_generic_args(assoc_args, krate, true);
                        format!(
                            "{}{}{}{} = {}", // Fixed: Added 5th placeholder
                            name,
                            if assoc_args_str.is_empty() { "" } else { "<" },
                            assoc_args_str,
                            if assoc_args_str.is_empty() { "" } else { ">" },
                            format_term(term, krate)
                        )
                    }
                    rustdoc_types::AssocItemConstraint {
                        name,
                        args: assoc_args,
                        binding: rustdoc_types::AssocItemConstraintKind::Constraint(bounds),
                    } => {
                        let assoc_args_str = format_generic_args(assoc_args, krate, true);
                        format!(
                            "{}{}{}{}: {}", // Fixed: Added 5th placeholder
                            name,
                            if assoc_args_str.is_empty() { "" } else { "<" },
                            assoc_args_str,
                            if assoc_args_str.is_empty() { "" } else { ">" },
                            bounds
                                .iter()
                                .map(|bnd| format_generic_bound(bnd, krate))
                                .collect::<Vec<_>>()
                                .join(" + ")
                        )
                    }
                })
                .collect();
            let mut all_strs = arg_strs;
            all_strs.extend(constraint_strs);
            all_strs.join(", ")
        }
        GenericArgs::Parenthesized { inputs, output, .. } => {
            if angle_brackets_only {
                "".to_string() // Don't format Fn() args when angle brackets are expected
            } else {
                // Format like function signature inputs/output
                format!(
                    "({}) -> {}",
                    inputs
                        .iter()
                        .map(|t| format_type(t, krate))
                        .collect::<Vec<_>>()
                        .join(", "),
                    output
                        .as_ref()
                        .map_or("()".to_string(), |t| format_type(t, krate))
                )
            }
        }
        GenericArgs::ReturnTypeNotation { .. } => "".to_string(), // How to format T::method(..) args? Ignore for now.
    }
}

fn format_const_expr(constant: &Constant) -> String {
    // Prefer `value` if present and different, otherwise use `expr`
    if let Some(v) = &constant.value {
        if v != &constant.expr {
            return format!("{} /* = {} */", constant.expr, v);
        }
    }
    constant.expr.clone()
}

/// Formats a discriminant expression, potentially showing the value if different.
fn format_discriminant_expr(discr: &Discriminant) -> String {
    if discr.value != discr.expr {
        format!("{} /* = {} */", discr.expr, discr.value)
    } else {
        discr.expr.clone()
    }
}

fn format_generic_arg(arg: &GenericArg, krate: &Crate) -> String {
    match arg {
        GenericArg::Lifetime(lt) => format!("'{}", lt), // Add quote
        GenericArg::Type(ty) => format_type(ty, krate),
        GenericArg::Const(c) => format_const_expr(c),
        GenericArg::Infer => "_".to_string(),
    }
}

fn format_generic_bound(bound: &GenericBound, krate: &Crate) -> String {
    match bound {
        GenericBound::TraitBound {
            trait_,         // Path struct
            generic_params, // HRTBs
            modifier,
            ..
        } => {
            let hrtb = if generic_params.is_empty() {
                "".to_string()
            } else {
                format!(
                    "for<{}> ",
                    generic_params
                        .iter()
                        .map(|p| format_generic_param_def(p, krate)) // Format full param def
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            let mod_str = match modifier {
                rustdoc_types::TraitBoundModifier::None => "",
                rustdoc_types::TraitBoundModifier::Maybe => "?",
                rustdoc_types::TraitBoundModifier::MaybeConst => "?const ", // Note the space
            };
            format!("{}{}{}", hrtb, mod_str, format_path(trait_, krate)) // Use format_path
        }
        GenericBound::Outlives(lifetime) => format!("'{}", lifetime), // Add quote
        GenericBound::Use(args) => {
            // use<'a, T> syntax
            format!(
                "use<{}>",
                args.iter()
                    .map(|a| match a {
                        rustdoc_types::PreciseCapturingArg::Lifetime(lt) => format!("'{}", lt),
                        rustdoc_types::PreciseCapturingArg::Param(id) => id.to_string(), // TODO: This ID might need resolving? Using raw name for now.
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}

fn format_term(term: &Term, krate: &Crate) -> String {
    match term {
        Term::Type(t) => format_type(t, krate),
        Term::Constant(c) => format_const_expr(c),
    }
}

fn format_generic_param_def(p: &GenericParamDef, krate: &Crate) -> String {
    match &p.kind {
        rustdoc_types::GenericParamDefKind::Lifetime { .. } => format!("'{}", p.name), // Add quote
        rustdoc_types::GenericParamDefKind::Type {
            bounds,
            default,
            is_synthetic, // Renamed from synthetic
            ..
        } => {
            format!(
                "{}{}{}{}",
                if *is_synthetic { "impl " } else { "" },
                p.name,
                if bounds.is_empty() {
                    "".to_string()
                } else {
                    format!(
                        ": {}",
                        bounds
                            .iter()
                            .map(|b| format_generic_bound(b, krate))
                            .collect::<Vec<_>>()
                            .join(" + ")
                    )
                },
                default
                    .as_ref()
                    .map(|t| format!(" = {}", format_type(t, krate)))
                    .unwrap_or_default()
            )
        }
        rustdoc_types::GenericParamDefKind::Const { type_, default, .. } => {
            format!(
                "const {}: {}{}",
                p.name,
                format_type(type_, krate),
                default
                    .as_deref()
                    .map(|d| format!(" = {}", d))
                    .unwrap_or_default()
            )
        }
    }
}

// Formats generics like <T: Bound> where T: OtherBound
fn format_generics_full(generics: &Generics, krate: &Crate) -> String {
    if generics.params.is_empty() && generics.where_predicates.is_empty() {
        return String::new();
    }

    let mut s = String::new();
    if !generics.params.is_empty() {
        s.push('<');
        let params_str = generics
            .params
            .iter()
            .map(|p| format_generic_param_def(p, krate))
            .collect::<Vec<_>>()
            .join(", ");
        s.push_str(&params_str);
        s.push('>');
    }

    if !generics.where_predicates.is_empty() {
        let where_clause = format_generics_where_only(&generics.where_predicates, krate);
        // Add newline and indent if params were also present and where clause is multiline
        if !generics.params.is_empty() && where_clause.contains('\n') {
            write!(s, "\n  {}", where_clause).unwrap();
        } else {
            write!(s, " {}", where_clause).unwrap(); // Append single line where clause or first line of multiline
        }
    }

    s
}

// Formats generics like <T: Bound>
fn format_generics_params_only(params: &[GenericParamDef], krate: &Crate) -> String {
    if params.is_empty() {
        return String::new();
    }
    format!(
        "<{}>",
        params
            .iter()
            .map(|p| format_generic_param_def(p, krate))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

// Formats only the where clause: "where T: Bound" or multi-line
fn format_generics_where_only(predicates: &[WherePredicate], krate: &Crate) -> String {
    if predicates.is_empty() {
        return String::new();
    }
    let clauses: Vec<String> = predicates
        .iter()
        .map(|p| match p {
            WherePredicate::BoundPredicate {
                type_,
                bounds,
                generic_params,
                ..
            } => {
                let hrtb = if generic_params.is_empty() {
                    "".to_string()
                } else {
                    format!(
                        "for<{}> ",
                        generic_params
                            .iter()
                            .map(|gp| format_generic_param_def(gp, krate))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                };
                format!(
                    "{}{}: {}",
                    hrtb,
                    format_type(type_, krate),
                    bounds
                        .iter()
                        .map(|b| format_generic_bound(b, krate))
                        .collect::<Vec<_>>()
                        .join(" + ")
                )
            }
            WherePredicate::LifetimePredicate {
                lifetime, outlives, ..
            } => {
                format!(
                    "'{}: {}",
                    lifetime,
                    outlives
                        .iter()
                        .map(|lt| format!("'{}", lt)) // Add quotes
                        .collect::<Vec<_>>()
                        .join(" + ")
                )
            }
            WherePredicate::EqPredicate { lhs, rhs, .. } => {
                format!("{} == {}", format_type(lhs, krate), format_term(rhs, krate))
            }
        })
        .collect();

    // Determine if multi-line formatting is needed
    let total_len = clauses.iter().map(|s| s.len()).sum::<usize>();
    let is_multiline = clauses.len() > 1 || total_len > 60; // Heuristic for multi-line

    if is_multiline {
        format!("where\n    {}", clauses.join(",\n    ")) // Indent contents
    } else {
        format!("where {}", clauses.join(", "))
    }
}

// --- Structured Printing Logic ---

/// Generates the primary declaration string for an item (e.g., `struct Foo`, `fn bar()`).
/// For functions, this is deliberately simplified (no attrs, no where clause).
fn generate_item_declaration(item: &Item, krate: &Crate) -> String {
    let name = item.name.as_deref().unwrap_or("{unnamed}");
    match &item.inner {
        ItemEnum::Struct(s) => format!(
            "struct {}{}",
            name,
            format_generics_params_only(&s.generics.params, krate)
        ),
        ItemEnum::Enum(e) => format!(
            "enum {}{}",
            name,
            format_generics_params_only(&e.generics.params, krate)
        ),
        ItemEnum::Union(u) => format!(
            "union {}{}",
            name,
            format_generics_params_only(&u.generics.params, krate)
        ),
        ItemEnum::Trait(t) => {
            let unsafe_kw = if t.is_unsafe { "unsafe " } else { "" };
            let auto = if t.is_auto { "auto " } else { "" };
            // Include param generics in trait header
            format!(
                "{}{}{}{}",
                auto,
                unsafe_kw,
                "trait ",
                name,
                format_generics_params_only(&t.generics.params, krate)
            )
        }
        ItemEnum::Function(f) => {
            // Simplified version for the header: no attrs, no where clause
            let mut decl = String::new();
            write!(decl, "fn {}", name).unwrap();
            // Include only param generics here
            write!(
                decl,
                "{}",
                format_generics_params_only(&f.generics.params, krate)
            )
            .unwrap();
            write!(decl, "(").unwrap();
            let args_str = f
                .sig
                .inputs
                .iter()
                .map(|(n, t)| format!("{}: {}", n, format_type(t, krate))) // Use arg name from tuple
                .collect::<Vec<_>>()
                .join(", ");
            write!(decl, "{}", args_str).unwrap();
            if f.sig.is_c_variadic {
                write!(decl, ", ...").unwrap();
            }
            write!(decl, ")").unwrap();
            if let Some(output_type) = &f.sig.output {
                write!(decl, " -> {}", format_type(output_type, krate)).unwrap();
            }
            decl
        }
        ItemEnum::TypeAlias(ta) => format!(
            "type {}{}",
            name,
            format_generics_params_only(&ta.generics.params, krate)
        ),
        ItemEnum::TraitAlias(ta) => format!(
            "trait {}{}",
            name,
            format_generics_params_only(&ta.generics.params, krate)
        ),
        ItemEnum::Constant { .. } => format!("const {}", name), // Type/value in code block
        ItemEnum::Static(s) => format!("static {}{}", if s.is_mutable { "mut " } else { "" }, name),
        ItemEnum::Macro(_) => format!("macro {}!", name),
        ItemEnum::ProcMacro(pm) => {
            let kind_str = match pm.kind {
                rustdoc_types::MacroKind::Bang => "!",
                rustdoc_types::MacroKind::Attr => "#[]",
                rustdoc_types::MacroKind::Derive => "#[derive]",
            };
            format!("proc_macro {}{}", name, kind_str)
        }
        ItemEnum::Primitive(_) => format!("primitive {}", name),
        ItemEnum::Module(_) => format!("mod {}", name),
        ItemEnum::ExternCrate {
            name: crate_name, ..
        } => format!("extern crate {}", crate_name),
        ItemEnum::Use(u) => {
            // TODO: format Use statement better
            format!("use {}", u.name)
        }
        ItemEnum::ExternType => format!("extern type {}", name),
        ItemEnum::Variant(v) => format_variant_signature(item, v, krate), // Use helper
        ItemEnum::StructField(_) => name.to_string(), // Field name only for header
        ItemEnum::AssocConst { .. } => format!("const {}", name),
        ItemEnum::AssocType { .. } => format!("type {}", name),
        ItemEnum::Impl(_) => "impl".to_string(), // Impls handled specially
    }
}

/// Generates the `struct { ... }` code block.
fn generate_struct_code_block(item: &Item, s: &Struct, krate: &Crate) -> String {
    // Fixed: Use item.name instead of s.name
    let name = item
        .name
        .as_deref()
        .expect("Struct item should have a name");
    let mut code = String::new();
    write!(code, "pub struct {}", name).unwrap();
    // Use full generics here, including where clause
    write!(code, "{}", format_generics_full(&s.generics, krate)).unwrap();

    match &s.kind {
        StructKind::Plain { fields, .. } => {
            // fields_stripped ignored
            // Check if generics caused a newline before deciding where to put opening brace
            if code.ends_with('>') {
                writeln!(code, " {{").unwrap(); // Generics on same line
            } else {
                write!(code, " {{").unwrap(); // No generics or multiline generics
            }
            for field_id in fields {
                if let Some(field_item) = krate.index.get(field_id) {
                    if let ItemEnum::StructField(field_type) = &field_item.inner {
                        let field_name = field_item.name.as_deref().unwrap_or("_");
                        writeln!(
                            code,
                            "    pub {}: {},",
                            field_name,
                            format_type(field_type, krate)
                        )
                        .unwrap();
                    }
                }
            }
            write!(code, "}}").unwrap();
        }
        StructKind::Tuple(fields) => {
            // fields_stripped ignored
            write!(code, "(").unwrap();
            let field_types: Vec<String> = fields
                .iter()
                .filter_map(|opt_id| {
                    opt_id
                        .as_ref()
                        .and_then(|id| krate.index.get(id))
                        .and_then(|field_item| {
                            if let ItemEnum::StructField(field_type) = &field_item.inner {
                                Some(format!("pub {}", format_type(field_type, krate)))
                            } else {
                                None
                            }
                        })
                })
                .collect();
            write!(code, "{}", field_types.join(", ")).unwrap();
            // Add semicolon only if where clause didn't add one implicitly via multiline format
            if !code.ends_with("where") && !code.contains("where\n") {
                write!(code, ");").unwrap();
            }
        }
        StructKind::Unit => {
            // Add semicolon only if where clause didn't add one implicitly
            if !code.ends_with("where") && !code.contains("where\n") {
                write!(code, ";").unwrap();
            }
        }
    }
    code
}

/// Generates the `enum { ... }` code block.
fn generate_enum_code_block(item: &Item, e: &Enum, krate: &Crate) -> String {
    // Fixed: Use item.name instead of e.name
    let name = item.name.as_deref().expect("Enum item should have a name");
    let mut code = String::new();
    write!(code, "pub enum {}", name).unwrap();
    write!(code, "{}", format_generics_full(&e.generics, krate)).unwrap();
    // Check if generics caused a newline before deciding where to put opening brace
    if code.ends_with('>') || code.contains("where\n") {
        writeln!(code, " {{").unwrap();
    } else {
        write!(code, " {{").unwrap();
    }

    for variant_id in &e.variants {
        if let Some(variant_item) = krate.index.get(variant_id) {
            if let ItemEnum::Variant(variant_data) = &variant_item.inner {
                write!(
                    code,
                    "    {}",
                    format_variant_definition(variant_item, variant_data, krate)
                )
                .unwrap();
                // Add discriminant if present
                if let Some(discr) = &variant_data.discriminant {
                    // Use format_discriminant_expr for discriminant
                    write!(code, " = {}", format_discriminant_expr(discr)).unwrap();
                }
                writeln!(code, ",").unwrap();
            }
        }
    }

    write!(code, "}}").unwrap();
    code
}

/// Generates the full trait declaration code block.
fn generate_trait_code_block(item: &Item, t: &Trait, krate: &Crate) -> String {
    // Fixed: Use item.name instead of t.name
    let name = item.name.as_deref().expect("Trait item should have a name");
    let mut code = String::new();

    if t.is_auto {
        write!(code, "pub auto ").unwrap();
    }
    if t.is_unsafe {
        write!(code, "pub unsafe ").unwrap();
    } else if !t.is_auto {
        // Add pub if not auto or unsafe (which imply pub sometimes)
        write!(code, "pub ").unwrap();
    }
    write!(code, "trait {}", name).unwrap();
    // Add generics params and supertraits (bounds)
    write!(
        code,
        "{}",
        format_generics_params_only(&t.generics.params, krate)
    )
    .unwrap();
    if !t.bounds.is_empty() {
        write!(
            code,
            ": {}",
            t.bounds
                .iter()
                .map(|b| format_generic_bound(b, krate))
                .collect::<Vec<_>>()
                .join(" + ")
        )
        .unwrap();
    }
    // Add where clause
    let where_clause = format_generics_where_only(&t.generics.where_predicates, krate);
    if !where_clause.is_empty() {
        if where_clause.contains('\n') {
            write!(code, "\n  {}", where_clause).unwrap(); // Multiline where
        } else {
            write!(code, " {}", where_clause).unwrap(); // Single line where
        }
    }

    // Body
    if t.items.is_empty() {
        write!(code, " {{}}").unwrap();
    } else {
        writeln!(code, " {{").unwrap();
        // Print associated items (simple versions)
        for item_id in &t.items {
            if let Some(assoc_item) = krate.index.get(item_id) {
                match &assoc_item.inner {
                    ItemEnum::AssocConst { type_, value, .. } => {
                        write!(
                            code,
                            "    const {}: {}",
                            assoc_item.name.as_deref().unwrap_or("_"),
                            format_type(type_, krate)
                        )
                        .unwrap();
                        if let Some(val) = value {
                            write!(code, " = {};", val).unwrap(); // Use raw default string
                        } else {
                            write!(code, ";").unwrap();
                        }
                        writeln!(code).unwrap();
                    }
                    ItemEnum::AssocType { bounds, type_, .. } => {
                        write!(
                            code,
                            "    type {}",
                            assoc_item.name.as_deref().unwrap_or("_")
                        )
                        .unwrap();
                        if !bounds.is_empty() {
                            write!(
                                code,
                                ": {}",
                                bounds
                                    .iter()
                                    .map(|b| format_generic_bound(b, krate))
                                    .collect::<Vec<_>>()
                                    .join(" + ")
                            )
                            .unwrap();
                        }
                        if let Some(ty) = type_ {
                            write!(code, " = {};", format_type(ty, krate)).unwrap();
                        } else {
                            write!(code, ";").unwrap();
                        }
                        writeln!(code).unwrap();
                    }
                    ItemEnum::Function(f) => {
                        // Print simple function signature within trait def
                        writeln!(
                            code,
                            "    {};",
                            generate_function_code_block(assoc_item, f, krate)
                        )
                        .unwrap();
                    }
                    _ => {} // Ignore others
                }
            }
        }
        write!(code, "}}").unwrap();
    }
    code
}

/// Generates the full function signature for a code block.
fn generate_function_code_block(item: &Item, f: &Function, krate: &Crate) -> String {
    // Fixed: Use item.name instead of f.name
    let name = item.name.as_deref().expect("Function should have a name");
    let mut code = String::new();

    // Attributes/Keywords
    // TODO: Add visibility? Assume pub for now.
    write!(code, "pub ").unwrap();
    if f.header.is_const {
        write!(code, "const ").unwrap();
    }
    if f.header.is_async {
        write!(code, "async ").unwrap();
    }
    if f.header.is_unsafe {
        write!(code, "unsafe ").unwrap();
    }
    if !matches!(f.header.abi, Abi::Rust) {
        write!(code, "extern \"{:?}\" ", f.header.abi).unwrap(); // Use Debug for Abi
    }

    // Core signature
    write!(code, "fn {}", name).unwrap();
    // Include full generics here, including where clause
    write!(code, "{}", format_generics_full(&f.generics, krate)).unwrap();

    // Parameters
    write!(code, "(").unwrap();
    let args_str = f
        .sig
        .inputs
        .iter()
        .map(|(n, t)| format!("{}: {}", n, format_type(t, krate))) // Use name from tuple
        .collect::<Vec<_>>()
        .join(", ");
    write!(code, "{}", args_str).unwrap();
    if f.sig.is_c_variadic {
        write!(code, ", ...").unwrap();
    }
    write!(code, ")").unwrap();

    // Return type
    if let Some(output_type) = &f.sig.output {
        write!(code, " -> {}", format_type(output_type, krate)).unwrap();
    }

    // Add semicolon or body indicator based on if it has implementation
    if f.has_body {
        // Check if where clause ended up on a newline
        if code.ends_with("where") || code.contains("where\n") {
            write!(code, " {{ ... }}").unwrap(); // Body on next line after multi-line where
        } else {
            write!(code, " {{ ... }}").unwrap(); // Body on same line
        }
    } else if !code.ends_with(';') {
        // Add semicolon if it's just a declaration and doesn't already end with one (e.g., from where clause)
        write!(code, ";").unwrap();
    }

    code
}

/// Formats a single enum variant's definition for the code block.
fn format_variant_definition(item: &Item, v: &Variant, krate: &Crate) -> String {
    let name = item.name.as_deref().unwrap_or("{Unnamed}");
    match &v.kind {
        VariantKind::Plain => name.to_string(),
        VariantKind::Tuple(fields) => {
            // fields_stripped ignored
            let types: Vec<String> = fields
                .iter()
                .filter_map(|opt_id| {
                    opt_id
                        .as_ref()
                        .and_then(|id| krate.index.get(id))
                        .and_then(|field_item| {
                            if let ItemEnum::StructField(ty) = &field_item.inner {
                                Some(format_type(ty, krate))
                            } else {
                                None
                            }
                        })
                })
                .collect();
            format!("{}({})", name, types.join(", "))
        }
        VariantKind::Struct { fields, .. } => {
            // fields_stripped ignored
            let fields_str: Vec<String> = fields
                .iter()
                .filter_map(|id| {
                    krate.index.get(id).and_then(|field_item| {
                        if let ItemEnum::StructField(ty) = &field_item.inner {
                            let field_name = field_item.name.as_deref().unwrap_or("_");
                            Some(format!("{}: {}", field_name, format_type(ty, krate)))
                        } else {
                            None
                        }
                    })
                })
                .collect();
            format!("{} {{ {} }}", name, fields_str.join(", "))
        }
    }
}

/// Formats an enum variant's signature for the `#####` header.
fn format_variant_signature(item: &Item, v: &Variant, krate: &Crate) -> String {
    // Similar to definition but potentially simpler, without pub, maybe add discriminant visually
    let mut sig = format_variant_definition(item, v, krate);
    if let Some(discr) = &v.discriminant {
        // Use format_discriminant_expr
        write!(sig, " = {}", format_discriminant_expr(discr)).unwrap();
    }
    sig
}

/// Cleans core:: and core::marker:: prefixes from a trait path string.
fn clean_trait_path(path_str: &str) -> String {
    path_str.replace("core::marker::", "").replace("core::", "") // Replace core:: after marker::
}

struct DocPrinter<'a> {
    krate: &'a Crate,
    selected_ids: &'a HashSet<Id>,
    printed_ids: HashSet<Id>,
    output: String,
    current_level: usize, // For markdown header levels
}

impl<'a> DocPrinter<'a> {
    fn new(krate: &'a Crate, selected_ids: &'a HashSet<Id>) -> Self {
        DocPrinter {
            krate,
            selected_ids,
            printed_ids: HashSet::new(),
            output: String::new(),
            current_level: 1,
        }
    }

    fn get_item_kind(&self, id: &Id) -> Option<ItemKind> {
        self.krate.paths.get(id).map(|summary| summary.kind)
    }

    // Fallback for inferring ItemKind if not found in paths map
    fn infer_item_kind(&self, item: &Item) -> ItemKind {
        match item.inner {
            ItemEnum::Module(_) => ItemKind::Module,
            ItemEnum::ExternCrate { .. } => ItemKind::ExternCrate,
            ItemEnum::Use { .. } => ItemKind::Use, // Renamed
            ItemEnum::Union(_) => ItemKind::Union,
            ItemEnum::Struct(_) => ItemKind::Struct,
            ItemEnum::StructField(_) => ItemKind::StructField,
            ItemEnum::Enum(_) => ItemKind::Enum,
            ItemEnum::Variant(_) => ItemKind::Variant,
            ItemEnum::Function(_) => ItemKind::Function,
            ItemEnum::Trait(_) => ItemKind::Trait,
            ItemEnum::TraitAlias(_) => ItemKind::TraitAlias,
            ItemEnum::Impl { .. } => ItemKind::Impl,
            ItemEnum::TypeAlias(_) => ItemKind::TypeAlias,
            // ItemEnum::OpaqueTy removed
            ItemEnum::Constant { .. } => ItemKind::Constant, // Use struct pattern
            ItemEnum::Static(_) => ItemKind::Static,
            ItemEnum::ExternType => ItemKind::ExternType, // Renamed
            ItemEnum::Macro(_) => ItemKind::Macro,
            ItemEnum::ProcMacro(ref pm) => match pm.kind {
                rustdoc_types::MacroKind::Bang => ItemKind::Macro, // Treat bang proc macro as Macro kind
                rustdoc_types::MacroKind::Attr => ItemKind::ProcAttribute,
                rustdoc_types::MacroKind::Derive => ItemKind::ProcDerive,
            },
            ItemEnum::Primitive(_) => ItemKind::Primitive,
            ItemEnum::AssocConst { .. } => ItemKind::AssocConst,
            ItemEnum::AssocType { .. } => ItemKind::AssocType,
        }
    }

    /// Prints the details of a single selected item.
    fn print_item_details(&mut self, id: &Id) {
        if !self.selected_ids.contains(id) || !self.printed_ids.insert(*id) {
            return; // Skip unselected or already printed items
        }

        if let Some(item) = self.krate.index.get(id) {
            let declaration = generate_item_declaration(item, self.krate);

            // Print Header (### `declaration`)
            writeln!(
                self.output,
                "\n{} `{}`\n",                      // Add newline after header
                "#".repeat(self.current_level + 2), // Item level is ###
                declaration
            )
            .unwrap();

            // Print Code Block for Struct/Enum/Trait/Function (if needed)
            let code_block = match &item.inner {
                ItemEnum::Struct(s) => Some(generate_struct_code_block(item, s, self.krate)),
                ItemEnum::Enum(e) => Some(generate_enum_code_block(item, e, self.krate)),
                ItemEnum::Trait(t) => Some(generate_trait_code_block(item, t, self.krate)),
                ItemEnum::Function(f) => {
                    // Check if function has attrs or where clause
                    let has_attrs = f.header.is_const
                        || f.header.is_async
                        || f.header.is_unsafe
                        || !matches!(f.header.abi, Abi::Rust);
                    let has_where = !f.generics.where_predicates.is_empty();
                    if has_attrs || has_where {
                        Some(generate_function_code_block(item, f, self.krate))
                    } else {
                        None // No code block needed for simple function
                    }
                }
                // TODO: Add code blocks for other types like TypeAlias, Constant if desired
                _ => None,
            };

            if let Some(code) = code_block {
                writeln!(self.output, "```rust\n{}\n```\n", code).unwrap();
            }

            // Print Documentation
            if let Some(docs) = &item.docs {
                if !docs.trim().is_empty() {
                    writeln!(self.output, "{}\n", docs.trim()).unwrap();
                }
            }

            // Print Specific Sections
            match &item.inner {
                ItemEnum::Struct(s) => self.print_struct_fields(item, s),
                ItemEnum::Enum(e) => self.print_enum_variants(item, e),
                ItemEnum::Trait(t) => self.print_trait_associated_items(item, t),
                // Add other kinds requiring detailed sections if necessary
                _ => {}
            }

            // Print Implementations (common to Struct, Enum, Trait, Primitive, etc.)
            match &item.inner {
                ItemEnum::Struct(s) => self.print_item_implementations(&s.impls, item),
                ItemEnum::Enum(e) => self.print_item_implementations(&e.impls, item),
                ItemEnum::Trait(t) => self.print_trait_implementors(&t.implementations, item), // Traits list implementors
                ItemEnum::Union(u) => self.print_item_implementations(&u.impls, item),
                ItemEnum::Primitive(p) => self.print_item_implementations(&p.impls, item),
                _ => {}
            }
        }
    }

    /// Prints the "Fields" section for a struct.
    fn print_struct_fields(&mut self, _item: &Item, s: &Struct) {
        let fields_to_print: Vec<Id> = match &s.kind {
            StructKind::Plain { fields, .. } => fields.clone(),
            StructKind::Tuple(fields) => fields.iter().filter_map(|opt_id| *opt_id).collect(),
            StructKind::Unit => vec![],
        };
        let has_stripped = matches!(
            &s.kind,
            StructKind::Plain {
                has_stripped_fields: true,
                ..
            }
        );

        if !fields_to_print.is_empty() || has_stripped {
            writeln!(
                self.output,
                "{} Fields\n",                      // Add newline after header
                "#".repeat(self.current_level + 3)  // Fields section level is ####
            )
            .unwrap();
            for field_id in &fields_to_print {
                self.print_field_details(field_id);
            }
            if has_stripped {
                writeln!(self.output, "\n_[Private fields hidden]_").unwrap();
            }
        }
    }

    /// Prints the details for a single struct field.
    fn print_field_details(&mut self, field_id: &Id) {
        if !self.selected_ids.contains(field_id) {
            return;
        } // Skip unselected
          // Avoid printing if already handled elsewhere (though unlikely for fields)
          // if !self.printed_ids.insert(*field_id) { return; }

        if let Some(item) = self.krate.index.get(field_id) {
            if let ItemEnum::StructField(_field_type) = &item.inner {
                let name = item.name.as_deref().unwrap_or("_");
                // Header: ##### `field_name`
                writeln!(
                    self.output,
                    "{} `{}`\n",                        // Add newline after header
                    "#".repeat(self.current_level + 4), // Field level is #####
                    name
                )
                .unwrap();
                // Docs
                if let Some(docs) = &item.docs {
                    if !docs.trim().is_empty() {
                        writeln!(self.output, "{}\n", docs.trim()).unwrap();
                    }
                }
                // Type (optional, could add here if needed)
                // writeln!(self.output, "_Type: `{}`_\n", format_type(field_type, self.krate)).unwrap();
            }
        }
    }

    /// Prints the "Variants" section for an enum.
    fn print_enum_variants(&mut self, _item: &Item, e: &Enum) {
        if !e.variants.is_empty() || e.has_stripped_variants {
            writeln!(
                self.output,
                "{} Variants\n",                    // Add newline after header
                "#".repeat(self.current_level + 3)  // Variants section level is ####
            )
            .unwrap();
            for variant_id in &e.variants {
                self.print_variant_details(variant_id);
            }
            if e.has_stripped_variants {
                writeln!(self.output, "\n_[Private variants hidden]_").unwrap();
            }
        }
    }

    /// Prints the details for a single enum variant.
    fn print_variant_details(&mut self, variant_id: &Id) {
        if !self.selected_ids.contains(variant_id) {
            return;
        } // Skip unselected
          // Avoid printing if already handled elsewhere
          // if !self.printed_ids.insert(*variant_id) { return; }

        if let Some(item) = self.krate.index.get(variant_id) {
            if let ItemEnum::Variant(variant_data) = &item.inner {
                let signature = format_variant_signature(item, variant_data, self.krate);
                // Header: ##### `VariantSignature`
                writeln!(
                    self.output,
                    "{} `{}`\n",                        // Add newline after header
                    "#".repeat(self.current_level + 4), // Variant level is #####
                    signature
                )
                .unwrap();
                // Docs
                if let Some(docs) = &item.docs {
                    if !docs.trim().is_empty() {
                        writeln!(self.output, "{}\n", docs.trim()).unwrap();
                    }
                }
            }
        }
    }

    /// Prints the "Associated Items" section for a trait.
    fn print_trait_associated_items(&mut self, _item: &Item, t: &Trait) {
        if t.items.is_empty() {
            return;
        }

        writeln!(
            self.output,
            "{} Associated Items\n",            // Add newline after header
            "#".repeat(self.current_level + 3)  // Associated Items section is ####
        )
        .unwrap();

        let mut assoc_consts = vec![];
        let mut assoc_types = vec![];
        let mut assoc_fns = vec![];

        for item_id in &t.items {
            if let Some(assoc_item) = self.krate.index.get(item_id) {
                if !self.selected_ids.contains(item_id) {
                    continue;
                }
                match &assoc_item.inner {
                    ItemEnum::AssocConst { .. } => assoc_consts.push(item_id),
                    ItemEnum::AssocType { .. } => assoc_types.push(item_id),
                    ItemEnum::Function(_) => assoc_fns.push(item_id),
                    _ => {} // Should not happen?
                }
            }
        }

        // Print in order: consts, types, fns
        if !assoc_consts.is_empty() {
            writeln!(
                self.output,
                "{} Associated Constants\n",
                "#".repeat(self.current_level + 4) // #####
            )
            .unwrap();
            for id in assoc_consts {
                self.print_associated_item_details(id);
            }
        }
        if !assoc_types.is_empty() {
            writeln!(
                self.output,
                "{} Associated Types\n",
                "#".repeat(self.current_level + 4) // #####
            )
            .unwrap();
            for id in assoc_types {
                self.print_associated_item_details(id);
            }
        }
        if !assoc_fns.is_empty() {
            writeln!(
                self.output,
                "{} Associated Functions\n",
                "#".repeat(self.current_level + 4) // #####
            )
            .unwrap();
            for id in assoc_fns {
                self.print_associated_item_details(id);
            }
        }
    }

    /// Prints the details for a single associated item (const, type, function).
    fn print_associated_item_details(&mut self, assoc_item_id: &Id) {
        if !self.selected_ids.contains(assoc_item_id) {
            return;
        }
        // Do not mark as printed here, let print_item_details handle it if called standalone

        if let Some(item) = self.krate.index.get(assoc_item_id) {
            let declaration = generate_item_declaration(item, self.krate);

            // Print Header (##### `declaration`) - Note: using +4 level assuming parent is ####
            writeln!(
                self.output,
                "{} `{}`\n",                        // Add newline after header
                "#".repeat(self.current_level + 4), // Assoc Item level is #####
                declaration
            )
            .unwrap();

            // Add code block for associated functions if they have attrs/where clauses
            if let ItemEnum::Function(f) = &item.inner {
                let has_attrs = f.header.is_const
                    || f.header.is_async
                    || f.header.is_unsafe
                    || !matches!(f.header.abi, Abi::Rust);
                let has_where = !f.generics.where_predicates.is_empty();
                if has_attrs || has_where {
                    let code = generate_function_code_block(item, f, self.krate);
                    writeln!(self.output, "```rust\n{}\n```\n", code).unwrap();
                }
            }

            // Print Documentation
            if let Some(docs) = &item.docs {
                if !docs.trim().is_empty() {
                    writeln!(self.output, "{}\n", docs.trim()).unwrap();
                }
            }

            // Potentially add default values/bounds for assoc const/type here
            match &item.inner {
                // Use correct fields { type_, value }
                ItemEnum::AssocConst { type_, value } => {
                    writeln!(
                        self.output,
                        "_Type: `{}`_\n",
                        format_type(type_, self.krate)
                    )
                    .unwrap();
                    if let Some(val) = value {
                        writeln!(self.output, "_Default: `{}`_\n", val).unwrap();
                    }
                }
                ItemEnum::AssocType { bounds, type_, .. } => {
                    // Use renamed field type_
                    if !bounds.is_empty() {
                        let bounds_str = bounds
                            .iter()
                            .map(|b| format_generic_bound(b, self.krate))
                            .collect::<Vec<_>>()
                            .join(" + ");
                        writeln!(self.output, "_Bounds: `{}`_\n", bounds_str).unwrap();
                    }
                    if let Some(ty) = type_ {
                        writeln!(
                            self.output,
                            "_Default: `{}`_\n",
                            format_type(ty, self.krate)
                        )
                        .unwrap();
                    }
                }
                _ => {}
            }
        }
    }

    /// Prints Inherent and Trait Implementations *for* an item (Struct, Enum, Union, Primitive).
    fn print_item_implementations(&mut self, impl_ids: &[Id], target_item: &Item) {
        let target_name = target_item.name.as_deref().unwrap_or_else(|| {
            // Fixed: Correctly handle primitive name extraction
            match &target_item.inner {
                ItemEnum::Primitive(Primitive { name, .. }) => name.as_str(),
                _ => "{unknown_primitive}", // Should not happen if called correctly
            }
        });

        let all_impls: Vec<&Item> = impl_ids
            .iter()
            .filter_map(|impl_id| self.krate.index.get(impl_id))
            .filter(|impl_item| self.selected_ids.contains(&impl_item.id))
            .collect();

        let (inherent_impls, trait_impls): (Vec<_>, Vec<_>) = all_impls.into_iter().partition(
            |impl_item| matches!(&impl_item.inner, ItemEnum::Impl(i) if i.trait_.is_none()),
        );

        // --- Inherent Impls ---
        if !inherent_impls.is_empty() {
            writeln!(
                self.output,
                "{} Implementations for `{}`\n", // Added target name, Add newline after header
                "#".repeat(self.current_level + 3), // #### Section Header
                target_name
            )
            .unwrap();
            for impl_item in inherent_impls {
                // Check printed_ids *before* printing the block
                if self.printed_ids.contains(&impl_item.id) {
                    continue;
                }
                if let ItemEnum::Impl(imp) = &impl_item.inner {
                    self.print_impl_block_details(impl_item, imp);
                }
            }
        }

        // --- Trait Impls ---
        if !trait_impls.is_empty() {
            writeln!(
                self.output,
                "{} Trait Implementations for `{}`\n", // Added target name, Add newline after header
                "#".repeat(self.current_level + 3),    // #### Section Header
                target_name
            )
            .unwrap();

            let mut simple_impls = Vec::new();
            let mut generic_impl_items = Vec::new();

            for impl_item in trait_impls {
                if let ItemEnum::Impl(imp) = &impl_item.inner {
                    if let Some(trait_path) = &imp.trait_ {
                        let is_simple = imp.generics.params.is_empty()
                            && imp.generics.where_predicates.is_empty()
                            && trait_path.args.as_ref().map_or(true, |args| {
                                // Check if args contain only lifetimes or are empty
                                match **args {
                                    GenericArgs::AngleBracketed {
                                        ref args,
                                        ref constraints,
                                        ..
                                    } => {
                                        args.iter().all(|a| matches!(a, GenericArg::Lifetime(_)))
                                            && constraints.is_empty()
                                    }
                                    _ => false, // Parenthesized or ReturnTypeNotation are not simple path args
                                }
                            });

                        if is_simple {
                            simple_impls
                                .push(clean_trait_path(&format_path(trait_path, self.krate)));
                        } else {
                            // Check printed_ids *before* adding to generic list
                            if !self.printed_ids.contains(&impl_item.id) {
                                generic_impl_items.push(impl_item);
                            }
                        }
                    }
                }
            }

            // Print simple impls as a list first
            if !simple_impls.is_empty() {
                simple_impls.sort(); // Sort alphabetically
                for cleaned_path in &simple_impls {
                    writeln!(self.output, "- `{}`", cleaned_path).unwrap();
                }
                writeln!(self.output).unwrap(); // Add blank line after list
            }

            // Print generic impls (complex ones) using their dedicated block printer
            for impl_item in generic_impl_items {
                if let ItemEnum::Impl(imp) = &impl_item.inner {
                    // print_impl_block_details marks as printed and adds ##### header
                    self.print_impl_block_details(impl_item, imp);
                }
            }
        }
    }

    /// Prints implementors *of* a trait.
    fn print_trait_implementors(&mut self, impl_ids: &[Id], _trait_item: &Item) {
        let implementors: Vec<&Item> = impl_ids
            .iter()
            .filter_map(|id| self.krate.index.get(id))
            .filter(|item| {
                self.selected_ids.contains(&item.id) && matches!(item.inner, ItemEnum::Impl(_))
            })
            .collect();

        if !implementors.is_empty() {
            writeln!(
                self.output,
                "{} Implementors\n",                // Add newline after header
                "#".repeat(self.current_level + 3)  // #### Section Header
            )
            .unwrap();

            for impl_item in implementors {
                if let ItemEnum::Impl(imp) = &impl_item.inner {
                    // Only print the header for the implementation here
                    let impl_header = self.format_impl_header(imp);
                    // Print the impl block header (##### `impl ...`)
                    writeln!(
                        self.output,
                        "{} `{}`\n",                        // Add newline after header
                        "#".repeat(self.current_level + 4), // Impl block level is #####
                        impl_header.trim()                  // Trim potential trailing space
                    )
                    .unwrap();
                    // Optionally, print docs for the impl block itself if available
                    if let Some(docs) = &impl_item.docs {
                        if !docs.trim().is_empty() {
                            writeln!(self.output, "{}\n", docs.trim()).unwrap();
                        }
                    }
                    // We don't print the associated items here, just list the implementor
                }
            }
        }
    }

    /// Helper to format just the header line of an impl block.
    fn format_impl_header(&self, imp: &Impl) -> String {
        let mut impl_header = String::new();
        if imp.is_unsafe {
            write!(impl_header, "unsafe ").unwrap();
        }
        write!(impl_header, "impl").unwrap();

        // Print impl generics only if they exist
        let impl_generics_str = format_generics_full(&imp.generics, self.krate);
        if !impl_generics_str.is_empty() && !impl_generics_str.starts_with('\n') {
            write!(impl_header, "{} ", impl_generics_str).unwrap(); // Space after generics if single line
        } else if !impl_generics_str.is_empty() {
            write!(impl_header, "{}", impl_generics_str).unwrap(); // Multiline generics include spacing
        }

        if let Some(trait_path) = &imp.trait_ {
            // Use format_path with the Path struct
            write!(impl_header, "{} for ", format_path(trait_path, self.krate)).unwrap();
        }
        write!(impl_header, "{}", format_type(&imp.for_, self.krate)).unwrap();

        // Append where clause from impl generics if needed and not already printed *within the header line*
        // (Multi-line where clauses were handled by format_generics_full)
        if !imp.generics.where_predicates.is_empty()
            && !impl_generics_str.contains("where\n")
            && !impl_generics_str.is_empty()
            && impl_generics_str.ends_with('>')
        {
            let where_clause =
                format_generics_where_only(&imp.generics.where_predicates, self.krate);
            write!(impl_header, "\n  {}", where_clause).unwrap(); // Add multiline where clause
        } else if !imp.generics.where_predicates.is_empty() && impl_generics_str.is_empty() {
            let where_clause =
                format_generics_where_only(&imp.generics.where_predicates, self.krate);
            write!(impl_header, " {}", where_clause).unwrap(); // Add single line where clause
        }
        impl_header
    }

    /// Prints the details of a specific impl block (header, associated items).
    fn print_impl_block_details(&mut self, impl_item: &Item, imp: &Impl) {
        // Mark as printed *now* before printing details
        if !self.printed_ids.insert(impl_item.id) {
            return;
        }

        let impl_header = self.format_impl_header(imp);

        // Print the impl block header (##### `impl ...`)
        writeln!(
            self.output,
            "{} `{}`\n",                        // Add newline after header
            "#".repeat(self.current_level + 4), // Impl block level is #####
            impl_header.trim() // Trim potential trailing space if no where clause added
        )
        .unwrap();

        // Print associated items within this impl block
        let mut assoc_consts = vec![];
        let mut assoc_types = vec![];
        let mut assoc_fns = vec![];
        for assoc_item_id in &imp.items {
            // Important: Only print associated items that are *selected*
            if !self.selected_ids.contains(assoc_item_id) {
                continue;
            }

            if let Some(assoc_item) = self.krate.index.get(assoc_item_id) {
                match &assoc_item.inner {
                    ItemEnum::AssocConst { .. } => assoc_consts.push(assoc_item_id),
                    ItemEnum::AssocType { .. } => assoc_types.push(assoc_item_id),
                    ItemEnum::Function(_) => assoc_fns.push(assoc_item_id),
                    _ => {} // Should not happen in impl block
                }
            }
        }

        // Print associated items using the dedicated detail printer
        // No extra sub-headers needed here, print_associated_item_details already uses #####
        if !assoc_consts.is_empty() {
            // writeln!(self.output, "\n###### Associated Constants\n").unwrap(); // No sub-sub-header
            for id in assoc_consts {
                self.print_associated_item_details(id);
            }
        }
        if !assoc_types.is_empty() {
            // writeln!(self.output, "\n###### Associated Types\n").unwrap();
            for id in assoc_types {
                self.print_associated_item_details(id);
            }
        }
        if !assoc_fns.is_empty() {
            // writeln!(self.output, "\n###### Associated Functions\n").unwrap();
            for id in assoc_fns {
                self.print_associated_item_details(id);
            }
        }
    }

    /// Prints the contents of a module, grouping items by kind.
    fn print_module_contents(&mut self, module: &Module) {
        // Group selected items by kind within this module
        // Use HashMap instead of BTreeMap for ItemKind
        let mut items_by_kind: HashMap<ItemKind, Vec<Id>> = HashMap::new();
        for id in &module.items {
            if !self.selected_ids.contains(id) || self.printed_ids.contains(id) {
                continue; // Skip unselected or already printed items
            }
            let kind = self.get_item_kind(id).unwrap_or_else(|| {
                // Fallback if not in paths (should be rare for local items)
                self.krate
                    .index
                    .get(id)
                    .map(|item| self.infer_item_kind(item))
                    .unwrap_or(ItemKind::Module) // Default fallback kind
            });

            // Skip kinds handled implicitly within others for grouping
            match kind {
                ItemKind::Impl
                | ItemKind::Variant
                | ItemKind::StructField
                | ItemKind::AssocConst
                | ItemKind::AssocType => continue,
                _ => {}
            }

            // Use or_insert_with for HashMap
            items_by_kind.entry(kind).or_default().push(*id);
        }

        // Sort items by name within each kind
        for ids in items_by_kind.values_mut() {
            ids.sort_by_key(|id| self.krate.index.get(id).and_then(|item| item.name.clone()));
        }

        // Defined printing order (subset relevant for module contents)
        let print_order = [
            ItemKind::Macro,
            ItemKind::ProcAttribute,
            ItemKind::ProcDerive,
            ItemKind::Module,
            ItemKind::ExternCrate,
            ItemKind::Use,
            ItemKind::Primitive, // Unlikely here unless re-exported
            ItemKind::Union,
            ItemKind::Struct,
            ItemKind::Enum,
            ItemKind::Trait,
            ItemKind::TypeAlias,
            ItemKind::TraitAlias,
            ItemKind::Static,
            ItemKind::Constant,
            ItemKind::Function,
            ItemKind::ExternType,
            ItemKind::Keyword, // Added Keyword to order
        ];

        let mut printed_headers = HashSet::new(); // Track headers printed at this level

        for kind in print_order {
            // Use .get() for HashMap
            if let Some(ids) = items_by_kind.get(&kind) {
                // Determine group header
                let header_name = match kind {
                    ItemKind::Module => "Modules",
                    ItemKind::Struct => "Structs",
                    ItemKind::Enum => "Enums",
                    ItemKind::Trait => "Traits",
                    ItemKind::Function => "Functions",
                    ItemKind::Macro | ItemKind::ProcAttribute | ItemKind::ProcDerive => "Macros",
                    ItemKind::Static => "Statics",
                    ItemKind::Constant => "Constants",
                    ItemKind::TypeAlias => "Type Aliases",
                    ItemKind::TraitAlias => "Trait Aliases",
                    ItemKind::Union => "Unions",
                    ItemKind::Use => "Imports",
                    ItemKind::ExternCrate => "External Crates",
                    ItemKind::ExternType => "External Types",
                    ItemKind::Primitive => "Primitives",
                    ItemKind::Keyword => "", // Handle Keyword - don't give it a header section
                    // These are skipped above, but listed defensively
                    ItemKind::Impl
                    | ItemKind::Variant
                    | ItemKind::StructField
                    | ItemKind::AssocConst
                    | ItemKind::AssocType => "",
                };

                if !header_name.is_empty() && printed_headers.insert(header_name) {
                    writeln!(
                        self.output,
                        "\n{} {}",
                        "#".repeat(self.current_level + 1), // Group level is ##
                        header_name
                    )
                    .unwrap();
                }

                // Print items of this kind
                for id in ids {
                    if let Some(item) = self.krate.index.get(id) {
                        // Handle nested modules recursively
                        if let ItemEnum::Module(sub_module) = &item.inner {
                            // Check printed_ids again before recursing
                            if !self.printed_ids.contains(id) {
                                let mod_name = item.name.as_deref().unwrap_or("{unnamed}");
                                writeln!(
                                    self.output,
                                    "\n{} `mod {}`\n", // Module header uses ###
                                    "#".repeat(self.current_level + 2),
                                    mod_name
                                )
                                .unwrap();
                                self.printed_ids.insert(*id); // Mark printed *before* recursion

                                // Print module docs
                                if let Some(docs) = &item.docs {
                                    if !docs.trim().is_empty() {
                                        writeln!(self.output, "{}\n", docs.trim()).unwrap();
                                    }
                                }

                                let previous_level = self.current_level;
                                self.current_level += 1;
                                self.print_module_contents(sub_module);
                                self.current_level = previous_level;
                            }
                        } else {
                            // Print other item types using the detail printer
                            self.print_item_details(id);
                        }
                    }
                }
            }
        }
    }

    /// Finalizes the documentation string, printing the crate header and contents.
    fn finalize(mut self) -> String {
        let root_item = self.krate.index.get(&self.krate.root).unwrap(); // Assume root exists
        let crate_name = root_item.name.as_deref().unwrap_or("Unknown Crate");
        let crate_version = self.krate.crate_version.as_deref().unwrap_or("");

        // Print Crate Header (# Crate Name (Version))
        writeln!(
            self.output,
            "{} {} API ({})\n", // Add newline after header
            "#".repeat(self.current_level),
            crate_name,
            crate_version
        )
        .unwrap();
        self.printed_ids.insert(self.krate.root); // Mark root as printed

        // Print Crate Documentation
        if let Some(docs) = &root_item.docs {
            if !docs.trim().is_empty() {
                writeln!(self.output, "{}\n", docs.trim()).unwrap();
            }
        }

        // Print Root Module Contents
        if let ItemEnum::Module(root_module) = &root_item.inner {
            self.print_module_contents(root_module);
        }

        // --- Check for Unprinted Selected Items ---
        let mut unprinted_ids = Vec::new();
        for id in self.selected_ids {
            if !self.printed_ids.contains(id) {
                // Additional check: skip impl items that might be selected but belong
                // to an unselected struct/enum/trait. Their details are printed
                // under their target type if that type *is* selected.
                if let Some(item) = self.krate.index.get(id) {
                    if !matches!(item.inner, ItemEnum::Impl(_)) {
                        unprinted_ids.push(*id);
                    }
                }
            }
        }

        if !unprinted_ids.is_empty() {
            warn!(
                "Found {} selected items that were not printed in the structured output. Adding them to an 'Other Items' section.",
                unprinted_ids.len()
            );
            writeln!(
                self.output,
                "\n{} Other Items", // Use ## level for this section
                "#".repeat(self.current_level + 1)
            )
            .unwrap();

            // Sort unprinted items for consistent output
            unprinted_ids.sort_by_key(|id| {
                (
                    self.krate.paths.get(id).map(|p| p.path.clone()),
                    self.krate.index.get(id).and_then(|i| i.name.clone()),
                )
            });

            for id in unprinted_ids {
                self.print_item_details(&id); // Print details for unprinted items
            }
        }

        self.output
    }
}

fn generate_documentation(krate: &Crate, selected_ids: &HashSet<Id>) -> Result<String> {
    info!(
        "Generating documentation for {} selected items.",
        selected_ids.len()
    );
    if selected_ids.is_empty() {
        return Ok("No items selected for documentation.".to_string());
    }

    let printer = DocPrinter::new(krate, selected_ids);
    let output = printer.finalize();

    Ok(output)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging based on RUST_LOG env var (e.g., RUST_LOG=info,crate_doc_extractor=debug)
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Install the required nightly toolchain
    rustup_toolchain::install(NIGHTLY_RUST_VERSION).unwrap();

    let args = Args::parse();
    let client = reqwest::Client::builder()
        .user_agent(format!(
            "crate-doc-extractor/{} (github.com/ai-sandbox/aidocs-rust)", // Updated repo
            env!("CARGO_PKG_VERSION")
        ))
        .build()?;

    let target_version = find_best_version(
        &client,
        &args.crate_name,
        &args.crate_version,
        args.include_prerelease,
    )
    .await?;

    info!(
        "Selected version {} for crate {}",
        target_version.num, target_version.crate_name
    );

    let output_path = PathBuf::from(args.output_dir);
    std::fs::create_dir_all(&output_path).with_context(|| {
        format!(
            "Failed to create output directory: {}",
            output_path.display()
        )
    })?;

    let crate_dir = download_and_unpack_crate(&client, &target_version, &output_path).await?;

    // Use the *actual* crate name from the API response, as it might differ in casing
    let actual_crate_name = &target_version.crate_name;
    let json_output_path = run_rustdoc(&crate_dir, actual_crate_name)?;

    // --- Load Rustdoc JSON ---
    info!("Parsing rustdoc JSON: {}", json_output_path.display());
    let file = File::open(&json_output_path)
        .with_context(|| format!("Failed to open JSON file: {}", json_output_path.display()))?;
    let reader = BufReader::new(file);
    let krate: Crate = serde_json::from_reader(reader)
        .with_context(|| format!("Failed to parse JSON file: {}", json_output_path.display()))?;
    info!(
        "Loaded rustdoc JSON for {} v{}",
        actual_crate_name,
        krate.crate_version.as_deref().unwrap_or("?")
    );
    info!("Found {} total items in the index.", krate.index.len());
    info!("Found {} items in the paths map.", krate.paths.len());

    // --- Select Items ---
    let selected_ids = select_items(&krate, &args.paths)?;

    // --- Generate Documentation ---
    let documentation = generate_documentation(&krate, &selected_ids)?;

    // --- Output Documentation ---
    // For now, just print to stdout
    print!("{}", documentation); // Use print! to avoid extra newline at the end

    Ok(())
}
