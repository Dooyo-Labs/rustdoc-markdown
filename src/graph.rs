use anyhow::{anyhow, Result};
use rustdoc_types::{
    Crate, GenericArg, GenericArgs, GenericBound,
    GenericParamDef, Generics, Id, ItemEnum, Path,
    Term, Type, WherePredicate,
};
use std::collections::{HashMap, HashSet, VecDeque}; // Use HashMap instead of BTreeMap where needed
use std::fmt::{Display, Formatter}; // Use FmtWrite alias
use std::hash::Hash;
use std::io::Write as IoWrite; // Use IoWrite alias and IMPORT Cursor
use tracing::{debug, info, warn};

use crate::get_type_id;


// --- ID Graph Structures ---

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EdgeLabel {
    Contains,             // Module contains Item (original structure)
    ReferencesType,       // Item references Type ID (e.g., field type, return type)
    GenericArgument,      // Path uses Type ID as generic arg
    AssociatedType,       // Item references Associated Type ID
    AssociatedConstant,   // Item references Associated Constant ID
    TraitBound,           // Generic Param/Where Clause has Trait Bound ID
    Implements,           // Impl block implements Trait ID
    ImplFor,              // Impl block is for Type ID
    ImplItem,             // Impl block contains Item ID
    TraitItem,            // Trait contains Item ID
    EnumVariant,          // Enum contains Variant ID
    VariantField,         // Variant contains Field ID
    StructField,          // Struct contains Field ID
    UnionField,           // Union contains Field ID
    FieldType,            // Field ID has Type ID
    AliasTo,              // TypeAlias/TraitAlias points to Type/Trait ID
    SignatureInput,       // Function signature references input Type ID
    SignatureOutput,      // Function signature references output Type ID
    SuperTrait,           // Trait has supertrait Trait ID
    Dependency,           // Generic catch-all for less specific type dependencies
    IntraDocLink,         // Doc comment links to Item ID
    AssociatedConstraint, // Generic Arg Constraint references Item ID
    ParamType,            // Generic Param Def references Type ID
    ParamBound,           // Generic Param Def references Bound/Trait ID
    PredicateType,        // Where Predicate references Type ID
    PredicateBound,       // Where Predicate references Bound/Trait ID
    PredicateEqLhs,       // Where Predicate Eq references LHS Type ID
    PredicateEqRhs,       // Where Predicate Eq references RHS Term ID
    DynTraitBound,        // DynTrait references Trait ID
    ImplTraitBound,       // ImplTrait references Bound/Trait ID
    UseTarget,            // Use item references target item/module ID
}

impl Display for EdgeLabel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Edge {
    pub source: Id,
    pub target: Id,
    pub label: EdgeLabel,
}

#[doc(hidden)]
#[derive(Debug, Default, Clone)] // Add Clone
pub struct IdGraph {
    pub edges: HashSet<Edge>, // Use HashSet to avoid duplicate edges
    // Add an adjacency list representation for easier traversal (target -> Vec<(source, label)>)
    // Note: We build the forward graph (source -> targets) for dependency finding.
    // For finding roots (no incoming edges), we analyze the final edge set.
    // For tree printing, we need source -> Vec<(target, label)>
    pub adjacency: HashMap<Id, Vec<(Id, EdgeLabel)>>,
    // Reverse adjacency list for filtering (target -> Vec<(source, label)>)
    pub reverse_adjacency: HashMap<Id, Vec<(Id, EdgeLabel)>>,
}

impl IdGraph {
    fn new() -> Self {
        Self::default()
    }

