use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder};
use retry::delay::Fibonacci;
use retry::{retry, Error as RetryError, OperationResult};
use serde::Deserialize;
use tracing::{error, info};
use version_compare::Version;
#[cfg(target_os = "windows")]
use zip::ZipArchive;

#[cfg(not(target_os = "windows"))]
use {flate2::read::GzDecoder, std::os::unix::fs::PermissionsExt, tar::Archive};

use crate::VERSION;

const RELEASES_API: &str = "https://api.github.com/repos/ZakisM/bl3_save_edit/releases";

#[cfg(target_os = "linux")]
const ASSET_ARCHIVE: &str = "bl3_save_edit-x86_64-unknown-linux-gnu.tar.gz";

#[cfg(target_os = "linux")]
const ASSET: &str = "bl3_save_editor.AppImage";

#[cfg(target_os = "macos")]
const ASSET_ARCHIVE: &str = "Bl3SaveEditor-x86_64-apple-darwin.tar.gz";

#[cfg(target_os = "macos")]
const ASSET: &str = "bl3_save_edit_ui";

#[cfg(target_os = "windows")]
const ASSET_ARCHIVE: &str = "Bl3SaveEditor-x86_64-pc-windows-msvc.zip";

#[cfg(target_os = "windows")]
const ASSET: &str = "Bl3SaveEditor.exe";

#[derive(Debug, Deserialize, Clone)]
pub struct Release {
    pub tag_name: String,
    pub prerelease: bool,
    pub assets: Vec<ReleaseAsset>,
    pub body: String,
}

