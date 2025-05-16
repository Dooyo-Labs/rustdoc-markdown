#![allow(clippy::uninlined_format_args)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::cognitive_complexity)] // Allow complex functions for now

use anyhow::{bail, Result};
use cargo_manifest::{FeatureSet, Manifest as CargoManifest}; // Renamed Manifest to CargoManifest
use graph::{Edge, IdGraph, ResolvedModule};
use rustdoc_json::Builder;
use rustdoc_types::{
    Abi, Constant, Crate, Discriminant, Enum, Function, GenericArg, GenericArgs, GenericBound,
    GenericParamDef, Generics, Id, Impl, Item, ItemEnum, ItemKind, Path, PolyTrait, Primitive,
    Struct, StructKind, Term, Trait, Type, Union, Variant, VariantKind, WherePredicate,
};
use std::collections::{HashMap, HashSet}; // Use HashMap instead of BTreeMap where needed
use std::fmt::Write as FmtWrite; // Use FmtWrite alias
use std::hash::{Hash, Hasher};
use std::path::{Path as FilePath, PathBuf}; // Corrected use statement
use tracing::{debug, info, trace, warn};

// Import pulldown-cmark related items
use pulldown_cmark::{Event, Parser as CmarkParser, Tag, TagEnd}; // Import Tag, TagEnd
use pulldown_cmark_to_cmark::cmark;

pub const NIGHTLY_RUST_VERSION: &str = "nightly-2025-03-24";

pub mod cratesio;
pub mod graph;

// --- Manifest Data ---

#[derive(Debug, Clone, Default)]
struct CrateManifestData {
    description: Option<String>,
    homepage: Option<String>,
    repository: Option<String>,
    categories: Vec<String>,
    license: Option<String>,
    rust_version: Option<String>,
    edition: Option<String>,
    features: FeatureSet, // Using cargo-manifest's FeatureSet
}

impl CrateManifestData {
    fn from_cargo_manifest(manifest: &CargoManifest) -> Self {
        let package_data = manifest.package.as_ref();
        CrateManifestData {
            description: package_data
                .and_then(|p| p.description.as_ref())
                .and_then(|d| d.as_ref().as_local())
                .cloned(),
            homepage: package_data
                .and_then(|p| p.homepage.as_ref())
                .and_then(|h| h.as_ref().as_local())
                .cloned(),
            repository: package_data
                .and_then(|p| p.repository.as_ref())
                .and_then(|r| r.as_ref().as_local())
                .cloned(),
            categories: package_data
                .and_then(|p| p.categories.as_ref())
                .and_then(|c| c.as_ref().as_local())
                .cloned()
                .unwrap_or_default(),
            license: package_data
                .and_then(|p| p.license.as_ref())
                .and_then(|l| l.as_ref().as_local())
                .cloned(),
            rust_version: package_data
                .and_then(|p| p.rust_version.as_ref())
                .and_then(|rv| rv.as_ref().as_local())
                .cloned(),
            edition: package_data
                .and_then(|p| p.edition.as_ref())
                .and_then(|e| e.as_ref().as_local())
                .map(|e| e.as_str().to_string()),
            features: manifest.features.clone().unwrap_or_default(),
        }
    }
}

pub fn run_rustdoc(
    crate_dir: &FilePath,
    crate_name: &str,
    features: Option<&str>,
    no_default_features: bool,
    target: Option<&str>,
) -> Result<PathBuf> {
    let manifest_path = crate_dir.join("Cargo.toml");
    if !manifest_path.exists() {
        bail!(
            "Cargo.toml not found in unpacked crate at {}",
            manifest_path.display()
        );
    }

    info!("Generating rustdoc JSON using rustdoc-json crate...");

    let json_output_path = crate_dir
        .join("target/doc")
        .join(format!("{}.json", crate_name));

    // Avoid regenerating if exists
    if json_output_path.exists() {
        info!(
            "rustdoc JSON already exists at: {}",
            json_output_path.display()
        );
        return Ok(json_output_path);
    }

    let mut builder = Builder::default()
        .manifest_path(manifest_path)
        .toolchain(NIGHTLY_RUST_VERSION) // Specify the nightly toolchain
        .target_dir(crate_dir.join("target/doc")) // Set the output directory
        .package(crate_name); // Specify the package

    // Apply feature flags
    if let Some(features_str) = features {
        let feature_list: Vec<String> = features_str.split_whitespace().map(String::from).collect();
        if !feature_list.is_empty() {
            info!("Enabling features: {:?}", feature_list);
            builder = builder.features(feature_list);
        }
    }

    if no_default_features {
        info!("Disabling default features.");
        builder = builder.no_default_features(true);
    }

    // Apply target
    if let Some(target_str) = target {
        info!("Setting target: {}", target_str);
        builder = builder.target(target_str.to_string());
    }

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
            if json_output_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&json_output_path) {
                    eprintln!(
                        "\n--- Potential content of {}: ---\n{}",
                        json_output_path.display(),
                        content
                    );
                }
            }

            bail!("rustdoc-json failed: {}", e);
        }
    }
}

/// Gets the `Id` associated with a type, if it's a path-based type.
pub(crate) fn get_type_id(ty: &Type) -> Option<Id> {
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
        Type::QualifiedPath { self_type, .. } => get_type_id(self_type), // Focus on self_type for impl matching
        Type::ImplTrait(_) => None,
        Type::DynTrait(_) => None,
    }
}

// --- Formatting Helpers ---

/// Formats a list of attributes, filtering out derive attributes.
/// Returns a string like `#[attr1] #[attr2] ` (with a trailing space if not empty).
fn format_attributes(attrs: &[String]) -> String {
    let filtered_attrs: Vec<String> = attrs
        .iter()
        .filter(|attr| !attr.starts_with("#[derive("))
        .cloned()
        .collect();

    if filtered_attrs.is_empty() {
        String::new()
    } else {
        format!("{} ", filtered_attrs.join(" "))
    }
}

/// Helper to check if an item has non-empty documentation.
fn has_docs(item: &Item) -> bool {
    item.docs.as_ref().is_some_and(|d| !d.trim().is_empty())
}

/// Adjusts the markdown header levels in a string using pulldown-cmark.
/// Increases the level of each header (e.g., `#` -> `###`) based on the base level.
/// Caps the maximum level at 6 (`######`).
fn adjust_markdown_headers(markdown: &str, base_level: usize) -> String {
    let parser = CmarkParser::new(markdown);
    let transformed_events = parser.map(|event| match event {
        Event::Start(Tag::Heading {
            level,
            id,
            classes,
            attrs,
        }) => {
            // Explicitly match on HeadingLevel variants to get usize
            let old_level_usize = match level {
                pulldown_cmark::HeadingLevel::H1 => 1,
                pulldown_cmark::HeadingLevel::H2 => 2,
                pulldown_cmark::HeadingLevel::H3 => 3,
                pulldown_cmark::HeadingLevel::H4 => 4,
                pulldown_cmark::HeadingLevel::H5 => 5,
                pulldown_cmark::HeadingLevel::H6 => 6,
            };
            let new_level_usize = std::cmp::min(old_level_usize + base_level, 6);
            let new_level = pulldown_cmark::HeadingLevel::try_from(new_level_usize)
                .unwrap_or(pulldown_cmark::HeadingLevel::H6);
            Event::Start(pulldown_cmark::Tag::Heading {
                level: new_level,
                id,
                classes,
                attrs,
            })
        }
        Event::End(TagEnd::Heading(level)) => {
            // Explicitly match on HeadingLevel variants to get usize
            let old_level_usize = match level {
                pulldown_cmark::HeadingLevel::H1 => 1,
                pulldown_cmark::HeadingLevel::H2 => 2,
                pulldown_cmark::HeadingLevel::H3 => 3,
                pulldown_cmark::HeadingLevel::H4 => 4,
                pulldown_cmark::HeadingLevel::H5 => 5,
                pulldown_cmark::HeadingLevel::H6 => 6,
            };
            let new_level_usize = std::cmp::min(old_level_usize + base_level, 6);
            let new_level = pulldown_cmark::HeadingLevel::try_from(new_level_usize)
                .unwrap_or(pulldown_cmark::HeadingLevel::H6);
            Event::End(pulldown_cmark::TagEnd::Heading(new_level))
        }
        _ => event,
    });

    let mut out_buf = String::with_capacity(markdown.len() + 128); // Pre-allocate slightly
    cmark(transformed_events, &mut out_buf).expect("Markdown formatting failed");
    out_buf
}