    /// Adds an edge, ensuring both source and target are in the crate index.
    pub(crate) fn add_edge(&mut self, source: Id, target: Id, label: EdgeLabel, krate: &Crate) {
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
    #[doc(hidden)]
    pub fn find_roots(&self) -> HashSet<Id> {
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
    pub(crate) fn find_incoming_edges(&self, target_id: &Id) -> Vec<&Edge> {
        self.edges
            .iter()
            .filter(|edge| edge.target == *target_id)
            .collect()
    }

    /// Filters the graph to keep only edges that are part of a path leading to the target_leaf_id.
    /// Returns a new `IdGraph` containing only the filtered edges.
    #[doc(hidden)]
    pub fn filter_to_leaf(&self, target_leaf_id: Id) -> IdGraph {
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

#[allow(unused)]
#[derive(Debug, Clone)]
enum ResolutionState {
    Unresolved,
    Resolving,
    Resolved(HashSet<Id>),
}

type ResolutionCache = HashMap<Id, ResolutionState>;

/// Represents a module with its fully resolved items after handling 'use' statements.
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct ResolvedModule {
    pub id: Id,
    /// The fully resolved set of item IDs directly accessible within this module.
    pub items: HashSet<Id>,
}

/// Recursively resolves items for a module, handling `use` statements and cycles.
fn resolve_module_items(module_id: Id, krate: &Crate, cache: &mut ResolutionCache) -> HashSet<Id> {
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
    cache.insert(module_id, ResolutionState::Resolved(resolved_items.clone()));
    resolved_items
}

/// Builds an index of all modules with their items resolved after handling 'use' statements.
#[doc(hidden)]
pub fn build_resolved_module_index(krate: &Crate) -> HashMap<Id, ResolvedModule> {
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
                        name: _,          // TODO: Could the name be an ID sometimes? Unlikely.
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
            // Process inputs
            for input_type in inputs {
                find_type_dependencies(
                    input_type,
                    source_id,
                    krate,
                    dependencies,
                    graph,
                    EdgeLabel::GenericArgument, // Or a more specific label if context implies Fn traits
                );
            }
            // Process output
            if let Some(output_type) = output {
                find_type_dependencies(
                    output_type,
                    source_id,
                    krate,
                    dependencies,
                    graph,
                    EdgeLabel::GenericArgument, // Or a more specific label
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
                find_generic_param_def_dependencies(
                    param_def,
                    source_id,
                    krate,
                    dependencies,
                    graph,
                );
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

/// Selects items based on path filters and recursively includes their dependencies.
/// Builds the graph for *all* items in the crate, regardless of filtering.
#[doc(hidden)]
pub fn select_items(
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
                            graph.add_edge(source_id, *impl_id, EdgeLabel::ImplFor, krate);
                            // Struct -> Impl relation
                        }
                    }
                }
                find_generics_dependencies(&s.generics, source_id, krate, &mut item_deps, graph);
                match &s.kind {
                    rustdoc_types::StructKind::Plain { fields, .. } => {
                        for field_id in fields {
                            if krate.index.contains_key(field_id) {
                                if item_deps.insert(*field_id) {
                                    graph.add_edge(
                                        source_id,
                                        *field_id,
                                        EdgeLabel::StructField,
                                        krate,
                                    );
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
                                        graph.add_edge(
                                            source_id,
                                            *field_id,
                                            EdgeLabel::StructField,
                                            krate,
                                        );
                                    }
                                    if let Some(field_item) = krate.index.get(field_id) {
                                        if let ItemEnum::StructField(field_type) = &field_item.inner
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
                                        graph.add_edge(
                                            source_id,
                                            *field_id,
                                            EdgeLabel::VariantField,
                                            krate,
                                        );
                                    }
                                    if let Some(field_item) = krate.index.get(field_id) {
                                        if let ItemEnum::StructField(field_type) = &field_item.inner
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
                                    graph.add_edge(
                                        source_id,
                                        *field_id,
                                        EdgeLabel::VariantField,
                                        krate,
                                    );
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
                generics,
                bounds,
                type_,
                ..
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
        .map(|item| format!("{:?}", crate::Printer::infer_item_kind(item))) // Reuse infer_item_kind
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
    graph: &IdGraph, // Use the potentially filtered graph
    krate: &Crate,
    writer: &mut dyn IoWrite,         // Changed to dyn Write for flexibility
    visited: &mut HashSet<Id>,        // Use mutable reference to shared visited set
    path_to_target: &mut HashSet<Id>, // Tracks current path to target leaf
    indent: usize,
    depth: usize,                     // Current recursion depth
    max_depth: Option<usize>,         // Maximum allowed depth
    prefix: &str,                     // Prefix like "├── " or "└── "
    parent_label: Option<&EdgeLabel>, // Label connecting this node to its parent
    is_root_call: bool,               // Flag to know if this is the initial call for a root
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
        let label_info = parent_label
            .map(|l| format!(" [{}]", l))
            .unwrap_or_default();
        // Add cycle marker only if globally visited before AND relevant to current path
        let cycle_marker =
            if !is_newly_visited && path_to_target.contains(&node_id) && !is_root_call {
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
            if is_newly_visited
                && graph
                    .get_children(&node_id)
                    .map_or(false, |c| !c.is_empty())
            {
                writeln!(
                    writer,
                    "{}{} [... children truncated due to max depth ...]",
                    " ".repeat(indent + 4), // Indent the truncation message
                    if graph.get_children(&node_id).unwrap().len() == 1 {
                        "└──"
                    } else {
                        "├──"
                    }  // Use appropriate prefix for one or more truncated children
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
                    visited,        // Pass mutable reference down
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

/// Dumps a subset of the dependency graph to a writer.
pub fn dump_graph_subset(
    graph: &IdGraph, // Use the potentially filtered graph
    krate: &Crate,
    root_ids: &HashSet<Id>,
    writer: &mut dyn IoWrite, // Changed to dyn Write
    dump_description: &str,
    max_depth: Option<usize>, // Add max_depth parameter
) -> Result<()> {
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
                    writer,
                    &mut visited,        // Pass shared mutable visited set
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
                    writer,
                    &mut visited,        // Pass shared mutable visited set
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
    Ok(())
}

// --- End Graph Dumping Logic ---