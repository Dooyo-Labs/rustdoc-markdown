use anyhow::{anyhow, Context, Result};
use cargo_manifest::Manifest;
use clap::Parser;
use rustdoc_markdown::{cratesio, graph, run_rustdoc, CrateExtraReader, Printer}; // Added CrateExtraReader
use rustdoc_types::{Crate, Id, ItemEnum};
use std::collections::HashSet;
use tracing_subscriber::EnvFilter;
// Keep this for parse_id
use std::fs::File;
use std::io::{BufWriter, Write as IoWrite}; // Use IoWrite alias
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
    /// Name of the crate on crates.io or from local manifest.
    /// If using --manifest or --git, this must match the package name in Cargo.toml.
    crate_name: String,

    /// Optional version requirement (e.g., "1.0", "1", "~1.2.3", "*").
    /// Ignored if --manifest or --git is used. Defaults to the latest suitable version.
    #[arg(default_value = "*")]
    crate_version: String,

    /// Include prerelease versions when selecting the latest version from crates.io.
    /// Ignored if --manifest or --git is used.
    #[arg(long)]
    include_prerelease: bool,

    /// Build directory for crate documentation artifacts (e.g., downloaded crate source, rustdoc JSON, cloned git repos).
    #[arg(long, default_value = ".ai/docs/rust/build")]
    build_dir: String,

    /// Path to write the generated Markdown documentation. Defaults to stdout.
    #[arg(long)]
    output: Option<PathBuf>,

    /// Filter documented items by module path (e.g., "::style", "widgets::Button").
    /// Can be specified multiple times.
    /// - Paths starting with `::` are absolute within the current crate.
    /// - Paths without `::` are relative to the crate root (e.g., `my_module` becomes `crate_name::my_module`).
    /// - Matches are prefix-based (e.g., `::style` matches `::style::TextStyle`).
    #[arg(long = "path")]
    paths: Vec<String>,

    /// Include items that don't fit standard categories (e.g., unprinted selected items)
    /// in a final 'Other' section. By default, these are logged as warnings and omitted.
    #[arg(long)]
    include_other: bool,

    /// Space-separated list of features to activate when running rustdoc.
    #[arg(long)]
    features: Option<String>,

    /// Do not activate the `default` feature when running rustdoc.
    #[arg(long)]
    no_default_features: bool,

    /// Build documentation for the specified target triple when running rustdoc.
    #[arg(long)]
    target: Option<String>,

    /// Output Mustache-like template markers (e.g., `{{MISSING_DOCS_1_2_1}}`)
    /// instead of the actual documentation content for items that have docstrings.
    /// Useful for identifying missing documentation in the source crate.
    #[arg(long)]
    template: bool,

    /// Do not embed the crate's README file in the generated Markdown.
    #[arg(long)]
    no_readme: bool,

    /// Disable the "Common Traits" summarization. If set, all implemented traits
    /// for each item will be listed directly with that item, instead of being
    /// summarized at the crate or module level.
    #[arg(long)]
    no_common_traits: bool,

    /// Do not include an "Examples Appendix" section, even if examples are found.
    #[arg(long)]
    no_examples: bool,

    /// Path to the Cargo.toml manifest file of a local crate.
    /// If provided, crates.io will not be queried, and the specified crate will be documented.
    /// The `crate_name` argument must match the `[package].name` in this manifest.
    /// Mutually exclusive with --git.
    #[arg(long, conflicts_with = "git_url")]
    manifest: Option<PathBuf>,

    /// URL of a Git repository to clone for documentation.
    /// If provided, crates.io will not be queried. The default branch will be used.
    /// The `crate_name` argument must match the `[package].name` in the located Cargo.toml.
    /// Mutually exclusive with --manifest.
    #[arg(long, conflicts_with = "manifest")]
    git_url: Option<String>,
}

#[derive(Parser, Debug)]
struct DumpGraphCommand {
    /// Name of the crate on crates.io or from local manifest
    crate_name: String,

