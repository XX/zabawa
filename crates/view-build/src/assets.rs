use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::{env, fs, io};

use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("cargo metadata failed, exit status: {0}")]
    CargoMetadataFailed(ExitStatus),

    #[error("failed to run cargo metadata")]
    CargoMetadataFailedToRun(io::Error),

    #[error("invalid cargo metadata json")]
    InvalidCargoMetadataJson(#[from] serde_json::Error),

    #[error("workspace root not found")]
    WorkspaceRootNotFound,

    #[error("crate root not found")]
    CrateRootNotFound(String),

    #[error("assets dir not found: {1} in crate {0}")]
    AssetsDirNotFound(String, PathBuf),

    #[error("failed to create target dir")]
    FailedToCreateTargetDir(io::Error),

    #[error("failed to copy assets dir from {} to {}: {}", .0.display(), .1.display(), .2)]
    FailedToCopyAssetsDir(PathBuf, PathBuf, io::Error),
}

#[derive(Default, Debug, Clone)]
pub struct Assets {
    exclude: HashSet<String>,
    target_dir: PathBuf,
    use_exclude_env_var: bool,
    use_workspace_root: bool,
    rerun_if_src_changed: bool,
}

impl Assets {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn exclude(mut self, excludes: impl IntoIterator<Item = String>) -> Self {
        self.exclude.extend(excludes);
        self
    }

    pub fn target_dir(mut self, target_dir: impl Into<PathBuf>) -> Self {
        self.target_dir = target_dir.into();
        self
    }

    pub fn use_exclude_env_var(mut self) -> Self {
        self.use_exclude_env_var = true;
        self
    }

    pub fn use_workspace_root(mut self) -> Self {
        self.use_workspace_root = true;
        self
    }

    pub fn rerun_if_src_changed(mut self) -> Self {
        self.rerun_if_src_changed = true;
        self
    }

    pub fn copy(self) -> Result<HashMap<String, (PathBuf, PathBuf)>, Error> {
        let Self {
            mut exclude,
            mut target_dir,
            use_exclude_env_var,
            use_workspace_root,
            rerun_if_src_changed,
        } = self;
        let metadata = cargo_metadata()?;
        let packages = metadata["packages"].as_array().expect("packages should be an array");

        if use_exclude_env_var {
            exclude.extend(excluded_packages());
        }

        if use_workspace_root {
            let workspace_root = PathBuf::from(
                metadata["workspace_root"]
                    .as_str()
                    .ok_or(Error::WorkspaceRootNotFound)?,
            );
            target_dir = workspace_root.join(target_dir);
        }

        fs::create_dir_all(&target_dir).map_err(Error::FailedToCreateTargetDir)?;

        let mut copied_asset_dirs = HashMap::new();
        for pkg in packages {
            let Some(name) = pkg["name"].as_str() else {
                continue;
            };

            if exclude.contains(name) {
                continue;
            }

            let web_assets = &pkg["metadata"]["web-assets"];
            if web_assets.is_null() {
                continue;
            }

            let asset_dir = web_assets["dir"].as_str().unwrap_or("assets");
            let manifest_path = PathBuf::from(
                pkg["manifest_path"]
                    .as_str()
                    .ok_or_else(|| Error::CrateRootNotFound(name.to_string()))?,
            );
            let crate_root = manifest_path
                .parent()
                .ok_or_else(|| Error::CrateRootNotFound(name.to_string()))?;
            let src_dir = crate_root.join(asset_dir);

            if !src_dir.exists() {
                return Err(Error::AssetsDirNotFound(name.to_string(), src_dir));
            }

            if rerun_if_src_changed {
                println!("cargo:rerun-if-changed={}", src_dir.display());
            }

            let dst_dir = target_dir.join(name);
            if let Err(err) = copy_dir_recursive(&src_dir, &dst_dir) {
                return Err(Error::FailedToCopyAssetsDir(src_dir, dst_dir, err));
            }
            println!("cargo:warning=web-assets: copied '{name}' → {}/", dst_dir.display());

            copied_asset_dirs.insert(name.to_string(), (src_dir, dst_dir));
        }

        Ok(copied_asset_dirs)
    }
}

fn cargo_metadata() -> Result<Value, Error> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version=1"])
        .output()
        .map_err(Error::CargoMetadataFailedToRun)?;

    if !output.status.success() {
        return Err(Error::CargoMetadataFailed(output.status));
    }

    Ok(serde_json::from_slice(&output.stdout)?)
}

fn excluded_packages() -> HashSet<String> {
    env::var("WEB_ASSETS_EXCLUDE")
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter_map(|var| if !var.is_empty() { Some(var.to_string()) } else { None })
        .collect()
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry.expect("dir entry failed");
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
