use anyhow::{Context, Result};
use core_foundation::{
    base::TCFType,
    url::{CFURLCreateFromFileSystemRepresentation, CFURL},
};
use objc::{class, msg_send, sel, sel_impl};
use std::path::{Path, PathBuf};
use tracing::info;

pub mod config {
    use directories::ProjectDirs;
    use serde::{Deserialize, Serialize};
    use std::path::PathBuf;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Config {
        pub external_apps_path: PathBuf,
        pub notify_on_disconnect: bool,
    }

    impl Config {
        pub fn load() -> Option<Self> {
            ProjectDirs::from("com", "external-app-sync", "ExternalAppSync")
                .and_then(|proj_dirs| {
                    let config_path = proj_dirs.config_dir().join("config.json");
                    std::fs::read_to_string(config_path)
                        .ok()
                        .and_then(|content| serde_json::from_str(&content).ok())
                })
        }

        pub fn save(&self) -> anyhow::Result<()> {
            if let Some(proj_dirs) = ProjectDirs::from("com", "external-app-sync", "ExternalAppSync") {
                let config_dir = proj_dirs.config_dir();
                std::fs::create_dir_all(config_dir)?;
                let config_path = config_dir.join("config.json");
                let content = serde_json::to_string_pretty(self)?;
                std::fs::write(config_path, content)?;
            }
            Ok(())
        }
    }
}

pub mod launch_agent;

pub fn create_alias(source: &Path, dest: &Path) -> Result<()> {
    unsafe {
        let source_url = path_to_cfurl(source)?;
        let dest_url = path_to_cfurl(dest)?;
        
        let workspace: cocoa::base::id = msg_send![class!(NSWorkspace), sharedWorkspace];
        let _: () = msg_send![workspace, createAliasAtURL:dest_url.as_concrete_TypeRef()
                                                toURL:source_url.as_concrete_TypeRef()];
        
        info!("Created alias from {:?} to {:?}", source, dest);
        Ok(())
    }
}

pub fn remove_alias(path: &Path) -> Result<()> {
    if path.exists() {
        std::fs::remove_file(path).context("Failed to remove alias")?;
        info!("Removed alias at {:?}", path);
    }
    Ok(())
}

fn path_to_cfurl(path: &Path) -> Result<CFURL> {
    let path_str = path.to_str().context("Invalid path")?;
    unsafe {
        let url = CFURLCreateFromFileSystemRepresentation(
            core_foundation::base::kCFAllocatorDefault,
            path_str.as_ptr(),
            path_str.len() as _,
            0,
        );
        if url.is_null() {
            anyhow::bail!("Failed to create CFURL");
        }
        Ok(CFURL::wrap_under_create_rule(url))
    }
}

pub fn is_app_bundle(path: &Path) -> bool {
    path.extension().map_or(false, |ext| ext == "app")
}

pub fn get_applications_dir() -> PathBuf {
    PathBuf::from("/Applications")
}
