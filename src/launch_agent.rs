use anyhow::{Context, Result};
use std::path::PathBuf;

const LAUNCH_AGENT_NAME: &str = "com.external-app-sync.daemon";
const LAUNCH_AGENT_LABEL: &str = "com.external-app-sync.daemon";

#[derive(Debug)]
pub struct LaunchAgent {
    plist_path: PathBuf,
    executable_path: PathBuf,
}

impl LaunchAgent {
    pub fn new(executable_path: PathBuf) -> Self {
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let plist_path = home_dir
            .join("Library/LaunchAgents")
            .join(format!("{}.plist", LAUNCH_AGENT_NAME));

        Self {
            plist_path,
            executable_path,
        }
    }

    pub fn is_installed(&self) -> bool {
        self.plist_path.exists()
    }

    pub fn install(&self) -> Result<()> {
        // Create LaunchAgents directory if it doesn't exist
        if let Some(parent) = self.plist_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Create plist content
        let plist_content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>{}</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>KeepAlive</key>
    <true/>
    <key>RunAtLoad</key>
    <true/>
    <key>StandardErrorPath</key>
    <string>{}/Library/Logs/{}.err</string>
    <key>StandardOutPath</key>
    <string>{}/Library/Logs/{}.out</string>
</dict>
</plist>"#,
            LAUNCH_AGENT_LABEL,
            self.executable_path.to_str().context("Invalid path")?,
            home_dir().to_str().context("Invalid home path")?,
            LAUNCH_AGENT_NAME,
            home_dir().to_str().context("Invalid home path")?,
            LAUNCH_AGENT_NAME
        );

        // Write plist file
        std::fs::write(&self.plist_path, plist_content)?;

        // Create log directory
        let log_dir = home_dir().join("Library/Logs");
        std::fs::create_dir_all(log_dir)?;

        // Load the agent
        self.load()?;

        Ok(())
    }

    pub fn uninstall(&self) -> Result<()> {
        if self.is_installed() {
            // Unload the agent first
            self.unload()?;

            // Remove the plist file
            std::fs::remove_file(&self.plist_path)?;

            // Clean up log files
            let log_dir = home_dir().join("Library/Logs");
            let _ = std::fs::remove_file(log_dir.join(format!("{}.out", LAUNCH_AGENT_NAME)));
            let _ = std::fs::remove_file(log_dir.join(format!("{}.err", LAUNCH_AGENT_NAME)));
        }
        Ok(())
    }

    fn load(&self) -> Result<()> {
        if self.is_installed() {
            run_launchctl(&["load", "-w", self.plist_path.to_str().unwrap()])?;
        }
        Ok(())
    }

    fn unload(&self) -> Result<()> {
        if self.is_installed() {
            run_launchctl(&["unload", "-w", self.plist_path.to_str().unwrap()])?;
        }
        Ok(())
    }
}

fn run_launchctl(args: &[&str]) -> Result<()> {
    let status = std::process::Command::new("launchctl")
        .args(args)
        .status()?;

    if !status.success() {
        anyhow::bail!("launchctl command failed");
    }
    Ok(())
}

fn home_dir() -> PathBuf {
    dirs::home_dir().expect("Could not find home directory")
}
