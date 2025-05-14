use anyhow::{Context, Result};
use cargo_manifest::Manifest;
use clap::Parser;
use rustdoc_markdown::{
    cratesio, generate_documentation, graph, run_rustdoc, NIGHTLY_RUST_VERSION,
}; // Use the library's Printer and GraphDumper
use rustdoc_types::{Crate, Id, ItemEnum};
use std::collections::HashSet;
use tracing_subscriber::EnvFilter;
// Keep this for parse_id
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write as IoWrite}; // Use IoWrite alias
use std::path::PathBuf;
use tracing::{info, warn};

/// Parses a string into an `Id`.
fn parse_id(s: &str) -> Result<Id, String> {
    s.parse::<u32>()
        .map(Id)
        .map_err(|_| format!("Invalid ID: '{}'. Must be a non-negative integer.", s))
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    /// Print crate documentation to Markdown
    Print(PrintCommand),
    /// Dump the crate's item dependency graph
    DumpGraph(DumpGraphCommand),
}

#[derive(Parser, Debug)]
struct PrintCommand {
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
    build_dir: String,

    /// Path to write the generated documentation (defaults to stdout)
    #[arg(long)]
    output: Option<PathBuf>,

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
    features: Option<String>,

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

    /// Do not generate "Common Traits" sections; list all traits for each item.
    #[arg(long)]
    no_common_traits: bool,

    /// Do not include an "Examples Appendix" section.
    #[arg(long)]
    no_examples: bool,
}

#[derive(Parser, Debug)]
struct DumpGraphCommand {
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
    build_dir: String,

    /// Path to write the graph dump (defaults to stdout)
    #[arg(long)]
    output: Option<PathBuf>,

    /// Dump graph starting only from module roots.
    #[arg(long)]
    modules: bool,

    /// Dump graph starting only from this ID. Takes precedence over --modules.
    #[arg(long, value_parser = parse_id)]
    from_id: Option<Id>,

    /// Filter graph dump to only include paths leading to this leaf ID.
    #[arg(long, value_parser = parse_id)]
    to_id: Option<Id>,

    /// Limit the maximum depth of the dumped graph.
    /// 0 means root only, 1 means root and direct children, etc.
    #[arg(long)]
    max_depth: Option<usize>,

    /// Space-separated list of features to activate
    #[arg(long)]
    features: Option<String>,

    /// Do not activate the `default` feature
    #[arg(long)]
    no_default_features: bool,

    /// Build documentation for the specified target triple
    #[arg(long)]
    target: Option<String>,