    /// Optional version requirement (e.g., "1.0", "1", "~1.2.3", "*"). Ignored if --manifest is used.
    #[arg(default_value = "*")]
    crate_version: String,

    /// Include prerelease versions when selecting the latest. Ignored if --manifest is used.
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

    /// Path to the Cargo.toml manifest file. If provided, crates.io will not be queried.
    #[arg(long)]
    manifest: Option<PathBuf>,
}

/// Extracts the repository name from a Git URL.
/// e.g., "https://github.com/user/repo.git" -> "repo"
/// e.g., "git@github.com:user/repo.git" -> "repo"
fn repo_name_from_url(url: &str) -> Result<String> {
    let path = url
        .split('/')
        .last()
        .ok_or_else(|| anyhow!("Could not extract repository name from URL: {}", url))?;
    Ok(path.trim_end_matches(".git").to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging: default to 'info' if RUST_LOG is not set, write to stderr
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr) // Ensure logs go to stderr
        .init();

    let args = Args::parse();

    let client = reqwest::Client::builder()
        .user_agent(format!(
            "rustdoc-markdown/{} (github.com/Dooyo-Labs/rustdoc-markdown)",
            env!("CARGO_PKG_VERSION")
        ))
        .build()?;

    match args.command {
        Command::Print(print_args) => {
            let build_dir_path = PathBuf::from(&print_args.build_dir);
            std::fs::create_dir_all(&build_dir_path).with_context(|| {
                format!(
                    "Failed to create build directory: {}",
                    build_dir_path.display()
                )
            })?;

            let (package_dir, manifest, actual_crate_name_from_manifest, _target_version_num) = {
                if let Some(manifest_path) = &print_args.manifest {
                    info!(
                        "Using local manifest: {}",
                        manifest_path.canonicalize()?.display()
                    );
                    let m_path = manifest_path.canonicalize()?;
                    let dir = m_path
                        .parent()
                        .ok_or_else(|| {
                            anyhow!(
                                "Could not get parent directory of manifest: {}",
                                m_path.display()
                            )
                        })?
                        .to_path_buf();
                    let m = Manifest::from_path(&m_path).with_context(|| {
                        format!("Failed to read or parse Cargo.toml: {}", m_path.display())
                    })?;
                    let name_from_manifest = m
                        .package
                        .as_ref()
                        .ok_or_else(|| anyhow!("Manifest is missing [package] table"))?
                        .name
                        .clone();
                    if name_from_manifest != print_args.crate_name {
                        return Err(anyhow!(
                            "Crate name mismatch: command line '{}' vs manifest '{}'",
                            print_args.crate_name,
                            name_from_manifest
                        ));
                    }
                    let version_from_manifest = m
                        .package
                        .as_ref()
                        .and_then(|p| p.version.as_ref())
                        .and_then(|v| v.as_ref().as_local().cloned());
                    (dir, m, name_from_manifest, version_from_manifest)
                } else if let Some(git_url) = &print_args.git_url {
                    let repo_name = repo_name_from_url(git_url)?;
                    let repo_clone_target_dir = build_dir_path.join(&repo_name);

                    if repo_clone_target_dir.exists() {
                        info!(
                            "Repository already cloned at: {}",
                            repo_clone_target_dir.display()
                        );
                    } else {
                        info!(
                            "Cloning repository '{}' into '{}'...",
                            git_url,
                            repo_clone_target_dir.display()
                        );
                        git2::Repository::clone(git_url, &repo_clone_target_dir).with_context(
                            || format!("Failed to clone repository from URL: {}", git_url),
                        )?;
                        info!("Successfully cloned repository.");
                    }

                    let root_manifest_path = repo_clone_target_dir.join("Cargo.toml");
                    if !root_manifest_path.exists() {
                        return Err(anyhow!(
                            "Cargo.toml not found at the root of the cloned repository: {}",
                            root_manifest_path.display()
                        ));
                    }

                    let root_manifest =
                        Manifest::from_path(&root_manifest_path).with_context(|| {
                            format!(
                                "Failed to read or parse root Cargo.toml: {}",
                                root_manifest_path.display()
                            )
                        })?;

                    if let Some(workspace) = &root_manifest.workspace {
                        info!(
                            "Repository is a workspace. Searching for package '{}'...",
                            print_args.crate_name
                        );
                        let mut found_member_manifest_path = None;
                        let mut found_member_dir = None;

                        for member_glob_pattern_str in &workspace.members {
                            let full_glob_pattern = repo_clone_target_dir
                                .join(member_glob_pattern_str)
                                .to_string_lossy()
                                .into_owned();
                            info!("Searching glob pattern: {}", full_glob_pattern);

                            for entry in glob::glob(&full_glob_pattern).with_context(|| {
                                format!("Failed to read glob pattern: {}", full_glob_pattern)
                            })? {
                                match entry {
                                    Ok(member_path) => {
                                        if member_path.is_dir() {
                                            let member_manifest_path =
                                                member_path.join("Cargo.toml");
                                            if member_manifest_path.exists() {
                                                let member_manifest =
                                                    Manifest::from_path(&member_manifest_path)
                                                        .with_context(|| {
                                                            format!(
                                                        "Failed to parse member manifest: {}",
                                                        member_manifest_path.display()
                                                    )
                                                        })?;
                                                if let Some(pkg) = &member_manifest.package {
                                                    if pkg.name == print_args.crate_name {
                                                        found_member_manifest_path =
                                                            Some(member_manifest_path);
                                                        found_member_dir = Some(member_path);
                                                        break; // Found the target package
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => warn!("Error matching glob entry: {:?}", e),
                                }
                            }
                            if found_member_manifest_path.is_some() {
                                break; // Found in this glob pattern
                            }
                        }

                        if let (Some(m_path), Some(dir)) =
                            (found_member_manifest_path, found_member_dir)
                        {
                            info!(
                                "Found package '{}' in workspace at: {}",
                                print_args.crate_name,
                                dir.display()
                            );
                            let m = Manifest::from_path(&m_path).with_context(|| {
                                format!("Failed to parse member manifest: {}", m_path.display())
                            })?;
                            let version_from_manifest = m
                                .package
                                .as_ref()
                                .and_then(|p| p.version.as_ref())
                                .and_then(|v| v.as_ref().as_local().cloned());
                            (dir, m, print_args.crate_name.clone(), version_from_manifest)
                        } else {
                            return Err(anyhow!(
                                "Package '{}' not found in workspace members of repository '{}'",
                                print_args.crate_name,
                                git_url
                            ));
                        }
                    } else if let Some(pkg) = &root_manifest.package {
                        // Root is a single package
                        if pkg.name == print_args.crate_name {
                            info!("Using root package '{}' from repository.", pkg.name);
                            let version_from_manifest = pkg
                                .version
                                .as_ref()
                                .and_then(|v| v.as_ref().as_local().cloned());
                            (
                                repo_clone_target_dir,
                                root_manifest.clone(), // Clone root_manifest here
                                pkg.name.clone(),
                                version_from_manifest,
                            )
                        } else {
                            return Err(anyhow!(
                                "Crate name mismatch: command line '{}' vs repository root package name '{}'",
                                print_args.crate_name,
                                pkg.name
                            ));
                        }
                    } else {
                        return Err(anyhow!(
                            "Root Cargo.toml in repository '{}' is neither a workspace nor a package.",
                            git_url
                        ));
                    }
                } else {
                    // Fallback to crates.io
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

                    let dir = cratesio::download_and_unpack_crate(
                        &client,
                        &target_version,
                        &build_dir_path,
                    )
                    .await?;
                    let m_path = dir.join("Cargo.toml");
                    let m = Manifest::from_path(&m_path).with_context(|| {
                        format!("Failed to read or parse Cargo.toml: {}", m_path.display())
                    })?;
                    (
                        dir,
                        m,
                        target_version.crate_name.clone(),
                        Some(target_version.num.clone()),
                    )
                }
            };

            let krate: Crate = run_rustdoc(
                &package_dir, // Use package_dir for rustdoc
                &actual_crate_name_from_manifest,
                print_args.features.as_deref(),
                print_args.no_default_features,
                print_args.target.as_deref(),
                true,
            )?;

            let mut printer = Printer::new(&manifest, &krate);

            if !print_args.paths.is_empty() {
                printer = printer.paths(&print_args.paths);
            }

            let mut extra_reader = CrateExtraReader::new();
            if print_args.no_readme {
                extra_reader = extra_reader.no_readme();
            }
            if print_args.no_examples {
                extra_reader = extra_reader.no_examples();
            }
            let crate_extra = extra_reader.read(&manifest, &package_dir)?; // Pass manifest and package_dir
            printer = printer.crate_extra(crate_extra);

            if print_args.include_other {
                printer = printer.include_other();
            }
            if print_args.template {
                printer = printer.template_mode();
            }
            if print_args.no_common_traits {
                printer = printer.no_common_traits();
            }

            let documentation = printer.print()?;

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
            let build_dir_path = PathBuf::from(&dump_args.build_dir);
            std::fs::create_dir_all(&build_dir_path).with_context(|| {
                format!(
                    "Failed to create build directory: {}",
                    build_dir_path.display()
                )
            })?;

            let (crate_dir, _manifest, actual_crate_name_from_manifest, _target_version_num) =
                if let Some(manifest_path) = &dump_args.manifest {
                    info!(
                        "Using local manifest: {}",
                        manifest_path.canonicalize()?.display()
                    );
                    let m_path = manifest_path.canonicalize()?;
                    let dir = m_path
                        .parent()
                        .ok_or_else(|| {
                            anyhow!(
                                "Could not get parent directory of manifest: {}",
                                m_path.display()
                            )
                        })?
                        .to_path_buf();
                    let m: Manifest = Manifest::from_path(&m_path).with_context(|| {
                        format!("Failed to read or parse Cargo.toml: {}", m_path.display())
                    })?;
                    let name_from_manifest = m
                        .package
                        .as_ref()
                        .ok_or_else(|| anyhow!("Manifest is missing [package] table"))?
                        .name
                        .clone();
                    if name_from_manifest != dump_args.crate_name {
                        return Err(anyhow!(
                            "Crate name mismatch: command line '{}' vs manifest '{}'",
                            dump_args.crate_name,
                            name_from_manifest
                        ));
                    }
                    let version_from_manifest = m
                        .package
                        .as_ref()
                        .and_then(|p| p.version.as_ref())
                        .and_then(|v| v.as_ref().as_local().cloned());
                    (dir, m, name_from_manifest, version_from_manifest)
                } else {
                    let target_version = cratesio::find_best_version(
                        &client,
                        &dump_args.crate_name,
                        &dump_args.crate_version,
                        dump_args.include_prerelease,
                    )
                    .await?;
                    info!(
                        "Selected version {} for crate {}",
                        target_version.num, target_version.crate_name
                    );

                    let dir = cratesio::download_and_unpack_crate(
                        &client,
                        &target_version,
                        &build_dir_path,
                    )
                    .await?;
                    let m_path = dir.join("Cargo.toml");
                    let m: Manifest = Manifest::from_path(&m_path).with_context(|| {
                        format!("Failed to read or parse Cargo.toml: {}", m_path.display())
                    })?;
                    (
                        dir,
                        m,
                        target_version.crate_name.clone(),
                        Some(target_version.num.clone()),
                    )
                };

            let krate: Crate = run_rustdoc(
                &crate_dir,
                &actual_crate_name_from_manifest,
                dump_args.features.as_deref(),
                dump_args.no_default_features,
                dump_args.target.as_deref(),
                true,
            )?;

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
