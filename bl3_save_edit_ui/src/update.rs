use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use flate2::read::GzDecoder;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder};
use retry::delay::Fibonacci;
use retry::{retry, Error as RetryError, OperationResult};
use serde::Deserialize;
use tar::Archive;
use tracing::info;
use version_compare::Version;

use crate::VERSION;

const RELEASES_API: &str = "https://api.github.com/repos/ZakisM/bl3_save_edit/releases";

const LINUX_ASSET: &str = "bl3_save_edit_ui-x86_64-unknown-linux-gnu.tar.gz";
const MAC_OS_ASSET: &str = "bl3_save_edit_ui-x86_64-apple-darwin.tar.gz";
const WINDOWS_ASSET: &str = "bl3_save_edit_ui-x86_64-pc-windows-msvc.zip";

#[derive(Debug, Deserialize, Clone)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<ReleaseAsset>,
    pub body: String,
}

impl Release {
    pub async fn download_asset(
        &self,
        asset_name: &str,
        client: &Client,
        new_release_executable_path: PathBuf,
    ) -> Result<()> {
        let asset = self
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .context("Failed to find Linux Asset")?;

        let asset_bytes = client
            .get(&asset.browser_download_url)
            .send()
            .await?
            .bytes()
            .await?;

        let asset_path = new_release_executable_path
            .parent()
            .context("Failed to read new_release_exec_path when downloading archived asset")?
            .join(asset_name);

        // First download the archive
        tokio::fs::write(&asset_path, asset_bytes).await?;

        // #[cfg(target_os = "windows")]
        // Unarchive zip

        #[cfg(target_os = "linux")]
        //Unarchive tar
        let asset_file = std::fs::File::open(&asset_path)?;

        let res: Result<bool> = tokio::task::spawn_blocking(move || {
            let mut archive = Archive::new(GzDecoder::new(asset_file));

            let mut new_release_executable_file =
                std::fs::File::create(new_release_executable_path)?;

            for file in archive.entries()? {
                let mut file = file?;

                let path = file.path()?;

                if let Some(name) = path.to_str() {
                    if name == "bl3_save_editor.AppImage" {
                        std::io::copy(&mut file, &mut new_release_executable_file)?;

                        return Ok(true);
                    }
                }
            }

            Ok(false)
        })
        .await?;

        let res = res?;

        if res {
            tokio::fs::remove_file(asset_path).await?;

            Ok(())
        } else {
            bail!("Failed to find new application inside downloaded archive")
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
}

pub async fn get_latest_release() -> Result<Release> {
    info!("Checking for the latest release...");

    let client = create_download_client()?;

    let res = client.get(RELEASES_API).send().await?.text().await?;

    let current_version = Version::from(VERSION).expect("Failed to read current_version");

    let releases = serde_json::from_str::<Vec<Release>>(&res)?;

    let latest_release = releases
        .into_iter()
        .find(|r| {
            if let Some(release_version) = Version::from(&r.tag_name.replace("v", "")) {
                release_version > current_version
            } else {
                false
            }
        })
        .context("Failed to find a newer release")?;

    info!("Found a new release: {}.", latest_release.tag_name);

    download_release_to_temp_dir(latest_release.clone()).await?;

    Ok(latest_release)
}

pub async fn download_release_to_temp_dir(release: Release) -> Result<()> {
    let binary_name = "bl3_save_edit_ui";

    #[cfg(not(target_os = "linux"))]
    let current_executable_path = std::env::current_exe()?;

    #[cfg(target_os = "linux")]
    let current_executable_path =
        PathBuf::from(std::env::var("APPIMAGE").context("Failed to read APPIMAGE env var")?);

    let current_executable_path_parent = current_executable_path
        .parent()
        .context("Failed to read current_executable_path_parent")?;

    let new_release_executable_path =
        current_executable_path_parent.join(&format!("new_{}", binary_name));

    // Rename the current running binary
    let current_executable_temp_path =
        current_executable_path_parent.join(&format!("current_temp_{}", binary_name));

    let client = create_download_client()?;

    #[cfg(target_os = "linux")]
    {
        release
            .download_asset(
                LINUX_ASSET,
                &client,
                new_release_executable_path.to_path_buf(),
            )
            .await?;
    }

    #[cfg(target_os = "macos")]
    {
        release
            .download_asset(MAC_OS_ASSET, &client, &new_release_executable_path)
            .await?;
    }

    #[cfg(target_os = "windows")]
    {
        release
            .download_asset(WINDOWS_ASSET, &client, &new_release_executable_path)
            .await?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        let mut permissions = tokio::fs::metadata(&new_release_executable_path)
            .await?
            .permissions();
        permissions.set_mode(0o755);

        tokio::fs::set_permissions(&new_release_executable_path, permissions).await?;
    }

    rename(&current_executable_path, &current_executable_temp_path)?;
    rename(&new_release_executable_path, &current_executable_path)?;

    std::process::Command::new(current_executable_path)
        .spawn()
        .context("Failed to start newly downloaded application")?;

    tokio::fs::remove_file(current_executable_temp_path).await?;

    std::process::exit(0);
}

pub fn create_download_client() -> Result<Client> {
    let mut default_headers = HeaderMap::new();
    default_headers.insert(
        "user-agent",
        HeaderValue::from_str(&format!("bl3_save_edit/{}", VERSION))
            .expect("Failed to create header value for latest release user-agent"),
    );

    let client = ClientBuilder::new()
        .default_headers(default_headers)
        .build()
        .context("Failed to build latest release client")?;

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