    /// Filter items by module path (e.g., "::style", "widgets::Button"). Can be specified multiple times.
    /// Paths starting with '::' imply the root of the current crate.
    /// Matches are prefix-based (e.g., "::style" matches "::style::TextStyle").
    /// This filter is applied *before* graph construction if --to-id is not used,
    /// or *after* graph filtering if --to-id is used.
    #[arg(long = "path")]
    paths: Vec<String>,
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
            "rustdoc-markdown/{} (github.com/Dooyo-Labs/rustdoc-markdown)",
            env!("CARGO_PKG_VERSION")
        ))
        .build()?;

    match args.command {
        Command::Print(print_args) => {
            let target_version = cratesio::find_best_version(
                &client,
                &print_args.crate_name,
                &print_args.crate_version,
                print_args.include_prerelease,
            )
            .await?;

            info!(
                "Selected version {} for crate {}",
                target_version.num, target_version.crate_name
            );

            let build_path = PathBuf::from(print_args.build_dir);
            std::fs::create_dir_all(&build_path).with_context(|| {
                format!("Failed to create build directory: {}", build_path.display())
            })?;

            let crate_dir =
                cratesio::download_and_unpack_crate(&client, &target_version, &build_path).await?;

            let manifest_path = crate_dir.join("Cargo.toml");
            let manifest: Manifest = Manifest::from_path(&manifest_path).with_context(|| {
                format!(
                    "Failed to read or parse Cargo.toml: {}",
                    manifest_path.display()
                )
            })?;

            let readme_content = if print_args.no_readme {
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
                // Clone readme_file_path before it's moved by and_then
                readme_file_path
                    .clone()
                    .and_then(|path| fs::read_to_string(&path).ok())
                    .or_else(|| {
                        if readme_file_path.is_some() {
                            warn!("Failed to read README.");
                        } else {
                            info!("No README found.");
                        }
                        None
                    })
            };

            let mut examples_readme_content: Option<String> = None;
            let mut examples_content: Option<Vec<(String, String)>> = None;
            if !print_args.no_examples {
                let examples_dir = crate_dir.join("examples");
                if examples_dir.is_dir() {
                    let ex_readme_md_path = examples_dir.join("README.md");
                    let ex_readme_path = examples_dir.join("README");
                    examples_readme_content = ex_readme_md_path
                        .exists()
                        .then_some(ex_readme_md_path)
                        .or_else(|| ex_readme_path.exists().then_some(ex_readme_path))
                        .and_then(|p| fs::read_to_string(p).ok());

                    let mut found_examples = Vec::new();
                    if let Ok(entries) = fs::read_dir(&examples_dir) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
                                if let Some(filename_str) =
                                    path.file_name().and_then(|n| n.to_str())
                                {
                                    if let Ok(content) = fs::read_to_string(&path) {
                                        found_examples.push((filename_str.to_string(), content));
                                    }
                                }
                            }
                        }
                    }
                    if !found_examples.is_empty() {
                        found_examples.sort_by(|a, b| a.0.cmp(&b.0));
                        examples_content = Some(found_examples);
                    }
                }
            }

            let actual_crate_name = &target_version.crate_name;
            let json_output_path = run_rustdoc(
                &crate_dir,
                actual_crate_name,
                print_args.features.as_deref(),
                print_args.no_default_features,
                print_args.target.as_deref(),
            )?;

            info!("Parsing rustdoc JSON: {}", json_output_path.display());
            let file = File::open(&json_output_path).with_context(|| {
                format!("Failed to open JSON file: {}", json_output_path.display())
            })?;
            let reader = BufReader::new(file);
            let krate: Crate = serde_json::from_reader(reader).with_context(|| {
                format!("Failed to parse JSON file: {}", json_output_path.display())
            })?;
            info!(
                "Loaded rustdoc JSON for {} v{}",
                actual_crate_name,
                krate.crate_version.as_deref().unwrap_or("?")
            );

            let documentation = generate_documentation(
                &manifest,
                &krate,
                readme_content,
                examples_readme_content,
                examples_content,
                &print_args.paths,
                print_args.include_other,
                print_args.template,
                print_args.no_common_traits,
                print_args.no_examples,
            )?;

            if let Some(output_file_path) = print_args.output {
                info!(
                    "Writing documentation to file: {}",
                    output_file_path.display()
                );
                let mut file = File::create(&output_file_path).with_context(|| {
                    format!(
                        "Failed to create output file: {}",
                        output_file_path.display()
                    )
                })?;
                file.write_all(documentation.as_bytes()).with_context(|| {
                    format!(
                        "Failed to write to output file: {}",
                        output_file_path.display()
                    )
                })?;
                info!(
                    "Successfully wrote documentation to {}",
                    output_file_path.display()
                );
            } else {
                info!("Printing documentation to stdout.");
                print!("{}", documentation);
            }
        }
        Command::DumpGraph(dump_args) => {
            let target_version = cratesio::find_best_version(
                &client,
                &dump_args.crate_name,
                &dump_args.crate_version,
                dump_args.include_prerelease,
            )
            .await?;

            let build_path = PathBuf::from(dump_args.build_dir);
            std::fs::create_dir_all(&build_path).with_context(|| {
                format!("Failed to create build directory: {}", build_path.display())
            })?;

            let crate_dir =
                cratesio::download_and_unpack_crate(&client, &target_version, &build_path).await?;

            let actual_crate_name = &target_version.crate_name;
            let json_output_path = run_rustdoc(
                &crate_dir,
                actual_crate_name,
                dump_args.features.as_deref(),
                dump_args.no_default_features,
                dump_args.target.as_deref(),
            )?;

            let file = File::open(&json_output_path)?;
            let reader = BufReader::new(file);
            let krate: Crate = serde_json::from_reader(reader)?;

            let resolved_modules = graph::build_resolved_module_index(&krate);
            let (_, full_graph) = graph::select_items(&krate, &dump_args.paths, &resolved_modules)?;

            let graph_to_dump = if let Some(target_leaf_id) = dump_args.to_id {
                info!(
                    "Filtering graph to include only paths leading to leaf ID: {}",
                    target_leaf_id.0
                );
                let filtered_graph = full_graph.filter_to_leaf(target_leaf_id);
                if filtered_graph.edges.is_empty() && !full_graph.edges.is_empty() {
                    warn!(
                        "Target leaf ID {} for --to-id not found or no paths lead to it in the graph. Dump will be empty.",
                        target_leaf_id.0
                    );
                }
                filtered_graph
            } else {
                full_graph.clone()
            };

            let (root_ids, dump_description) = if let Some(root_id) = dump_args.from_id {
                let roots: HashSet<Id> = [root_id].into_iter().collect();
                let description = format!("ID {}", root_id.0);
                let root_exists_in_graph = graph_to_dump.adjacency.contains_key(&root_id)
                    || graph_to_dump.reverse_adjacency.contains_key(&root_id);
                if !root_exists_in_graph {
                    warn!(
                        "Root ID {} provided via --from-id not found in the {}graph. Dump will be empty.",
                        root_id.0,
                        if dump_args.to_id.is_some() { "filtered " } else { "" },
                    );
                    (HashSet::new(), description)
                } else {
                    (roots, description)
                }
            } else if dump_args.modules {
                let module_roots: HashSet<Id> = krate
                    .index
                    .iter()
                    .filter(|(_, item)| matches!(item.inner, ItemEnum::Module(_)))
                    .map(|(id, _)| *id)
                    .filter(|id| {
                        graph_to_dump.adjacency.contains_key(id)
                            || graph_to_dump.reverse_adjacency.contains_key(id)
                    })
                    .collect();
                (module_roots, "modules".to_string())
            } else {
                (graph_to_dump.find_roots(), "full".to_string())
            };

            if !root_ids.is_empty() {
                if let Some(output_path) = dump_args.output {
                    info!(
                        "Dumping {} graph to: {}",
                        dump_description,
                        output_path.display()
                    );
                    let file = File::create(&output_path).with_context(|| {
                        format!(
                            "Failed to create graph dump file: {}",
                            output_path.display()
                        )
                    })?;
                    let mut writer = BufWriter::new(file);
                    graph::dump_graph_subset(
                        &graph_to_dump,
                        &krate,
                        &root_ids,
                        &mut writer,
                        &dump_description,
                        dump_args.max_depth,
                    )?;
                    writer.flush().with_context(|| {
                        format!("Failed to write graph to file: {}", output_path.display())
                    })?;
                    info!("Successfully dumped graph to {}", output_path.display());
                } else {
                    info!("Dumping {} graph to stdout.", dump_description);
                    let mut stdout_writer = BufWriter::new(std::io::stdout());
                    graph::dump_graph_subset(
                        &graph_to_dump,
                        &krate,
                        &root_ids,
                        &mut stdout_writer,
                        &dump_description,
                        dump_args.max_depth,
                    )?;
                    stdout_writer.flush()?;
                }
            } else if dump_args.output.is_some() {
                // If roots are empty (e.g. --from-id specified a non-existent ID) and output file is given, create an empty file.
                File::create(dump_args.output.unwrap())?;
                info!("Graph dump is empty, created empty file.");
            } else if root_ids.is_empty() {
                info!("Graph dump is empty, nothing to print to stdout.");
            }
        }
    }

    Ok(())
}