/// Indents each line of a string by the specified amount.
fn indent_string(s: &str, amount: usize) -> String {
    let prefix = " ".repeat(amount);
    s.lines()
        .map(|line| format!("{}{}", prefix, line))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Cleans common prefixes like `core::marker::`, `core::ops::`, `alloc::`, `std::` from a path string.
fn clean_trait_path(path_str: &str) -> String {
    path_str
        .replace("core::marker::", "")
        .replace("core::ops::", "") // Add common core paths
        .replace("core::fmt::", "")
        .replace("core::cmp::", "")
        .replace("core::clone::", "")
        .replace("core::hash::", "")
        .replace("core::panic::unwind_safe::", "") // For UnwindSafe/RefUnwindSafe
        // Keep core::option::Option ? Maybe not needed as often in where clauses.
        .replace("core::", "") // General core removal last
        .replace("alloc::string::", "") // Clean alloc paths too
        .replace("alloc::vec::", "")
        .replace("alloc::boxed::", "")
        .replace("alloc::borrow::", "") // For Borrow/BorrowMut/ToOwned
        .replace("alloc::", "") // General alloc removal
        .replace("std::", "") // Also clean std paths potentially used via prelude
}

/// Formats the canonical path to an item ID, using its path from krate.paths.
fn format_id_path_canonical(id: &Id, krate: &Crate) -> String {
    krate
        .paths
        .get(id)
        .map(|p| p.path.join("::"))
        .unwrap_or_else(|| {
            // Fallback if not in paths (e.g., some external or generated IDs)
            krate
                .index
                .get(id)
                .and_then(|item| item.name.as_deref())
                .map_or_else(|| format!("{{id:{}}}", id.0), |name| name.to_string())
        })
}

/// Formats a Path struct, trying to use the canonical path for the ID.
fn format_path(path: &Path, krate: &Crate) -> String {
    // Use the canonical path if available, otherwise use the path string in the struct
    let base_path = format_id_path_canonical(&path.id, krate);

    let cleaned_base_path = clean_trait_path(&base_path); // Clean the base path
                                                          // Use as_ref() to get Option<&GenericArgs> from Option<Box<GenericArgs>>
    if let Some(args) = path.args.as_ref() {
        let args_str = format_generic_args(args, krate);
        if !args_str.is_empty() {
            format!("{}<{}>", cleaned_base_path, args_str) // Use cleaned path
        } else {
            cleaned_base_path // Use cleaned path
        }
    } else {
        cleaned_base_path // Use cleaned path
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
                .map(|lt| format!(" + {}", lt)) // Add quote for lifetime
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
                .map(|lt| format!("{} ", lt)) // Add quote
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
            let args_str = format_generic_args(args, krate);

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

fn format_generic_args(args: &GenericArgs, krate: &Crate) -> String {
    match args {
        GenericArgs::AngleBracketed {
            args, constraints, ..
        } => {
            let arg_strs: Vec<String> = args.iter().map(|a| format_generic_arg(a, krate)).collect();
            let constraint_strs: Vec<String> = constraints
                .iter()
                .map(|c| match c {
                    rustdoc_types::AssocItemConstraint {
                        name,
                        args: assoc_args,
                        binding: rustdoc_types::AssocItemConstraintKind::Equality(term),
                    } => {
                        let assoc_args_str = format_generic_args(assoc_args, krate);
                        format!(
                            "{}{}{}{} = {}",
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
                        let assoc_args_str = format_generic_args(assoc_args, krate);
                        format!(
                            "{}{}{}{}: {}",
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
        GenericArgs::ReturnTypeNotation => String::new(),
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
        GenericArg::Lifetime(lt) => lt.to_string(), // Add quote
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
        GenericBound::Outlives(lifetime) => lifetime.to_string(), // Add quote
        GenericBound::Use(args) => {
            // use<'a, T> syntax
            format!(
                "use<{}>",
                args.iter()
                    .map(|a| match a {
                        rustdoc_types::PreciseCapturingArg::Lifetime(lt) => format!("'{}", lt),
                        rustdoc_types::PreciseCapturingArg::Param(id_str) => id_str.clone(), // Use string name directly
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
        rustdoc_types::GenericParamDefKind::Lifetime { .. } => p.name.to_string(), // Add quote
        rustdoc_types::GenericParamDefKind::Type {
            bounds,
            default,
            is_synthetic,
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
    let params_str = if !generics.params.is_empty() {
        format!(
            "<{}>",
            generics
                .params
                .iter()
                .map(|p| format_generic_param_def(p, krate))
                .collect::<Vec<_>>()
                .join(", ")
        )
    } else {
        String::new()
    };

    let where_clause = format_generics_where_only(&generics.where_predicates, krate);

    if !params_str.is_empty() {
        write!(s, "{}", params_str).unwrap();
    }
    if !where_clause.is_empty() {
        // Add newline and indent if params were also present and where clause is multiline
        if !params_str.is_empty() && where_clause.contains('\n') {
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
                    "{}: {}",
                    lifetime,
                    outlives
                        .iter()
                        .map(|lt| lt.to_string()) // Add quotes
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

/// Category of a trait implementation for display purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TraitImplCategory {
    /// Simple, non-generic, non-blanket, non-auto impls.
    Simple,
    /// Impls with generics, associated items, or other complexities.
    GenericOrComplex,
    /// Auto traits (e.g., Send, Sync).
    Auto,
    /// Blanket implementations.
    Blanket,
}

/// Represents a normalized trait implementation for comparison and storage.
///
/// Apart from the optional `impl_id`, this trait description is decoupled
/// from any specific implementation so it can be used to render common
/// traits for a crate or module. Unless changing the design, keep this
/// comment, otherwise someone will make the mistake of adding implementation
/// state to this struct.
#[derive(Debug, Clone)]
struct FormattedTraitImpl {
    trait_id: Id,
    /// Generics of the trait path itself (e.g., `<'a>` in `Trait<'a>`).
    /// This `Generics` is from `rustdoc_types` and its internal `CowStr` will have lifetime 'a.
    trait_generics: Generics,
    is_unsafe_impl: bool,
    is_negative: bool,
    /// The category this trait implementation falls into.
    category: TraitImplCategory,
    /// The pre-formatted Markdown list entry for this trait.
    formatted_markdown_list_entry: String,
    /// Optionally link back to the real Impl item in the krate index.
    /// This is useful when representing implementation details for a specific
    /// type, as opposed to common trait implementations for a crate / module.
    /// This link helps track what items have been printed so far.
    impl_id: Option<Id>,
}

impl PartialEq for FormattedTraitImpl {
    /// Compares two FormattedTraitImpl instances for equality.
    /// For common trait identification, `impl_id` and `formatted_markdown_list_entry` are ignored.
    fn eq(&self, other: &Self) -> bool {
        self.trait_id == other.trait_id
            && self.trait_generics == other.trait_generics // Compare trait generics structure
            && self.is_unsafe_impl == other.is_unsafe_impl
            && self.is_negative == other.is_negative
            && self.category == other.category // Compare category
    }
}
impl Eq for FormattedTraitImpl {}

impl Hash for FormattedTraitImpl {
    /// Hashes the FormattedTraitImpl instance.
    /// For common trait identification, `impl_id` and `formatted_markdown_list_entry` are ignored.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.trait_id.hash(state);
        self.trait_generics.hash(state); // Hash trait generics structure
        self.is_unsafe_impl.hash(state);
        self.is_negative.hash(state);
        self.category.hash(state); // Hash category
                                   // Do not hash formatted_markdown_list_entry or impl_id for common trait grouping
    }
}

// Helper function to convert GenericArgs to Generics
// This function attempts to create a Generics struct from GenericArgs.
// It's a simplification, primarily for representing the generics *of a path* (like a trait path).
fn generic_args_to_generics(args_opt: Option<Box<GenericArgs>>, krate: &Crate) -> Generics {
    let mut params = Vec::new();
    let mut where_predicates = Vec::new(); // Not typically part of GenericArgs directly

    if let Some(args_box) = args_opt {
        match *args_box {
            GenericArgs::AngleBracketed {
                args, constraints, ..
            } => {
                for arg in args {
                    match arg {
                        GenericArg::Type(t) => {
                            let name = match t {
                                Type::Generic(g_name) => g_name,
                                _ => format_type(&t, krate), // Fallback to formatted type if not simple generic.
                            };
                            params.push(GenericParamDef {
                                name,
                                kind: rustdoc_types::GenericParamDefKind::Type {
                                    bounds: vec![], // Bounds are in `constraints` or `where_predicates`
                                    default: None,
                                    is_synthetic: false,
                                },
                            });
                        }
                        GenericArg::Lifetime(lt_name) => {
                            params.push(GenericParamDef {
                                name: lt_name,
                                kind: rustdoc_types::GenericParamDefKind::Lifetime {
                                    outlives: vec![],
                                },
                            });
                        }
                        GenericArg::Const(c) => {
                            params.push(GenericParamDef {
                                name: c.expr,
                                kind: rustdoc_types::GenericParamDefKind::Const {
                                    type_: Type::Infer, // Type info might be lost here or in `c.type_`
                                    default: None,
                                },
                            });
                        }
                        GenericArg::Infer => {
                            params.push(GenericParamDef {
                                name: "_".to_string(),
                                kind: rustdoc_types::GenericParamDefKind::Type {
                                    bounds: vec![],
                                    default: None,
                                    is_synthetic: true,
                                },
                            });
                        }
                    }
                }
                // Convert AssocItemConstraints to WherePredicates (simplified)
                for constraint in constraints {
                    match constraint {
                        rustdoc_types::AssocItemConstraint {
                            name: assoc_name,
                            args: assoc_args, // GenericArgs for the associated type itself
                            binding: rustdoc_types::AssocItemConstraintKind::Equality(term),
                        } => {
                            // Construct a Type for the LHS: Self::AssocName<Args>
                            let lhs_type = Type::QualifiedPath {
                                name: assoc_name,
                                args: Box::new(assoc_args),
                                self_type: Box::new(Type::Generic("Self".to_string())), // Placeholder "Self"
                                trait_: None, // Assuming it's an associated type on "Self"
                            };
                            where_predicates.push(WherePredicate::EqPredicate {
                                lhs: lhs_type,
                                rhs: term,
                            });
                        }
                        rustdoc_types::AssocItemConstraint {
                            name: assoc_name,
                            args: assoc_args,
                            binding: rustdoc_types::AssocItemConstraintKind::Constraint(bounds),
                        } => {
                            let for_type = Type::QualifiedPath {
                                name: assoc_name,
                                args: Box::new(assoc_args),
                                self_type: Box::new(Type::Generic("Self".to_string())),
                                trait_: None,
                            };
                            where_predicates.push(WherePredicate::BoundPredicate {
                                type_: for_type,
                                bounds,
                                generic_params: vec![], // HRTBs not directly in constraints
                            });
                        }
                    }
                }
            }
            _ => {} // Parenthesized and ReturnTypeNotation don't map cleanly to Generics params/predicates
        }
    }

    Generics {
        params,
        where_predicates,
    }
}

fn get_trait_for_type_generics(item: &Item) -> Option<&Generics> {
    match item.inner {
        ItemEnum::Struct(ref s) => Some(&s.generics),
        ItemEnum::Enum(ref e) => Some(&e.generics),
        ItemEnum::Union(ref u) => Some(&u.generics),
        _ => None,
    }
}

fn trait_impl_has_associated_items(imp: &Impl, krate: &Crate) -> bool {
    imp.items.iter().any(|id| {
        if let Some(item) = krate.index.get(id) {
            matches!(
                item.inner,
                ItemEnum::AssocType { .. } | ItemEnum::AssocConst { .. }
            )
        } else {
            false
        }
    })
}

impl FormattedTraitImpl {
    /// Creates a FormattedTraitImpl from a rustdoc_types::Impl and the krate context.
    fn from_impl(
        imp: &Impl,
        impl_id: Option<Id>,
        trait_path: &Path,
        krate: &Crate,
        printer: &Printer, // Pass printer for generate_impl_trait_block
    ) -> Self {
        let trait_path_str = format_id_path_canonical(&trait_path.id, krate);
        let cleaned_trait_path = clean_trait_path(&trait_path_str);

        let display_path_with_generics = format!(
            "{}{}{}",
            imp.is_negative.then_some("!").unwrap_or_default(),
            cleaned_trait_path,
            if let Some(args) = &trait_path.args {
                let args_str = format_generic_args(args, krate);
                if !args_str.is_empty() {
                    format!("<{}>", args_str)
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        );

        let mut is_passthrough_generic_impl = false;
        if trait_path.args.as_ref().is_none_or(|ga| {
            matches!(ga.as_ref(), GenericArgs::AngleBracketed { args, constraints } if args.is_empty() && constraints.is_empty())
        }) && !trait_impl_has_associated_items(imp, krate)
        {
            if let Type::ResolvedPath(for_path) = &imp.for_ {
                let impl_params: HashMap<&String, &GenericParamDef> =
                    imp.generics.params.iter().map(|p| (&p.name, p)).collect();
                let mut for_path_args_match_impl_params = true;
                let for_type_id = for_path.id;
                if let Some(for_type_item) = krate.index.get(&for_type_id) {
                    if let Some(for_generics) = get_trait_for_type_generics(for_type_item) {
                        if let Some(for_args_box) = &for_path.args {
                            if let GenericArgs::AngleBracketed { args: for_args, .. } =
                                for_args_box.as_ref()
                            {
                                if for_generics.params.len() == for_args.len()
                                    && for_generics.params.len() == impl_params.len()
                                    && for_generics.where_predicates == imp.generics.where_predicates
                                {
                                    for (for_arg, for_param) in
                                        for_args.iter().zip(for_generics.params.iter())
                                    {
                                        let for_arg_name = match for_arg {
                                            GenericArg::Lifetime(name) => Some(name),
                                            GenericArg::Type(Type::Generic(param)) => Some(param),
                                            _ => None,
                                        };
                                        if let Some(for_arg_name) = for_arg_name {
                                            if let Some(impl_param) = impl_params.get(for_arg_name)
                                            {
                                                match (&impl_param.kind, &for_param.kind) {
                                                    (rustdoc_types::GenericParamDefKind::Lifetime { outlives: impl_outlives }, rustdoc_types::GenericParamDefKind::Lifetime { outlives: for_outlives }) => {
                                                        if impl_outlives != for_outlives {
                                                            for_path_args_match_impl_params = false;
                                                        }
                                                    },
                                                    (rustdoc_types::GenericParamDefKind::Type { bounds: impl_bounds, .. }, rustdoc_types::GenericParamDefKind::Type { bounds: for_bounds, .. }) => {
                                                        if impl_bounds != for_bounds {
                                                            for_path_args_match_impl_params = false;
                                                        }
                                                    },
                                                    (rustdoc_types::GenericParamDefKind::Const { .. }, rustdoc_types::GenericParamDefKind::Const { .. }) => {},
                                                    _ => {
                                                        for_path_args_match_impl_params = false;
                                                        break;
                                                    }
                                                }
                                            } else {
                                                for_path_args_match_impl_params = false;
                                                break;
                                            }
                                        } else {
                                            for_path_args_match_impl_params = false;
                                            break;
                                        }
                                    }
                                } else {
                                    for_path_args_match_impl_params = false;
                                }
                            } else {
                                for_path_args_match_impl_params = false;
                            }
                        } else {
                            if !impl_params.is_empty() {
                                for_path_args_match_impl_params = false;
                            }
                        }
                    } else {
                        for_path_args_match_impl_params = false;
                    }
                } else {
                    for_path_args_match_impl_params = false;
                }
                if for_path_args_match_impl_params && imp.generics.where_predicates.is_empty() {
                    is_passthrough_generic_impl = true;
                }
            }
        }

        let category = if imp.is_synthetic {
            TraitImplCategory::Auto
        } else if imp.blanket_impl.is_some() {
            TraitImplCategory::Blanket
        } else if is_passthrough_generic_impl && !trait_impl_has_associated_items(imp, krate) {
            TraitImplCategory::Simple
        } else {
            TraitImplCategory::GenericOrComplex
        };

        let mut list_entry = String::new();
        match category {
            TraitImplCategory::Simple | TraitImplCategory::Auto => {
                write!(list_entry, "- `{}`", display_path_with_generics).unwrap();
            }
            TraitImplCategory::GenericOrComplex => {
                // Need a mutable clone of printer to call generate_impl_trait_block
                let mut temp_printer = printer.clone_with_new_output();
                if let Some(impl_block_str) = temp_printer.generate_impl_trait_block(imp) {
                    if !impl_block_str.trim_end_matches("{\n}").trim().is_empty() {
                        writeln!(list_entry, "- `{}`", display_path_with_generics).unwrap();
                        writeln!(list_entry).unwrap();
                        let full_code_block = format!("```rust\n{}\n```", impl_block_str);
                        let indented_block = indent_string(&full_code_block, 4);
                        write!(list_entry, "{}\n", indented_block).unwrap(); // Keep trailing newline from indent
                    } else {
                        write!(list_entry, "- `{}`", display_path_with_generics).unwrap();
                    }
                } else {
                    write!(list_entry, "- `{}`", display_path_with_generics).unwrap();
                }
            }
            TraitImplCategory::Blanket => {
                let where_clause =
                    format_generics_where_only(&imp.generics.where_predicates, krate);
                if !where_clause.is_empty() {
                    if where_clause.lines().count() == 1 {
                        write!(
                            list_entry,
                            "- `{}` (`{}`)",
                            display_path_with_generics, where_clause,
                        )
                        .unwrap();
                    } else {
                        writeln!(list_entry, "- `{}`", display_path_with_generics).unwrap();
                        let code_block = format!("```rust\n{}\n```", where_clause);
                        let indented_block = indent_string(&code_block, 4);
                        write!(list_entry, "\n{}\n", indented_block).unwrap(); // Keep trailing newline
                    }
                } else {
                    write!(list_entry, "- `{}`", display_path_with_generics).unwrap();
                }
            }
        }

        FormattedTraitImpl {
            trait_id: trait_path.id,
            trait_generics: generic_args_to_generics(trait_path.args.clone(), krate),
            is_unsafe_impl: imp.is_unsafe,
            is_negative: imp.is_negative,
            category,
            formatted_markdown_list_entry: list_entry.trim_end().to_string(), // Trim trailing newline for consistency
            impl_id,
        }
    }

    /// Retrieves the `Impl` struct from the crate index if `impl_id` is Some.
    fn get_impl_data<'krate_borrow>(
        &self,
        krate: &'krate_borrow Crate,
    ) -> Option<(&'krate_borrow Impl, Id)> {
        self.impl_id
            .and_then(|id| krate.index.get(&id))
            .and_then(|item| match &item.inner {
                ItemEnum::Impl(imp_data) => Some((imp_data, item.id)),
                _ => None,
            })
    }

    /// Checks if the trait (referenced by trait_id) has any associated types or consts.
    #[allow(dead_code)] // Potentially useful later
    fn has_associated_items(&self, krate: &Crate) -> bool {
        if let Some(trait_item) = krate.index.get(&self.trait_id) {
            if let ItemEnum::Trait(trait_data) = &trait_item.inner {
                return trait_data.items.iter().any(|assoc_id| {
                    if let Some(assoc_item_details) = krate.index.get(assoc_id) {
                        matches!(
                            assoc_item_details.inner,
                            ItemEnum::AssocType { .. } | ItemEnum::AssocConst { .. }
                        )
                    } else {
                        false
                    }
                });
            }
        }
        false
    }
}

/// Generates the primary declaration string for an item (e.g., `struct Foo`, `fn bar()`).
/// For functions, this is deliberately simplified (no attrs, no where clause).
/// For traits, structs, and enums, prepends the current module path.
fn generate_item_declaration(item: &Item, krate: &Crate, current_module_path: &[String]) -> String {
    let name = item.name.as_deref().unwrap_or(match &item.inner {
        ItemEnum::StructField(_) => "{unnamed_field}", // Special case for unnamed fields
        _ => "{unnamed}",
    });
    match &item.inner {
        ItemEnum::Struct(s) => {
            let mut fq_path_parts = current_module_path.to_vec();
            if !name.is_empty() {
                fq_path_parts.push(name.to_string());
            }
            let fq_path = fq_path_parts.join("::");
            format!(
                "struct {}{}",
                fq_path,
                format_generics_params_only(&s.generics.params, krate)
            )
        }
        ItemEnum::Enum(e) => {
            let mut fq_path_parts = current_module_path.to_vec();
            if !name.is_empty() {
                fq_path_parts.push(name.to_string());
            }
            let fq_path = fq_path_parts.join("::");
            format!(
                "enum {}{}",
                fq_path,
                format_generics_params_only(&e.generics.params, krate)
            )
        }
        ItemEnum::Union(u) => {
            let mut fq_path_parts = current_module_path.to_vec();
            if !name.is_empty() {
                fq_path_parts.push(name.to_string());
            }
            let fq_path = fq_path_parts.join("::");
            format!(
                "union {}{}",
                fq_path,
                format_generics_params_only(&u.generics.params, krate)
            )
        }
        ItemEnum::Trait(t) => {
            let unsafe_kw = if t.is_unsafe { "unsafe " } else { "" };
            let auto = if t.is_auto { "auto " } else { "" };
            let mut fq_path_parts = current_module_path.to_vec();
            if !name.is_empty() {
                fq_path_parts.push(name.to_string());
            }
            let fq_path = fq_path_parts.join("::");

            format!(
                "{}{}{}{}{}",
                auto,
                unsafe_kw,
                "trait ",
                fq_path, // Use fully qualified path
                format_generics_params_only(&t.generics.params, krate)
            )
        }
        ItemEnum::Function(f) => {
            // Simplified version for the header: no where clause, but include attributes
            let mut code = String::new();
            write!(code, "{}", format_attributes(&item.attrs)).unwrap(); // Add attributes
            write!(code, "fn {}", name).unwrap();
            // Include only param generics here
            write!(
                code,
                "{}",
                format_generics_params_only(&f.generics.params, krate)
            )
            .unwrap();
            write!(code, "(").unwrap();
            let args_str = f
                .sig
                .inputs
                .iter()
                .map(|(n, t)| format!("{}: {}", n, format_type(t, krate))) // Use arg name from tuple
                .collect::<Vec<_>>()
                .join(", ");
            write!(code, "{}", args_str).unwrap();
            if f.sig.is_c_variadic {
                write!(code, ", ...").unwrap();
            }
            write!(code, ")").unwrap();
            if let Some(output_type) = &f.sig.output {
                write!(code, " -> {}", format_type(output_type, krate)).unwrap();
            }
            code
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
        ItemEnum::Module(_) => format!("mod {}", name), // Use simple name for items within modules
        ItemEnum::ExternCrate {
            name: crate_name, ..
        } => format!("extern crate {}", crate_name),
        ItemEnum::Use(_) => format!("use {}", name), // Basic format for Use items
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
    let name = item
        .name
        .as_deref()
        .expect("Struct item should have a name");
    let mut code = String::new();
    write!(
        code,
        "{}pub struct {}",
        format_attributes(&item.attrs),
        name
    )
    .unwrap();
    // Use full generics here, including where clause
    let generics_str = format_generics_full(&s.generics, krate);
    let where_is_multiline = generics_str.contains("where\n");
    write!(code, "{}", generics_str).unwrap();

    match &s.kind {
        StructKind::Plain { fields, .. } => {
            // fields_stripped ignored
            if where_is_multiline {
                write!(code, " {{").unwrap(); // Open brace on same line as multiline where
            } else {
                write!(code, " {{").unwrap(); // Open brace on same line as generics or no generics
            }

            if !fields.is_empty() {
                writeln!(code).unwrap();
            }
            for field_id in fields {
                if let Some(field_item) = krate.index.get(field_id) {
                    if let ItemEnum::StructField(field_type) = &field_item.inner {
                        let field_name = field_item.name.as_deref().unwrap_or("_");
                        writeln!(
                            code,
                            "    {}pub {}: {},",
                            format_attributes(&field_item.attrs),
                            field_name,
                            format_type(field_type, krate)
                        )
                        .unwrap();
                    }
                }
            }
            if !fields.is_empty() && !code.ends_with('\n') {
                writeln!(code).unwrap();
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
                                Some(format!(
                                    "{}pub {}",
                                    format_attributes(&field_item.attrs),
                                    format_type(field_type, krate)
                                ))
                            } else {
                                None
                            }
                        })
                })
                .collect();
            write!(code, "{}", field_types.join(", ")).unwrap();
            write!(code, ")").unwrap();
            // Add semicolon only if where clause didn't add one implicitly via multiline format
            if !where_is_multiline {
                write!(code, ";").unwrap();
            }
        }
        StructKind::Unit => {
            // Add semicolon only if where clause didn't add one implicitly
            if !where_is_multiline {
                write!(code, ";").unwrap();
            }
        }
    }
    code
}

/// Generates the `enum { ... }` code block.
fn generate_enum_code_block(item: &Item, e: &Enum, krate: &Crate) -> String {
    let name = item.name.as_deref().expect("Enum item should have a name");
    let mut code = String::new();
    write!(code, "{}pub enum {}", format_attributes(&item.attrs), name).unwrap();
    let generics_str = format_generics_full(&e.generics, krate);
    write!(code, "{}", generics_str).unwrap();
    write!(code, " {{").unwrap();

    if !e.variants.is_empty() {
        writeln!(code).unwrap();
    }
    for variant_id in &e.variants {
        if let Some(variant_item) = krate.index.get(variant_id) {
            if let ItemEnum::Variant(variant_data) = &variant_item.inner {
                write!(
                    code,
                    "    {}",
                    format_variant_definition(variant_item, variant_data, krate) // Pass variant_item
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
    if !e.variants.is_empty() && !code.ends_with('\n') {
        writeln!(code).unwrap();
    }
    write!(code, "}}").unwrap();
    code
}

/// Generates the `union { ... }` code block.
fn generate_union_code_block(item: &Item, u: &Union, krate: &Crate) -> String {
    let name = item.name.as_deref().expect("Union item should have a name");
    let mut code = String::new();
    write!(code, "{}pub union {}", format_attributes(&item.attrs), name).unwrap();
    let generics_str = format_generics_full(&u.generics, krate);
    write!(code, "{}", generics_str).unwrap();
    write!(code, " {{").unwrap();

    if !u.fields.is_empty() {
        writeln!(code).unwrap();
    }
    for field_id in &u.fields {
        if let Some(field_item) = krate.index.get(field_id) {
            if let ItemEnum::StructField(field_type) = &field_item.inner {
                let field_name = field_item.name.as_deref().unwrap_or("_");
                writeln!(
                    code,
                    "    {}pub {}: {},",
                    format_attributes(&field_item.attrs),
                    field_name,
                    format_type(field_type, krate)
                )
                .unwrap();
            }
        }
    }
    if !u.fields.is_empty() && !code.ends_with('\n') {
        writeln!(code).unwrap();
    }
    write!(code, "}}").unwrap();
    code
}

/// Generates the full trait declaration code block.
fn generate_trait_code_block(item: &Item, t: &Trait, krate: &Crate) -> String {
    let name = item.name.as_deref().expect("Trait item should have a name");
    let mut code = String::new();

    // Attributes for the trait itself are not typically shown here, but on the `pub trait` line.
    // However, `format_attributes` is for non-derive. If other attributes are common, they could be added.
    // For now, sticking to `pub auto/unsafe trait ...`
    write!(code, "{}", format_attributes(&item.attrs)).unwrap();

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
        if where_clause.contains('\n') {
            write!(code, " {{").unwrap(); // Open brace on same line as multiline where
        } else {
            write!(code, " {{").unwrap(); // Open brace on same line as signature
        }
        writeln!(code).unwrap();

        // Print associated items (simple versions)
        for item_id in &t.items {
            if let Some(assoc_item) = krate.index.get(item_id) {
                match &assoc_item.inner {
                    ItemEnum::AssocConst { type_, value, .. } => {
                        write!(
                            code,
                            "    {}const {}: {}",
                            format_attributes(&assoc_item.attrs),
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
                            "    {}type {}",
                            format_attributes(&assoc_item.attrs),
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
                            generate_function_code_block(assoc_item, f, krate) // Attributes handled by generate_function_code_block
                        )
                        .unwrap();
                    }
                    _ => {} // Ignore others
                }
            }
        }
        if !code.ends_with('\n') {
            writeln!(code).unwrap();
        }
        write!(code, "}}").unwrap();
    }
    code
}

/// Generates the full function signature for a code block.
fn generate_function_code_block(item: &Item, f: &Function, krate: &Crate) -> String {
    let name = item.name.as_deref().expect("Function should have a name");
    let mut code = String::new();

    // Attributes/Keywords
    write!(code, "{}", format_attributes(&item.attrs)).unwrap(); // Add attributes
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
    let generics_str = format_generics_full(&f.generics, krate);
    let where_is_multiline = generics_str.contains("where\n");
    write!(code, "{}", generics_str).unwrap();

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
        // Correctly write to the 'code' buffer
        write!(code, ", ...").unwrap();
    }
    write!(code, ")").unwrap();

    // Return type
    if let Some(output_type) = &f.sig.output {
        write!(code, " -> {}", format_type(output_type, krate)).unwrap();
    }

    // Add semicolon or body indicator based on if it has implementation
    if f.has_body {
        if where_is_multiline {
            write!(code, " {{ ... }}").unwrap(); // Body on same line as multiline where
        } else {
            write!(code, " {{ ... }}").unwrap(); // Body on same line
        }
    } else if !where_is_multiline {
        // Add semicolon if it's just a declaration and doesn't already end with one (e.g., from multiline where clause)
        write!(code, ";").unwrap();
    }

    code
}

/// Formats a single enum variant's definition for the code block.
fn format_variant_definition(item: &Item, v: &Variant, krate: &Crate) -> String {
    let name = item.name.as_deref().unwrap_or("{Unnamed}");
    let attrs_str = format_attributes(&item.attrs);
    match &v.kind {
        VariantKind::Plain => format!("{}{}", attrs_str, name),
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
                                Some(format!(
                                    "{}{}", // No pub for tuple variant fields
                                    format_attributes(&field_item.attrs),
                                    format_type(ty, krate)
                                ))
                            } else {
                                None
                            }
                        })
                })
                .collect();
            format!("{}{}({})", attrs_str, name, types.join(", "))
        }
        VariantKind::Struct { fields, .. } => {
            // fields_stripped ignored
            let fields_str: Vec<String> = fields
                .iter()
                .filter_map(|id| {
                    krate.index.get(id).and_then(|field_item| {
                        if let ItemEnum::StructField(ty) = &field_item.inner {
                            let field_name = field_item.name.as_deref().unwrap_or("_");
                            Some(format!(
                                "{}{}: {}", // No pub for struct variant fields
                                format_attributes(&field_item.attrs),
                                field_name,
                                format_type(ty, krate)
                            ))
                        } else {
                            None
                        }
                    })
                })
                .collect();
            format!("{}{}{{ {} }}", attrs_str, name, fields_str.join(", "))
        }
    }
}

/// Formats an enum variant's signature for the `#####` header.
fn format_variant_signature(item: &Item, v: &Variant, krate: &Crate) -> String {
    // Similar to definition but potentially simpler, without pub, maybe add discriminant visually
    // Attributes are NOT included in the Hx header for variants.
    let name = item.name.as_deref().unwrap_or("{Unnamed}");
    let mut sig = match &v.kind {
        VariantKind::Plain => name.to_string(),
        VariantKind::Tuple(fields) => {
            let types: Vec<String> = fields
                .iter()
                .filter_map(|opt_id| {
                    opt_id
                        .as_ref()
                        .and_then(|id| krate.index.get(id))
                        .and_then(|field_item| {
                            if let ItemEnum::StructField(ty) = &field_item.inner {
                                Some(format_type(ty, krate)) // No attributes here
                            } else {
                                None
                            }
                        })
                })
                .collect();
            format!("{}({})", name, types.join(", "))
        }
        VariantKind::Struct { fields, .. } => {
            let fields_str: Vec<String> = fields
                .iter()
                .filter_map(|id| {
                    krate.index.get(id).and_then(|field_item| {
                        if let ItemEnum::StructField(ty) = &field_item.inner {
                            let field_name = field_item.name.as_deref().unwrap_or("_");
                            Some(format!("{}: {}", field_name, format_type(ty, krate)))
                        // No attributes here
                        } else {
                            None
                        }
                    })
                })
                .collect();
            format!("{} {{ {} }}", name, fields_str.join(", "))
        }
    };

    if let Some(discr) = &v.discriminant {
        // Use format_discriminant_expr
        write!(sig, " = {}", format_discriminant_expr(discr)).unwrap();
    }
    sig
}

/// Represents the module hierarchy.
#[derive(Debug, Default, Clone)] // Added Clone derive
struct ModuleTree {
    /// Maps a module ID to its direct submodule IDs.
    children: HashMap<Id, Vec<Id>>,
    /// Stores the IDs of all known modules.
    all_modules: HashSet<Id>,
    /// Stores the IDs of top-level modules (excluding crate root).
    top_level_modules: Vec<Id>,
}

/// `Printer` is responsible for generating Markdown documentation from a `rustdoc_types::Crate`.
///
/// It uses a builder pattern for configuration. The typical workflow is:
/// 1. Create a `Printer` with `Printer::new(&manifest, &krate)`.
/// 2. Configure it using builder methods like `paths()`, `readme()`, etc.
/// 3. Call `print()` to generate the Markdown string.
///
/// The "Common Traits" feature summarizes traits frequently implemented by types within
/// the crate or specific modules. This can be disabled using `no_common_traits()`.
pub struct Printer<'a> {
    krate: &'a Crate,
    manifest_data: CrateManifestData,
    // Builder options
    paths: Vec<String>,
    readme_content: Option<String>,
    examples_readme_content: Option<String>,
    examples: Vec<(String, String)>, // Store multiple examples
    include_other: bool,
    template_mode: bool,
    no_common_traits: bool,
    // Internal state
    selected_ids: HashSet<Id>,
    resolved_modules: HashMap<Id, ResolvedModule>,
    graph: IdGraph,
    printed_ids: HashMap<Id, String>, // Stores ID and the header prefix where it was first printed
    output: String,
    module_tree: ModuleTree,
    doc_path: Vec<usize>,
    current_module_path: Vec<String>,
    crate_common_traits: HashSet<FormattedTraitImpl>,
    all_type_ids_with_impls: HashSet<Id>,
    module_common_traits: HashMap<Id, HashSet<FormattedTraitImpl>>,
}

impl<'a> Printer<'a> {
    /// Creates a new `Printer` instance.
    ///
    /// # Arguments
    ///
    /// * `manifest`: The parsed `Cargo.toml` data for the crate.
    /// * `krate`: The `rustdoc_types::Crate` data produced by rustdoc.
    pub fn new(manifest: &'a CargoManifest, krate: &'a Crate) -> Self {
        Printer {
            krate,
            manifest_data: CrateManifestData::from_cargo_manifest(manifest),
            paths: Vec::new(),
            readme_content: None,
            examples_readme_content: None,
            examples: Vec::new(),
            include_other: false,
            template_mode: false,
            no_common_traits: false,
            selected_ids: HashSet::new(), // Will be populated by print()
            resolved_modules: HashMap::new(), // Will be populated by print()
            graph: IdGraph::default(),    // Will be populated by print()
            printed_ids: HashMap::new(),  // Changed to HashMap
            output: String::new(),
            module_tree: Self::build_module_tree(krate), // Initial build based on krate
            doc_path: Vec::new(),
            current_module_path: vec![],
            crate_common_traits: HashSet::new(), // Will be populated by print()
            all_type_ids_with_impls: HashSet::new(), // Will be populated by print()
            module_common_traits: HashMap::new(), // Will be populated during printing
        }
    }

    /// Sets the item path filters for documentation generation.
    ///
    /// Items matching these paths (and their dependencies) will be included.
    /// Paths starting with `::` imply the root of the current crate (e.g., `::my_module::MyStruct`).
    /// Paths without `::` are assumed to be relative to the crate root (e.g., `my_module::MyStruct` is treated as `crate_name::my_module::MyStruct`).
    /// Matches are prefix-based (e.g., "::style" matches "::style::TextStyle").
    /// If no paths are provided, all items are considered for selection (default behavior).
    pub fn paths(mut self, paths: &[String]) -> Self {
        self.paths = paths.to_vec();
        self
    }

    /// Adds README content to be included in the documentation.
    ///
    /// The content should be a string containing the Markdown text of the README.
    /// It will be placed after the crate's manifest details and before common traits or module listings.
    /// Headers within the README will be adjusted to fit the overall document structure.
    pub fn readme(mut self, content: String) -> Self {
        self.readme_content = Some(content);
        self
    }

    /// Adds an example to be included in the "Examples Appendix".
    ///
    /// # Arguments
    ///
    /// * `name`: The filename or identifier for the example (e.g., "simple.rs").
    /// * `content`: The Rust code content of the example.
    ///
    /// Examples are listed at the end of the generated documentation.
    pub fn example(mut self, name: String, content: String) -> Self {
        self.examples.push((name, content));
        self
    }

    /// Adds the content of `examples/README.md` to be included before other examples
    /// in the "Examples Appendix".
    pub fn examples_readme(mut self, content: String) -> Self {
        self.examples_readme_content = Some(content);
        self
    }

    /// Includes items that don't fit standard categories in a final "Other" section.
    ///
    /// By default, such items are logged as warnings and not included.
    /// If enabled, these items will appear at the end of the documentation,
    /// potentially with their source location and graph context.
    pub fn include_other(mut self) -> Self {
        self.include_other = true;
        self
    }

    /// Enables template mode.
    ///
    /// In template mode, instead of item documentation, Mustache-like markers
    /// (e.g., `{{MISSING_DOCS_1_2_1}}`) are inserted where documentation
    /// for an item would normally appear. This is useful for identifying
    /// where documentation is present or missing in the source crate.
    /// The default is `false`.
    pub fn template_mode(mut self) -> Self {
        self.template_mode = true;
        self
    }

    /// Disables the "Common Traits" sections.
    ///
    /// By default, traits frequently implemented by types within the crate or
    /// specific modules are summarized in "Common Traits" sections. If this method
    /// is called, these summary sections are omitted, and all implemented traits
    /// for each item will be listed directly with that item.
    pub fn no_common_traits(mut self) -> Self {
        self.no_common_traits = true;
        self
    }

    /// Generates the Markdown documentation based on the configured options.
    ///
    /// This method consumes the `Printer` and returns the generated Markdown as a `String`.
    pub fn print(mut self) -> Result<String> {
        self.resolved_modules = graph::build_resolved_module_index(self.krate);
        let (selected_ids, graph) =
            graph::select_items(self.krate, &self.paths, &self.resolved_modules)?;
        self.selected_ids = selected_ids;
        self.graph = graph;

        info!(
            "Generating documentation for {} selected items.",
            self.selected_ids.len()
        );
        if self.selected_ids.is_empty() && self.examples.is_empty() {
            return Ok("No items selected for documentation and no examples found.".to_string());
        }

        let (crate_common_traits, all_type_ids_with_impls) = Self::calculate_crate_common_traits(
            self.krate,
            &self.selected_ids, // Pass reference directly
            self.no_common_traits,
            self, // Pass self for FormattedTraitImpl::from_impl
        );
        self.crate_common_traits = crate_common_traits;
        self.all_type_ids_with_impls = all_type_ids_with_impls;

        // The finalize method consumes self and returns the String
        Ok(self.finalize())
    }

    /// Pre-calculates common traits for the entire crate.
    fn calculate_crate_common_traits<'krate_lifetime>(
        krate: &'krate_lifetime Crate,
        selected_ids: &HashSet<Id>, // Accept any lifetime for the HashSet ref
        no_common_traits: bool,
        printer: &Printer, // Pass printer for FormattedTraitImpl::from_impl
    ) -> (HashSet<FormattedTraitImpl>, HashSet<Id>) {
        let mut all_type_ids_with_impls = HashSet::new();
        if no_common_traits {
            // Still calculate all_type_ids_with_impls for other logic if needed
            for item in krate.index.values() {
                if let ItemEnum::Impl(imp) = &item.inner {
                    if let Some(for_type_id) = get_type_id(&imp.for_) {
                        if selected_ids.contains(&for_type_id) {
                            all_type_ids_with_impls.insert(for_type_id);
                        }
                    }
                }
            }
            return (HashSet::new(), all_type_ids_with_impls);
        }

        let mut trait_counts: HashMap<FormattedTraitImpl, usize> = HashMap::new();
        for item in krate.index.values() {
            if let ItemEnum::Impl(imp) = &item.inner {
                if let Some(for_type_id) = get_type_id(&imp.for_) {
                    if selected_ids.contains(&for_type_id) {
                        all_type_ids_with_impls.insert(for_type_id);
                        if let Some(trait_path) = &imp.trait_ {
                            let norm_impl = FormattedTraitImpl::from_impl(
                                imp, None, trait_path, krate, printer,
                            );
                            *trait_counts.entry(norm_impl).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
        debug!(
            "Found {} types with trait implementations for crate-level common trait calculation.",
            all_type_ids_with_impls.len()
        );

        let mut common_traits_set = HashSet::new();
        if all_type_ids_with_impls.is_empty() {
            return (common_traits_set, all_type_ids_with_impls);
        }

        let threshold = (all_type_ids_with_impls.len() as f32 * 0.5).ceil() as usize;
        debug!(
            "Crate common trait threshold: {} (out of {} types)",
            threshold,
            all_type_ids_with_impls.len()
        );

        for (norm_impl, count) in trait_counts {
            if count >= threshold {
                common_traits_set.insert(norm_impl.clone());
                debug!(
                    "Identified crate-common trait: {:?} (implemented by {} of {} types)",
                    norm_impl.formatted_markdown_list_entry, // Use formatted entry for debug
                    count,
                    all_type_ids_with_impls.len()
                );
            } else {
                trace!(
                    "Trait {:?} not common enough ({} of {} types)",
                    norm_impl.formatted_markdown_list_entry, // Use formatted entry for debug
                    count,
                    all_type_ids_with_impls.len()
                );
            }
        }
        (common_traits_set, all_type_ids_with_impls)
    }

    /// Calculates common traits for a specific module.
    fn calculate_module_common_traits(&self, module_id: &Id) -> HashSet<FormattedTraitImpl> {
        if self.no_common_traits {
            return HashSet::new();
        }

        let mut module_common_traits = self.crate_common_traits.clone();

        if let Some(resolved_mod) = self.resolved_modules.get(module_id) {
            let mut module_types_considered = HashSet::new();
            for item_id in &resolved_mod.items {
                if let Some(item) = self.krate.index.get(item_id) {
                    if matches!(
                        item.inner,
                        ItemEnum::Struct(_)
                            | ItemEnum::Enum(_)
                            | ItemEnum::Union(_)
                            | ItemEnum::Primitive(_)
                    ) && self.selected_ids.contains(item_id)
                    {
                        let has_impls = self.krate.index.values().any(|idx_item| {
                            if let ItemEnum::Impl(imp) = &idx_item.inner {
                                if let Some(for_id) = get_type_id(&imp.for_) {
                                    return for_id == *item_id;
                                }
                            }
                            false
                        });
                        if has_impls {
                            module_types_considered.insert(*item_id);
                        }
                    }
                }
            }
            let module_types_with_impls_count = module_types_considered.len();
            if module_types_with_impls_count <= 1 {
                return module_common_traits;
            }

            let mut trait_counts: HashMap<FormattedTraitImpl, usize> = HashMap::new();
            for item_id in &module_types_considered {
                for krate_item in self.krate.index.values() {
                    if let ItemEnum::Impl(imp) = &krate_item.inner {
                        if let Some(for_id) = get_type_id(&imp.for_) {
                            if for_id == *item_id {
                                if let Some(trait_path) = &imp.trait_ {
                                    let norm_impl = FormattedTraitImpl::from_impl(
                                        imp, None, trait_path, self.krate, self,
                                    );
                                    *trait_counts.entry(norm_impl).or_insert(0) += 1;
                                }
                            }
                        }
                    }
                }
            }

            let threshold = (module_types_with_impls_count as f32 * 0.5).ceil() as usize;
            debug!(
                "Module {:?} common trait threshold: {} (out of {} types in module)",
                module_id, threshold, module_types_with_impls_count
            );

            for (norm_impl, count) in trait_counts {
                if count >= threshold {
                    if module_common_traits.insert(norm_impl.clone()) {
                        debug!(
                            "Identified module-specific common trait for {:?}: {:?} (implemented by {} of {} module types)",
                            self.krate.paths.get(module_id).map(|p|p.path.join("::")).unwrap_or_default(),
                            norm_impl.formatted_markdown_list_entry, // Use formatted entry
                            count,
                            module_types_with_impls_count
                        );
                    }
                } else {
                    trace!(
                        "Module {:?} trait {:?} not common enough ({} of {} module types)",
                        self.krate
                            .paths
                            .get(module_id)
                            .map(|p| p.path.join("::"))
                            .unwrap_or_default(),
                        norm_impl.formatted_markdown_list_entry, // Use formatted entry
                        count,
                        module_types_with_impls_count
                    );
                }
            }
        }
        module_common_traits
    }

    /// Builds the module hierarchy tree.
    fn build_module_tree(krate: &'a Crate) -> ModuleTree {
        let mut tree = ModuleTree::default();
        let mut parent_map: HashMap<Id, Id> = HashMap::new(); // Child -> Parent

        for (id, item) in &krate.index {
            if let ItemEnum::Module(module_data) = &item.inner {
                tree.all_modules.insert(*id);
                let mut children = Vec::new();
                for child_id in &module_data.items {
                    if let Some(child_item) = krate.index.get(child_id) {
                        if let ItemEnum::Module(_) = child_item.inner {
                            children.push(*child_id);
                            parent_map.insert(*child_id, *id);
                        }
                    }
                }
                if !children.is_empty() {
                    // Sort children alphabetically by name/path here for consistent ordering within a parent
                    children.sort_by_key(|child_id| {
                        krate
                            .paths
                            .get(child_id)
                            .map(|p| p.path.join("::"))
                            .unwrap_or_default()
                    });
                    tree.children.insert(*id, children);
                }
            }
        }

        // Identify top-level modules (excluding crate root)
        for module_id in &tree.all_modules {
            if *module_id != krate.root && !parent_map.contains_key(module_id) {
                tree.top_level_modules.push(*module_id);
            }
        }

        // Sort top-level modules alphabetically by path
        tree.top_level_modules.sort_by_key(|id| {
            krate
                .paths
                .get(id)
                .map(|p| p.path.join("::"))
                .unwrap_or_default()
        });

        tree
    }

    /// Gets the current markdown header level based on the doc_path length.
    fn get_current_header_level(&self) -> usize {
        self.doc_path.len() + 1 // H1 if path is empty, H2 if path has one element, etc.
    }

    /// Generates the header prefix string (e.g., "1.2.1:") based on the doc_path stack.
    /// H2 headers always get just "N:". H3+ get the full path "N.M.O:".
    fn get_header_prefix(&self) -> String {
        let level = self.get_current_header_level();
        if self.doc_path.is_empty() || level < 2 {
            return String::new(); // No prefix for H1 or if stack is empty
        }

        if level == 2 {
            // Module headers (H2) use only the last element (the H2 counter)
            format!("{}:", self.doc_path.last().unwrap_or(&0))
        } else {
            // H3+ headers use the full path stored in doc_path
            self.doc_path
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(".")
                + ":"
        }
    }

    /// Generates the template marker string (e.g., "{{MISSING_DOCS_1_2_1}}")
    fn get_template_marker(&self) -> String {
        if self.doc_path.is_empty() {
            "{{MISSING_DOCS}}".to_string()
        } else {
            format!(
                "{{{{MISSING_DOCS_{}}}}}", // Double {{ for literal {
                self.doc_path
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join("_")
            )
        }
    }

    /// Increments the counter for the current document level.
    /// Note: we increment _after_ outputting something at a given level because
    /// we always initialize the level to 1 via `doc_path.push(1)`
    fn post_increment_current_level(&mut self) {
        if let Some(last) = self.doc_path.last_mut() {
            *last += 1;
        } else {
            warn!("Attempted to increment document path level when path was empty.");
        }
    }

    /// Pushes a new level (starting at 1) onto the document path.
    fn push_level(&mut self) {
        self.doc_path.push(1);
    }

    /// Pops the last level from the document path.
    fn pop_level(&mut self) {
        self.doc_path.pop();
    }

    fn get_item_kind(&self, id: &Id) -> Option<ItemKind> {
        // Prefer index over paths for kind, as paths might be missing for some items?
        self.krate
            .index
            .get(id)
            .map(Printer::infer_item_kind) // Use associated function syntax
            .or_else(|| self.krate.paths.get(id).map(|summary| summary.kind))
    }

    // Fallback for inferring ItemKind if not found in paths map (should be equivalent to index anyway)
    pub(crate) fn infer_item_kind(item: &Item) -> ItemKind {
        match item.inner {
            ItemEnum::Module(_) => ItemKind::Module,
            ItemEnum::ExternCrate { .. } => ItemKind::ExternCrate,
            ItemEnum::Use { .. } => ItemKind::Use, // Keep Use kind for completeness
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

    /// Prints the documentation string for an item, applying template mode if active.
    /// Header level is determined internally by the doc_path.
    fn print_docs(&mut self, item: &Item) {
        let header_level = self.get_current_header_level(); // Level of the item owning the docs
        match (&item.docs, self.template_mode) {
            // Template mode and docs exist: Print mustache marker
            (Some(_), true) => {
                let marker = self.get_template_marker();
                writeln!(self.output, "{}\n", marker).unwrap();
            }
            // Not template mode or no docs: Print original docs if non-empty
            (Some(docs), false) => {
                if !docs.trim().is_empty() {
                    // Use the new adjust_markdown_headers function
                    let adjusted_docs = adjust_markdown_headers(docs.trim(), header_level);
                    writeln!(self.output, "{}\n", adjusted_docs).unwrap();
                }
                // If docs are Some but empty, print nothing (existing behavior)
            }
            // Docs are None: Print nothing
            (None, _) => {}
        }
    }

    /// Prints the details of a single selected item.
    /// Manages the doc_path stack for the item's header.
    /// Returns true if full details were printed, false if a cross-reference was printed or skipped.
    fn print_item_details(&mut self, id: &Id) -> bool {
        if !self.selected_ids.contains(id) {
            return false; // Skip unselected items
        }

        let Some(item) = self.krate.index.get(id) else {
            warn!("Item details for ID {id:?} not found in index");
            return false;
        };

        // Skip printing details for 'Use' items, they are handled by resolution
        // Also skip Modules here, they are handled by the main traversal in finalize
        if matches!(item.inner, ItemEnum::Use(_) | ItemEnum::Module(_)) {
            return false;
        }

        let item_header_level = self.get_current_header_level();
        let header_prefix = self.get_header_prefix();
        let declaration = generate_item_declaration(item, self.krate, &self.current_module_path);

        if let Some(existing_prefix) = self.printed_ids.get(id) {
            // Item already printed, print cross-reference instead of full details
            // This case is primarily for when print_item_details is called directly
            // (e.g., from print_items_of_kind) for an item that was already
            // printed via a different module path.
            writeln!(
                self.output,
                "\n{} {} `{}` (See section {} for details)\n",
                "#".repeat(item_header_level),
                header_prefix,
                declaration,
                existing_prefix
            )
            .unwrap();
            // Do not push/pop level or print further details for cross-referenced item
            return false; // Indicate that full details were not printed
        }

        // Store the prefix *before* printing details, as this is its first detailed print
        self.printed_ids.insert(*id, header_prefix.clone());

        // Print Header (e.g. `### 1.1.1: `declaration``)
        writeln!(
            self.output,
            "\n{} {} `{}`\n", // Add newline after header
            "#".repeat(item_header_level),
            header_prefix,
            declaration
        )
        .unwrap();

        self.push_level();

        // Print Code Block for Struct/Enum/Trait/Function (if needed)
        let code_block = match &item.inner {
            ItemEnum::Struct(s) => Some(generate_struct_code_block(item, s, self.krate)),
            ItemEnum::Enum(e) => Some(generate_enum_code_block(item, e, self.krate)),
            ItemEnum::Union(u) => Some(generate_union_code_block(item, u, self.krate)),
            ItemEnum::Trait(t) => Some(generate_trait_code_block(item, t, self.krate)),
            ItemEnum::Function(f) => {
                // Check if function has attrs or where clause
                let has_attrs = f.header.is_const
                    || f.header.is_async
                    || f.header.is_unsafe
                    || !matches!(f.header.abi, Abi::Rust)
                    || !item.attrs.is_empty(); // Check item.attrs for function attributes
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

        let has_stripped = matches!(
            &item.inner,
            ItemEnum::Struct(Struct {
                kind: StructKind::Plain {
                    has_stripped_fields: true,
                    ..
                },
                ..
            })
        );

        if has_stripped {
            writeln!(self.output, "_[Private fields hidden]_\n").unwrap();
        }

        // Print Documentation (using the helper method)
        self.print_docs(item);

        match &item.inner {
            ItemEnum::Struct(s) => self.print_struct_fields(item, s),
            ItemEnum::Enum(e) => self.print_enum_variants(item, e),
            ItemEnum::Union(u) => self.print_union_fields(item, u),
            ItemEnum::Trait(t) => self.print_trait_associated_items(item, t),
            // Add other kinds requiring detailed sections if necessary
            _ => {}
        }

        // Print Implementations (common to Struct, Enum, Trait, Primitive, etc.)
        let impl_ids = match &item.inner {
            ItemEnum::Struct(s) => Some(&s.impls),
            ItemEnum::Enum(e) => Some(&e.impls),
            ItemEnum::Trait(t) => Some(&t.implementations), // Traits list implementors
            ItemEnum::Union(u) => Some(&u.impls),
            ItemEnum::Primitive(p) => Some(&p.impls),
            _ => None,
        };

        if let Some(ids) = impl_ids {
            match &item.inner {
                ItemEnum::Trait(_) => self.print_trait_implementors(ids, item),
                _ => self.print_item_implementations(ids, item),
            }
        }

        self.pop_level();

        true // Full details were printed
    }

    /// Checks if any selected field within a struct has documentation or if template mode is on.
    #[allow(unused)]
    fn has_documented_fields(&self, s: &Struct) -> bool {
        let field_ids = match &s.kind {
            StructKind::Plain { fields, .. } => fields.clone(),
            StructKind::Tuple(fields) => fields.iter().filter_map(|opt_id| *opt_id).collect(),
            StructKind::Unit => vec![],
        };
        field_ids.iter().any(|field_id| {
            self.selected_ids.contains(field_id)
                && self.krate.index.get(field_id).is_some_and(|item| {
                    // Consider it "documented" if template mode is on and docs are Some
                    (self.template_mode && item.docs.is_some()) || has_docs(item)
                })
        })
    }

    /// Prints the "Fields" section for a struct, only if needed.
    /// Also marks fields without documentation as printed.
    fn print_struct_fields(&mut self, _item: &Item, s: &Struct) {
        let all_field_ids: Vec<Id> = match &s.kind {
            StructKind::Plain { fields, .. } => fields.clone(),
            StructKind::Tuple(fields) => fields.iter().filter_map(|opt_id| *opt_id).collect(),
            StructKind::Unit => vec![],
        };

        let mut has_printable_field = false;

        // First pass: Mark unselected/undocumented/non-templated fields printed and check if any are printable.
        for field_id in &all_field_ids {
            if !self.selected_ids.contains(field_id) {
                continue; // Skip unselected fields
            }

            if let Some(item) = self.krate.index.get(field_id) {
                let field_has_printable_docs =
                    (self.template_mode && item.docs.is_some()) || has_docs(item);
                if field_has_printable_docs {
                    // Check if it's already printed to avoid double counting
                    if !self.printed_ids.contains_key(field_id) {
                        has_printable_field = true;
                    }
                } else {
                    // Mark non-printable field as printed immediately
                    self.printed_ids.insert(*field_id, self.get_header_prefix());
                }
            } else {
                // If item doesn't exist in index but ID was present, mark it printed to avoid issues
                self.printed_ids.insert(*field_id, self.get_header_prefix());
            }
        }

        // Only print the "Fields" section if there's a printable field
        if !has_printable_field {
            return;
        }

        let fields_header_level = self.get_current_header_level();
        let header_prefix = self.get_header_prefix();
        writeln!(
            self.output,
            "{} {} Fields\n", // Add newline after header
            "#".repeat(fields_header_level),
            header_prefix
        )
        .unwrap();

        // Push a new level for the field items themselves
        self.push_level();
        for field_id in &all_field_ids {
            if self.print_field_details(field_id) {
                self.post_increment_current_level();
            }
        }
        self.pop_level(); // Pop the field item level

        self.post_increment_current_level();
    }

    /// Prints the "Fields" section for a union, only if needed.
    fn print_union_fields(&mut self, _item: &Item, u: &Union) {
        let all_field_ids: Vec<Id> = u.fields.clone();
        let mut has_printable_field = false;

        for field_id in &all_field_ids {
            if !self.selected_ids.contains(field_id) {
                continue;
            }
            if let Some(item) = self.krate.index.get(field_id) {
                let field_has_printable_docs =
                    (self.template_mode && item.docs.is_some()) || has_docs(item);
                if field_has_printable_docs {
                    if !self.printed_ids.contains_key(field_id) {
                        has_printable_field = true;
                    }
                } else {
                    self.printed_ids.insert(*field_id, self.get_header_prefix());
                }
            } else {
                self.printed_ids.insert(*field_id, self.get_header_prefix());
            }
        }

        if !has_printable_field && !u.has_stripped_fields {
            return;
        }

        let fields_header_level = self.get_current_header_level();
        let header_prefix = self.get_header_prefix();
        writeln!(
            self.output,
            "{} {} Fields\n",
            "#".repeat(fields_header_level),
            header_prefix
        )
        .unwrap();

        self.push_level();
        for field_id in &all_field_ids {
            if self.print_field_details(field_id) {
                self.post_increment_current_level();
            }
        }
        if u.has_stripped_fields {
            writeln!(self.output, "_[Private fields hidden]_").unwrap();
        }
        self.pop_level();
        self.post_increment_current_level();
    }

    /// Prints the details for a single struct field, only if it has printable documentation.
    /// Returns true if the field was printed, false otherwise.
    fn print_field_details(&mut self, field_id: &Id) -> bool {
        if !self.selected_ids.contains(field_id) || self.printed_ids.contains_key(field_id) {
            return false; // Skip unselected or already printed
        }

        if let Some(item) = self.krate.index.get(field_id) {
            let field_has_printable_docs =
                (self.template_mode && item.docs.is_some()) || has_docs(item);

            // Only proceed if the field has printable documentation
            if !field_has_printable_docs {
                // Should already be marked printed in print_struct_fields
                return false;
            }

            let header_prefix = self.get_header_prefix();
            // Mark as printed *before* printing details
            self.printed_ids.insert(*field_id, header_prefix.clone());

            if let ItemEnum::StructField(_field_type) = &item.inner {
                let name = item.name.as_deref().unwrap_or("_");
                let field_header_level = self.get_current_header_level();

                // Header: e.g., ##### 1.1.1.1: `field_name`
                writeln!(
                    self.output,
                    "{} {} `{}`\n", // Add newline after header
                    "#".repeat(field_header_level),
                    header_prefix,
                    name
                )
                .unwrap();

                // Print docs (using helper, handles template mode)
                self.print_docs(item);

                // Type (optional, could add here if needed)
                // writeln!(self.output, "_Type: `{}`_\n", format_type(field_type, self.krate)).unwrap();
                return true; // Field was printed
            }
        }
        // Mark as printed even if item lookup failed (shouldn't happen ideally)
        self.printed_ids.insert(*field_id, self.get_header_prefix());
        false
    }

    /// Prints the details for a single enum variant field, only if it has printable documentation.
    /// Returns true if the field was printed, false otherwise.
    fn print_variant_field_details(&mut self, field_id: &Id) -> bool {
        if !self.selected_ids.contains(field_id) || self.printed_ids.contains_key(field_id) {
            return false; // Skip unselected or already printed
        }

        if let Some(item) = self.krate.index.get(field_id) {
            let field_has_printable_docs =
                (self.template_mode && item.docs.is_some()) || has_docs(item);

            // Only proceed if the field has printable documentation
            if !field_has_printable_docs {
                // If no docs, the ID should already be marked printed in print_variant_details
                return false;
            }
            let header_prefix = self.get_header_prefix();
            // Mark as printed *before* printing details
            self.printed_ids.insert(*field_id, header_prefix.clone());

            if let ItemEnum::StructField(_field_type) = &item.inner {
                let name = item.name.as_deref().unwrap_or("_"); // Might be _ for tuple fields
                let field_header_level = self.get_current_header_level();

                // Header: e.g., ###### 1.1.1.1.1: `field_name`
                // Use field index for tuple fields if name is "_" (name is often '0', '1' etc.)
                let header_name = if name == "_" || name.chars().all(|c| c.is_ascii_digit()) {
                    format!("Field {}", name)
                } else {
                    name.to_string()
                };
                writeln!(
                    self.output,
                    "{} {} `{}`\n", // Add newline after header
                    "#".repeat(field_header_level),
                    header_prefix,
                    header_name
                )
                .unwrap();

                // Print Docs (using helper, handles template mode)
                self.print_docs(item);

                // Increment level counter for this field item
                self.post_increment_current_level();

                // Type (optional)
                // writeln!(self.output, "_Type: `{}`_\n", format_type(field_type, self.krate)).unwrap();
                return true; // Field was printed
            }
        }
        // Mark as printed even if item lookup failed
        self.printed_ids.insert(*field_id, self.get_header_prefix());
        false
    }

    /// Checks if any selected variant or its fields have printable documentation.
    #[allow(unused)]
    fn has_printable_variants(&self, e: &Enum) -> bool {
        e.variants.iter().any(|variant_id| {
            if !self.selected_ids.contains(variant_id) {
                return false;
            }
            if let Some(item) = self.krate.index.get(variant_id) {
                // Check variant itself
                if (self.template_mode && item.docs.is_some()) || has_docs(item) {
                    return true;
                }
                // Check fields within the variant
                if let ItemEnum::Variant(v) = &item.inner {
                    let field_ids: Vec<Id> = match &v.kind {
                        VariantKind::Plain => vec![],
                        VariantKind::Tuple(fields) => {
                            fields.iter().filter_map(|opt_id| *opt_id).collect()
                        }
                        VariantKind::Struct { fields, .. } => fields.clone(),
                    };
                    for field_id in field_ids {
                        if self.selected_ids.contains(&field_id) {
                            if let Some(f_item) = self.krate.index.get(&field_id) {
                                if (self.template_mode && f_item.docs.is_some()) || has_docs(f_item)
                                {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            false
        })
    }

    /// Prints the "Variants" section for an enum, only if needed.
    /// Also marks variants *and their fields* without printable documentation as printed.
    fn print_enum_variants(&mut self, _item: &Item, e: &Enum) {
        let mut has_printable_variant_or_field = false;
        let mut printed_any_variant = false;

        // First pass: Mark non-printable variants/fields printed and check if any are printable.
        for variant_id in &e.variants {
            if !self.selected_ids.contains(variant_id) {
                continue; // Skip unselected variants
            }

            if let Some(item) = self.krate.index.get(variant_id) {
                let variant_has_printable_docs =
                    (self.template_mode && item.docs.is_some()) || has_docs(item);
                let mut variant_has_printable_field = false;

                // Check fields within the variant
                if let ItemEnum::Variant(v) = &item.inner {
                    let field_ids: Vec<Id> = match &v.kind {
                        VariantKind::Plain => vec![],
                        VariantKind::Tuple(fields) => {
                            fields.iter().filter_map(|opt_id| *opt_id).collect()
                        }
                        VariantKind::Struct { fields, .. } => fields.clone(),
                    };

                    for field_id in field_ids {
                        if self.selected_ids.contains(&field_id) {
                            let field_has_printable_docs =
                                self.krate.index.get(&field_id).is_some_and(|f_item| {
                                    (self.template_mode && f_item.docs.is_some())
                                        || has_docs(f_item)
                                });
                            if field_has_printable_docs {
                                if !self.printed_ids.contains_key(&field_id) {
                                    variant_has_printable_field = true;
                                }
                            } else {
                                self.printed_ids.insert(field_id, self.get_header_prefix());
                                // Mark non-printable field printed
                            }
                        } else {
                            // Mark unselected field id printed if present
                            self.printed_ids.insert(field_id, self.get_header_prefix());
                        }
                    }
                }

                if variant_has_printable_docs || variant_has_printable_field {
                    // Check if the variant itself is already printed to avoid double counting
                    if !self.printed_ids.contains_key(variant_id) {
                        has_printable_variant_or_field = true;
                    }
                } else {
                    // Mark non-printable variant (with no printable fields) as printed immediately
                    self.printed_ids
                        .insert(*variant_id, self.get_header_prefix());
                }
            } else {
                // If item doesn't exist in index but ID was present, mark it printed
                self.printed_ids
                    .insert(*variant_id, self.get_header_prefix());
            }
        }

        // Only print the "Variants" section if there's a printable variant/field or stripped variants exist
        if !has_printable_variant_or_field && !e.has_stripped_variants {
            return;
        }

        let variants_header_level = self.get_current_header_level();
        let header_prefix = self.get_header_prefix();
        writeln!(
            self.output,
            "{} {} Variants\n", // Add newline after header
            "#".repeat(variants_header_level),
            header_prefix
        )
        .unwrap();

        // Push a new level for the variant items themselves
        self.push_level();
        // Second pass: Print details for variants that have printable docs or contain printable fields
        for variant_id in &e.variants {
            if self.print_variant_details(variant_id) {
                printed_any_variant = true;
            }
        }

        if e.has_stripped_variants {
            // Add newline before stripped message only if variants were printed
            if printed_any_variant {
                writeln!(self.output).unwrap();
            }
            writeln!(self.output, "_[Private variants hidden]_").unwrap();
        }
        self.pop_level(); // Pop the variant item level
        self.post_increment_current_level();
    }

    /// Prints the details for a single enum variant. Includes variant docs and docs for its fields if present.
    /// Returns true if the variant was printed (because it or its fields had printable docs), false otherwise.
    fn print_variant_details(&mut self, variant_id: &Id) -> bool {
        if !self.selected_ids.contains(variant_id) {
            // If already printed, do not print full details again.
            // Cross-referencing for variants re-exported in other modules is not typical.
            // If a variant is listed in a module, it's usually because its enum is listed.
            if self.printed_ids.contains_key(variant_id) {
                return false;
            }
        } else if self.printed_ids.contains_key(variant_id) {
            // Already printed, skip
            return false;
        }

        if let Some(item) = self.krate.index.get(variant_id) {
            if let ItemEnum::Variant(variant_data) = &item.inner {
                let variant_has_printable_docs =
                    (self.template_mode && item.docs.is_some()) || has_docs(item);
                let mut printable_fields = Vec::new();
                let mut printed_any_field = false;

                // Determine fields and check their printable docs
                let (field_ids, stripped) = match &variant_data.kind {
                    VariantKind::Plain => (vec![], false),
                    VariantKind::Tuple(fields) => {
                        (fields.iter().filter_map(|opt_id| *opt_id).collect(), false)
                    }
                    VariantKind::Struct {
                        fields,
                        has_stripped_fields: s,
                    } => (fields.clone(), *s),
                };

                for field_id in &field_ids {
                    if self.selected_ids.contains(field_id) {
                        let field_has_printable_docs =
                            self.krate.index.get(field_id).is_some_and(|f_item| {
                                (self.template_mode && f_item.docs.is_some()) || has_docs(f_item)
                            });
                        if field_has_printable_docs && !self.printed_ids.contains_key(field_id) {
                            printable_fields.push(*field_id);
                        } else {
                            // Mark unselected or non-printable field printed
                            self.printed_ids.insert(*field_id, self.get_header_prefix());
                        }
                    } else {
                        // Mark unselected field printed
                        self.printed_ids.insert(*field_id, self.get_header_prefix());
                    }
                }

                // Only print the variant if it has printable docs OR it has printable fields to print
                if !variant_has_printable_docs && printable_fields.is_empty() {
                    // Mark variant as printed if skipped
                    self.printed_ids
                        .insert(*variant_id, self.get_header_prefix());
                    return false;
                }

                let header_prefix = self.get_header_prefix();
                // Mark as printed *before* printing details
                self.printed_ids.insert(*variant_id, header_prefix.clone());

                let signature = format_variant_signature(item, variant_data, self.krate);
                let variant_header_level = self.get_current_header_level();

                // Header: e.g., ##### 1.1.1.1: `VariantSignature`
                writeln!(
                    self.output,
                    "{} {} `{}`\n", // Add newline after header
                    "#".repeat(variant_header_level),
                    header_prefix,
                    signature
                )
                .unwrap();
                self.push_level();

                // Print Variant Docs (using helper)
                self.print_docs(item);

                // Print documented fields (if any)
                if !printable_fields.is_empty() || stripped {
                    let field_section_level = self.get_current_header_level();
                    let fields_header_prefix = self.get_header_prefix();
                    writeln!(
                        self.output,
                        "{} {} Fields\n", // Add newline after header
                        "#".repeat(field_section_level),
                        fields_header_prefix
                    )
                    .unwrap();
                    self.push_level();

                    for field_id in printable_fields {
                        if self.print_variant_field_details(&field_id) {
                            printed_any_field = true;
                        }
                    }

                    if stripped {
                        if printed_any_field {
                            writeln!(self.output).unwrap(); // Add newline before stripped message
                        }
                        writeln!(self.output, "_[Private fields hidden]_").unwrap();
                    }
                    self.pop_level();
                }

                self.pop_level();
                self.post_increment_current_level();

                return true; // Variant (or its fields) was printed
            }
        }
        // Mark as printed even if item lookup failed
        self.printed_ids
            .insert(*variant_id, self.get_header_prefix());
        false
    }

    /// Prints the "Associated Items" section for a trait, categorized.
    fn print_trait_associated_items(&mut self, _trait_item: &Item, t: &Trait) {
        let mut required_types = Vec::new();
        let mut required_methods = Vec::new();
        let mut provided_methods = Vec::new();
        let mut has_printable_assoc_item = false;

        // Filter and categorize selected associated items.
        for item_id in &t.items {
            if !self.selected_ids.contains(item_id) {
                continue;
            }
            // Mark the item as printed now, regardless of docs, to prevent it from going to "Other"
            // Only mark if not already printed elsewhere with a different prefix.
            // This is tricky because an assoc item's "primary" print location is under its trait.
            if !self.printed_ids.contains_key(item_id) {
                self.printed_ids.insert(*item_id, self.get_header_prefix());
            }

            if let Some(assoc_item) = self.krate.index.get(item_id) {
                let item_has_printable_docs =
                    (self.template_mode && assoc_item.docs.is_some()) || has_docs(assoc_item);
                if item_has_printable_docs {
                    has_printable_assoc_item = true;
                }

                match &assoc_item.inner {
                    ItemEnum::AssocType { .. } => {
                        required_types.push((*item_id, item_has_printable_docs));
                    }
                    ItemEnum::Function(f) => {
                        if !f.has_body {
                            required_methods.push((*item_id, item_has_printable_docs));
                        } else {
                            provided_methods.push((*item_id, item_has_printable_docs));
                        }
                    }
                    ItemEnum::AssocConst { .. } => {
                        // For now, treat associated consts similarly to required types or methods.
                        // Could be a separate category if needed.
                        // Let's put them with required types for now as they don't have 'body'.
                        required_types.push((*item_id, item_has_printable_docs));
                    }
                    _ => {} // Ignore others
                }
            }
        }

        // If no selected associated item has printable documentation, skip printing the entire section
        if !has_printable_assoc_item {
            return;
        }

        // Sort items within each category
        required_types.sort_by_key(|(id, _)| self.krate.index.get(id).and_then(|i| i.name.clone()));
        required_methods
            .sort_by_key(|(id, _)| self.krate.index.get(id).and_then(|i| i.name.clone()));
        provided_methods
            .sort_by_key(|(id, _)| self.krate.index.get(id).and_then(|i| i.name.clone()));

        if required_types.iter().any(|(_, has_docs)| *has_docs) {
            let sub_level = self.get_current_header_level();
            let sub_prefix = self.get_header_prefix();
            writeln!(
                self.output,
                "{} {} Required Associated Types\n",
                "#".repeat(sub_level),
                sub_prefix
            )
            .unwrap();
            self.push_level();
            for (id, has_docs) in required_types {
                if has_docs {
                    self.print_associated_item_summary(&id);
                }
            }
            self.pop_level();
            self.post_increment_current_level();
        }

        if required_methods.iter().any(|(_, has_docs)| *has_docs) {
            let sub_level = self.get_current_header_level();
            let sub_prefix = self.get_header_prefix();
            writeln!(
                self.output,
                "{} {} Required Methods\n",
                "#".repeat(sub_level),
                sub_prefix
            )
            .unwrap();
            self.push_level();
            for (id, has_docs) in required_methods {
                if has_docs {
                    self.print_associated_item_summary(&id);
                }
            }
            self.pop_level();
            self.post_increment_current_level();
        }

        if provided_methods.iter().any(|(_, has_docs)| *has_docs) {
            let sub_level = self.get_current_header_level();
            let sub_prefix = self.get_header_prefix();
            writeln!(
                self.output,
                "{} {} Provided Methods\n",
                "#".repeat(sub_level),
                sub_prefix
            )
            .unwrap();
            self.push_level();
            for (id, has_docs) in provided_methods {
                if has_docs {
                    self.print_associated_item_summary(&id);
                }
            }
            self.pop_level();
            self.post_increment_current_level();
        }
    }

    /// Generates the formatted summary string for an associated item (for use within impl blocks or trait defs).
    /// Does NOT include the markdown header. Includes docs with adjusted headers, respecting template mode.
    fn generate_associated_item_summary(&mut self, assoc_item_id: &Id) -> Option<String> {
        if !self.selected_ids.contains(assoc_item_id) {
            return None;
        }
        if let Some(item) = self.krate.index.get(assoc_item_id) {
            let mut summary = String::new();
            // Get level AFTER incrementing for the item header
            // let assoc_item_header_level = self.get_current_header_level(); // Level not needed here

            // Add code block for associated functions if they have attrs/where clauses
            if let ItemEnum::Function(f) = &item.inner {
                let has_attrs = f.header.is_const
                    || f.header.is_async
                    || f.header.is_unsafe
                    || !matches!(f.header.abi, Abi::Rust)
                    || !item.attrs.is_empty(); // Check item.attrs for function attributes
                let has_where = !f.generics.where_predicates.is_empty();
                if has_attrs || has_where {
                    let code = generate_function_code_block(item, f, self.krate);
                    writeln!(summary, "```rust\n{}\n```\n", code).unwrap();
                }
            }

            // Print Documentation (using helper)
            // Create a temporary DocPrinter to isolate output
            let mut temp_printer = self.clone_with_new_output();
            // Copy current doc path to temp printer for correct template marker generation
            temp_printer.doc_path = self.doc_path.clone();
            temp_printer.print_docs(item);
            write!(summary, "{}", temp_printer.output).unwrap();

            // Potentially add default values/bounds for assoc const/type here
            match &item.inner {
                // Use correct fields { type_, value }
                ItemEnum::AssocConst { type_, value } => {
                    writeln!(summary, "_Type: `{}`_", format_type(type_, self.krate)).unwrap();
                    if let Some(val) = value {
                        writeln!(summary, "_Default: `{}`_\n", val).unwrap(); // Add newline
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
                        writeln!(summary, "_Bounds: `{}`_", bounds_str).unwrap();
                    }
                    if let Some(ty) = type_ {
                        writeln!(summary, "_Default: `{}`_\n", format_type(ty, self.krate))
                            .unwrap(); // Add newline
                    }
                }
                _ => {}
            }
            Some(summary)
        } else {
            None
        }
    }

    /// Prints the header and summary for a single associated item (const, type, function).
    fn print_associated_item_summary(&mut self, assoc_item_id: &Id) {
        if let Some(item) = self.krate.index.get(assoc_item_id) {
            // Generate summary first (handles template mode internally)
            if let Some(summary) = self.generate_associated_item_summary(assoc_item_id) {
                let declaration =
                    generate_item_declaration(item, self.krate, &self.current_module_path);
                let assoc_item_header_level = self.get_current_header_level();
                let header_prefix = self.get_header_prefix();
                // Print Header (e.g. ##### 1.1.1.1: `declaration`)
                writeln!(
                    self.output,
                    "{} {} `{}`\n", // Add newline after header
                    "#".repeat(assoc_item_header_level),
                    header_prefix,
                    declaration
                )
                .unwrap();
                // Print the generated summary
                if !summary.trim().is_empty() {
                    writeln!(self.output, "{}", summary.trim()).unwrap();
                }
                writeln!(self.output).unwrap(); // Ensure a blank line afterwards

                self.post_increment_current_level();
            }
            // If generate_associated_item_summary returns None, the item wasn't selected,
            // so we don't print anything, and the level increment effectively skips it.
        }
    }

    /// Helper to categorize and format a list of FormattedTraitImpls for display.
    fn format_trait_list(&mut self, traits_to_format: &[FormattedTraitImpl]) -> String {
        if traits_to_format.is_empty() {
            return String::new();
        }

        let mut output = String::new();
        let mut simple_impls = Vec::new();
        let mut generic_or_complex_impls = Vec::new();
        let mut auto_traits = Vec::new();
        let mut blanket_impls = Vec::new();

        for norm_trait in traits_to_format {
            match norm_trait.category {
                TraitImplCategory::Simple => simple_impls.push(norm_trait),
                TraitImplCategory::GenericOrComplex => generic_or_complex_impls.push(norm_trait),
                TraitImplCategory::Auto => auto_traits.push(norm_trait),
                TraitImplCategory::Blanket => blanket_impls.push(norm_trait),
            }
        }

        // Sort each category by the pre-formatted list entry string
        simple_impls.sort_by_key(|t| &t.formatted_markdown_list_entry);
        generic_or_complex_impls.sort_by_key(|t| &t.formatted_markdown_list_entry);
        auto_traits.sort_by_key(|t| &t.formatted_markdown_list_entry);
        blanket_impls.sort_by_key(|t| &t.formatted_markdown_list_entry);

        self.push_level();
        let mut preceding_section = false;

        let mut print_section =
            |traits: &[&FormattedTraitImpl], current_output: &mut String, _section_name: &str| {
                if !traits.is_empty() {
                    if preceding_section {
                        writeln!(current_output).unwrap();
                    }
                    for norm_trait in traits {
                        writeln!(
                            current_output,
                            "{}",
                            norm_trait.formatted_markdown_list_entry
                        )
                        .unwrap();
                        if let Some((trait_impl, impl_id)) = norm_trait.get_impl_data(self.krate) {
                            self.printed_ids.insert(impl_id, self.get_header_prefix());
                            for assoc_item_id in &trait_impl.items {
                                if self.selected_ids.contains(assoc_item_id) {
                                    self.printed_ids
                                        .insert(*assoc_item_id, self.get_header_prefix());
                                }
                            }
                        }
                        self.post_increment_current_level();
                    }
                    preceding_section = true;
                }
            };

        print_section(&simple_impls, &mut output, "Simple");
        print_section(&generic_or_complex_impls, &mut output, "Generic or Complex");
        print_section(&auto_traits, &mut output, "Auto");
        print_section(&blanket_impls, &mut output, "Blanket");

        self.pop_level();
        output
    }

    /// Prints Inherent and Trait Implementations *for* an item (Struct, Enum, Union, Primitive).
    fn print_item_implementations(&mut self, impl_ids: &[Id], target_item: &Item) {
        let target_item_id = target_item.id;
        let target_name = target_item
            .name
            .as_deref()
            .unwrap_or(match &target_item.inner {
                ItemEnum::Primitive(Primitive { name, .. }) => name.as_str(),
                _ => "{unknown_item_type}",
            });

        let mut item_specific_impl_data = Vec::new();
        for impl_id in impl_ids {
            if let Some(impl_item) = self.krate.index.get(impl_id) {
                if self.selected_ids.contains(&impl_item.id) {
                    if let ItemEnum::Impl(imp) = &impl_item.inner {
                        // Critical: Only consider this impl if it's FOR the target_item_id
                        if get_type_id(&imp.for_) == Some(target_item_id) {
                            item_specific_impl_data.push((impl_item, imp.clone()));
                        }
                    }
                }
            }
        }

        // --- Inherent Impls ---
        let inherent_impl_items: Vec<_> = item_specific_impl_data
            .iter()
            .filter(|(_, imp)| imp.trait_.is_none())
            .collect();

        if !inherent_impl_items.is_empty() {
            for (impl_item, imp) in inherent_impl_items {
                if self.printed_ids.contains_key(&impl_item.id) {
                    continue;
                }
                self.print_impl_block_details(impl_item, imp);
            }
        }

        // --- Trait Impls ---
        let trait_impl_data: Vec<FormattedTraitImpl> = item_specific_impl_data
            .iter()
            .filter_map(|(impl_item, imp)| {
                if self.printed_ids.contains_key(&impl_item.id) {
                    return None; // Skip already printed impls
                }
                imp.trait_.as_ref().map(|tp| {
                    FormattedTraitImpl::from_impl(imp, Some(impl_item.id), tp, self.krate, self)
                })
            })
            .collect();

        if trait_impl_data.is_empty() {
            return;
        }

        let current_module_id = self
            .current_module_path
            .last()
            .and_then(|mod_name| {
                self.resolved_modules
                    .values()
                    .find(|rm| {
                        self.krate
                            .paths
                            .get(&rm.id)
                            .is_some_and(|p| p.path.last() == Some(mod_name))
                    })
                    .map(|rm| rm.id)
            })
            .unwrap_or(self.krate.root);

        let module_common_traits = self
            .module_common_traits
            .get(&current_module_id)
            .cloned()
            .unwrap_or_default();

        let mut non_common_trait_impls = Vec::new();
        let mut missing_module_common = module_common_traits.clone();

        for norm_trait in &trait_impl_data {
            // Critical: Ensure norm_trait.for_type_id matches target_item_id
            // This check is now part of the construction of trait_impl_data
            // and in format_trait_list, so it should be correct here.

            let is_module_common = module_common_traits.contains(norm_trait);
            if is_module_common {
                missing_module_common.remove(norm_trait);
            }

            if !is_module_common {
                non_common_trait_impls.push(norm_trait.clone());
            } else {
                // Mark common trait impl as printed (and its items)
                if let Some((trait_impl, impl_id)) = norm_trait.get_impl_data(self.krate) {
                    self.printed_ids.insert(impl_id, self.get_header_prefix());
                    for assoc_item_id in &trait_impl.items {
                        if self.selected_ids.contains(assoc_item_id) {
                            self.printed_ids
                                .insert(*assoc_item_id, self.get_header_prefix());
                        }
                    }
                }
            }
        }

        if !non_common_trait_impls.is_empty() || !missing_module_common.is_empty() {
            let trait_impl_header_level = self.get_current_header_level();
            let header_prefix = self.get_header_prefix();
            writeln!(
                self.output,
                "{} {} Trait Implementations for `{}`\n",
                "#".repeat(trait_impl_header_level),
                header_prefix,
                target_name
            )
            .unwrap();

            if !missing_module_common.is_empty() {
                let mut sorted_missing_common_traits: Vec<String> = missing_module_common
                    .iter()
                    .map(|nt| {
                        // Use the pre-formatted entry, but extract the `<code>` part
                        // This is a bit hacky, ideally we'd have the cleaned path separately
                        let path_part = nt
                            .formatted_markdown_list_entry
                            .split_once("`")
                            .and_then(|(_, rest)| rest.split_once("`"))
                            .map(|(path, _)| path)
                            .unwrap_or(&nt.formatted_markdown_list_entry);
                        path_part.to_string()
                    })
                    .collect();
                sorted_missing_common_traits.sort_unstable();
                writeln!(
                    self.output,
                    "**(Note: Does not implement `{}`)**\n",
                    sorted_missing_common_traits.join("`, `")
                )
                .unwrap();
            }

            let formatted_list = self.format_trait_list(&non_common_trait_impls);
            if !formatted_list.is_empty() {
                write!(self.output, "{}", formatted_list).unwrap();
            }

            self.post_increment_current_level();
        }
    }

    /// Prints implementors *of* a trait. Handles template mode for the impl docs.
    fn print_trait_implementors(&mut self, impl_ids: &[Id], _trait_item: &Item) {
        let implementors: Vec<&Item> = impl_ids
            .iter()
            .filter_map(|id| self.krate.index.get(id))
            .filter(|item| {
                self.selected_ids.contains(&item.id) && matches!(item.inner, ItemEnum::Impl(_))
            })
            .collect();

        if !implementors.is_empty() {
            let implementors_section_level = self.get_current_header_level();
            let header_prefix = self.get_header_prefix();
            writeln!(
                self.output,
                "{} {} Implementors\n",
                "#".repeat(implementors_section_level),
                header_prefix
            )
            .unwrap();

            self.push_level();
            for impl_item in implementors {
                if let ItemEnum::Impl(imp) = &impl_item.inner {
                    let impl_header_only = self.format_impl_decl_header_only(imp);
                    let impl_header_level = self.get_current_header_level();
                    let impl_prefix = self.get_header_prefix();

                    writeln!(
                        self.output,
                        "{} {} `{}`\n",
                        "#".repeat(impl_header_level),
                        impl_prefix,
                        impl_header_only.trim()
                    )
                    .unwrap();

                    // Print where clause if it exists
                    if !imp.generics.where_predicates.is_empty() {
                        let where_clause =
                            format_generics_where_only(&imp.generics.where_predicates, self.krate);
                        writeln!(self.output, "```rust\n{}\n```\n", where_clause).unwrap();
                    }

                    // Print docs for the impl block itself
                    let mut temp_printer = self.clone_with_new_output();
                    temp_printer.doc_path = self.doc_path.clone();
                    temp_printer.print_docs(impl_item);
                    write!(self.output, "{}", temp_printer.output).unwrap();

                    // Mark the impl_item ID and its associated items as printed
                    self.printed_ids
                        .insert(impl_item.id, self.get_header_prefix());
                    for assoc_item_id in &imp.items {
                        if self.selected_ids.contains(assoc_item_id) {
                            self.printed_ids
                                .insert(*assoc_item_id, self.get_header_prefix());
                        }
                    }

                    self.post_increment_current_level();
                }
            }
            self.pop_level();
            self.post_increment_current_level();
        }
    }

    /// Helper to format only the header part of an impl declaration (e.g., `impl MyTrait for MyStruct<T>`)
    fn format_impl_decl_header_only(&self, imp: &Impl) -> String {
        let mut decl = String::new();
        if imp.is_unsafe {
            write!(decl, "unsafe ").unwrap();
        }
        write!(decl, "impl").unwrap();

        // Add generics params <...> to the impl block itself (not the trait part)
        let generics_params = format_generics_params_only(&imp.generics.params, self.krate);
        if !generics_params.is_empty() {
            write!(decl, "{}", generics_params).unwrap();
        }

        // Add Trait (if it's a trait impl)
        if let Some(trait_path) = &imp.trait_ {
            // For trait impl header, format trait_path with its own generics
            write!(decl, " {} for", format_path(trait_path, self.krate)).unwrap();
        }

        // Add Type it's for
        write!(decl, " {}", format_type(&imp.for_, self.krate)).unwrap();

        // DO NOT add where clause here
        decl
    }

    /// Helper to format an impl block or trait impl declaration line.
    fn format_impl_decl(&self, imp: &Impl) -> String {
        let mut decl = String::new();
        if imp.is_unsafe {
            write!(decl, "unsafe ").unwrap();
        }
        write!(decl, "impl").unwrap();

        // Add generics params <...>
        let generics_params = format_generics_params_only(&imp.generics.params, self.krate);
        if !generics_params.is_empty() {
            write!(decl, "{}", generics_params).unwrap();
        }

        // Add Trait for Type
        if let Some(trait_path) = &imp.trait_ {
            write!(decl, " {} for", format_path(trait_path, self.krate)).unwrap();
        }
        write!(decl, " {}", format_type(&imp.for_, self.krate)).unwrap();

        // Add where clause
        let where_clause = format_generics_where_only(&imp.generics.where_predicates, self.krate);
        if !where_clause.is_empty() {
            if where_clause.contains('\n') {
                write!(decl, "\n  {}", where_clause).unwrap(); // Multiline where
            } else {
                write!(decl, " {}", where_clause).unwrap(); // Single line where
            }
        }
        decl
    }

    /// Generates the full code block string for a trait impl, including associated items.
    /// Returns None if the impl block was already printed or is effectively empty.
    /// Skips methods within the impl block.
    fn generate_impl_trait_block(&mut self, imp: &Impl) -> Option<String> {
        let mut code = String::new();
        let impl_header = self.format_impl_decl(imp);
        writeln!(code, "{} {{", impl_header).unwrap();

        let mut assoc_items_content = String::new();
        let mut has_printable_assoc_items = false;
        let original_doc_path = self.doc_path.clone(); // Save original path

        self.push_level();
        for assoc_item_id in &imp.items {
            if !self.selected_ids.contains(assoc_item_id) {
                continue;
            }
            if !self.printed_ids.contains_key(assoc_item_id) {
                self.printed_ids
                    .insert(*assoc_item_id, self.get_header_prefix());
            }

            if let Some(assoc_item) = self.krate.index.get(assoc_item_id) {
                match &assoc_item.inner {
                    ItemEnum::AssocConst { type_, value, .. } => {
                        has_printable_assoc_items = true;
                        self.doc_path = original_doc_path.clone(); // Reset for each item
                        self.doc_path.push(1); // Simulate being under a new header for template marker
                        let assoc_item_docs = if self.template_mode && assoc_item.docs.is_some() {
                            format!("\n    // {}", self.get_template_marker())
                        } else {
                            "".to_string()
                        };
                        self.doc_path.pop(); // Restore

                        write!(
                            assoc_items_content,
                            "    {}const {}: {}",
                            format_attributes(&assoc_item.attrs),
                            assoc_item.name.as_deref().unwrap_or("_"),
                            format_type(type_, self.krate)
                        )
                        .unwrap();
                        if let Some(val) = value {
                            write!(assoc_items_content, " = {};{}", val, assoc_item_docs).unwrap();
                        } else {
                            write!(assoc_items_content, ";{}", assoc_item_docs).unwrap();
                        }
                        writeln!(assoc_items_content).unwrap();
                    }
                    ItemEnum::AssocType { bounds, type_, .. } => {
                        has_printable_assoc_items = true;
                        self.doc_path = original_doc_path.clone();
                        self.doc_path.push(1);
                        let assoc_item_docs = if self.template_mode && assoc_item.docs.is_some() {
                            format!("\n    // {}", self.get_template_marker())
                        } else {
                            "".to_string()
                        };
                        self.doc_path.pop();

                        write!(
                            assoc_items_content,
                            "    {}type {}",
                            format_attributes(&assoc_item.attrs),
                            assoc_item.name.as_deref().unwrap_or("_")
                        )
                        .unwrap();
                        if !bounds.is_empty() {
                            let bounds_str = bounds
                                .iter()
                                .map(|b| format_generic_bound(b, self.krate))
                                .collect::<Vec<_>>()
                                .join(" + ");
                            write!(assoc_items_content, ": {}", bounds_str).unwrap();
                        }
                        if let Some(ty) = type_ {
                            write!(assoc_items_content, " = {}", format_type(ty, self.krate))
                                .unwrap();
                        }
                        write!(assoc_items_content, ";{}", assoc_item_docs).unwrap();
                        writeln!(assoc_items_content).unwrap();
                    }
                    _ => {}
                }
            }
        }
        self.pop_level();
        self.doc_path = original_doc_path; // Restore original path fully

        if has_printable_assoc_items {
            if impl_header.contains('\n') && !assoc_items_content.starts_with('\n') {
                writeln!(code).unwrap();
            }
            write!(code, "{}", assoc_items_content).unwrap();
            if !code.ends_with('\n') && !assoc_items_content.is_empty() {
                writeln!(code).unwrap();
            }
        } else if impl_header.contains('\n') {
            writeln!(code).unwrap();
        }

        write!(code, "}}").unwrap();
        if !has_printable_assoc_items {
            return None;
        }
        Some(code)
    }

    /// Prints the details of a specific impl block (header, associated items).
    /// Handles template mode for the impl block's docs.
    fn print_impl_block_details(&mut self, impl_item: &Item, imp: &Impl) {
        let header_prefix = self.get_header_prefix();
        // Mark as printed *now* before printing details
        if self
            .printed_ids
            .insert(impl_item.id, header_prefix.clone())
            .is_some()
        {
            // Already printed with a (potentially different) prefix, skip full details.
            // This case should ideally be caught by the caller, but as a safeguard.
            return;
        }

        // Increment level counter for this impl block
        self.post_increment_current_level();
        let impl_header_level = self.get_current_header_level();
        let impl_header = self.format_impl_decl(imp);

        // Print the impl block header (e.g. #### 1.1.1: `impl ...`)
        writeln!(
            self.output,
            "{} {} `{}`\n", // Add newline after header
            "#".repeat(impl_header_level),
            header_prefix,      // Use the stored/current prefix
            impl_header.trim()  // Trim potential trailing space if no where clause added
        )
        .unwrap();

        // Print impl block docs (using helper)
        // Create a temporary DocPrinter to isolate output
        let mut temp_printer = self.clone_with_new_output();
        // Copy current doc path to temp printer for correct template marker generation
        temp_printer.doc_path = self.doc_path.clone();
        temp_printer.print_docs(impl_item);
        write!(self.output, "{}", temp_printer.output).unwrap();

        // Print associated items within this impl block
        let mut assoc_consts = vec![];
        let mut assoc_types = vec![];
        let mut assoc_fns = vec![];
        for assoc_item_id in &imp.items {
            // Important: Only process associated items that are *selected*
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

        // Push level for associated items within the impl block
        self.push_level();

        if !assoc_consts.is_empty() {
            for id in assoc_consts {
                self.print_associated_item_summary(id);
                if !self.printed_ids.contains_key(id) {
                    self.printed_ids.insert(*id, self.get_header_prefix());
                }
            }
        }
        if !assoc_types.is_empty() {
            for id in assoc_types {
                self.print_associated_item_summary(id);
                if !self.printed_ids.contains_key(id) {
                    self.printed_ids.insert(*id, self.get_header_prefix());
                }
            }
        }
        if !assoc_fns.is_empty() {
            for id in assoc_fns {
                self.print_associated_item_summary(id);
                if !self.printed_ids.contains_key(id) {
                    self.printed_ids.insert(*id, self.get_header_prefix());
                }
            }
        }

        self.pop_level(); // Pop associated item level
    }

    /// Prints items of a specific kind within a given list of IDs.
    fn print_items_of_kind(&mut self, item_ids: &[Id], kind: ItemKind, header_name: &str) -> bool {
        // Filter and sort items of the target kind
        let mut items_to_print: Vec<&Id> = item_ids
            .iter()
            .filter(|id| self.selected_ids.contains(id))
            .filter(|id| self.get_item_kind(id) == Some(kind))
            .collect();

        if items_to_print.is_empty() {
            return false; // Nothing to print for this kind
        }

        items_to_print
            .sort_by_key(|id| self.krate.index.get(id).and_then(|item| item.name.clone()));

        let section_header_level = self.get_current_header_level();
        let header_prefix = self.get_header_prefix();
        writeln!(
            self.output,
            "\n{} {} {}",
            "#".repeat(section_header_level),
            header_prefix,
            header_name
        )
        .unwrap();

        self.push_level();
        // Print item details
        for id in items_to_print {
            // print_item_details now returns true if full details were printed
            if self.print_item_details(id) {
                self.post_increment_current_level();
            } else {
                // If it was a cross-reference or skipped, we still need to increment
                // the counter for the list item itself if we decide to print a list item.
                // For now, print_item_details handles the cross-ref header.
                // If we change print_module_contents to print list items for cross-refs,
                // then this post_increment might need adjustment.
                // For now, if print_item_details printed a cross-ref header,
                // it means an "item" was output, so we increment.
                self.post_increment_current_level();
            }
        }
        self.pop_level(); // Pop the item level for this section

        true
    }

    /// Prints the non-module contents of a specific module (identified by its ID).
    /// Uses the `resolved_modules` index to get the list of items.
    fn print_module_contents(&mut self, module_id: &Id) {
        if let Some(resolved_module) = self.resolved_modules.get(module_id) {
            let mut items_by_kind: HashMap<ItemKind, Vec<Id>> = HashMap::new();
            let mut cross_referenced_items: Vec<(Id, String, String)> = Vec::new(); // (Id, Declaration, Prefix)

            for id in &resolved_module.items {
                if !self.selected_ids.contains(id) {
                    continue;
                }

                if let Some(existing_prefix) = self.printed_ids.get(id) {
                    if let Some(item) = self.krate.index.get(id) {
                        // Only add to cross-reference list if it's a kind we'd normally list directly
                        if !matches!(
                            item.inner,
                            ItemEnum::Impl(_)
                                | ItemEnum::Use { .. }
                                | ItemEnum::StructField(_)
                                | ItemEnum::Variant(_) // Variants are part of enums
                                | ItemEnum::AssocConst { .. } // Assoc items are part of traits/impls
                                | ItemEnum::AssocType { .. }
                                | ItemEnum::Module(_)
                        ) {
                            let decl = generate_item_declaration(
                                item,
                                self.krate,
                                &self.current_module_path,
                            );
                            cross_referenced_items.push((*id, decl, existing_prefix.clone()));
                        }
                    }
                    continue; // Skip adding to items_by_kind if already printed
                }

                if let Some(kind) = self.get_item_kind(id) {
                    match kind {
                        ItemKind::Impl
                        | ItemKind::Variant // Handled by Enum
                        | ItemKind::StructField // Handled by Struct/Union/Variant
                        | ItemKind::AssocConst // Handled by Trait/Impl
                        | ItemKind::AssocType // Handled by Trait/Impl
                        | ItemKind::Use // Resolved, not printed directly
                        | ItemKind::Module => continue, // Handled by main loop
                        _ => {}
                    }
                    items_by_kind.entry(kind).or_default().push(*id);
                }
            }

            // Sort items by name within each kind
            for ids in items_by_kind.values_mut() {
                ids.sort_by_key(|id| self.krate.index.get(id).and_then(|item| item.name.clone()));
            }
            cross_referenced_items.sort_by_key(|(_, decl, _)| decl.clone());

            let print_order = [
                (ItemKind::Macro, "Macros"),
                (ItemKind::ProcAttribute, "Attribute Macros"),
                (ItemKind::ProcDerive, "Derive Macros"),
                (ItemKind::Struct, "Structs"),
                (ItemKind::Enum, "Enums"),
                (ItemKind::Union, "Unions"),
                (ItemKind::Trait, "Traits"),
                (ItemKind::Function, "Functions"),
                (ItemKind::TypeAlias, "Type Aliases"),
                (ItemKind::TraitAlias, "Trait Aliases"),
                (ItemKind::Static, "Statics"),
                (ItemKind::Constant, "Constants"),
                (ItemKind::ExternCrate, "External Crates"),
                (ItemKind::ExternType, "External Types"),
                (ItemKind::Primitive, "Primitives"),
            ];

            for (kind, header_name) in print_order {
                if let Some(ids) = items_by_kind.get(&kind) {
                    if ids.is_empty() {
                        continue;
                    }
                    if self.print_items_of_kind(ids, kind, header_name) {
                        self.post_increment_current_level();
                    }
                }
            }

            // Print cross-referenced items at the end of the module's direct items
            if !cross_referenced_items.is_empty() {
                let re_exports_header_level = self.get_current_header_level();
                let re_exports_prefix = self.get_header_prefix();
                writeln!(
                    self.output,
                    "\n{} {} Re-exports\n",
                    "#".repeat(re_exports_header_level),
                    re_exports_prefix
                )
                .unwrap();
                for (_id, declaration, original_prefix) in cross_referenced_items {
                    writeln!(
                        self.output,
                        "- `{}` (See section {} for details)",
                        declaration, original_prefix
                    )
                    .unwrap();
                }
                writeln!(self.output).unwrap(); // Add a blank line after the list
                self.post_increment_current_level();
            }
        } else {
            warn!(
                "Could not find resolved module data for ID: {:?}",
                module_id
            );
        }
    }

    /// Prints graph context for an unprinted item.
    fn print_graph_context(&mut self, id: &Id) {
        // Collect incoming edges first to release immutable borrow on self.graph
        let incoming_edges_data: Vec<Edge> = self
            .graph
            .find_incoming_edges(id)
            .into_iter()
            .cloned()
            .collect();

        if !incoming_edges_data.is_empty() {
            writeln!(self.output, "_Referenced by:_").unwrap();
            // Sort edges for consistent output
            let mut sorted_edges = incoming_edges_data;
            sorted_edges.sort_by_key(|edge| {
                (
                    format_id_path_canonical(&edge.source, self.krate),
                    format!("{:?}", edge.label),
                )
            });

            // Push level for this list (for template markers)
            self.push_level();
            for edge in sorted_edges {
                self.post_increment_current_level(); // Increment for this list item
                let source_path = format_id_path_canonical(&edge.source, self.krate);
                let template_marker = if self.template_mode
                    && self
                        .krate
                        .index
                        .get(&edge.source)
                        .is_some_and(|i| i.docs.is_some())
                {
                    format!("\n  {}", self.get_template_marker())
                } else {
                    "".to_string()
                };
                writeln!(
                    self.output,
                    "- `{}` ({}){}",
                    source_path,
                    edge.label, // Use Display impl for EdgeLabel
                    template_marker
                )
                .unwrap();
            }
            self.pop_level(); // Pop list level
            writeln!(self.output).unwrap(); // Add trailing newline
        } else {
            writeln!(
                self.output,
                "_Item has no known incoming references in the graph._\n"
            )
            .unwrap();
        }
    }

    /// Creates a clone of the printer with an empty output buffer.
    fn clone_with_new_output(&self) -> Self {
        Printer {
            krate: self.krate,
            manifest_data: self.manifest_data.clone(),
            paths: self.paths.clone(),
            readme_content: self.readme_content.clone(),
            examples_readme_content: self.examples_readme_content.clone(),
            examples: self.examples.clone(),
            include_other: self.include_other,
            template_mode: self.template_mode,
            no_common_traits: self.no_common_traits,
            selected_ids: self.selected_ids.clone(), // Clone relevant fields
            resolved_modules: self.resolved_modules.clone(),
            graph: self.graph.clone(),
            printed_ids: self.printed_ids.clone(),
            output: String::new(), // New output buffer
            module_tree: self.module_tree.clone(),
            doc_path: self.doc_path.clone(),
            current_module_path: self.current_module_path.clone(),
            crate_common_traits: self.crate_common_traits.clone(),
            all_type_ids_with_impls: self.all_type_ids_with_impls.clone(),
            module_common_traits: self.module_common_traits.clone(),
        }
    }

    /// Recursive function to print modules and their contents depth-first.
    fn print_module_recursive(&mut self, module_id: Id) {
        // Skip if not selected. If already printed, we still need to list its re-exports.
        if module_id != self.krate.root && !self.selected_ids.contains(&module_id) {
            return;
        }

        if let Some(item) = self.krate.index.get(&module_id) {
            // Update current_module_path
            let module_segment = item.name.as_deref().unwrap_or("").to_string();
            if module_id == self.krate.root {
                // For root module, use the crate name
                self.current_module_path = vec![self
                    .krate
                    .index
                    .get(&self.krate.root)
                    .unwrap()
                    .name
                    .as_ref()
                    .unwrap()
                    .replace('-', "_")];
            } else {
                self.current_module_path.push(module_segment);
            }

            let module_header_level = self.get_current_header_level(); // Should be 2
            let header_prefix = self.get_header_prefix();
            let module_path_str = self.current_module_path.join("::");
            let display_path = if module_path_str.is_empty() {
                item.name.as_deref().unwrap_or("::")
            } else {
                &module_path_str
            };

            // Print module header (always H2)
            writeln!(
                self.output,
                "\n{} {} Module: `{}`\n", // Module header uses level 2
                "#".repeat(module_header_level),
                header_prefix,
                display_path
            )
            .unwrap();

            // Mark module as printed only AFTER printing its header, if not already printed
            // This ensures the first time a module is encountered, its prefix is stored.
            if !self.printed_ids.contains_key(&module_id) {
                self.printed_ids.insert(module_id, header_prefix.clone());
            }

            self.push_level();

            // Print module docs (using helper)
            self.print_docs(item);

            // --- Module Common Traits ---
            if !self.no_common_traits {
                let mod_common = self.calculate_module_common_traits(&module_id);
                self.module_common_traits
                    .insert(module_id, mod_common.clone()); // Store for later use

                let displayable_module_common: Vec<FormattedTraitImpl> = mod_common
                    .iter()
                    .filter(|nt| !self.crate_common_traits.contains(nt)) // Only those not in crate common
                    .cloned()
                    .collect();

                if !displayable_module_common.is_empty() {
                    let common_traits_header_level = self.get_current_header_level(); // Should be H3
                    let common_traits_prefix = self.get_header_prefix();
                    writeln!(
                        self.output,
                        "{} {} Common Traits\n",
                        "#".repeat(common_traits_header_level),
                        common_traits_prefix
                    )
                    .unwrap();
                    writeln!(self.output, "In addition to the crate's 'Common Traits', the following traits are commonly implemented by types in this module. Unless otherwise noted, you can assume these traits are implemented:\n").unwrap();
                    let formatted_list = self.format_trait_list(&displayable_module_common);
                    if !formatted_list.is_empty() {
                        write!(self.output, "{}", formatted_list).unwrap();
                    }
                    self.post_increment_current_level(); // Increment for this section
                }
            }

            // Print module contents (non-module items only)
            self.print_module_contents(&module_id);

            self.pop_level();
            self.post_increment_current_level();

            // Recursively print child modules
            if let Some(children) = self.module_tree.children.get(&module_id).cloned() {
                for child_id in children {
                    self.print_module_recursive(child_id);
                }
            }

            // Restore current_module_path
            if module_id != self.krate.root {
                self.current_module_path.pop();
            }
        }
    }

    /// Finalizes the documentation string, printing the crate header and contents.
    fn finalize(mut self) -> String {
        let root_item = self.krate.index.get(&self.krate.root).unwrap(); // Assume root exists
        let crate_name = root_item.name.as_deref().unwrap_or("Unknown Crate");
        let crate_version = self.krate.crate_version.as_deref().unwrap_or("");
        let crate_header_level = 1; // Level 1 for crate header

        // Clear doc path before starting
        self.doc_path.clear();

        // Print Crate Header (# Crate Name (Version)) - No prefix
        writeln!(
            self.output,
            "{} {} API ({})\n", // Add newline after header
            "#".repeat(crate_header_level),
            crate_name,
            crate_version
        )
        .unwrap();
        // Push H2 level before starting sections/modules
        self.push_level();

        // Print Crate Description (if available) - NEW
        if let Some(desc) = &self.manifest_data.description {
            writeln!(self.output, "{}\n", desc).unwrap();
        }

        // Print Manifest Section (H2) - NEW
        let manifest_section_level = self.get_current_header_level(); // Should be 2
        let manifest_header_prefix = self.get_header_prefix();
        writeln!(
            self.output,
            "{} {} Manifest\n",
            "#".repeat(manifest_section_level),
            manifest_header_prefix
        )
        .unwrap();

        // Use simple list format for manifest details
        if let Some(hp) = &self.manifest_data.homepage {
            writeln!(self.output, "- Homepage: <{}>", hp).unwrap();
        }
        if let Some(repo) = &self.manifest_data.repository {
            writeln!(self.output, "- Repository: <{}>", repo).unwrap();
        }
        if !self.manifest_data.categories.is_empty() {
            writeln!(
                self.output,
                "- Categories: {}",
                self.manifest_data.categories.join(", ")
            )
            .unwrap();
        }
        if let Some(lic) = &self.manifest_data.license {
            writeln!(self.output, "- License: {}", lic).unwrap();
        }
        if let Some(rv) = &self.manifest_data.rust_version {
            writeln!(self.output, "- rust-version: `{}`", rv).unwrap();
        }
        if let Some(ed) = &self.manifest_data.edition {
            writeln!(self.output, "- edition: `{}`", ed).unwrap();
        }
        writeln!(self.output).unwrap(); // Add a newline after the list

        // Print Features Sub-section (H3) - NEW
        let features_section_level = self.get_current_header_level() + 1; // H3
        self.push_level(); // Push for the H3 features section
        let features_header_prefix = self.get_header_prefix();
        writeln!(
            self.output,
            "{} {} Features\n",
            "#".repeat(features_section_level),
            features_header_prefix
        )
        .unwrap();

        // List features or state None
        if self.manifest_data.features.is_empty() {
            writeln!(self.output, "- None").unwrap();
        } else {
            // Sort features for consistent output
            let mut sorted_features: Vec<_> = self.manifest_data.features.keys().collect();
            sorted_features.sort_unstable();
            for feature_name in sorted_features {
                // TODO: Maybe show what features a feature enables? Requires more parsing.
                writeln!(self.output, "- `{}`", feature_name).unwrap();
            }
        }
        writeln!(self.output).unwrap(); // Add newline after features list
        self.pop_level(); // Pop H3 features level

        // Increment H2 counter for the next section (README or Common Traits)
        self.post_increment_current_level();

        // Print README content if available
        if let Some(readme) = &self.readme_content {
            info!("Injecting README content.");
            let section_level = self.get_current_header_level(); // Should be 2
            let header_prefix = self.get_header_prefix();
            writeln!(
                self.output,
                "\n{} {} README\n",
                "#".repeat(section_level),
                header_prefix
            )
            .unwrap();
            // Adjust headers starting from level 2 (since it's under the H1 crate header)
            // Use the new adjust_markdown_headers function
            let adjusted_readme = adjust_markdown_headers(readme, section_level);
            writeln!(self.output, "{}\n", adjusted_readme).unwrap();
            self.post_increment_current_level(); // Increment H2 counter
        }

        // Print Crate Common Traits Section (H2)
        if !self.no_common_traits && !self.crate_common_traits.is_empty() {
            let common_traits_level = self.get_current_header_level(); // Should be 2
            let common_traits_prefix = self.get_header_prefix();
            writeln!(
                self.output,
                "\n{} {} Common Traits\n",
                "#".repeat(common_traits_level),
                common_traits_prefix
            )
            .unwrap();
            writeln!(self.output, "The following traits are commonly implemented by types in this crate. Unless otherwise noted, you can assume these traits are implemented:\n").unwrap();

            let sorted_common_traits: Vec<FormattedTraitImpl> = {
                let mut traits: Vec<_> = self.crate_common_traits.iter().cloned().collect();
                traits.sort_by_key(|t| t.formatted_markdown_list_entry.clone());
                traits
            };

            let formatted_list = self.format_trait_list(&sorted_common_traits);
            if !formatted_list.is_empty() {
                write!(self.output, "{}", formatted_list).unwrap();
            }
            writeln!(self.output).unwrap();
            self.post_increment_current_level(); // Increment H2 counter
        }

        // --- Print Top-Level Sections (Macros first, then Modules) ---

        // --- Macros Section (Level 2) ---
        // Find macros directly under the resolved root module
        if let Some(resolved_root_module) = self.resolved_modules.get(&self.krate.root) {
            let macro_ids: Vec<Id> = resolved_root_module
                .items
                .iter()
                .filter(|id| self.selected_ids.contains(id))
                .filter(|id| {
                    matches!(
                        self.get_item_kind(id),
                        Some(ItemKind::Macro | ItemKind::ProcAttribute | ItemKind::ProcDerive)
                    )
                })
                .cloned() // Clone the IDs
                .collect();

            if !macro_ids.is_empty() {
                let section_level = self.get_current_header_level(); // Should be 2
                let header_prefix = self.get_header_prefix();
                writeln!(
                    self.output,
                    "\n{} {} Macros",
                    "#".repeat(section_level),
                    header_prefix
                )
                .unwrap();

                self.push_level(); // Push H3 level for macro items
                let mut sorted_macros = macro_ids;
                sorted_macros
                    .sort_by_key(|id| self.krate.index.get(id).and_then(|item| item.name.clone()));
                for id in sorted_macros {
                    self.print_item_details(&id); // Macro details at level 3
                }
                self.pop_level(); // Pop H3 level
                self.post_increment_current_level(); // Increment H2 counter
            }
        }

        // --- Modules (Depth-First Traversal) ---

        // 1. Print Crate Root Module explicitly (will increment H2 counter)
        self.print_module_recursive(self.krate.root);

        // 2. Iterate through sorted top-level modules and print recursively
        // Clone the list to avoid borrowing issues
        let top_level_ids = self.module_tree.top_level_modules.clone();
        for module_id in top_level_ids {
            self.print_module_recursive(module_id); // Will increment H2 counter
        }

        // --- Handle "Other" Items ---
        let mut unprinted_ids = Vec::new();
        for id in &self.selected_ids {
            if !self.printed_ids.contains_key(id) {
                // Skip impl items and use items as they are handled implicitly or ignored
                // Also skip struct fields as they are handled within their containers
                // Also skip Modules as they are handled explicitly above
                if let Some(item) = self.krate.index.get(id) {
                    if !matches!(
                        item.inner,
                        ItemEnum::Impl(_)
                            | ItemEnum::Use { .. }
                            | ItemEnum::StructField(_)
                            | ItemEnum::Module(_) // Modules explicitly skipped here
                    ) && item.name.is_some()
                    {
                        unprinted_ids.push(*id);
                    }
                    // If it doesn't have a name or is a StructField/Module but is selected & unprinted, mark it printed now to avoid the warning
                    else if item.name.is_none()
                        || matches!(item.inner, ItemEnum::StructField(_) | ItemEnum::Module(_))
                    {
                        self.printed_ids.insert(*id, "SKIPPED_OTHER".to_string());
                    }
                } else {
                    // ID selected but not in index - treat as unprinted for "Other"
                    unprinted_ids.push(*id);
                }
            }
        }

        if !unprinted_ids.is_empty() {
            if self.include_other {
                warn!(
                    "Found {} selected items that were not printed in the main structure. Including them in the 'Other' section.",
                    unprinted_ids.len()
                );
                let other_section_level = self.get_current_header_level(); // Should be 2
                let header_prefix = self.get_header_prefix();
                writeln!(
                    self.output,
                    "\n{} {} Other", // Use ## level for this section
                    "#".repeat(other_section_level),
                    header_prefix
                )
                .unwrap();

                // Push H3 level for items in Other
                self.push_level();
                // Sort unprinted items for consistent output
                unprinted_ids.sort_by_key(|id| {
                    (
                        self.krate.paths.get(id).map(|p| p.path.clone()),
                        self.krate.index.get(id).and_then(|i| i.name.clone()),
                    )
                });

                for id in &unprinted_ids {
                    let path_str = format_id_path_canonical(id, self.krate);
                    warn!("Including unprinted item in 'Other' section: {}", path_str);

                    // Fetch the item to print its header and span
                    if let Some(item) = self.krate.index.get(id) {
                        // Print details (handles level incrementing internally)
                        self.print_item_details(id);

                        // Get level AFTER printing details for graph context
                        // let item_level = self.get_current_header_level(); // Should be H3+1 = H4 // Level not needed here

                        // Print Source Location (if available) ONLY for "Other" items
                        if let Some(span) = &item.span {
                            writeln!(
                                self.output,
                                "_Source: `{}:{}:{}`_\n", // Italic, newline after
                                span.filename.display(),
                                span.begin.0 + 1, // Line numbers are 0-based
                                span.begin.1 + 1  // Column numbers are 0-based
                            )
                            .unwrap();
                        }
                        // Always print graph context afterwards for items in "Other"
                        self.print_graph_context(id);
                    } else {
                        // Handle case where ID is selected but not in index (rare)
                        self.post_increment_current_level(); // Increment level for this item
                        let other_item_level = self.get_current_header_level();
                        let item_prefix = self.get_header_prefix();
                        writeln!(
                            self.output,
                            "\n{} {} `{}`\n",
                            "#".repeat(other_item_level),
                            item_prefix,
                            path_str // Use path string as header
                        )
                        .unwrap();
                        writeln!(self.output, "_Error: Item details not found in index._\n")
                            .unwrap();
                        self.print_graph_context(id); // Still print graph context
                    }
                }
                self.pop_level(); // Pop H3 level for items
            } else {
                // Group by kind and log counts
                let mut counts_by_kind: HashMap<ItemKind, usize> = HashMap::new(); // Use HashMap
                for id in &unprinted_ids {
                    if let Some(kind) = self.get_item_kind(id) {
                        *counts_by_kind.entry(kind).or_insert(0) += 1;
                    } else {
                        // Count items where kind couldn't be determined (e.g., ID not in index)
                        *counts_by_kind.entry(ItemKind::StructField).or_insert(0) += 1;
                        // Use a placeholder kind like StructField
                    }
                }
                warn!(
                    "Skipped printing {} items not fitting into standard sections (use --include-other to see them):",
                    unprinted_ids.len()
                );
                // Convert HashMap to Vec for sorting before printing warnings
                let mut sorted_counts: Vec<_> = counts_by_kind.into_iter().collect();
                sorted_counts.sort_by_key(|(kind, _)| format!("{:?}", kind)); // Sort by debug representation for consistency

                for (kind, count) in sorted_counts {
                    warn!("  - {:?}: {}", kind, count);
                }
            }
        }

        // --- Examples Appendix ---
        // Clone self.examples before iterating to avoid borrow checker issues with mutable self calls
        let examples_clone = self.examples.clone();
        if !examples_clone.is_empty() || self.examples_readme_content.is_some() {
            let examples_section_level = self.get_current_header_level(); // Should be 2
            let header_prefix = self.get_header_prefix();
            writeln!(
                self.output,
                "\n{} {} Examples Appendix\n",
                "#".repeat(examples_section_level),
                header_prefix
            )
            .unwrap();
            self.push_level(); // Push for H3 example headers

            if let Some(readme) = &self.examples_readme_content {
                let adjusted_readme = adjust_markdown_headers(readme, examples_section_level);
                writeln!(self.output, "{}\n", adjusted_readme).unwrap();
            }

            for (filename, content) in &examples_clone {
                let example_header_level = self.get_current_header_level(); // Should be 3
                let example_prefix = self.get_header_prefix();
                writeln!(
                    self.output,
                    "{} {} `{}`\n",
                    "#".repeat(example_header_level),
                    example_prefix,
                    filename
                )
                .unwrap();
                writeln!(self.output, "```rust\n{}\n```\n", content).unwrap();
                self.post_increment_current_level(); // Increment H3 counter for next example
            }
            self.pop_level(); // Pop H3 example level
            self.post_increment_current_level(); // Increment H2 counter for next top-level section
        }
        self.output
    }
}
