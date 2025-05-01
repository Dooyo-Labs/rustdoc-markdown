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
//! std = "1.0"
//! tar = "0.4"
//! tempfile = "3.8"
//! tokio = { version = "1.34", features = ["full"] }
//! tracing = "0.1"
//! tracing-subscriber = { version = "0.3", features = ["env-filter"] }
//! rustdoc-json = "*"
//! rustup-toolchain = "0.1"
//! cargo-manifest = "0.19"
//! ```
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::cognitive_complexity)] // Allow complex functions for now

use anyhow::{anyhow, bail, Context, Result};
use cargo_manifest::{FeatureSet, Manifest};
use clap::Parser;
use flate2::read::GzDecoder;
use rustdoc_json::Builder;
use rustdoc_types::{
    Abi, Constant, Crate, Discriminant, Enum, Function, GenericArg, GenericArgs, GenericBound,
    GenericParamDef, Generics, Id, Impl, Item, ItemEnum, ItemKind, Path, PolyTrait, Primitive,
    Struct, StructKind, Term, Trait, Type, Variant, VariantKind, WherePredicate,
};
use semver::{Version, VersionReq};
use serde::Deserialize;
use std::collections::{HashMap, HashSet, VecDeque}; // Use HashMap instead of BTreeMap where needed
use std::fmt::{Display, Formatter, Write as FmtWrite}; // Use FmtWrite alias
use std::fs::{self, File}; // Import fs module
use std::io::{BufReader, BufWriter, Cursor, Write as IoWrite}; // Use IoWrite alias and IMPORT Cursor
use std::path::{Path as FilePath, PathBuf}; // Corrected use statement
use tar::Archive;
use tracing::{debug, info, warn};
use tracing_subscriber::EnvFilter;

const NIGHTLY_RUST_VERSION: &str = "nightly-2025-03-24";
const AUTO_TRAITS: &[&str] = &[
    "Send",
    "Sync",
    "Unpin",
    "UnwindSafe",
    "RefUnwindSafe",
    "Freeze",
    // Add other common auto traits if needed
];

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

    /// Build directory for crate documentation artifacts
    #[arg(long, default_value = ".ai/docs/rust/build")]
    build_dir: String, // Renamed from output_dir

    /// Path to write the generated documentation (defaults to stdout)
    #[arg(long)]
    output: Option<PathBuf>, // New argument for output file

    /// Enable dependency graph dumping and specify the output file.
    #[arg(long)]
    dump_graph: Option<PathBuf>,

    /// Dump graph starting only from module roots (requires --dump-graph).
    #[arg(long, requires = "dump_graph")]
    dump_modules: bool,

    /// Dump graph starting only from this ID (requires --dump-graph). Takes precedence over --dump-modules.
    #[arg(long, value_parser = parse_id, requires = "dump_graph")]
    dump_from_id: Option<Id>,

    /// Filter graph dump to only include paths leading to this leaf ID (requires --dump-graph).
    #[arg(long, value_parser = parse_id, requires = "dump_graph")]
    dump_to_id: Option<Id>,

    /// Limit the maximum depth of the dumped graph (requires --dump-graph).
    /// 0 means root only, 1 means root and direct children, etc.
    #[arg(long, requires = "dump_graph")]
    dump_max_depth: Option<usize>,

    /// Filter documented items by module path (e.g., "::style", "widgets::Button"). Can be specified multiple times.
    /// Paths starting with '::' imply the root of the current crate.
    /// Matches are prefix-based (e.g., "::style" matches "::style::TextStyle").
    #[arg(long = "path")]
    paths: Vec<String>,

    /// Include items that don't fit standard categories in a final 'Other' section.
    #[arg(long)]
    include_other: bool,

    /// Space-separated list of features to activate
    #[arg(long)]
    features: Option<String>, // Vec<String>? Clap might handle space separation

    /// Do not activate the `default` feature
    #[arg(long)]
    no_default_features: bool,

    /// Build documentation for the specified target triple
    #[arg(long)]
    target: Option<String>,

    /// Output mustache template markers (`{{MISSING_DOCS_1_2_...}}`) instead of the actual documentation for items with docstrings.
    #[arg(long)]
    template: bool,

    /// Do not embed the crate's README file in the output.
    #[arg(long)]
    no_readme: bool,
}

