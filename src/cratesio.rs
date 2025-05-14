

use anyhow::{anyhow, bail, Context, Result};
use flate2::read::GzDecoder;

use semver::{Version, VersionReq};
use serde::Deserialize;
use std::io::Cursor; // Use IoWrite alias and IMPORT Cursor
use std::path::{Path as FilePath, PathBuf}; // Corrected use statement
use tar::Archive;
use tracing::{debug, info, warn};

#[derive(Deserialize, Debug)]
struct CratesApiResponse {
    versions: Vec<CrateVersion>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CrateVersion {
    #[serde(rename = "crate")]
    pub crate_name: String,
    pub num: String, // Version number string
    pub yanked: bool,
    #[serde(skip)]
    pub semver: Option<Version>, // Parsed version, populated later
}

pub async fn find_best_version(
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

pub async fn download_and_unpack_crate(
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