impl Release {
    pub async fn download_asset(
        &self,
        client: &Client,
        new_release_executable_path: PathBuf,
    ) -> Result<()> {
        let asset = self
            .assets
            .iter()
            .find(|a| a.name == ASSET_ARCHIVE)
            .with_context(|| format!("failed to find Asset - {}", ASSET_ARCHIVE))?;

        let asset_bytes = client
            .get(&asset.browser_download_url)
            .send()
            .await?
            .bytes()
            .await?;

        let asset_path = new_release_executable_path
            .parent()
            .context("failed to read new_release_exec_path when downloading archived asset.")?
            .join(ASSET_ARCHIVE);

        // First download the archive
        tokio::fs::write(&asset_path, asset_bytes).await?;

        let asset_file = std::fs::File::open(&asset_path)?;

        #[cfg(target_os = "windows")]
        // Unarchive zip
        let res: Result<bool> = {
            tokio::task::spawn_blocking(move || {
                let mut archive = ZipArchive::new(asset_file)?;

                for i in 0..archive.len() {
                    if let Ok(mut file) = archive.by_index(i) {
                        if let Some(name) = file.enclosed_name().and_then(|n| n.file_name()) {
                            if name == ASSET {
                                let mut new_release_executable_file =
                                    std::fs::File::create(new_release_executable_path)?;

                                std::io::copy(&mut file, &mut new_release_executable_file)?;

                                return Ok(true);
                            }
                        }
                    }
                }

                Ok(false)
            })
            .await?
        };

        #[cfg(not(target_os = "windows"))]
        //Unarchive tarball
        let res: Result<bool> = {
            tokio::task::spawn_blocking(move || {
                let mut archive = Archive::new(GzDecoder::new(asset_file));

                for file in archive.entries()? {
                    let mut file = file?;

                    let path = file.path()?;

                    let file_name = path.file_name();

                    if let Some(name) = file_name {
                        if name == ASSET {
                            let mut new_release_executable_file =
                                std::fs::File::create(new_release_executable_path)?;

                            std::io::copy(&mut file, &mut new_release_executable_file)?;

                            return Ok(true);
                        }
                    }
                }

                Ok(false)
            })
            .await?
        };

        let res = res?;

        tokio::task::spawn_blocking(move || remove_file(asset_path)).await??;

        if res {
            Ok(())
        } else {
            bail!("failed to find new application inside downloaded archive.")
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
}

pub async fn get_latest_release() -> Result<Release> {
    #[cfg(not(target_arch = "x86_64"))]
    {
        bail!("Automatic updates are only currently supported for x86_64 architectures.");
    }

    info!("Checking for the latest release...");

    let client = create_download_client()?;

    let res = client.get(RELEASES_API).send().await?.text().await?;

    let current_version =
        Version::from(VERSION).expect("failed to read current Application version.");

    let releases = serde_json::from_str::<Vec<Release>>(&res)?;

    let latest_release = releases
        .into_iter()
        .filter(|r| !r.prerelease)
        .find(|r| {
            if let Some(release_version) = Version::from(&r.tag_name.replace("v", "")) {
                release_version > current_version
                    && r.assets.iter().any(|a| a.name == ASSET_ARCHIVE)
            } else {
                false
            }
        })
        .context("failed to find a newer release.")?;

    info!("Found a new release: {}.", latest_release.tag_name);

    Ok(latest_release)
}

pub async fn download_release(release: Release) -> Result<()> {
    info!("Downloading release: {}", release.tag_name);

    #[cfg(target_os = "windows")]
    let binary_name = "Bl3SaveEditor.exe";

    #[cfg(not(target_os = "windows"))]
    let binary_name = "bl3_save_edit_ui";

    #[cfg(not(target_os = "linux"))]
    let current_executable_path = std::env::current_exe()?;

    #[cfg(target_os = "linux")]
    let current_executable_path =
        PathBuf::from(std::env::var("APPIMAGE").context("Failed to read APPIMAGE env var")?);

    let current_executable_path_parent = current_executable_path
        .parent()
        .context("failed to read current_executable_path_parent.")?;

    let new_release_executable_path =
        current_executable_path_parent.join(&format!("new_{}", binary_name));

    // Rename the current running binary
    let current_executable_temp_path =
        current_executable_path_parent.join(&format!("current_temp_{}", binary_name));

    let client = create_download_client()?;

    release
        .download_asset(&client, new_release_executable_path.to_path_buf())
        .await?;

    #[cfg(not(target_os = "windows"))]
    {
        let mut permissions = tokio::fs::metadata(&new_release_executable_path)
            .await?
            .permissions();
        permissions.set_mode(0o755);

        tokio::fs::set_permissions(&new_release_executable_path, permissions).await?;
    }

    let current_executable_path_clone = current_executable_path.clone();
    let current_executable_temp_path_clone = current_executable_temp_path.clone();

    let _: Result<()> = tokio::task::spawn_blocking(move || {
        rename(&current_executable_path, &current_executable_temp_path)?;
        rename(&new_release_executable_path, &current_executable_path)?;

        Ok(())
    })
    .await?;

    std::process::Command::new(current_executable_path_clone)
        .arg("--cleanup_previous_path")
        .arg(current_executable_temp_path_clone.to_str().unwrap_or(""))
        .spawn()
        .context("failed to start newly downloaded application.")?;

    Ok(())
}

pub fn create_download_client() -> Result<Client> {
    let mut default_headers = HeaderMap::new();
    default_headers.insert(
        "user-agent",
        HeaderValue::from_str(&format!("bl3_save_edit/{}", VERSION))
            .expect("failed to create header value for latest release user-agent."),
    );

    let client = ClientBuilder::new()
        .default_headers(default_headers)
        .build()
        .context("failed to build release client downloader.")?;

    Ok(client)
}

pub fn rename<F, T>(from: F, to: T) -> std::io::Result<()>
where
    F: AsRef<Path>,
    T: AsRef<Path>,
{
    // 21 Fibonacci steps starting at 1 ms is ~28 seconds total
    // See https://github.com/rust-lang/rustup/pull/1873 where this was used by Rustup to work around
    // virus scanning file locks
    let from = from.as_ref();
    let to = to.as_ref();

    retry(
        Fibonacci::from_millis(1).take(21),
        || match std::fs::rename(from, to) {
            Ok(_) => OperationResult::Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::PermissionDenied => OperationResult::Retry(e),
                _ => OperationResult::Err(e),
            },
        },
    )
    .map_err(|e| match e {
        RetryError::Operation { error, .. } => error,
        RetryError::Internal(message) => std::io::Error::new(std::io::ErrorKind::Other, message),
    })
}

pub fn remove_file<P>(path: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    // 21 Fibonacci steps starting at 1 ms is ~28 seconds total
    // See https://github.com/rust-lang/rustup/pull/1873 where this was used by Rustup to work around
    // virus scanning file locks
    let path = path.as_ref();

    retry(
        Fibonacci::from_millis(1).take(21),
        || match std::fs::remove_file(path) {
            Ok(_) => {
                info!("successfully removed file.");
                OperationResult::Ok(())
            }
            Err(e) => {
                error!("failed to remove file: {}.", e);

                match e.kind() {
                    std::io::ErrorKind::PermissionDenied => OperationResult::Retry(e),
                    _ => OperationResult::Err(e),
                }
            }
        },
    )
    .map_err(|e| match e {
        RetryError::Operation { error, .. } => error,
        RetryError::Internal(message) => std::io::Error::new(std::io::ErrorKind::Other, message),
    })
}