/// Parses a string into an `Id`.
fn parse_id(s: &str) -> Result<Id, String> {
    s.parse::<u32>()
        .map(Id)
        .map_err(|_| format!("Invalid ID: '{}'. Must be a non-negative integer.", s))
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

// --- ID Graph Structures ---

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum EdgeLabel {
    Contains,          // Module contains Item (original structure)
    ReferencesType,    // Item references Type ID (e.g., field type, return type)
    GenericArgument,   // Path uses Type ID as generic arg
    AssociatedType,    // Item references Associated Type ID
    AssociatedConstant, // Item references Associated Constant ID
    TraitBound,        // Generic Param/Where Clause has Trait Bound ID
    Implements,        // Impl block implements Trait ID
    ImplFor,           // Impl block is for Type ID
    ImplItem,          // Impl block contains Item ID
    TraitItem,         // Trait contains Item ID
    EnumVariant,       // Enum contains Variant ID
    VariantField,      // Variant contains Field ID
    StructField,       // Struct contains Field ID
    UnionField,        // Union contains Field ID
    FieldType,         // Field ID has Type ID
    AliasTo,           // TypeAlias/TraitAlias points to Type/Trait ID
    SignatureInput,    // Function signature references input Type ID
    SignatureOutput,   // Function signature references output Type ID
    SuperTrait,        // Trait has supertrait Trait ID
    Dependency,        // Generic catch-all for less specific type dependencies
    IntraDocLink,      // Doc comment links to Item ID
    AssociatedConstraint, // Generic Arg Constraint references Item ID
    ParamType,         // Generic Param Def references Type ID
    ParamBound,        // Generic Param Def references Bound/Trait ID
    PredicateType,     // Where Predicate references Type ID
    PredicateBound,    // Where Predicate references Bound/Trait ID
    PredicateEqLhs,    // Where Predicate Eq references LHS Type ID
    PredicateEqRhs,    // Where Predicate Eq references RHS Term ID
    DynTraitBound,     // DynTrait references Trait ID
    ImplTraitBound,    // ImplTrait references Bound/Trait ID
    UseTarget,         // Use item references target item/module ID
}

impl Display for EdgeLabel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Edge {
    source: Id,
    target: Id,
    label: EdgeLabel,
}

#[derive(Debug, Default, Clone)] // Add Clone
struct IdGraph {
    edges: HashSet<Edge>, // Use HashSet to avoid duplicate edges
    // Add an adjacency list representation for easier traversal (target -> Vec<(source, label)>)
    // Note: We build the forward graph (source -> targets) for dependency finding.
    // For finding roots (no incoming edges), we analyze the final edge set.
    // For tree printing, we need source -> Vec<(target, label)>
    adjacency: HashMap<Id, Vec<(Id, EdgeLabel)>>,
    // Reverse adjacency list for filtering (target -> Vec<(source, label)>)
    reverse_adjacency: HashMap<Id, Vec<(Id, EdgeLabel)>>,
}

impl IdGraph {
    fn new() -> Self {
        Self::default()
    }

    /// Adds an edge, ensuring both source and target are in the crate index.
    fn add_edge(&mut self, source: Id, target: Id, label: EdgeLabel, krate: &Crate) {
        // Only add edges where both nodes are part of the local crate
        if krate.index.contains_key(&source) && krate.index.contains_key(&target) {
            let edge = Edge {
                source,
                target,
                label: label.clone(),
            };
            // Clone edge before inserting into the HashSet to avoid move error
            if self.edges.insert(edge.clone()) {
                // Also update the adjacency list for forward traversal (needed for dump)
                self.adjacency
                    .entry(source)
                    .or_default()
                    .push((target, label.clone()));
                // Update reverse adjacency list
                self.reverse_adjacency
                    .entry(target)
                    .or_default()
                    .push((edge.source, label)); // Correct tuple syntax
            }
        }
    }

    /// Finds all direct children of a node (source -> targets).
    fn get_children(&self, source_id: &Id) -> Option<&Vec<(Id, EdgeLabel)>> {
        self.adjacency.get(source_id)
    }

    /// Finds all nodes that have no incoming edges *from within the graph*.
    fn find_roots(&self) -> HashSet<Id> {
        let mut all_nodes: HashSet<Id> = HashSet::new();
        let mut targets: HashSet<Id> = HashSet::new();

        for edge in &self.edges {
            all_nodes.insert(edge.source);
            all_nodes.insert(edge.target);
            targets.insert(edge.target);
        }

        all_nodes.difference(&targets).cloned().collect()
    }

    #[allow(dead_code)] // Keep for future debugging use
    fn find_incoming_edges(&self, target_id: &Id) -> Vec<&Edge> {
        self.edges
            .iter()
            .filter(|edge| edge.target == *target_id)
            .collect()
    }

    /// Filters the graph to keep only edges that are part of a path leading to the target_leaf_id.
    /// Returns a new `IdGraph` containing only the filtered edges.
    fn filter_to_leaf(&self, target_leaf_id: Id) -> IdGraph {
        let mut filtered_graph = IdGraph::new();
        let mut reachable_nodes = HashSet::new(); // Nodes that can reach the target
        let mut queue = VecDeque::new();

        // Start BFS from the target node using the reverse adjacency list
        // Check existence in reverse_adjacency OR adjacency (node might exist but have no incoming edges)
        if self.reverse_adjacency.contains_key(&target_leaf_id)
            || self.adjacency.contains_key(&target_leaf_id)
        {
            reachable_nodes.insert(target_leaf_id);
            queue.push_back(target_leaf_id);
        } else {
            // Target ID doesn't exist in the original graph's node set
            return filtered_graph; // Return empty graph
        }

        while let Some(current_id) = queue.pop_front() {
            if let Some(parents) = self.reverse_adjacency.get(&current_id) {
                for (parent_id, _) in parents {
                    if reachable_nodes.insert(*parent_id) {
                        queue.push_back(*parent_id);
                    }
                }
            }
        }

        // Now, add edges from the original graph *only if both* source and target are in reachable_nodes
        for edge in &self.edges {
            if reachable_nodes.contains(&edge.source) && reachable_nodes.contains(&edge.target) {
                // Manually add to filtered graph components (avoiding add_edge's krate check)
                if filtered_graph.edges.insert(edge.clone()) {
                    filtered_graph
                        .adjacency
                        .entry(edge.source)
                        .or_default()
                        .push((edge.target, edge.label.clone()));
                    filtered_graph
                        .reverse_adjacency
                        .entry(edge.target)
                        .or_default()
                        .push((edge.source, edge.label.clone())); // Correct tuple syntax
                }
            }
        }

        filtered_graph // Return the newly constructed filtered graph
    }
}

// --- End ID Graph Structures ---

// --- Module Resolution Structures ---

#[derive(Debug, Clone)]
enum ResolutionState {
    Unresolved,
    Resolving,
    Resolved(HashSet<Id>),
}

type ResolutionCache = HashMap<Id, ResolutionState>;

/// Represents a module with its fully resolved items after handling 'use' statements.
#[derive(Debug, Clone)]
struct ResolvedModule {
    id: Id,
    /// The fully resolved set of item IDs directly accessible within this module.
    items: HashSet<Id>,
}

/// Recursively resolves items for a module, handling `use` statements and cycles.
fn resolve_module_items(
    module_id: Id,
    krate: &Crate,
    cache: &mut ResolutionCache,
) -> HashSet<Id> {
    // Check cache for cycle or previous result
    match cache.get(&module_id) {
        Some(ResolutionState::Resolving) => {
            debug!("Cycle detected resolving module ID: {:?}", module_id);
            return HashSet::new(); // Break cycle
        }
        Some(ResolutionState::Resolved(items)) => {
            return items.clone();
        }
        Some(ResolutionState::Unresolved) | None => {
            // Continue resolution
        }
    }

    // Mark as resolving
    cache.insert(module_id, ResolutionState::Resolving);
    debug!("Resolving module ID: {:?}", module_id);

    let mut resolved_items = HashSet::new();

    // Get the original module definition
    if let Some(module_item) = krate.index.get(&module_id) {
        if let ItemEnum::Module(module_data) = &module_item.inner {
            for item_id in &module_data.items {
                if let Some(item) = krate.index.get(item_id) {
                    match &item.inner {
                        ItemEnum::Use(use_item) => {
                            if let Some(target_id) = use_item.id {
                                if use_item.is_glob {
                                    // Glob import: Recursively resolve the target module/enum/etc.
                                    // Check if target_id points to a Module. Glob imports
                                    // can also be used on Enums, but we primarily care
                                    // about module imports here for bringing items into scope.
                                    if let Some(target_item) = krate.index.get(&target_id) {
                                        if matches!(target_item.inner, ItemEnum::Module(_)) {
                                            debug!(
                                                "Glob import from {:?} to {:?} in module {:?}",
                                                target_id, item_id, module_id
                                            );
                                            let imported_items =
                                                resolve_module_items(target_id, krate, cache);
                                            resolved_items.extend(imported_items);
                                        } else {
                                            // Glob import from something not a module (e.g., Enum)
                                            // Just add the enum ID itself for now.
                                            resolved_items.insert(target_id);
                                        }
                                    }
                                } else {
                                    // Single item import: Add the target ID directly
                                    resolved_items.insert(target_id);
                                }
                            }
                            // Ignore use items with id: None (primitive re-exports) for resolution
                        }
                        _ => {
                            // Not a use statement, add the item ID directly
                            resolved_items.insert(*item_id);
                        }
                    }
                }
            }
        } // Module item might not have Module inner if it's a re-export target itself? No, index should contain the real item.
          // Let's warn if the found item isn't a module.
          // else {
          //     warn!("Item with module ID {:?} is not actually a Module kind: {:?}", module_id, module_item.inner);
          // }
    } else {
        warn!("Module ID {:?} not found in crate index.", module_id);
    }

    // Mark as resolved and cache the result
    debug!(
        "Resolved module ID {:?} with {} items.",
        module_id,
        resolved_items.len()
    );
    cache.insert(
        module_id,
        ResolutionState::Resolved(resolved_items.clone()),
    );
    resolved_items
}

/// Builds an index of all modules with their items resolved after handling 'use' statements.
fn build_resolved_module_index(krate: &Crate) -> HashMap<Id, ResolvedModule> {
    info!("Building resolved module index...");
    let mut resolved_index = HashMap::new();
    let mut cache: ResolutionCache = HashMap::new();

    for (id, item) in &krate.index {
        if let ItemEnum::Module(_) = &item.inner {
            if !resolved_index.contains_key(id) {
                let resolved_items = resolve_module_items(*id, krate, &mut cache);
                resolved_index.insert(
                    *id,
                    ResolvedModule {
                        id: *id,
                        items: resolved_items,
                    },
                );
            }
        }
    }
    info!(
        "Built resolved module index for {} modules.",
        resolved_index.len()
    );
    resolved_index
}

// --- End Module Resolution Structures ---

// --- Graph Dumping Logic ---

/// Helper to get item info string (name, path, kind)
fn get_item_info_string(id: &Id, krate: &Crate) -> String {
    let name_str = krate
        .index
        .get(id)
        .and_then(|item| item.name.as_deref())
        .unwrap_or("{unnamed}");
    let path_str = krate
        .paths
        .get(id)
        .map(|p| p.path.join("::"))
        .unwrap_or_else(|| "{no_path}".to_string());
    let kind_str = krate
        .index
        .get(id)
        .map(|item| format!("{:?}", DocPrinter::infer_item_kind(item))) // Reuse infer_item_kind
        .or_else(|| {
            krate
                .paths
                .get(id)
                .map(|summary| format!("{:?}", summary.kind))
        })
        .unwrap_or_else(|| "{UnknownKind}".to_string());

    format!(
        "Id({}): {} (Path: {}, Kind: {})",
        id.0, name_str, path_str, kind_str
    )
}

/// Recursive function to dump the graph structure.
fn dump_node(
    node_id: Id,
    graph: &IdGraph,           // Use the potentially filtered graph
    krate: &Crate,
    writer: &mut BufWriter<File>,
    visited: &mut HashSet<Id>, // Use mutable reference to shared visited set
    path_to_target: &mut HashSet<Id>, // Tracks current path to target leaf
    indent: usize,
    depth: usize,             // Current recursion depth
    max_depth: Option<usize>, // Maximum allowed depth
    prefix: &str,             // Prefix like "├── " or "└── "
    parent_label: Option<&EdgeLabel>, // Label connecting this node to its parent
    is_root_call: bool, // Flag to know if this is the initial call for a root
) -> Result<()> {
    // Track current node in the path being explored towards the target
    let inserted_in_path = path_to_target.insert(node_id);

    // Determine if this node has already been visited *globally*
    let is_newly_visited = visited.insert(node_id);

    // Determine if we should print this node
    // Print if:
    // 1. It's the root of the current dump traversal (is_root_call is true)
    // 2. OR it's newly visited globally
    // 3. OR it's already visited globally BUT it's part of the current path to the target
    let should_print = is_root_call || is_newly_visited || path_to_target.contains(&node_id);

    if should_print {
        // Format the current node information
        let node_info = get_item_info_string(&node_id, krate);
        let label_info = parent_label.map(|l| format!(" [{}]", l)).unwrap_or_default();
        // Add cycle marker only if globally visited before AND relevant to current path
        let cycle_marker = if !is_newly_visited && path_to_target.contains(&node_id) && !is_root_call {
            " [... cycle or previously visited on current path ...]"
        } else if !is_newly_visited && !is_root_call {
            // This case should ideally not be reached often if filtering works, but indicates a visited node NOT on the current path
            " [... previously visited (not on current path) ...]" // This might still be printed if filter is off
        } else {
            ""
        };

        writeln!(
            writer,
            "{}{}{}{}{}",
            " ".repeat(indent),
            prefix,
            node_info,
            label_info,
            cycle_marker
        )?;
    }

    // Check depth limit *before* recursing
    if let Some(max) = max_depth {
        if depth >= max {
            // If we've reached max depth and there are children, indicate truncation
            if is_newly_visited && graph.get_children(&node_id).map_or(false, |c| !c.is_empty()) {
                writeln!(
                    writer,
                    "{}{} [... children truncated due to max depth ...]",
                    " ".repeat(indent + 4), // Indent the truncation message
                    if graph.get_children(&node_id).unwrap().len() == 1 { "└──" } else { "├──" } // Use appropriate prefix for one or more truncated children
                )?;
            }
            // Backtrack and return early if max depth is reached
            if inserted_in_path {
                path_to_target.remove(&node_id);
            }
            return Ok(());
        }
    }

    // Recurse only if newly visited globally
    // (If !is_newly_visited, we've already explored its children from a previous encounter)
    if is_newly_visited {
        // Get children from the potentially filtered graph and sort them
        if let Some(children) = graph.get_children(&node_id) {
            let mut sorted_children = children.to_vec(); // Clone to sort

            // Sort by target Id primarily, then label for stability
            sorted_children.sort_by_key(|(target_id, label)| (target_id.0, format!("{}", label)));

            let num_children = sorted_children.len();
            for (i, (child_id, child_label)) in sorted_children.iter().enumerate() {
                let new_prefix = if i == num_children - 1 {
                    "└── "
                } else {
                    "├── "
                };
                let child_indent = indent + 4; // Indent children further

                // Recurse with the same mutable visited set and path_to_target set
                dump_node(
                    *child_id,
                    graph, // Pass the same graph down
                    krate,
                    writer,
                    visited,         // Pass mutable reference down
                    path_to_target, // Pass mutable reference down
                    child_indent,
                    depth + 1, // Increment depth for child
                    max_depth, // Pass max_depth down
                    new_prefix,
                    Some(child_label),
                    false, // Not a root call anymore
                )?;
            }
        }
    }

    // Backtrack: Remove current node from the path_to_target set *if* it was added by this call
    if inserted_in_path {
        path_to_target.remove(&node_id);
    }

    Ok(())
}

/// Dumps a subset of the dependency graph to a file based on specified roots.
fn dump_graph_subset(
    graph: &IdGraph, // Use the potentially filtered graph
    krate: &Crate,
    root_ids: &HashSet<Id>,
    output_path: &FilePath,
    dump_description: &str,
    max_depth: Option<usize>, // Add max_depth parameter
) -> Result<()> {
    info!(
        "Dumping {} graph to: {}",
        dump_description,
        output_path.display()
    );
    let file = File::create(output_path)
        .with_context(|| format!("Failed to create graph dump file: {}", output_path.display()))?;
    let mut writer = BufWriter::new(file);

    // Use a single visited set for the entire dump process across all roots
    let mut visited = HashSet::new();

    let mut sorted_roots: Vec<_> = root_ids.iter().cloned().collect();
    // Sort roots by Id for consistent output
    sorted_roots.sort_by_key(|id| id.0);

    if sorted_roots.is_empty() && !graph.edges.is_empty() {
        writeln!(writer, "Warning: Graph has edges but no {} roots found (potentially due to filtering or cycles). Dumping all nodes alphabetically:", dump_description)?;
        // Fallback: dump all nodes if no roots found
        let mut all_nodes: Vec<_> = graph.adjacency.keys().cloned().collect();
        all_nodes.sort_by_key(|id| id.0);
        for node_id in all_nodes {
            // Check if already visited globally
            if !visited.contains(&node_id) {
                // Initialize an empty path_to_target for this arbitrary root start
                let mut path_to_target = HashSet::new();
                dump_node(
                    node_id,
                    graph,
                    krate,
                    &mut writer,
                    &mut visited,     // Pass shared mutable visited set
                    &mut path_to_target, // Pass new mutable path set
                    0,
                    0,         // Initial depth is 0
                    max_depth, // Pass max_depth
                    "",        // No prefix for top-level nodes in fallback
                    None,
                    true, // It's a root call in this fallback context
                )?;
            }
        }
    } else {
        writeln!(writer, "Graph Roots ({}):", dump_description)?;
        for root_id in sorted_roots {
            // Check if already visited globally before starting a new root traversal
            if !visited.contains(&root_id) {
                // Initialize a path_to_target set for *each* root dump traversal
                let mut path_to_target = HashSet::new();
                dump_node(
                    root_id,
                    graph,
                    krate,
                    &mut writer,
                    &mut visited,     // Pass shared mutable visited set
                    &mut path_to_target, // Pass new mutable path set for this root
                    0,
                    0,         // Initial depth is 0
                    max_depth, // Pass max_depth
                    "",        // No prefix for root nodes
                    None,
                    true, // It's a root call
                )?;
            }
        }
    }

    writer
        .flush()
        .with_context(|| format!("Failed to write graph to file: {}", output_path.display()))?;
    info!("Successfully dumped graph to {}", output_path.display());
    Ok(())
}

// --- End Graph Dumping Logic ---

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
    build_path: &FilePath, // Renamed from output_path
) -> Result<PathBuf> {
    let crate_dir_name = format!("{}-{}", krate.crate_name, krate.num);
    let target_dir = build_path.join(crate_dir_name); // Use build_path

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
    let reader = Cursor::new(content); // Cursor is now in scope

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

fn run_rustdoc(
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

    let json_path = crate_dir
        .join("target/doc")
        .join(format!("{}.json", crate_name));

    // Avoid regenerating if exists
    if json_path.exists() {
        info!("rustdoc JSON already exists at: {}", json_path.display());
        return Ok(json_path);
    }

    let mut builder = Builder::default()
        .manifest_path(manifest_path)
        .toolchain(NIGHTLY_RUST_VERSION) // Specify the nightly toolchain
        .target_dir(crate_dir.join("target/doc")) // Set the output directory
        .package(crate_name); // Specify the package

    // Apply feature flags
    if let Some(features_str) = features {
        let feature_list: Vec<String> =
            features_str.split_whitespace().map(String::from).collect();
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
fn find_type_dependencies(
    ty: &Type,
    source_id: Id, // The ID of the item *containing* this type reference
    krate: &Crate,
    dependencies: &mut HashSet<Id>,
    graph: &mut IdGraph,
    edge_label: EdgeLabel, // How the source_id relates to this type
) {
    // Add the direct ID if the type itself resolves to one
    if let Some(id) = get_type_id(ty) {
        if krate.index.contains_key(&id) {
            if dependencies.insert(id) {
                graph.add_edge(source_id, id, edge_label.clone(), krate);
            }
        }
    }

    // Recursively check inner types and generic arguments
    match ty {
        Type::ResolvedPath(Path { args, id, .. }) => {
            // Add the path's own ID
            if krate.index.contains_key(id) {
                if dependencies.insert(*id) {
                    graph.add_edge(source_id, *id, edge_label.clone(), krate);
                }
            }
            // Check generic arguments
            if let Some(args_box) = args.as_ref() {
                // args is &Box<GenericArgs>, need to get &GenericArgs
                find_generic_args_dependencies(
                    args_box,
                    source_id, // The source item uses these generic args
                    krate,
                    dependencies,
                    graph,
                );
            }
        }
        Type::Tuple(inner_types) => {
            for inner_ty in inner_types {
                find_type_dependencies(
                    inner_ty,
                    source_id,
                    krate,
                    dependencies,
                    graph,
                    EdgeLabel::Dependency, // Generic dependency for tuple contents
                );
            }
        }
        Type::Slice(inner_ty) => {
            find_type_dependencies(
                inner_ty,
                source_id,
                krate,
                dependencies,
                graph,
                EdgeLabel::Dependency, // Type contained in slice
            );
        }
        Type::Array { type_, .. } => {
            find_type_dependencies(
                type_,
                source_id,
                krate,
                dependencies,
                graph,
                EdgeLabel::Dependency, // Type contained in array
            );
        }
        Type::Pat { type_, .. } => {
            find_type_dependencies(
                type_,
                source_id,
                krate,
                dependencies,
                graph,
                EdgeLabel::Dependency, // Type in pattern
            );
        }
        Type::RawPointer { type_, .. } => {
            find_type_dependencies(
                type_,
                source_id,
                krate,
                dependencies,
                graph,
                EdgeLabel::Dependency, // Pointee type
            );
        }
        Type::BorrowedRef { type_, .. } => {
            find_type_dependencies(
                type_,
                source_id,
                krate,
                dependencies,
                graph,
                EdgeLabel::Dependency, // Referenced type
            );
        }
        Type::QualifiedPath {
            args,
            self_type,
            trait_,
            ..
        } => {
            find_type_dependencies(
                self_type,
                source_id,
                krate,
                dependencies,
                graph,
                EdgeLabel::Dependency, // Self type in qualified path
            );
            if let Some(trait_path) = trait_ {
                if krate.index.contains_key(&trait_path.id) {
                    if dependencies.insert(trait_path.id) {
                        // This source_id uses an associated type from trait_path.id
                        graph.add_edge(
                            source_id,
                            trait_path.id,
                            EdgeLabel::AssociatedType, // Or AssociatedConstant? Ambiguous here. Use AssociatedType as default.
                            krate,
                        );
                    }
                }
            }
            find_generic_args_dependencies(args, source_id, krate, dependencies, graph);
        }
        Type::DynTrait(dyn_trait) => {
            for poly_trait in &dyn_trait.traits {
                if krate.index.contains_key(&poly_trait.trait_.id) {
                    if dependencies.insert(poly_trait.trait_.id) {
                        graph.add_edge(
                            source_id,
                            poly_trait.trait_.id,
                            EdgeLabel::DynTraitBound,
                            krate,
                        );
                    }
                }
                // Check generic param defs within the poly trait
                for param_def in &poly_trait.generic_params {
                    find_generic_param_def_dependencies(
                        param_def,
                        source_id,
                        krate,
                        dependencies,
                        graph,
                    );
                }
            }
        }
        Type::ImplTrait(bounds) => {
            for bound in bounds {
                find_generic_bound_dependencies(
                    bound,
                    source_id,
                    krate,
                    dependencies,
                    graph,
                    EdgeLabel::ImplTraitBound,
                );
            }
        }
        Type::FunctionPointer(fp) => {
            // generic_params are HRTBs for the pointer itself
            for param_def in &fp.generic_params {
                find_generic_param_def_dependencies(
                    param_def,
                    source_id, // The source item uses this function pointer type
                    krate,
                    dependencies,
                    graph,
                );
            }
            // sig contains input/output types
            for (_name, input_type) in &fp.sig.inputs {
                find_type_dependencies(
                    input_type,
                    source_id,
                    krate,
                    dependencies,
                    graph,
                    EdgeLabel::SignatureInput,
                );
            }
            if let Some(output) = &fp.sig.output {
                find_type_dependencies(
                    output,
                    source_id,
                    krate,
                    dependencies,
                    graph,
                    EdgeLabel::SignatureOutput,
                );
            }
        }
        // Types without complex inner structures or IDs
        Type::Generic(_) | Type::Primitive(_) | Type::Infer => {}
    }
}

fn find_generic_args_dependencies(
    args: &GenericArgs,
    source_id: Id, // The ID of the item whose path includes these args
    krate: &Crate,
    dependencies: &mut HashSet<Id>,
    graph: &mut IdGraph,
) {
    match args {
        GenericArgs::AngleBracketed {
            args, constraints, ..
        } => {
            for arg in args {
                match arg {
                    GenericArg::Type(t) => find_type_dependencies(
                        t,
                        source_id,
                        krate,
                        dependencies,
                        graph,
                        EdgeLabel::GenericArgument,
                    ),
                    GenericArg::Const(_) => {}
                    GenericArg::Lifetime(_) | GenericArg::Infer => {}
                }
            }
            for constraint in constraints {
                // AssocItemConstraint { name: String, kind: AssocItemConstraintKind }
                match constraint {
                    // Use tuple variant matching
                    rustdoc_types::AssocItemConstraint {
                        name: _, // TODO: Could the name be an ID sometimes? Unlikely.
                        args: assoc_args, // args for the associated type constraint itself
                        binding: rustdoc_types::AssocItemConstraintKind::Equality(term),
                    } => {
                        // The source_id uses this associated type constraint.
                        // Find dependencies within the term (RHS of equality).
                        match term {
                            Term::Type(t) => find_type_dependencies(
                                t,
                                source_id,
                                krate,
                                dependencies,
                                graph,
                                EdgeLabel::AssociatedConstraint, // Term type referenced in constraint
                            ),
                            Term::Constant(_) => {} // Constant expr/value are stringly typed
                        }
                        // Also find dependencies in the arguments *to* the associated type
                        find_generic_args_dependencies(
                            assoc_args,
                            source_id,
                            krate,
                            dependencies,
                            graph,
                        );
                    }
                    rustdoc_types::AssocItemConstraint {
                        name: _,
                        args: assoc_args,
                        binding: rustdoc_types::AssocItemConstraintKind::Constraint(bounds),
                    } => {
                        // The source_id uses this associated type constraint.
                        for bound in bounds {
                            find_generic_bound_dependencies(
                                bound,
                                source_id,
                                krate,
                                dependencies,
                                graph,
                                EdgeLabel::AssociatedConstraint, // Bound referenced in constraint
                            );
                        }
                        // Also find dependencies in the arguments *to* the associated type
                        find_generic_args_dependencies(
                            assoc_args,
                            source_id,
                            krate,
                            dependencies,
                            graph,
                        );
                    }
                }
            }
        }
        GenericArgs::Parenthesized { inputs, output, .. } => {
            // Usually for Fn traits. source_id references this Fn trait path.
            for input in inputs {
                find_type_dependencies(
                    input,
                    source_id,
                    krate,
                    dependencies,
                    graph,
                    EdgeLabel::Dependency, // Input type for Fn trait arg
                );
            }
            if let Some(out) = output {
                find_type_dependencies(
                    out,
                    source_id,
                    krate,
                    dependencies,
                    graph,
                    EdgeLabel::Dependency, // Output type for Fn trait arg
                );
            }
        }
        GenericArgs::ReturnTypeNotation { .. } => {} // TODO: Handle this? T::method(..) - maybe the T part?
    }
}

fn find_generic_bound_dependencies(
    bound: &GenericBound,
    source_id: Id, // The ID of the item imposing this bound (e.g., in where clause, or on param)
    krate: &Crate,
    dependencies: &mut HashSet<Id>,
    graph: &mut IdGraph,
    edge_label: EdgeLabel, // e.g., ParamBound, PredicateBound
) {
    match bound {
        GenericBound::TraitBound {
            trait_, // This is a Path struct
            generic_params,
            ..
        } => {
            if krate.index.contains_key(&trait_.id) {
                if dependencies.insert(trait_.id) {
                    graph.add_edge(source_id, trait_.id, edge_label.clone(), krate);
                }
            }
            // Trait path itself might have generic args
            if let Some(args) = trait_.args.as_ref() {
                find_generic_args_dependencies(args, source_id, krate, dependencies, graph);
            }
            // Check HRTBs (generic_params)
            for param_def in generic_params {
                find_generic_param_def_dependencies(param_def, source_id, krate, dependencies, graph);
            }
        }
        GenericBound::Outlives(_) | GenericBound::Use(_) => {}
    }
}

fn find_generics_dependencies(
    generics: &Generics,
    source_id: Id, // ID of the item defining these generics
    krate: &Crate,
    dependencies: &mut HashSet<Id>,
    graph: &mut IdGraph,
) {
    for param in &generics.params {
        find_generic_param_def_dependencies(param, source_id, krate, dependencies, graph);
    }
    for predicate in &generics.where_predicates {
        match predicate {
            WherePredicate::BoundPredicate {
                type_,
                bounds,
                generic_params, // HRTBs for the predicate
                ..
            } => {
                // source_id imposes a bound on type_
                find_type_dependencies(
                    type_,
                    source_id,
                    krate,
                    dependencies,
                    graph,
                    EdgeLabel::PredicateType,
                );
                for bound in bounds {
                    // source_id uses 'bound' in a where predicate
                    find_generic_bound_dependencies(
                        bound,
                        source_id,
                        krate,
                        dependencies,
                        graph,
                        EdgeLabel::PredicateBound,
                    );
                }
                // Check HRTBs (generic_params)
                for param_def in generic_params {
                    find_generic_param_def_dependencies(
                        param_def,
                        source_id, // HRTB defined on item source_id
                        krate,
                        dependencies,
                        graph,
                    );
                }
            }
            WherePredicate::LifetimePredicate { .. } => {} // Lifetimes don't have IDs
            WherePredicate::EqPredicate { lhs, rhs, .. } => {
                // source_id requires lhs == rhs
                find_type_dependencies(
                    lhs,
                    source_id,
                    krate,
                    dependencies,
                    graph,
                    EdgeLabel::PredicateEqLhs,
                );
                match rhs {
                    Term::Type(t) => find_type_dependencies(
                        t,
                        source_id,
                        krate,
                        dependencies,
                        graph,
                        EdgeLabel::PredicateEqRhs,
                    ),
                    Term::Constant(_) => {} // Constant expr/value are stringly typed
                }
            }
        }
    }
}

fn find_generic_param_def_dependencies(
    param_def: &GenericParamDef,
    source_id: Id, // ID of the item defining this parameter
    krate: &Crate,
    dependencies: &mut HashSet<Id>,
    graph: &mut IdGraph,
) {
    match &param_def.kind {
        rustdoc_types::GenericParamDefKind::Lifetime { .. } => {}
        rustdoc_types::GenericParamDefKind::Type {
            bounds, default, ..
        } => {
            for bound in bounds {
                // source_id adds 'bound' to its generic param 'param_def.name'
                find_generic_bound_dependencies(
                    bound,
                    source_id,
                    krate,
                    dependencies,
                    graph,
                    EdgeLabel::ParamBound,
                );
            }
            if let Some(ty) = default {
                // source_id provides default type 'ty' for its generic param 'param_def.name'
                find_type_dependencies(
                    ty,
                    source_id,
                    krate,
                    dependencies,
                    graph,
                    EdgeLabel::ParamType, // Label indicating it's a default type for a param
                );
            }
        }
        rustdoc_types::GenericParamDefKind::Const { type_, .. } => {
            // source_id uses 'type_' for its const generic param 'param_def.name'
            // Ignore default string
            find_type_dependencies(
                type_,
                source_id,
                krate,
                dependencies,
                graph,
                EdgeLabel::ParamType, // Label indicating it's the type of a const param
            );
        }
    }
}

/// Selects items based on path filters and recursively includes their dependencies.
/// Builds the graph for *all* items in the crate, regardless of filtering.
fn select_items(
    krate: &Crate,
    user_paths: &[String],
    resolved_modules: &HashMap<Id, ResolvedModule>,
) -> Result<(HashSet<Id>, IdGraph)> {
    let mut selected_ids: HashSet<Id> = HashSet::new();
    let mut graph = IdGraph::new(); // Instantiate the graph

    // --- Build the full graph first ---
    info!("Building full dependency graph...");
    for id in krate.index.keys() {
        build_graph_for_item(*id, krate, &mut graph);
    }
    info!("Built full graph with {} edges.", graph.edges.len());

    // --- Now select items based on filters ---
    if user_paths.is_empty() {
        info!("No path filters specified, selecting all items.");
        selected_ids.extend(krate.index.keys().cloned());
        return Ok((selected_ids, graph));
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

    // Initial selection based on paths matching items in resolved modules
    // Iterate through resolved modules instead of krate.paths directly
    for resolved_mod in resolved_modules.values() {
        for item_id in &resolved_mod.items {
            // Get the summary for the item (if it exists) to check its path
            if let Some(item_summary) = krate.paths.get(item_id) {
                // We only care about items from the local crate for initial selection (crate_id 0)
                if item_summary.crate_id == 0 {
                    let mut qualified_item_path = item_summary.path.clone();
                    // Ensure the path starts with the crate name if it doesn't already
                    if !qualified_item_path.is_empty()
                        && qualified_item_path[0] != normalized_crate_name
                    {
                        qualified_item_path.insert(0, normalized_crate_name.clone());
                    }

                    for filter in &normalized_filters {
                        if path_matches(&qualified_item_path, filter) {
                            debug!(
                                "Path filter {:?} matched item {:?} ({:?}) via module {:?}",
                                filter, qualified_item_path, item_id, resolved_mod.id
                            );
                            selected_ids.insert(*item_id);
                            // No break here, an item might be reachable via multiple modules/paths
                        }
                    }
                }
            }
        }
    }

    if selected_ids.is_empty() {
        warn!(
            "No items matched the provided path filters: {:?}",
            user_paths
        );
        // Still return the full graph even if selection is empty
        return Ok((selected_ids, graph));
    }

    info!(
        "Initially selected {} items based on path filters and resolved modules.",
        selected_ids.len()
    );

    // --- Iterative dependency selection (using the pre-built graph) ---
    let mut queue: VecDeque<Id> = selected_ids.iter().cloned().collect();
    let mut visited_for_selection = HashSet::new(); // Keep track of visited nodes during selection traversal

    while let Some(id) = queue.pop_front() {
        if !visited_for_selection.insert(id) {
            continue; // Already processed this item for dependency selection
        }

        // Find dependencies using the graph's adjacency list
        if let Some(children) = graph.get_children(&id) {
            for (dep_id, _label) in children {
                // Check if dep_id exists in krate.index before adding
                if krate.index.contains_key(dep_id) && selected_ids.insert(*dep_id) {
                    debug!("Including dependency {:?} from item {:?}", dep_id, id);
                    queue.push_back(*dep_id);
                }
            }
        }
    }

    info!(
        "Selected {} items after including dependencies.",
        selected_ids.len()
    );

    Ok((selected_ids, graph))
}

/// Finds dependencies for a single item AND adds corresponding edges to the graph.
/// Returns a HashSet of dependent IDs found for this item.
fn build_graph_for_item(source_id: Id, krate: &Crate, graph: &mut IdGraph) -> HashSet<Id> {
    let mut item_deps: HashSet<Id> = HashSet::new();

    if let Some(item) = krate.index.get(&source_id) {
        // 1. Direct Links (value is Id)
        for (_link_text, link_id_val) in &item.links {
            // Check if link_id_val exists in krate.index before adding
            if krate.index.contains_key(link_id_val) {
                if item_deps.insert(*link_id_val) {
                    graph.add_edge(source_id, *link_id_val, EdgeLabel::IntraDocLink, krate);
                }
            }
        }

        // 2. Item Kind Specific Dependencies
        match &item.inner {
            ItemEnum::Module(m) => {
                for item_id in &m.items {
                    if krate.index.contains_key(item_id) {
                        // Note: This edge represents the *original* module structure
                        // Resolution of 'use' happens separately for documentation generation.
                        graph.add_edge(source_id, *item_id, EdgeLabel::Contains, krate);
                        // Do NOT add to item_deps here, Contains edge handles it.
                        // Dependency resolution follows the graph edges later.
                    }
                }
            }
            ItemEnum::Use(use_item) => {
                // Add edge from Use item to its target ID (if it exists)
                if let Some(target_id) = use_item.id {
                    if krate.index.contains_key(&target_id) {
                        if item_deps.insert(target_id) {
                            graph.add_edge(source_id, target_id, EdgeLabel::UseTarget, krate);
                        }
                    }
                }
            }
            ItemEnum::Struct(s) => {
                for impl_id in &s.impls {
                    if krate.index.contains_key(impl_id) {
                        if item_deps.insert(*impl_id) {
                            graph.add_edge(source_id, *impl_id, EdgeLabel::ImplFor, krate); // Struct -> Impl relation
                        }
                    }
                }
                find_generics_dependencies(
                    &s.generics,
                    source_id,
                    krate,
                    &mut item_deps,
                    graph,
                );
                match &s.kind {
                    rustdoc_types::StructKind::Plain { fields, .. } => {
                        for field_id in fields {
                            if krate.index.contains_key(field_id) {
                                if item_deps.insert(*field_id) {
                                    graph.add_edge(source_id, *field_id, EdgeLabel::StructField, krate);
                                }
                                // Also get dependencies of the field's type
                                if let Some(field_item) = krate.index.get(field_id) {
                                    if let ItemEnum::StructField(field_type) = &field_item.inner {
                                        find_type_dependencies(
                                            field_type,
                                            *field_id, // Source is the field ID
                                            krate,
                                            &mut item_deps,
                                            graph,
                                            EdgeLabel::FieldType,
                                        );
                                    }
                                }
                            }
                        }
                    }
                    rustdoc_types::StructKind::Tuple(fields) => {
                        for field_id_opt in fields {
                            if let Some(field_id) = field_id_opt {
                                if krate.index.contains_key(field_id) {
                                    if item_deps.insert(*field_id) {
                                        graph.add_edge(source_id, *field_id, EdgeLabel::StructField, krate);
                                    }
                                    if let Some(field_item) = krate.index.get(field_id) {
                                        if let ItemEnum::StructField(field_type) =
                                            &field_item.inner
                                        {
                                            find_type_dependencies(
                                                field_type,
                                                *field_id, // Source is the field ID
                                                krate,
                                                &mut item_deps,
                                                graph,
                                                EdgeLabel::FieldType,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                    rustdoc_types::StructKind::Unit => {}
                }
            }
            ItemEnum::Enum(e) => {
                for variant_id in &e.variants {
                    if krate.index.contains_key(variant_id) {
                        if item_deps.insert(*variant_id) {
                            graph.add_edge(source_id, *variant_id, EdgeLabel::EnumVariant, krate);
                        }
                    }
                }
                for impl_id in &e.impls {
                    if krate.index.contains_key(impl_id) {
                        if item_deps.insert(*impl_id) {
                            graph.add_edge(source_id, *impl_id, EdgeLabel::ImplFor, krate);
                        }
                    }
                }
                find_generics_dependencies(&e.generics, source_id, krate, &mut item_deps, graph);
            }
            ItemEnum::Variant(v) => {
                // Source is the enum containing this variant
                match &v.kind {
                    rustdoc_types::VariantKind::Plain => {}
                    rustdoc_types::VariantKind::Tuple(fields) => {
                        for field_id_opt in fields {
                            if let Some(field_id) = field_id_opt {
                                if krate.index.contains_key(field_id) {
                                    if item_deps.insert(*field_id) {
                                        graph.add_edge(source_id, *field_id, EdgeLabel::VariantField, krate);
                                    }
                                    if let Some(field_item) = krate.index.get(field_id) {
                                        if let ItemEnum::StructField(field_type) =
                                            &field_item.inner
                                        {
                                            find_type_dependencies(
                                                field_type,
                                                *field_id,
                                                krate,
                                                &mut item_deps,
                                                graph,
                                                EdgeLabel::FieldType,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                    rustdoc_types::VariantKind::Struct { fields, .. } => {
                        for field_id in fields {
                            if krate.index.contains_key(field_id) {
                                if item_deps.insert(*field_id) {
                                    graph.add_edge(source_id, *field_id, EdgeLabel::VariantField, krate);
                                }
                                if let Some(field_item) = krate.index.get(field_id) {
                                    if let ItemEnum::StructField(field_type) = &field_item.inner {
                                        find_type_dependencies(
                                            field_type,
                                            *field_id,
                                            krate,
                                            &mut item_deps,
                                            graph,
                                            EdgeLabel::FieldType,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
            ItemEnum::Function(f) => {
                find_generics_dependencies(&f.generics, source_id, krate, &mut item_deps, graph);
                for (_name, param_type) in &f.sig.inputs {
                    find_type_dependencies(
                        param_type,
                        source_id,
                        krate,
                        &mut item_deps,
                        graph,
                        EdgeLabel::SignatureInput,
                    );
                }
                if let Some(output) = &f.sig.output {
                    find_type_dependencies(
                        output,
                        source_id,
                        krate,
                        &mut item_deps,
                        graph,
                        EdgeLabel::SignatureOutput,
                    );
                }
            }
            ItemEnum::Trait(t) => {
                for item_id in &t.items {
                    if krate.index.contains_key(item_id) {
                        if item_deps.insert(*item_id) {
                            graph.add_edge(source_id, *item_id, EdgeLabel::TraitItem, krate);
                        }
                    }
                }
                find_generics_dependencies(&t.generics, source_id, krate, &mut item_deps, graph);
                for bound in &t.bounds {
                    find_generic_bound_dependencies(
                        bound,
                        source_id,
                        krate,
                        &mut item_deps,
                        graph,
                        EdgeLabel::SuperTrait,
                    );
                }
                for impl_id in &t.implementations {
                    if krate.index.contains_key(impl_id) {
                        if item_deps.insert(*impl_id) {
                            // Relation Trait -> Impl Block (Implementor)
                            graph.add_edge(source_id, *impl_id, EdgeLabel::Implements, krate);
                        }
                    }
                }
            }
            ItemEnum::Impl(imp) => {
                for item_id in &imp.items {
                    if krate.index.contains_key(item_id) {
                        if item_deps.insert(*item_id) {
                            graph.add_edge(source_id, *item_id, EdgeLabel::ImplItem, krate);
                        }
                    }
                }
                if let Some(trait_path) = &imp.trait_ {
                    if krate.index.contains_key(&trait_path.id) {
                        if item_deps.insert(trait_path.id) {
                            graph.add_edge(source_id, trait_path.id, EdgeLabel::Implements, krate);
                        }
                    }
                    if let Some(args) = trait_path.args.as_ref() {
                        find_generic_args_dependencies(
                            args,
                            source_id,
                            krate,
                            &mut item_deps,
                            graph,
                        );
                    }
                }
                find_type_dependencies(
                    &imp.for_,
                    source_id,
                    krate,
                    &mut item_deps,
                    graph,
                    EdgeLabel::ImplFor,
                );
                find_generics_dependencies(&imp.generics, source_id, krate, &mut item_deps, graph);
            }
            ItemEnum::TypeAlias(ta) => {
                find_type_dependencies(
                    &ta.type_,
                    source_id,
                    krate,
                    &mut item_deps,
                    graph,
                    EdgeLabel::AliasTo,
                );
                find_generics_dependencies(&ta.generics, source_id, krate, &mut item_deps, graph);
            }
            ItemEnum::Constant { type_, .. } => {
                find_type_dependencies(
                    type_,
                    source_id,
                    krate,
                    &mut item_deps,
                    graph,
                    EdgeLabel::ReferencesType,
                );
            }
            ItemEnum::Static(s) => {
                find_type_dependencies(
                    &s.type_,
                    source_id,
                    krate,
                    &mut item_deps,
                    graph,
                    EdgeLabel::ReferencesType,
                );
            }
            ItemEnum::AssocConst { type_, .. } => {
                find_type_dependencies(
                    type_,
                    source_id,
                    krate,
                    &mut item_deps,
                    graph,
                    EdgeLabel::ReferencesType,
                );
            }
            ItemEnum::AssocType {
                generics, bounds, type_, ..
            } => {
                find_generics_dependencies(generics, source_id, krate, &mut item_deps, graph);
                for bound in bounds {
                    find_generic_bound_dependencies(
                        bound,
                        source_id,
                        krate,
                        &mut item_deps,
                        graph,
                        EdgeLabel::TraitBound, // Bound on associated type
                    );
                }
                if let Some(def_type) = type_ {
                    find_type_dependencies(
                        def_type,
                        source_id,
                        krate,
                        &mut item_deps,
                        graph,
                        EdgeLabel::ReferencesType, // Default type for assoc type
                    );
                }
            }
            ItemEnum::Union(u) => {
                find_generics_dependencies(&u.generics, source_id, krate, &mut item_deps, graph);
                for field_id in &u.fields {
                    if krate.index.contains_key(field_id) {
                        if item_deps.insert(*field_id) {
                            graph.add_edge(source_id, *field_id, EdgeLabel::UnionField, krate);
                        }
                        if let Some(field_item) = krate.index.get(field_id) {
                            if let ItemEnum::StructField(field_type) = &field_item.inner {
                                find_type_dependencies(
                                    field_type,
                                    *field_id,
                                    krate,
                                    &mut item_deps,
                                    graph,
                                    EdgeLabel::FieldType,
                                );
                            }
                        }
                    }
                }
                for impl_id in &u.impls {
                    if krate.index.contains_key(impl_id) {
                        if item_deps.insert(*impl_id) {
                            graph.add_edge(source_id, *impl_id, EdgeLabel::ImplFor, krate);
                        }
                    }
                }
            }
            ItemEnum::TraitAlias(ta) => {
                find_generics_dependencies(&ta.generics, source_id, krate, &mut item_deps, graph);
                for bound in &ta.params {
                    find_generic_bound_dependencies(
                        bound,
                        source_id,
                        krate,
                        &mut item_deps,
                        graph,
                        EdgeLabel::AliasTo, // Bounds defining the alias
                    );
                }
            }
            ItemEnum::StructField(ty) => {
                // source_id is the StructField item itself
                find_type_dependencies(
                    ty,
                    source_id,
                    krate,
                    &mut item_deps,
                    graph,
                    EdgeLabel::FieldType,
                );
            }
            // Items with no obvious ID dependencies representable in the graph
            ItemEnum::ExternType
            | ItemEnum::Macro(_)
            | ItemEnum::ProcMacro(_)
            | ItemEnum::Primitive(_)
            | ItemEnum::ExternCrate { .. } => {}
        }
    }
    item_deps
}

// --- Formatting Helpers ---

/// Helper to check if an item has non-empty documentation.
fn has_docs(item: &Item) -> bool {
    item.docs.as_ref().map_or(false, |d| !d.trim().is_empty())
}

/// Adjusts the markdown header levels in a string.
/// Increases the level of each header (e.g., `#` -> `###`) based on the base level.
/// Caps the maximum level at 6 (`######`).
fn adjust_markdown_headers(markdown: &str, base_level: usize) -> String {
    let mut adjusted_markdown = String::new();
    for line in markdown.lines() {
        if line.starts_with('#') {
            let mut level = 0;
            for char in line.chars() {
                if char == '#' {
                    level += 1;
                } else {
                    break;
                }
            }
            // Ensure the first character after # is a space for valid markdown header
            if line.chars().nth(level) == Some(' ') {
                let new_level = std::cmp::min(base_level + level, 6); // Cap at level 6
                let adjusted_line = format!(
                    "{} {}",
                    "#".repeat(new_level),
                    line[level..].trim_start() // Get content after #s
                );
                adjusted_markdown.push_str(&adjusted_line);
            } else {
                // Not a valid header line, keep it as is
                adjusted_markdown.push_str(line);
            }
        } else {
            adjusted_markdown.push_str(line);
        }
        adjusted_markdown.push('\n'); // Add newline back
    }
    // Remove trailing newline if the original didn't have one (or if it was empty)
    if !markdown.ends_with('\n') && !markdown.is_empty() {
        adjusted_markdown.pop();
    }
    adjusted_markdown
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
        .unwrap_or_else(|| format!("{{id:{}}}", id.0)) // Fallback
}

/// Formats a Path struct, trying to use the canonical path for the ID.
fn format_path(path: &Path, krate: &Crate) -> String {
    // Use the canonical path if available, otherwise use the path string in the struct
    let base_path = format_id_path_canonical(&path.id, krate);

    let cleaned_base_path = clean_trait_path(&base_path); // Clean the base path
                                                          // Use as_ref() to get Option<&GenericArgs> from Option<Box<GenericArgs>>
    if let Some(args) = path.args.as_ref() {
        let args_str = format_generic_args(args, krate, true); // Angle brackets only
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
                            "{}{}{}{}{}",
                            name,
                            if assoc_args_str.is_empty() { "" } else { "<" },
                            assoc_args_str,
                            if assoc_args_str.is_empty() { "" } else { ">" },
                            format!(" = {}", format_term(term, krate)) // Put equality inside
                        )
                    }
                    rustdoc_types::AssocItemConstraint {
                        name,
                        args: assoc_args,
                        binding: rustdoc_types::AssocItemConstraintKind::Constraint(bounds),
                    } => {
                        let assoc_args_str = format_generic_args(assoc_args, krate, true);
                        format!(
                            "{}{}{}{}{}",
                            name,
                            if assoc_args_str.is_empty() { "" } else { "<" },
                            assoc_args_str,
                            if assoc_args_str.is_empty() { "" } else { ">" },
                            format!(
                                // Put constraint inside
                                ": {}",
                                bounds
                                    .iter()
                                    .map(|bnd| format_generic_bound(bnd, krate))
                                    .collect::<Vec<_>>()
                                    .join(" + ")
                            )
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
        GenericArg::Lifetime(lt) => format!("{}", lt), // Add quote
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
        GenericBound::Outlives(lifetime) => format!("{}", lifetime), // Add quote
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
        rustdoc_types::GenericParamDefKind::Lifetime { .. } => format!("{}", p.name), // Add quote
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
                        .map(|lt| format!("{}", lt)) // Add quotes
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
    let name = item.name.as_deref().unwrap_or_else(|| match &item.inner {
        ItemEnum::StructField(_) => "{unnamed_field}", // Special case for unnamed fields
        _ => "{unnamed}",
    });
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
                "{}{}{}{}{}",
                auto,
                unsafe_kw,
                "trait ",
                name,
                format_generics_params_only(&t.generics.params, krate)
            )
        }
        ItemEnum::Function(f) => {
            // Simplified version for the header: no attrs, no where clause
            let mut code = String::new();
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
        ItemEnum::StructField(_) => name.to_string(),                     // Field name only for header
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
    write!(code, "pub struct {}", name).unwrap();
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
                write!(code, "\n").unwrap();
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
                                Some(format!("pub {}", format_type(field_type, krate)))
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
    write!(code, "pub enum {}", name).unwrap();
    let generics_str = format_generics_full(&e.generics, krate);
    let where_is_multiline = generics_str.contains("where\n");
    write!(code, "{}", generics_str).unwrap();

    // Check if generics caused a newline before deciding where to put opening brace
    if where_is_multiline {
        write!(code, " {{").unwrap();
    } else {
        write!(code, " {{").unwrap();
    }

    if !e.variants.is_empty() {
        write!(code, "\n").unwrap();
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
    if !e.variants.is_empty() && !code.ends_with('\n') {
        writeln!(code).unwrap();
    }
    write!(code, "}}").unwrap();
    code
}

/// Generates the full trait declaration code block.
fn generate_trait_code_block(item: &Item, t: &Trait, krate: &Crate) -> String {
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

struct DocPrinter<'a> {
    krate: &'a Crate,
    manifest: &'a CrateManifestData, // Add field for manifest data
    readme_content: Option<String>,   // Add field for README content
    selected_ids: &'a HashSet<Id>,
    resolved_modules: &'a HashMap<Id, ResolvedModule>, // Add resolved module index
    graph: &'a IdGraph,                                // Add graph reference
    include_other: bool,
    template_mode: bool, // New flag for template mode
    printed_ids: HashSet<Id>,
    output: String,
    module_tree: ModuleTree, // Add module tree
    // Tracks the current path in the document structure (e.g., [1, 2, 1] -> 1.2.1)
    doc_path: Vec<usize>,
}

impl<'a> DocPrinter<'a> {
    fn new(
        krate: &'a Crate,
        manifest: &'a CrateManifestData, // Accept manifest data
        readme_content: Option<String>,   // Accept README content
        selected_ids: &'a HashSet<Id>,
        resolved_modules: &'a HashMap<Id, ResolvedModule>, // Accept resolved modules
        graph: &'a IdGraph,                                // Add graph parameter
        include_other: bool,
        template_mode: bool, // Add template mode flag
    ) -> Self {
        let module_tree = Self::build_module_tree(krate);
        DocPrinter {
            krate,
            manifest,         // Store manifest data
            readme_content,   // Store README content
            selected_ids,
            resolved_modules, // Store resolved modules
            graph,            // Store graph reference
            include_other,
            template_mode, // Store template mode flag
            printed_ids: HashSet::new(),
            output: String::new(),
            module_tree,                // Initialize module tree
            doc_path: Vec::new(), // Initialize empty document path
        }
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
    fn increment_current_level(&mut self) {
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
            .map(|item| DocPrinter::infer_item_kind(item)) // Use associated function syntax
            .or_else(|| self.krate.paths.get(id).map(|summary| summary.kind))
    }

    // Fallback for inferring ItemKind if not found in paths map (should be equivalent to index anyway)
    fn infer_item_kind(item: &Item) -> ItemKind {
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
    fn print_item_details(&mut self, id: &Id) -> bool {
        if !self.selected_ids.contains(id) || !self.printed_ids.insert(*id) {
            return false; // Skip unselected or already printed items
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
        let declaration = generate_item_declaration(item, self.krate);

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

        let has_stripped = matches!(
            &item.inner,
            ItemEnum::Struct(Struct {
                kind: StructKind::Plain { has_stripped_fields: true, .. },
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

        true
    }

    /// Checks if any selected field within a struct has documentation or if template mode is on.
    fn has_documented_fields(&self, s: &Struct) -> bool {
        let field_ids = match &s.kind {
            StructKind::Plain { fields, .. } => fields.clone(),
            StructKind::Tuple(fields) => fields.iter().filter_map(|opt_id| *opt_id).collect(),
            StructKind::Unit => vec![],
        };
        field_ids.iter().any(|field_id| {
            self.selected_ids.contains(field_id)
                && self.krate.index.get(field_id).map_or(false, |item| {
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
                    if !self.printed_ids.contains(field_id) {
                        has_printable_field = true;
                    }
                } else {
                    // Mark non-printable field as printed immediately
                    self.printed_ids.insert(*field_id);
                }
            } else {
                // If item doesn't exist in index but ID was present, mark it printed to avoid issues
                self.printed_ids.insert(*field_id);
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
                self.increment_current_level();
            }
        }
        self.pop_level(); // Pop the field item level

        self.increment_current_level();
    }

    /// Prints the details for a single struct field, only if it has printable documentation.
    /// Returns true if the field was printed, false otherwise.
    fn print_field_details(&mut self, field_id: &Id) -> bool {
        if !self.selected_ids.contains(field_id) || self.printed_ids.contains(field_id) {
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

            // Mark as printed *before* printing details
            self.printed_ids.insert(*field_id);

            if let ItemEnum::StructField(_field_type) = &item.inner {
                let name = item.name.as_deref().unwrap_or("_");
                let field_header_level = self.get_current_header_level();
                let header_prefix = self.get_header_prefix();

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
        self.printed_ids.insert(*field_id);
        false
    }

    /// Prints the details for a single enum variant field, only if it has printable documentation.
    /// Returns true if the field was printed, false otherwise.
    fn print_variant_field_details(&mut self, field_id: &Id) -> bool {
        if !self.selected_ids.contains(field_id) || self.printed_ids.contains(field_id) {
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

            // Mark as printed *before* printing details
            self.printed_ids.insert(*field_id);

            // Increment level counter for this field item
            self.increment_current_level();

            if let ItemEnum::StructField(_field_type) = &item.inner {
                let name = item.name.as_deref().unwrap_or("_"); // Might be _ for tuple fields
                let field_header_level = self.get_current_header_level();
                let header_prefix = self.get_header_prefix();

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

                // Type (optional)
                // writeln!(self.output, "_Type: `{}`_\n", format_type(field_type, self.krate)).unwrap();
                return true; // Field was printed
            }
        }
        // Mark as printed even if item lookup failed
        self.printed_ids.insert(*field_id);
        false
    }

    /// Checks if any selected variant or its fields have printable documentation.
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
                                if (self.template_mode && f_item.docs.is_some())
                                    || has_docs(f_item)
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
                            let field_has_printable_docs = self
                                .krate
                                .index
                                .get(&field_id)
                                .map_or(false, |f_item| {
                                    (self.template_mode && f_item.docs.is_some())
                                        || has_docs(f_item)
                                });
                            if field_has_printable_docs {
                                if !self.printed_ids.contains(&field_id) {
                                    variant_has_printable_field = true;
                                }
                            } else {
                                self.printed_ids.insert(field_id); // Mark non-printable field printed
                            }
                        } else {
                            // Mark unselected field id printed if present
                            self.printed_ids.insert(field_id);
                        }
                    }
                }

                if variant_has_printable_docs || variant_has_printable_field {
                    // Check if the variant itself is already printed to avoid double counting
                    if !self.printed_ids.contains(variant_id) {
                        has_printable_variant_or_field = true;
                    }
                } else {
                    // Mark non-printable variant (with no printable fields) as printed immediately
                    self.printed_ids.insert(*variant_id);
                }
            } else {
                // If item doesn't exist in index but ID was present, mark it printed
                self.printed_ids.insert(*variant_id);
            }
        }

        // Only print the "Variants" section if there's a printable variant/field or stripped variants exist
        if !has_printable_variant_or_field && !e.has_stripped_variants {
            return;
        }

        // Increment level counter for this section
        self.increment_current_level();
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
        self.pop_level(); // Pop the variant item level

        if e.has_stripped_variants {
            // Add newline before stripped message only if variants were printed
            if printed_any_variant {
                writeln!(self.output).unwrap();
            }
            writeln!(self.output, "_[Private variants hidden]_").unwrap();
        }
    }

    /// Prints the details for a single enum variant. Includes variant docs and docs for its fields if present.
    /// Returns true if the variant was printed (because it or its fields had printable docs), false otherwise.
    fn print_variant_details(&mut self, variant_id: &Id) -> bool {
        if !self.selected_ids.contains(variant_id) || self.printed_ids.contains(variant_id) {
            return false; // Skip unselected or already printed
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
                        let field_has_printable_docs = self
                            .krate
                            .index
                            .get(field_id)
                            .map_or(false, |f_item| {
                                (self.template_mode && f_item.docs.is_some()) || has_docs(f_item)
                            });
                        if field_has_printable_docs && !self.printed_ids.contains(field_id) {
                            printable_fields.push(*field_id);
                        } else {
                            // Mark unselected or non-printable field printed
                            self.printed_ids.insert(*field_id);
                        }
                    } else {
                        // Mark unselected field printed
                        self.printed_ids.insert(*field_id);
                    }
                }

                // Only print the variant if it has printable docs OR it has printable fields to print
                if !variant_has_printable_docs && printable_fields.is_empty() {
                    // Mark variant as printed if skipped
                    self.printed_ids.insert(*variant_id);
                    return false;
                }

                // Mark as printed *before* printing details
                self.printed_ids.insert(*variant_id);

                // Increment level counter for this variant item
                self.increment_current_level();

                let signature = format_variant_signature(item, variant_data, self.krate);
                let variant_header_level = self.get_current_header_level();
                let header_prefix = self.get_header_prefix();

                // Header: e.g., ##### 1.1.1.1: `VariantSignature`
                writeln!(
                    self.output,
                    "{} {} `{}`\n", // Add newline after header
                    "#".repeat(variant_header_level),
                    header_prefix,
                    signature
                )
                .unwrap();

                // Print Variant Docs (using helper)
                self.print_docs(item);

                // Print documented fields (if any)
                if !printable_fields.is_empty() || stripped {
                    // Push a new level for the fields section
                    self.push_level();
                    self.increment_current_level(); // Increment for the "Fields" header itself
                    let field_section_level = self.get_current_header_level();
                    let fields_header_prefix = self.get_header_prefix();
                    writeln!(
                        self.output,
                        "{} {} Fields\n", // Add newline after header
                        "#".repeat(field_section_level),
                        fields_header_prefix
                    )
                    .unwrap();

                    // Push another level for the field items
                    self.push_level();
                    for field_id in printable_fields {
                        if self.print_variant_field_details(&field_id) {
                            printed_any_field = true;
                        }
                    }
                    self.pop_level(); // Pop field item level
                    self.pop_level(); // Pop fields section level

                    if stripped {
                        if printed_any_field {
                            writeln!(self.output).unwrap(); // Add newline before stripped message
                        }
                        writeln!(self.output, "_[Private fields hidden]_").unwrap();
                    }
                }

                return true; // Variant (or its fields) was printed
            }
        }
        // Mark as printed even if item lookup failed
        self.printed_ids.insert(*variant_id);
        false
    }

    /// Checks if any selected associated item has printable documentation.
    fn has_printable_associated_items(&self, t: &Trait) -> bool {
        t.items.iter().any(|item_id| {
            self.selected_ids.contains(item_id)
                && self
                    .krate
                    .index
                    .get(item_id)
                    .map_or(false, |assoc_item| {
                        (self.template_mode && assoc_item.docs.is_some()) || has_docs(assoc_item)
                    })
        })
    }

    /// Prints the "Associated Items" section for a trait, only if any selected item has printable docs.
    /// Also marks *all* selected associated items as printed.
    fn print_trait_associated_items(&mut self, _item: &Item, t: &Trait) {
        let mut assoc_consts = vec![];
        let mut assoc_types = vec![];
        let mut assoc_fns = vec![];
        let mut any_assoc_has_printable_docs = false;

        // Filter and categorize selected associated items, and mark *all* selected ones as printed.
        for item_id in &t.items {
            if !self.selected_ids.contains(item_id) {
                continue;
            }
            // Mark the item as printed now, regardless of docs, to prevent it from going to "Other"
            self.printed_ids.insert(*item_id);

            if let Some(assoc_item) = self.krate.index.get(item_id) {
                let item_has_printable_docs =
                    (self.template_mode && assoc_item.docs.is_some()) || has_docs(assoc_item);
                if item_has_printable_docs {
                    any_assoc_has_printable_docs = true; // Mark if any item has printable docs
                }
                match &assoc_item.inner {
                    ItemEnum::AssocConst { .. } => {
                        assoc_consts.push((item_id, item_has_printable_docs))
                    }
                    ItemEnum::AssocType { .. } => {
                        assoc_types.push((item_id, item_has_printable_docs))
                    }
                    ItemEnum::Function(_) => assoc_fns.push((item_id, item_has_printable_docs)),
                    _ => {} // Ignore others
                }
            }
        }

        // If no selected associated item has printable documentation, skip printing the entire section
        if !any_assoc_has_printable_docs {
            return;
        }

        // Increment level counter for this section
        self.increment_current_level();
        let assoc_items_header_level = self.get_current_header_level();
        let header_prefix = self.get_header_prefix();
        writeln!(
            self.output,
            "{} {} Associated Items\n", // Add newline after header
            "#".repeat(assoc_items_header_level),
            header_prefix
        )
        .unwrap();

        // Push a new level for the subsections (Consts, Types, Fns)
        self.push_level();

        // Print in order: consts, types, fns, only if they have printable items
        if assoc_consts.iter().any(|(_, has_docs)| *has_docs) {
            self.increment_current_level(); // Increment for this subsection
            let sub_section_level = self.get_current_header_level();
            let sub_header_prefix = self.get_header_prefix();
            writeln!(
                self.output,
                "{} {} Associated Constants\n",
                "#".repeat(sub_section_level),
                sub_header_prefix
            )
            .unwrap();
            self.push_level(); // Push level for items within subsection
            for (id, has_item_docs) in assoc_consts {
                if has_item_docs {
                    self.print_associated_item_summary(id);
                }
            }
            self.pop_level(); // Pop item level
        }
        if assoc_types.iter().any(|(_, has_docs)| *has_docs) {
            self.increment_current_level(); // Increment for this subsection
            let sub_section_level = self.get_current_header_level();
            let sub_header_prefix = self.get_header_prefix();
            writeln!(
                self.output,
                "{} {} Associated Types\n",
                "#".repeat(sub_section_level),
                sub_header_prefix
            )
            .unwrap();
            self.push_level(); // Push level for items within subsection
            for (id, has_item_docs) in assoc_types {
                if has_item_docs {
                    self.print_associated_item_summary(id);
                }
            }
            self.pop_level(); // Pop item level
        }
        if assoc_fns.iter().any(|(_, has_docs)| *has_docs) {
            self.increment_current_level(); // Increment for this subsection
            let sub_section_level = self.get_current_header_level();
            let sub_header_prefix = self.get_header_prefix();
            writeln!(
                self.output,
                "{} {} Associated Functions\n",
                "#".repeat(sub_section_level),
                sub_header_prefix
            )
            .unwrap();
            self.push_level(); // Push level for items within subsection
            for (id, has_item_docs) in assoc_fns {
                if has_item_docs {
                    self.print_associated_item_summary(id);
                }
            }
            self.pop_level(); // Pop item level
        }

        self.pop_level(); // Pop the subsection level
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
                    || !matches!(f.header.abi, Abi::Rust);
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
            // Increment level counter for this associated item
            self.increment_current_level();

            // Generate summary first (handles template mode internally)
            if let Some(summary) = self.generate_associated_item_summary(assoc_item_id) {
                let declaration = generate_item_declaration(item, self.krate);
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
            }
            // If generate_associated_item_summary returns None, the item wasn't selected,
            // so we don't print anything, and the level increment effectively skips it.
        }
    }

    /// Prints Inherent and Trait Implementations *for* an item (Struct, Enum, Union, Primitive).
    /// Handles template mode for impl docs. Handles blanket impls specially.
    fn print_item_implementations(&mut self, impl_ids: &[Id], target_item: &Item) {
        let target_name = target_item.name.as_deref().unwrap_or_else(|| {
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
            for impl_item in inherent_impls {
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
            let trait_impl_header_level = self.get_current_header_level();
            let header_prefix = self.get_header_prefix();
            writeln!(
                self.output,
                "{} {} Trait Implementations for `{}`\n", // Keep this header, Add newline after header
                "#".repeat(trait_impl_header_level),
                header_prefix,
                target_name // Use the name we got earlier
            )
            .unwrap();

            let mut auto_trait_impls: Vec<(&Item, String)> = Vec::new();
            let mut blanket_impl_data: Vec<(&Item, &Impl, String)> = Vec::new();
            let mut simple_impl_data: Vec<(&Item, &Impl, String)> = Vec::new();
            let mut generic_or_complex_impls = Vec::new(); // Store items for later processing

            // First pass: Categorize trait impls
            for impl_item in &trait_impls {
                if let ItemEnum::Impl(imp) = &impl_item.inner {
                    if let Some(trait_path) = &imp.trait_ {
                        let full_path_str = format_id_path_canonical(&trait_path.id, self.krate);
                        let cleaned_path = clean_trait_path(&format_path(trait_path, self.krate));

                        // Check for Auto Traits (synthetic only)
                        if imp.is_synthetic && AUTO_TRAITS.contains(&cleaned_path.as_str()) {
                            auto_trait_impls.push((impl_item, cleaned_path));
                            continue; // Handled as auto trait
                        }

                        // Check for Blanket Impls (non-auto-trait)
                        if imp.blanket_impl.is_some() {
                            blanket_impl_data.push((impl_item, imp, cleaned_path));
                            continue; // Handled as blanket impl
                        }

                        // Check if it's core/alloc or selected - treat as simple if so
                        let is_core_or_alloc = full_path_str.starts_with("core::")
                            || full_path_str.starts_with("alloc::");
                        let is_selected_trait = self.selected_ids.contains(&trait_path.id);

                        if is_core_or_alloc || is_selected_trait {
                            simple_impl_data.push((impl_item, imp, cleaned_path));
                            continue; // Handle as simple list item
                        }

                        // Check for Simple Impls (non-blanket, non-auto, non-core/selected)
                        let is_simple_structure = imp.generics.params.is_empty()
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

                        if is_simple_structure {
                            simple_impl_data.push((impl_item, imp, cleaned_path));
                        } else {
                            generic_or_complex_impls.push(*impl_item);
                        }
                    }
                }
            }

            // Sort each category
            auto_trait_impls.sort_by(|a, b| a.1.cmp(&b.1));
            blanket_impl_data.sort_by(|a, b| a.2.cmp(&b.2));
            simple_impl_data.sort_by(|a, b| a.2.cmp(&b.2));
            generic_or_complex_impls.sort_by_key(|item| {
                if let ItemEnum::Impl(imp) = &item.inner {
                    imp.trait_.as_ref().map(|p| format_path(p, self.krate))
                } else {
                    None // Should not happen
                }
            });

            // Push a new level for the items within the Trait Impls section
            self.push_level();

            // Print Auto Traits first (simple list) AND mark them printed
            if !auto_trait_impls.is_empty() {
                for (impl_item, cleaned_path) in &auto_trait_impls {
                    self.increment_current_level(); // Increment for this list item
                    let template_marker = if self.template_mode && impl_item.docs.is_some() {
                        format!("\n\n{}", self.get_template_marker())
                    } else {
                        "".to_string()
                    };
                    writeln!(self.output, "- `{}`{}", cleaned_path, template_marker).unwrap();
                    self.printed_ids.insert(impl_item.id);
                    // Also mark associated items (usually none for auto traits)
                    if let ItemEnum::Impl(imp) = &impl_item.inner {
                        for assoc_item_id in &imp.items {
                            if self.selected_ids.contains(assoc_item_id) {
                                self.printed_ids.insert(*assoc_item_id);
                            }
                        }
                    }
                }
                writeln!(self.output).unwrap(); // Add blank line after list
            }

            // Print Simple non-blanket/non-auto impls next (simple list) AND mark them printed
            if !simple_impl_data.is_empty() {
                for (impl_item, imp, cleaned_path) in &simple_impl_data {
                    self.increment_current_level(); // Increment for this list item
                    let template_marker = if self.template_mode && impl_item.docs.is_some() {
                        format!("\n\n{}", self.get_template_marker())
                    } else {
                        "".to_string()
                    };
                    writeln!(self.output, "- `{}`{}", cleaned_path, template_marker).unwrap();
                    self.printed_ids.insert(impl_item.id);
                    // Mark associated items
                    for assoc_item_id in &imp.items {
                        if self.selected_ids.contains(assoc_item_id) {
                            self.printed_ids.insert(*assoc_item_id);
                        }
                    }
                }
                writeln!(self.output).unwrap(); // Add blank line after list
            }

            // Print Blanket Impls next (list + optional where clause) AND mark them printed
            if !blanket_impl_data.is_empty() {
                for (impl_item, imp, cleaned_path) in &blanket_impl_data {
                    self.increment_current_level(); // Increment for this list item
                    self.printed_ids.insert(impl_item.id);
                    let template_marker = if self.template_mode && impl_item.docs.is_some() {
                        format!("\n\n{}", self.get_template_marker())
                    } else {
                        "".to_string()
                    };
                    if !imp.generics.where_predicates.is_empty() {
                        let where_clause = format_generics_where_only(
                            &imp.generics.where_predicates,
                            self.krate,
                        );

                        if where_clause.lines().count() == 1 {
                            writeln!(
                                self.output,
                                "- `{cleaned_path}` (`{where_clause}`){}",
                                template_marker
                            )
                            .unwrap();
                        } else {
                            writeln!(self.output, "- `{cleaned_path}`{}", template_marker)
                                .unwrap();
                            // Format and indent the where clause
                            let code_block = format!("```rust\n{}\n```", where_clause);
                            let indented_block = indent_string(&code_block, 4);
                            writeln!(self.output, "\n{}\n", indented_block).unwrap();
                        }
                    } else {
                        writeln!(self.output, "- `{cleaned_path}`{}", template_marker).unwrap();
                    }
                    // Mark associated items
                    for assoc_item_id in &imp.items {
                        if self.selected_ids.contains(assoc_item_id) {
                            self.printed_ids.insert(*assoc_item_id);
                        }
                    }
                }
                // No extra blank line needed here, handled by item printing or next section
            }

            // Print generic/complex non-blanket/non-auto impls last (full blocks)
            for impl_item in generic_or_complex_impls {
                // Skip if already printed (shouldn't happen with current logic, but safe)
                if self.printed_ids.contains(&impl_item.id) {
                    continue;
                }
                if let ItemEnum::Impl(imp) = &impl_item.inner {
                    // generate_impl_trait_block marks printed IDs internally
                    if let Some(impl_block_str) = self.generate_impl_trait_block(impl_item, imp) {
                        // Get cleaned path for list item
                        let cleaned_path = imp
                            .trait_
                            .as_ref()
                            .map(|tp| clean_trait_path(&format_path(tp, self.krate)))
                            .unwrap_or_else(|| "{InherentImpl}".to_string()); // Should not happen here
                        // Increment level counter for this list item *before* getting template marker
                        self.increment_current_level();
                        let template_marker = if self.template_mode && impl_item.docs.is_some() {
                            format!("\n\n{}", self.get_template_marker())
                        } else {
                            "".to_string()
                        };
                        writeln!(self.output, "- `{}`{}", cleaned_path, template_marker).unwrap();
                        writeln!(self.output).unwrap(); // Blank line after list item

                        let full_code_block = format!("```rust\n{}\n```", impl_block_str);
                        let indented_block = indent_string(&full_code_block, 4);
                        writeln!(self.output, "{}\n", indented_block).unwrap();
                    }
                }
            }
            self.pop_level(); // Pop the item level for trait impls

            self.increment_current_level();
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
            // Increment level counter for this section
            self.increment_current_level();
            let implementors_section_level = self.get_current_header_level();
            let header_prefix = self.get_header_prefix();
            writeln!(
                self.output,
                "{} {} Implementors\n", // Add newline after header
                "#".repeat(implementors_section_level),
                header_prefix
            )
            .unwrap();

            // Push a new level for the impl items
            self.push_level();
            for impl_item in implementors {
                if let ItemEnum::Impl(imp) = &impl_item.inner {
                    // Increment level counter for this implementor header
                    self.increment_current_level();
                    // Only print the header for the implementation here
                    let impl_header = self.format_impl_decl(imp);
                    let impl_header_level = self.get_current_header_level();
                    let impl_prefix = self.get_header_prefix();
                    // Print the impl block header (e.g. ##### 1.1.1.1: `impl ...`)
                    writeln!(
                        self.output,
                        "{} {} `{}`\n", // Add newline after header
                        "#".repeat(impl_header_level),
                        impl_prefix,
                        impl_header.trim() // Trim potential trailing space
                    )
                    .unwrap();

                    // Print docs for the impl block itself (using helper)
                    // Create a temporary DocPrinter to isolate output
                    let mut temp_printer = self.clone_with_new_output();
                    // Copy current doc path to temp printer for correct template marker generation
                    temp_printer.doc_path = self.doc_path.clone();
                    temp_printer.print_docs(impl_item);
                    write!(self.output, "{}", temp_printer.output).unwrap();

                    // We don't print the associated items here, just list the implementor
                }
            }
            self.pop_level(); // Pop the impl item level
        }
    }

    /// Helper to format a single-line impl block or trait impl header
    #[allow(dead_code)] // Keep for potential future use
    fn format_impl_header(&self, imp: &Impl) -> String {
        let mut impl_header = String::new();
        if imp.is_unsafe {
            write!(impl_header, "unsafe ").unwrap();
        }

        if let Some(trait_path) = &imp.trait_ {
            write!(impl_header, "impl {}", format_path(trait_path, self.krate)).unwrap();
        } else {
            write!(impl_header, "impl {}", format_type(&imp.for_, self.krate)).unwrap();
        }

        impl_header
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
    /// Returns None if the impl block was already printed or shouldn't be printed (e.g., simple impl handled elsewhere).
    fn generate_impl_trait_block(&mut self, impl_item: &Item, imp: &Impl) -> Option<String> {
        // Mark as printed *now*
        if !self.printed_ids.insert(impl_item.id) {
            return None;
        }

        let mut code = String::new();
        let impl_header = self.format_impl_decl(imp); // Get the potentially multi-line header

        writeln!(code, "{} {{", impl_header).unwrap(); // Add opening brace

        // Process associated items
        let mut assoc_items_content = String::new();
        let mut has_items = false;

        // Push level for associated items within the code block (doesn't affect headers)
        self.push_level();
        for assoc_item_id in &imp.items {
            if !self.selected_ids.contains(assoc_item_id) {
                continue; // Skip unselected items
            }
            if let Some(assoc_item) = self.krate.index.get(assoc_item_id) {
                has_items = true; // Mark that we found at least one selected item
                self.increment_current_level(); // Increment for this associated item
                match &assoc_item.inner {
                    ItemEnum::AssocConst { type_, value, .. } => {
                        let assoc_item_docs = if self.template_mode && assoc_item.docs.is_some() {
                            format!("\n    // {}", self.get_template_marker())
                        } else {
                            "".to_string()
                        };
                        write!(
                            assoc_items_content,
                            "    const {}: {}",
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
                        let assoc_item_docs = if self.template_mode && assoc_item.docs.is_some() {
                            format!("\n    // {}", self.get_template_marker())
                        } else {
                            "".to_string()
                        };
                        write!(
                            assoc_items_content,
                            "    type {}",
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
                    ItemEnum::Function(f) => {
                        // Generate the full function block if selected
                        let func_block = generate_function_code_block(assoc_item, f, self.krate);
                        // Indent the function block
                        writeln!(assoc_items_content, "{}", indent_string(&func_block, 4))
                            .unwrap();
                        // Add template marker after the function block if applicable
                        if self.template_mode && assoc_item.docs.is_some() {
                            writeln!(
                                assoc_items_content,
                                "    // {}",
                                self.get_template_marker()
                            )
                            .unwrap();
                        }
                    }
                    _ => {} // Ignore others
                }
                // Mark associated item as printed *after* processing it for the code block
                self.printed_ids.insert(*assoc_item_id);
            }
        }
        self.pop_level(); // Pop associated item level

        if has_items {
            // Add a newline before the first item if the header was multi-line
            if impl_header.contains('\n') {
                writeln!(code).unwrap();
            }
            write!(code, "{}", assoc_items_content).unwrap();
            // Add a newline before the closing brace if needed
            if !code.ends_with('\n') {
                writeln!(code).unwrap();
            }
        } else if impl_header.contains('\n') {
            // If no items but multiline header, add newline before closing brace
            writeln!(code).unwrap();
        }

        write!(code, "}}").unwrap(); // Add closing brace
        Some(code)
    }

    /// Prints the details of a specific impl block (header, associated items).
    /// Handles template mode for the impl block's docs.
    fn print_impl_block_details(&mut self, impl_item: &Item, imp: &Impl) {
        // Mark as printed *now* before printing details
        if !self.printed_ids.insert(impl_item.id) {
            return;
        }

        // Increment level counter for this impl block
        self.increment_current_level();
        let impl_header_level = self.get_current_header_level();
        let header_prefix = self.get_header_prefix();
        let impl_header = self.format_impl_decl(imp);

        // Print the impl block header (e.g. #### 1.1.1: `impl ...`)
        writeln!(
            self.output,
            "{} {} `{}`\n", // Add newline after header
            "#".repeat(impl_header_level),
            header_prefix,
            impl_header.trim() // Trim potential trailing space if no where clause added
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
                self.printed_ids.insert(*id); // Mark printed here too
            }
        }
        if !assoc_types.is_empty() {
            for id in assoc_types {
                self.print_associated_item_summary(id);
                self.printed_ids.insert(*id); // Mark printed here too
            }
        }
        if !assoc_fns.is_empty() {
            for id in assoc_fns {
                self.print_associated_item_summary(id);
                self.printed_ids.insert(*id); // Mark printed here too
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
            // Print item details using the detail printer (increments/decrements its level)
            if self.print_item_details(id) {
                self.increment_current_level();
            }
        }
        self.pop_level(); // Pop the item level for this section

        true
    }

    /// Prints the non-module contents of a specific module (identified by its ID).
    /// Uses the `resolved_modules` index to get the list of items.
    fn print_module_contents(&mut self, module_id: &Id) {
        // Get the resolved items for this module
        if let Some(resolved_module) = self.resolved_modules.get(module_id) {
            // Group selected items by kind within this module's *resolved* items
            let mut items_by_kind: HashMap<ItemKind, Vec<Id>> = HashMap::new();
            for id in &resolved_module.items {
                // Check if the item is selected and hasn't been printed yet
                if !self.selected_ids.contains(id) || self.printed_ids.contains(id) {
                    continue;
                }
                // Get the item kind from the main krate index
                if let Some(kind) = self.get_item_kind(id) {
                    // Skip kinds handled implicitly or that shouldn't be grouped here
                    // Also skip Module kind explicitly, as they are handled by the main loop.
                    match kind {
                        ItemKind::Impl
                        | ItemKind::Variant
                        | ItemKind::StructField
                        | ItemKind::AssocConst
                        | ItemKind::AssocType
                        | ItemKind::Use
                        | ItemKind::Module => continue, // Skip modules, impls, variants etc. here
                        _ => {}
                    }
                    items_by_kind.entry(kind).or_default().push(*id);
                }
            }

            // Sort items by name within each kind
            for ids in items_by_kind.values_mut() {
                ids.sort_by_key(|id| {
                    self.krate.index.get(id).and_then(|item| item.name.clone())
                });
            }

            // Defined printing order for sections WITHIN a module
            let print_order = [
                (ItemKind::Macro, "Macros"), // Includes ProcMacros displayed as Macro
                (ItemKind::ProcAttribute, "Attribute Macros"),
                (ItemKind::ProcDerive, "Derive Macros"),
                // Submodules are NOT printed here anymore
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
                (ItemKind::Primitive, "Primitives"), // Unlikely unless re-exported
            ];

            // Push a level for the sections within the module
            //self.push_level();

            for (kind, header_name) in print_order {
                if let Some(ids) = items_by_kind.get(&kind) {
                    if ids.is_empty() {
                        continue;
                    } // Skip empty sections

                    // print_items_of_kind will increment the level for the section header
                    // and push/pop a level for the items inside it
                    if self.print_items_of_kind(ids, kind, header_name) {
                        self.increment_current_level();
                    }
                }
            }
            //self.pop_level(); // Pop the section level
        } else {
            warn!(
                "Could not find resolved module data for ID: {:?}",
                module_id
            );
        }
    }

    /// Prints graph context for an unprinted item.
    fn print_graph_context(&mut self, id: &Id) {
        let incoming_edges: Vec<&Edge> = self.graph.find_incoming_edges(id);
        if !incoming_edges.is_empty() {
            writeln!(self.output, "_Referenced by:_").unwrap();
            // Sort edges for consistent output
            let mut sorted_edges = incoming_edges;
            sorted_edges.sort_by_key(|edge| {
                (
                    format_id_path_canonical(&edge.source, self.krate),
                    format!("{:?}", edge.label),
                )
            });

            // Push level for this list (for template markers)
            self.push_level();
            for edge in sorted_edges {
                self.increment_current_level(); // Increment for this list item
                let source_path = format_id_path_canonical(&edge.source, self.krate);
                let template_marker = if self.template_mode
                    && self
                        .krate
                        .index
                        .get(&edge.source)
                        .map_or(false, |i| i.docs.is_some())
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
            writeln!(self.output, "_Item has no known incoming references in the graph._\n")
                .unwrap();
        }
    }

    /// Creates a clone of the printer with an empty output buffer.
    fn clone_with_new_output(&self) -> Self {
        DocPrinter {
            krate: self.krate,
            manifest: self.manifest, // Pass manifest reference
            readme_content: self.readme_content.clone(), // Clone README content
            selected_ids: self.selected_ids,
            resolved_modules: self.resolved_modules,
            graph: self.graph,
            include_other: self.include_other,
            template_mode: self.template_mode,
            printed_ids: self.printed_ids.clone(), // Clone printed IDs too? Or share? Let's clone for isolation.
            output: String::new(),                 // New empty output
            module_tree: self.module_tree.clone(), // Clone module tree as well
            doc_path: self.doc_path.clone(),       // Clone doc path
        }
    }

    /// Recursive function to print modules and their contents depth-first.
    fn print_module_recursive(&mut self, module_id: Id) {
        // Skip if not selected or already printed (except root module)
        if module_id != self.krate.root
            && (!self.selected_ids.contains(&module_id) || self.printed_ids.contains(&module_id))
        {
            return;
        }

        if let Some(item) = self.krate.index.get(&module_id) {
            let module_header_level = self.get_current_header_level(); // Should be 2
            let header_prefix = self.get_header_prefix();
            let module_path_str = format_id_path_canonical(&module_id, self.krate); // Use canonical path
            let display_path = if module_path_str.is_empty() {
                item.name.as_deref().unwrap_or("::") // Use item name for root if path is empty
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

            // Mark module as printed only AFTER printing its header
            self.printed_ids.insert(module_id);

            self.push_level();

            // Print module docs (using helper)
            self.print_docs(item);

            // Print module contents (non-module items only)
            self.print_module_contents(&module_id);

            self.pop_level();
            self.increment_current_level();

            // Recursively print child modules
            // Clone children list to avoid borrow checker issue
            if let Some(children) = self.module_tree.children.get(&module_id).cloned() {
                // No need to manage level here, the recursive call handles it
                for child_id in children {
                    self.print_module_recursive(child_id);
                }
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
        if let Some(desc) = &self.manifest.description {
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
        if let Some(hp) = &self.manifest.homepage {
            writeln!(self.output, "- Homepage: <{}>", hp).unwrap();
        }
        if let Some(repo) = &self.manifest.repository {
            writeln!(self.output, "- Repository: <{}>", repo).unwrap();
        }
        if !self.manifest.categories.is_empty() {
            writeln!(
                self.output,
                "- Categories: {}",
                self.manifest.categories.join(", ")
            )
            .unwrap();
        }
        if let Some(lic) = &self.manifest.license {
            writeln!(self.output, "- License: {}", lic).unwrap();
        }
        if let Some(rv) = &self.manifest.rust_version {
            writeln!(self.output, "- rust-version: `{}`", rv).unwrap();
        }
        if let Some(ed) = &self.manifest.edition {
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
        if self.manifest.features.is_empty() {
            writeln!(self.output, "- None").unwrap();
        } else {
            // Sort features for consistent output
            let mut sorted_features: Vec<_> = self.manifest.features.keys().collect();
            sorted_features.sort_unstable();
            for feature_name in sorted_features {
                // TODO: Maybe show what features a feature enables? Requires more parsing.
                writeln!(self.output, "- `{}`", feature_name).unwrap();
            }
        }
        writeln!(self.output).unwrap(); // Add newline after features list
        self.pop_level(); // Pop H3 features level

        // Increment H2 counter for the next section (README or Macros)
        self.increment_current_level();

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
            let adjusted_readme = adjust_markdown_headers(readme, 2);
            writeln!(self.output, "{}\n", adjusted_readme).unwrap();
            self.increment_current_level(); // Increment H2 counter
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
                sorted_macros.sort_by_key(|id| {
                    self.krate.index.get(id).and_then(|item| item.name.clone())
                });
                for id in sorted_macros {
                    self.print_item_details(&id); // Macro details at level 3
                }
                self.pop_level(); // Pop H3 level
                self.increment_current_level(); // Increment H2 counter
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
        for id in self.selected_ids {
            if !self.printed_ids.contains(id) {
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
                        self.printed_ids.insert(*id);
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
                        self.increment_current_level(); // Increment level for this item
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
                        writeln!(
                            self.output,
                            "_Error: Item details not found in index._\n"
                        )
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
                        *counts_by_kind
                            .entry(ItemKind::StructField)
                            .or_insert(0) += 1; // Use a placeholder kind like StructField
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

        self.output
    }
}

fn generate_documentation(
    krate: &Crate,
    manifest: &CrateManifestData, // Accept manifest data
    readme_content: Option<String>, // Accept README content
    selected_ids: &HashSet<Id>,
    resolved_modules: &HashMap<Id, ResolvedModule>, // Accept resolved modules
    graph: &IdGraph,                                 // Accept graph
    include_other: bool,
    template_mode: bool, // Add template mode flag
) -> Result<String> {
    info!(
        "Generating documentation for {} selected items.",
        selected_ids.len()
    );
    if selected_ids.is_empty() {
        return Ok("No items selected for documentation.".to_string());
    }

    let printer = DocPrinter::new(
        krate,
        manifest,         // Pass manifest data
        readme_content,   // Pass README content
        selected_ids,
        resolved_modules,
        graph,
        include_other,
        template_mode, // Pass template mode flag
    );
    let output = printer.finalize();

    Ok(output)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging: default to 'info' if RUST_LOG is not set, write to stderr
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr) // Ensure logs go to stderr
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

    let build_path = PathBuf::from(args.build_dir); // Use build_dir
    std::fs::create_dir_all(&build_path)
        .with_context(|| format!("Failed to create build directory: {}", build_path.display()))?;

    let crate_dir = download_and_unpack_crate(&client, &target_version, &build_path).await?;

    // --- Load Cargo.toml ---
    let manifest_path = crate_dir.join("Cargo.toml");
    let manifest: Manifest = Manifest::from_path(&manifest_path).with_context(|| {
        format!(
            "Failed to read or parse Cargo.toml: {}",
            manifest_path.display()
        )
    })?;

    // Extract relevant manifest data (handle potential missing fields)
    // Use .package field (Option<Package>) then access fields on the Package struct
    let package_data = manifest.package.as_ref().unwrap(); // Assume package exists
    let manifest_data = CrateManifestData {
        description: package_data
            .description
            .as_ref()
            .and_then(|d| d.as_ref().as_local())
            .cloned(),
        homepage: package_data
            .homepage
            .as_ref()
            .and_then(|h| h.as_ref().as_local())
            .cloned(),
        repository: package_data
            .repository
            .as_ref()
            .and_then(|r| r.as_ref().as_local())
            .cloned(),
        categories: package_data
            .categories
            .as_ref()
            .and_then(|c| c.as_ref().as_local())
            .cloned()
            .unwrap_or_default(),
        license: package_data
            .license
            .as_ref()
            .and_then(|l| l.as_ref().as_local())
            .cloned(),
        rust_version: package_data
            .rust_version
            .as_ref()
            .and_then(|rv| rv.as_ref().as_local())
            .cloned(),
        edition: package_data
            .edition
            .as_ref()
            .and_then(|e| e.as_ref().as_local()) // Get Option<&Edition>
            .map(|e| e.as_str().to_string()), // Use as_str() then to_string()
        features: manifest.features.clone().unwrap_or_default(), // Use manifest.features field
    };

    // --- Locate and Read README ---
    let readme_content = if args.no_readme {
        None
    } else {
        let readme_md_path = crate_dir.join("README.md");
        let readme_path = crate_dir.join("README");
        let readme_file_path = if readme_md_path.exists() {
            Some(readme_md_path)
        } else if readme_path.exists() {
            Some(readme_path)
        } else {
            None
        };

        if let Some(path) = readme_file_path {
            info!("Found README file: {}", path.display());
            match fs::read_to_string(&path) {
                Ok(content) => Some(content),
                Err(e) => {
                    warn!("Failed to read README file at {}: {}", path.display(), e);
                    None
                }
            }
        } else {
            info!("No README.md or README file found in crate root.");
            None
        }
    };

    // Use the *actual* crate name from the API response, as it might differ in casing
    let actual_crate_name = &target_version.crate_name;
    let json_output_path = run_rustdoc(
        &crate_dir,
        actual_crate_name,
        args.features.as_deref(),
        args.no_default_features,
        args.target.as_deref(),
    )?;

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

    // --- Resolve Modules ---
    let resolved_modules = build_resolved_module_index(&krate);

    // --- Select Items & Build Graph ---
    let (selected_ids, full_graph) = select_items(&krate, &args.paths, &resolved_modules)?;

    // --- Graph Dumping ---
    if let Some(dump_output_path) = &args.dump_graph {
        // Determine the graph to dump based on --dump-to-id
        let graph_to_dump = if let Some(target_leaf_id) = args.dump_to_id {
            info!(
                "Filtering graph to include only paths leading to leaf ID: {}",
                target_leaf_id.0
            );
            let filtered_graph = full_graph.filter_to_leaf(target_leaf_id);
            if filtered_graph.edges.is_empty() && !full_graph.edges.is_empty() {
                warn!(
                    "Target leaf ID {} for --dump-to-id not found or no paths lead to it in the graph. Dump will be empty.",
                    target_leaf_id.0
                );
            }
            filtered_graph // Use the filtered graph for dumping
        } else {
            full_graph.clone() // Use the full graph if no filtering needed
        };

        // Determine roots and description based on --dump-from-id and --dump-modules
        let (root_ids, dump_description) = if let Some(root_id) = args.dump_from_id {
            // --dump-from-id takes precedence
            let roots: HashSet<Id> = [root_id].into_iter().collect();
            let description = format!("ID {}", root_id.0);
            // Check if the specified root exists in the *graph_to_dump*
            let root_exists_in_graph = graph_to_dump.adjacency.contains_key(&root_id)
                || graph_to_dump.reverse_adjacency.contains_key(&root_id);
            if !root_exists_in_graph {
                warn!(
                    "Root ID {} provided via --dump-from-id not found in the {}graph. Dump file will be empty.",
                    root_id.0,
                    if args.dump_to_id.is_some() { "filtered " } else { "" },
                );
                // Create an empty file and skip dumping
                File::create(dump_output_path)?;
                (HashSet::new(), description) // Set roots to empty to skip dump_graph_subset logic
            } else {
                (roots, description)
            }
        } else if args.dump_modules {
            // --dump-modules is used
            let module_roots: HashSet<Id> = krate
                .index
                .iter()
                .filter(|(_, item)| matches!(item.inner, ItemEnum::Module(_)))
                .map(|(id, _)| *id)
                // Only include module roots that actually exist in the potentially filtered graph's nodes
                .filter(|id| {
                    graph_to_dump.adjacency.contains_key(id)
                        || graph_to_dump.reverse_adjacency.contains_key(id)
                })
                .collect();
            (module_roots, "modules".to_string())
        } else {
            // Default: dump all roots
            (graph_to_dump.find_roots(), "full".to_string())
        };

        // Dump the subset if root_ids is not empty (handles the case where dump_from_id root didn't exist)
        if !root_ids.is_empty() {
            dump_graph_subset(
                &graph_to_dump,
                &krate,
                &root_ids,
                dump_output_path,
                &dump_description,
                args.dump_max_depth,
            )?;
        }
    }

    // --- Generate Documentation ---
    // Pass resolved modules, full graph, template flag, README content, and manifest data
    let documentation = generate_documentation(
        &krate,
        &manifest_data, // Pass manifest data here
        readme_content, // Pass README content here
        &selected_ids,
        &resolved_modules,
        &full_graph,
        args.include_other,
        args.template,
    )?;

    // --- Output Documentation ---
    if let Some(output_file_path) = args.output {
        info!("Writing documentation to file: {}", output_file_path.display());
        let mut file = File::create(&output_file_path).with_context(|| {
            format!("Failed to create output file: {}", output_file_path.display())
        })?;
        file.write_all(documentation.as_bytes())
            .with_context(|| {
                format!("Failed to write to output file: {}", output_file_path.display())
            })?;
        info!(
            "Successfully wrote documentation to {}",
            output_file_path.display()
        );
    } else {
        info!("Printing documentation to stdout.");
        print!("{}", documentation); // Use print! to avoid extra newline at the end
    }

    Ok(())
}