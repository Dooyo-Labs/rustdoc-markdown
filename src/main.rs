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
    Abi, Crate, Function, GenericArg, GenericArgs, GenericBound, GenericParamDef, Generics, Id,
    Impl, Item, ItemEnum, ItemKind, Module, Path, PolyTrait, Struct, Term, Type, WherePredicate,
};
use semver::{Version, VersionReq};
use serde::Deserialize;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Write;
use std::fs::File;
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
            if let Some(args) = args.as_ref() {
                // args is &Box<GenericArgs>
                find_generic_args_dependencies(args, krate, dependencies);
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
                    GenericArg::Const(c) => find_type_dependencies(
                        &c.expr
                            .as_ref()
                            .map_or(Type::Infer, |s| Type::Primitive(s.clone())),
                        krate,
                        dependencies,
                    ), // c is Constant struct
                    GenericArg::Lifetime(_) | GenericArg::Infer => {}
                }
            }
            for constraint in constraints {
                // AssocItemConstraint { name: String, kind: AssocItemConstraintKind }
                match constraint {
                    // Use tuple variant matching
                    rustdoc_types::AssocItemConstraintKind::Equality(term) => match term {
                        Term::Type(t) => find_type_dependencies(t, krate, dependencies),
                        Term::Constant(c) => find_type_dependencies(
                            &c.expr
                                .as_ref()
                                .map_or(Type::Infer, |s| Type::Primitive(s.clone())),
                            krate,
                            dependencies,
                        ), // c is Constant struct
                    },
                    rustdoc_types::AssocItemConstraintKind::Constraint(bounds) => {
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
        GenericBound::Outlives(_) => {} // Lifetimes don't have IDs
        GenericBound::Use(args) => {
            // args: Vec<PreciseCapturingArg>
            for arg in args {
                match arg {
                    rustdoc_types::PreciseCapturingArg::Lifetime(_) => {}
                    rustdoc_types::PreciseCapturingArg::Param(id) => {
                        // id is Id, check if it's in the local crate
                        if krate.index.contains_key(id) {
                            dependencies.insert(*id);
                        }
                    }
                }
            }
        }
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
                    Term::Constant(c) => find_type_dependencies(
                        &c.expr
                            .as_ref()
                            .map_or(Type::Infer, |s| Type::Primitive(s.clone())),
                        krate,
                        dependencies,
                    ), // c is Constant struct
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
                        rustdoc_types::StructKind::Plain {
                            fields,
                            has_stripped_fields,
                        } => {
                            let mut dependencies = String::new();
                            if !fields.is_empty() || *has_stripped_fields {
                                for field_id in fields {
                                    if krate.index.contains_key(field_id) {
                                        item_deps.insert(*field_id);
                                        // Also get dependencies of the field's type
                                        if let Some(field_item) = krate.index.get(field_id) {
                                            if let ItemEnum::StructField(field_type) =
                                                &field_item.inner
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
                                if *has_stripped_fields {
                                    writeln!(dependencies, "\n_[Private fields hidden]_").unwrap();
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
                        // find_type_dependencies(&disr.type_, krate, &mut item_deps); // Removed: Discriminant has no type_
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
                // ItemEnum::OpaqueTy removed
                // Use struct pattern matching for Constant
                ItemEnum::Constant { type_, const_, .. } => {
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
                ItemEnum::AssocConst { type_, .. } => {
                    // Ignore default string (const_)
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
                                                                          // ItemEnum::Keyword removed
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
        let args_str = format_generic_args(args, krate);
        if !args_str.is_empty() {
            // Determine brackets based on the GenericArgs variant
            match **args {
                GenericArgs::AngleBracketed { .. } => format!("{}<{}>", base_path, args_str),
                GenericArgs::Parenthesized { .. } => format!("{}{}", base_path, args_str), // Parentheses are included in format_generic_args
                GenericArgs::ReturnTypeNotation { .. } => {
                    format!("{}::{}", base_path, args_str)
                } // Needs context
            }
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
            // Use as_ref() for Option<Box<GenericArgs>>
            let args_str = args
                .as_ref()
                .map(|a| format!("<{}>", format_generic_args(a, krate)))
                .unwrap_or_default();

            format!("<{} as {}>::{}{}", self_type_str, trait_str, name, args_str)
        }
    }
}

fn format_generic_args(args: &GenericArgs, krate: &Crate) -> String {
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
                    rustdoc_types::AssocItemConstraintKind::Equality(term) => {
                        format!("{} = {}", c.name, format_term(term, krate))
                    }
                    rustdoc_types::AssocItemConstraintKind::Constraint(bounds) => format!(
                        "{}: {}",
                        c.name,
                        bounds
                            .iter()
                            .map(|bnd| format_generic_bound(bnd, krate))
                            .collect::<Vec<_>>()
                            .join(" + ")
                    ),
                })
                .collect();
            let mut all_strs = arg_strs;
            all_strs.extend(constraint_strs);
            all_strs.join(", ")
        }
        GenericArgs::Parenthesized { inputs, output, .. } => {
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
        GenericArgs::ReturnTypeNotation { .. } => "".to_string(), // How to format T::method(..) args? Ignore for now.
    }
}

fn format_generic_arg(arg: &GenericArg, krate: &Crate) -> String {
    match arg {
        GenericArg::Lifetime(lt) => format!("'{}", lt), // Add quote
        GenericArg::Type(ty) => format_type(ty, krate),
        GenericArg::Const(c) => format_type(
            &c.expr
                .as_ref()
                .map_or(Type::Infer, |s| Type::Primitive(s.clone())),
            krate,
        ), // Just show type for now, c is Constant
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
                        rustdoc_types::PreciseCapturingArg::Lifetime(lt) => format!("'{}", lt), // Add quote
                        rustdoc_types::PreciseCapturingArg::Param(id) => format_id_path(id, krate), // Pass &id
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
        Term::Constant(c) => format_type(
            &c.expr
                .as_ref()
                .map_or(Type::Infer, |s| Type::Primitive(s.clone())),
            krate,
        ), // Just show type for now, c is Constant
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

fn format_generics(generics: &Generics, krate: &Crate) -> String {
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
        s.push_str("\n  where\n    "); // Indent where clauses for readability
        let predicates_str = generics
            .where_predicates
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
            .collect::<Vec<_>>()
            .join(",\n    ");
        s.push_str(&predicates_str);
        // If generics were present, add newline before where clause for better formatting
        if !generics.params.is_empty() && !s.ends_with('\n') {
            // Check if the generics part already ended with a newline (due to complex generics)
            if let Some(last_char) = format_generics_params_only(&generics.params, krate).pop() {
                if last_char != '\n' {
                    s.insert(s.find("where").unwrap_or(s.len()), '\n'); // Insert newline before where if params didn't have one
                }
            }
        }
    }

    s
}

// Helper to format only the <...> part for layout checks
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

fn format_function_signature(func: &Function, name: &str, krate: &Crate) -> String {
    let mut sig_str = String::new();
    write!(sig_str, "pub ").unwrap(); // Assuming pub for now
    if func.header.is_const {
        write!(sig_str, "const ").unwrap();
    }
    if func.header.is_unsafe {
        write!(sig_str, "unsafe ").unwrap();
    }
    if func.header.is_async {
        write!(sig_str, "async ").unwrap();
    }
    // Add ABI if not Rust
    if !matches!(func.header.abi, Abi::Rust) {
        write!(sig_str, "extern \"{:?}\" ", func.header.abi).unwrap(); // Use Debug for Abi
    }

    write!(sig_str, "fn {}", name).unwrap();
    write!(sig_str, "{}", format_generics(&func.generics, krate)).unwrap();

    write!(sig_str, "(").unwrap();
    let args_str = func
        .sig
        .inputs
        .iter()
        .map(|(arg_name, arg_type)| {
            // TODO: Handle patterns in arg names if needed (arg_name is currently String)
            format!("{}: {}", arg_name, format_type(arg_type, krate))
        })
        .collect::<Vec<_>>()
        .join(", ");
    write!(sig_str, "{}", args_str).unwrap();
    // Use correct field name is_c_variadic
    if func.sig.is_c_variadic {
        write!(sig_str, ", ...").unwrap();
    }
    write!(sig_str, ")").unwrap();

    if let Some(output_type) = &func.sig.output {
        write!(sig_str, " -> {}", format_type(output_type, krate)).unwrap();
    }

    // Append where clause if it wasn't already part of format_generics output
    if !func.generics.where_predicates.is_empty() && !sig_str.contains("where") {
        let where_clause = format_generics_where_only(&func.generics.where_predicates, krate);
        write!(sig_str, " {}", where_clause).unwrap();
    }

    sig_str
}

// Helper to format only the where clause
fn format_generics_where_only(predicates: &[WherePredicate], krate: &Crate) -> String {
    if predicates.is_empty() {
        return String::new();
    }
    format!(
        "where {}",
        predicates
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
                            .map(|lt| format!("'{}", lt))
                            .collect::<Vec<_>>()
                            .join(" + ")
                    )
                }
                WherePredicate::EqPredicate { lhs, rhs, .. } => {
                    format!("{} == {}", format_type(lhs, krate), format_term(rhs, krate))
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    )
}

// --- Structured Printing Logic ---

struct DocPrinter<'a> {
    krate: &'a Crate,
    selected_ids: &'a HashSet<Id>,
    printed_ids: HashSet<Id>,
    output: String,
    level: usize, // For markdown header levels
}

impl<'a> DocPrinter<'a> {
    fn new(krate: &'a Crate, selected_ids: &'a HashSet<Id>) -> Self {
        DocPrinter {
            krate,
            selected_ids,
            printed_ids: HashSet::new(),
            output: String::new(),
            level: 1,
        }
    }

    fn get_item_kind(&self, id: &Id) -> Option<ItemKind> {
        self.krate.paths.get(id).map(|summary| summary.kind)
    }

    fn print_item(&mut self, id: &Id) {
        if !self.selected_ids.contains(id) || !self.printed_ids.insert(*id) {
            return; // Skip unselected or already printed items
        }

        if let Some(item) = self.krate.index.get(id) {
            let name = item.name.as_deref().unwrap_or("{unnamed}");
            let path_str = format_id_path(id, self.krate);
            // Get kind from ItemSummary (paths map) if possible for accuracy
            let item_kind = self
                .get_item_kind(id)
                .unwrap_or_else(|| self.infer_item_kind(item)); // Fallback if not in paths

            writeln!(
                self.output,
                "\n{} Item: {} ({:?})",
                "#".repeat(self.level + 1),
                path_str,
                item_kind // Use kind from ItemSummary or inferred
            )
            .unwrap();
            writeln!(
                self.output,
                "{}",
                "-".repeat(path_str.len() + format!("{:?}", item_kind).len() + 8)
            )
            .unwrap(); // Separator

            // Print signature/summary based on kind
            match &item.inner {
                ItemEnum::Function(func) => {
                    writeln!(
                        self.output,
                        "```rust\n{}\n```",
                        format_function_signature(func, name, self.krate)
                    )
                    .unwrap();
                }
                ItemEnum::Struct(s) => {
                    writeln!(
                        self.output,
                        "```rust\nstruct {}{}; // Fields omitted\n```",
                        name,
                        format_generics(&s.generics, self.krate)
                    )
                    .unwrap();
                }
                ItemEnum::Enum(e) => {
                    writeln!(
                        self.output,
                        "```rust\nenum {}{}; // Variants omitted\n```",
                        name,
                        format_generics(&e.generics, self.krate)
                    )
                    .unwrap();
                }
                ItemEnum::Trait(t) => {
                    writeln!(
                        self.output,
                        "```rust\ntrait {}{}; // Items omitted\n```",
                        name,
                        format_generics(&t.generics, self.krate)
                    )
                    .unwrap();
                }
                ItemEnum::TypeAlias(ta) => {
                    writeln!(
                        self.output,
                        "```rust\ntype {}{} = {};\n```",
                        name,
                        format_generics(&ta.generics, self.krate),
                        format_type(&ta.type_, self.krate)
                    )
                    .unwrap();
                }
                // Use struct pattern matching
                ItemEnum::Constant { type_, const_, .. } => {
                    writeln!(
                        self.output,
                        "```rust\nconst {}: {} = {};\n```",
                        name,
                        format_type(type_, self.krate),
                        const_.value.as_deref().unwrap_or("...") // Use expr field
                    )
                    .unwrap();
                }
                ItemEnum::Static(s) => {
                    writeln!(
                        self.output,
                        "```rust\nstatic {}{}: {} = {};\n```",
                        if s.is_mutable { "mut " } else { "" },
                        name,
                        format_type(&s.type_, self.krate),
                        s.expr.as_ref().unwrap_or("...") // Use Option::as_deref
                    )
                    .unwrap();
                }
                // ... other kinds ...
                _ => {}
            }

            if let Some(docs) = &item.docs {
                if !docs.trim().is_empty() {
                    writeln!(self.output, "\n{}", docs.trim()).unwrap();
                }
            }

            // Specific sections for complex items
            match &item.inner {
                ItemEnum::Struct(s) => self.print_struct_details(item, s),
                ItemEnum::Enum(e) => self.print_enum_details(item, e),
                ItemEnum::Trait(t) => self.print_trait_details(item, t),
                _ => {}
            }
        }
    }

    fn print_struct_details(&mut self, item: &Item, s: &Struct) {
        // Print Fields
        match &s.kind {
            rustdoc_types::StructKind::Plain {
                fields,
                has_stripped_fields,
            } => {
                if !fields.is_empty() || *has_stripped_fields {
                    writeln!(self.output, "\n{} Fields", "#".repeat(self.level + 2)).unwrap();
                    for field_id in fields {
                        self.print_item(field_id); // Will print StructField item
                    }
                    if *has_stripped_fields {
                        writeln!(self.output, "\n_[Private fields hidden]_").unwrap();
                    }
                }
            }
            rustdoc_types::StructKind::Tuple(fields) => {
                // fields is Vec<Option<Id>>
                if !fields.is_empty() {
                    writeln!(self.output, "\n{} Fields", "#".repeat(self.level + 2)).unwrap();
                    for field_id_opt in fields {
                        if let Some(field_id) = field_id_opt {
                            self.print_item(field_id);
                        } else {
                            writeln!(self.output, "\n_[Hidden field]_").unwrap();
                        }
                    }
                }
            }
            rustdoc_types::StructKind::Unit => {} // No fields
        }

        // Print Implementations
        self.print_implementations(&s.impls, item.id);
    }

    fn print_enum_details(&mut self, item: &Item, e: &rustdoc_types::Enum) {
        // Print Variants
        if !e.variants.is_empty() || e.has_stripped_variants {
            writeln!(self.output, "\n{} Variants", "#".repeat(self.level + 2)).unwrap();
            for variant_id in &e.variants {
                self.print_item(variant_id); // Will print Variant item
            }
            if e.has_stripped_variants {
                writeln!(self.output, "\n_[Private variants hidden]_").unwrap();
            }
        }

        // Print Implementations
        self.print_implementations(&e.impls, item.id);
    }

    fn print_trait_details(&mut self, _item: &Item, t: &rustdoc_types::Trait) {
        // Print Associated Items
        if !t.items.is_empty() {
            writeln!(
                self.output,
                "\n{} Associated Items",
                "#".repeat(self.level + 2)
            )
            .unwrap();
            // Group associated items by kind?
            let mut assoc_consts = vec![];
            let mut assoc_types = vec![];
            let mut assoc_fns = vec![];
            for item_id in &t.items {
                if let Some(item) = self.krate.index.get(item_id) {
                    match &item.inner {
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
                    "\n{} Associated Constants",
                    "#".repeat(self.level + 3)
                )
                .unwrap();
                for id in assoc_consts {
                    self.print_item(id);
                }
            }
            if !assoc_types.is_empty() {
                writeln!(
                    self.output,
                    "\n{} Associated Types",
                    "#".repeat(self.level + 3)
                )
                .unwrap();
                for id in assoc_types {
                    self.print_item(id);
                }
            }
            if !assoc_fns.is_empty() {
                writeln!(
                    self.output,
                    "\n{} Associated Functions",
                    "#".repeat(self.level + 3)
                )
                .unwrap();
                for id in assoc_fns {
                    self.print_item(id);
                }
            }
        }
        // Print Implementors? (t.implementations) - maybe too verbose
    }

    fn print_implementations(&mut self, impl_ids: &[Id], _target_id: Id) {
        let impls: Vec<&Item> = impl_ids
            .iter()
            .filter_map(|impl_id| self.krate.index.get(impl_id))
            .filter(|impl_item| self.selected_ids.contains(&impl_item.id))
            .collect();

        let (inherent_impls, trait_impls): (Vec<_>, Vec<_>) = impls.into_iter().partition(
            |impl_item| matches!(&impl_item.inner, ItemEnum::Impl(i) if i.trait_.is_none()),
        );

        if !inherent_impls.is_empty() {
            writeln!(
                self.output,
                "\n{} Implementations",
                "#".repeat(self.level + 2)
            )
            .unwrap();
            for impl_item in inherent_impls {
                if let ItemEnum::Impl(imp) = &impl_item.inner {
                    self.print_impl_block(impl_item, imp);
                }
            }
        }
        if !trait_impls.is_empty() {
            writeln!(
                self.output,
                "\n{} Trait Implementations",
                "#".repeat(self.level + 2)
            )
            .unwrap();
            for impl_item in trait_impls {
                if let ItemEnum::Impl(imp) = &impl_item.inner {
                    self.print_impl_block(impl_item, imp);
                }
            }
        }
        // TODO: Auto Trait Implementations (might need filtering imp.is_auto)
        // TODO: Blanket Implementations (harder to detect, maybe filter imp.is_blanket if field exists?)
    }

    fn print_impl_block(&mut self, impl_item: &Item, imp: &Impl) {
        let header_level = self.level + 3;
        let mut impl_header = String::new();
        if imp.is_unsafe {
            write!(impl_header, "unsafe ").unwrap();
        }
        write!(
            impl_header,
            "impl{}",
            format_generics(&imp.generics, self.krate)
        )
        .unwrap();

        if let Some(trait_path) = &imp.trait_ {
            // Use format_path with the Path struct
            write!(
                impl_header,
                " {} for",
                format_path(trait_path, self.krate) // Format the full trait path
            )
            .unwrap();
        }
        write!(impl_header, " {}", format_type(&imp.for_, self.krate)).unwrap();

        // Append where clause from impl generics if needed and not already printed
        if !imp.generics.where_predicates.is_empty() && !impl_header.contains("where") {
            let where_clause =
                format_generics_where_only(&imp.generics.where_predicates, self.krate);
            write!(impl_header, " {}", where_clause).unwrap();
        }

        writeln!(
            self.output,
            "\n{} `{}`",
            "#".repeat(header_level),
            impl_header
        )
        .unwrap();
        self.printed_ids.insert(impl_item.id); // Mark the impl item itself as printed

        // Group associated items
        let mut assoc_consts = vec![];
        let mut assoc_types = vec![];
        let mut assoc_fns = vec![];
        for assoc_item_id in &imp.items {
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

        // Print in order: consts, types, fns
        if !assoc_consts.is_empty() {
            writeln!(
                self.output,
                "\n{} Associated Constants",
                "#".repeat(header_level + 1)
            )
            .unwrap();
            for id in assoc_consts {
                self.print_item(id); // Will print AssocConst item
            }
        }
        if !assoc_types.is_empty() {
            writeln!(
                self.output,
                "\n{} Associated Types",
                "#".repeat(header_level + 1)
            )
            .unwrap();
            for id in assoc_types {
                self.print_item(id); // Will print AssocType item
            }
        }
        if !assoc_fns.is_empty() {
            writeln!(
                self.output,
                "\n{} Associated Functions",
                "#".repeat(header_level + 1)
            )
            .unwrap();
            for id in assoc_fns {
                self.print_item(id); // Will print Function item
            }
        }
    }

    fn print_module_contents(&mut self, module: &Module) {
        let mut items_by_kind: HashMap<ItemKind, Vec<Id>> = HashMap::new();
        for id in &module.items {
            if !self.selected_ids.contains(id) {
                continue;
            }
            // Use ItemSummary from krate.paths to get the kind if possible
            if let Some(kind) = self.get_item_kind(id) {
                items_by_kind.entry(kind).or_default().push(*id);
            } else if let Some(item) = self.krate.index.get(id) {
                // Fallback: infer kind from ItemEnum
                let kind = self.infer_item_kind(item);
                items_by_kind.entry(kind).or_default().push(*id);
                warn!(
                    "Used fallback kind detection for item ID {:?} ({:?})",
                    id,
                    item.name.as_deref().unwrap_or("")
                );
            }
        }

        // Defined printing order
        let print_order = [
            ItemKind::Macro, // Includes bang procedural macros
            ItemKind::ProcAttribute,
            ItemKind::ProcDerive,
            ItemKind::Module,
            ItemKind::ExternCrate,
            ItemKind::Use,       // Renamed from Import
            ItemKind::Primitive, // Only if explicitly selected? unlikely
            ItemKind::Union,
            ItemKind::Struct,
            ItemKind::Enum,
            ItemKind::Trait,
            ItemKind::TypeAlias,
            ItemKind::TraitAlias,
            ItemKind::Static,
            ItemKind::Constant,
            ItemKind::Function,
            // ItemKind::Impl, // Handled within struct/enum/trait
            ItemKind::ExternType, // Renamed from ForeignType
            // ItemKind::Variant, // Handled within enum
            // ItemKind::StructField, // Handled within struct/variant
            // ItemKind::AssocConst, // Handled within trait/impl
            // ItemKind::AssocType, // Handled within trait/impl
            ItemKind::Keyword, // Only if explicitly selected? unlikely
        ];

        for kind in print_order {
            if let Some(ids) = items_by_kind.get_mut(&kind) {
                // Sort items by name within each kind
                ids.sort_by_key(|id| {
                    self.krate.index.get(id).and_then(|item| item.name.clone()) // Clone name for sorting
                });

                if !ids.is_empty() {
                    // Add section header only if needed and not implicitly handled
                    let header = match kind {
                        ItemKind::Module => "Modules",
                        ItemKind::Struct => "Structs",
                        ItemKind::Enum => "Enums",
                        ItemKind::Trait => "Traits",
                        ItemKind::Function => "Functions",
                        ItemKind::Macro | ItemKind::ProcAttribute | ItemKind::ProcDerive => {
                            "Macros"
                        }
                        ItemKind::Static => "Statics",
                        ItemKind::Constant => "Constants",
                        ItemKind::TypeAlias => "Type Aliases",
                        ItemKind::TraitAlias => "Trait Aliases",
                        ItemKind::Union => "Unions",
                        ItemKind::Use => "Imports",
                        ItemKind::ExternCrate => "External Crates",
                        ItemKind::ExternType => "External Types",
                        _ => "", // No header for implicitly handled kinds
                    };

                    if !header.is_empty() {
                        // Check if this specific header was already printed (e.g., multiple macro kinds)
                        let full_header = format!("\n{} {}", "#".repeat(self.level + 1), header);
                        // A bit hacky check to avoid duplicate "Macros" headers
                        if !(header == "Macros" && self.output.contains(&full_header)) {
                            writeln!(self.output, "{}", full_header).unwrap();
                        }
                    }

                    for id in ids.clone() {
                        // Clone ids to allow modification of self.printed_ids inside loop
                        if let Some(item) = self.krate.index.get(&id) {
                            match &item.inner {
                                ItemEnum::Module(sub_module) => {
                                    if !self.printed_ids.contains(&id) {
                                        // Avoid re-printing modules already handled? No, print structure.
                                        writeln!(
                                            self.output,
                                            "\n{} Module {}",
                                            "#".repeat(self.level + 1),
                                            item.name.as_deref().unwrap_or("{unnamed}")
                                        )
                                        .unwrap();
                                        self.printed_ids.insert(id); // Mark module printed before recursion
                                        let current_level = self.level;
                                        self.level += 1;
                                        // Print module docs first
                                        if let Some(docs) = &item.docs {
                                            if !docs.trim().is_empty() {
                                                writeln!(self.output, "\n{}", docs.trim()).unwrap();
                                            }
                                        }
                                        self.print_module_contents(sub_module);
                                        self.level = current_level;
                                    }
                                }
                                // Skip Impl items here, they are handled inside Struct/Enum/Trait print methods
                                ItemEnum::Impl(_) => {}
                                // Skip Variant/Field/Assoc items here
                                ItemEnum::Variant(_)
                                | ItemEnum::StructField(_)
                                | ItemEnum::AssocConst { .. }
                                | ItemEnum::AssocType { .. } => {}
                                _ => {
                                    // Only print if not already handled (e.g., via impl block)
                                    if !self.printed_ids.contains(&id) {
                                        self.print_item(&id);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
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
            // ItemEnum::Keyword removed
        }
    }

    fn finalize(mut self) -> String {
        let root_item = self.krate.index.get(&self.krate.root).unwrap(); // Assume root exists
        let crate_name = root_item.name.as_deref().unwrap_or("Unknown Crate");
        let crate_version = self.krate.crate_version.as_deref().unwrap_or("");

        // Use writeln! to ensure newline at the end of the header
        writeln!(
            self.output,
            "{} {} API ({})",
            "#".repeat(self.level),
            crate_name,
            crate_version
        )
        .unwrap();
        self.printed_ids.insert(self.krate.root); // Mark root as "printed"

        if let Some(docs) = &root_item.docs {
            if !docs.trim().is_empty() {
                writeln!(self.output, "\n{}", docs.trim()).unwrap();
            }
        }

        if let ItemEnum::Module(root_module) = &root_item.inner {
            self.print_module_contents(root_module);
        }

        // Check for unprinted selected items
        let mut unprinted_count = 0;
        let mut unprinted_output = String::new();
        let mut temp_output_store = String::new(); // Store the main output temporarily

        // Swap buffers to collect unprinted items separately
        std::mem::swap(&mut self.output, &mut temp_output_store);

        for id in self.selected_ids {
            if !self.printed_ids.contains(id) {
                if unprinted_count == 0 {
                    writeln!(
                        unprinted_output,
                        "\n{} Other Items",
                        "#".repeat(self.level + 1) // Use self.level which is 1 here
                    )
                    .unwrap();
                    warn!("Found selected items that were not printed in the structured output. Adding them to an 'Other Items' section.");
                }
                unprinted_count += 1;
                self.print_item(id); // This now writes to self.output (which was swapped)
                write!(unprinted_output, "{}", self.output).unwrap(); // Append the printed item
                self.output.clear(); // Clear for the next unprinted item
            }
        }

        // Swap back and append unprinted items
        std::mem::swap(&mut self.output, &mut temp_output_store);
        if unprinted_count > 0 {
            write!(self.output, "{}", unprinted_output).unwrap();
            warn!(
                "{} unprinted items were added to the 'Other Items' section.",
                unprinted_count
            );
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
