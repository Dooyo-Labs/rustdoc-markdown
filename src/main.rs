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

use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use flate2::read::GzDecoder;
use rustdoc_json::Builder;
use rustdoc_types::{Crate, ItemEnum};
use semver::{Version, VersionReq};
use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::{Path, PathBuf};
// Removed unused import: use std::process::Command;
use tar::Archive;
// Removed unused import: use tempfile::TempDir;
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

    /// Output directory for crate documentation
    #[arg(long, default_value = ".ai/docs/rust/build")]
    output_dir: String,
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
    output_path: &Path,
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

fn run_rustdoc(crate_dir: &Path, crate_name: &str) -> Result<PathBuf> {
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

    let builder = Builder::default()
        .manifest_path(manifest_path)
        .toolchain(NIGHTLY_RUST_VERSION) // Specify the nightly toolchain
        .target_dir(crate_dir.join("target/doc")) // Set the output directory
        .package(crate_name); // Specify the package

    // Generate the JSON file
    match builder.build() {
        Ok(s) => {
            info!("Generated rustdoc JSON at: {}", json_path.display());
            Ok(s)
        }
        Err(e) => {
            eprintln!("--- rustdoc-json stderr ---");
            eprintln!("{:?}", e);
            bail!("rustdoc-json failed: {}", e);
        }
    }
}

fn parse_and_print_docs(json_path: &Path) -> Result<()> {
    info!("Parsing rustdoc JSON: {}", json_path.display());

    // Open the file and use serde_json::from_reader
    let file = File::open(json_path)
        .with_context(|| format!("Failed to open JSON file: {}", json_path.display()))?;
    let reader = BufReader::new(file);
    let krate: Crate = serde_json::from_reader(reader)
        .with_context(|| format!("Failed to parse JSON file: {}", json_path.display()))?;

    info!("Found {} items in the index.", krate.index.len());
    let mut doc_count = 0;

    println!("\n--- Docstrings for Crate: {:?} ---", krate.root);

    for (id, item) in &krate.index {
        if let Some(docs) = &item.docs {
            if !docs.trim().is_empty() {
                doc_count += 1;
                let path_str = krate
                    .paths
                    .get(id)
                    .map(|p| p.path.join("::"))
                    .unwrap_or_else(|| format!("Unknown Path (ID: {:?})", id));

                let item_kind = match item.inner {
                    ItemEnum::Module(_) => "Module",
                    ItemEnum::ExternCrate { .. } => "Extern Crate",
                    ItemEnum::Union(_) => "Union",
                    ItemEnum::Struct(_) => "Struct",
                    ItemEnum::StructField(_) => "Struct Field",
                    ItemEnum::Enum(_) => "Enum",
                    ItemEnum::Variant(_) => "Variant",
                    ItemEnum::Function(_) => "Function",
                    ItemEnum::Trait(_) => "Trait",
                    ItemEnum::TraitAlias(_) => "Trait Alias",
                    ItemEnum::Impl(_) => "Impl",
                    ItemEnum::TypeAlias(_) => "Type Alias",
                    ItemEnum::Constant { .. } => "Constant",
                    ItemEnum::Static(_) => "Static",
                    ItemEnum::Macro(_) => "Macro (Declarative)",
                    ItemEnum::ProcMacro(_) => "Proc Macro",
                    ItemEnum::Primitive(_) => "Primitive",
                    ItemEnum::AssocConst { .. } => "Associated Constant",
                    ItemEnum::AssocType { .. } => "Associated Type",
                    ItemEnum::Use(_) => "Use",
                    ItemEnum::ExternType => "Extern Type",
                };

                println!("\n## Item: {} ({})", path_str, item_kind);
                println!("{}", "-".repeat(path_str.len() + item_kind.len() + 6)); // Separator line
                println!("{}", docs.trim());
            }
        }
    }

    if doc_count == 0 {
        println!("\nNo docstrings found in the crate.");
    } else {
        println!("\n--- End of Docstrings ({} items found) ---", doc_count);
    }

    Ok(())
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
            "crate-doc-extractor/{} (github.com/your-repo)", // Replace with actual repo if applicable
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

    parse_and_print_docs(&json_output_path)?;

    Ok(())
}
