//! Shared utilities for host command execution
//! Handles Flatpak sandbox escape transparently

use std::process::Command;
use std::path::Path;
use anyhow::{Result, Context};
use once_cell::sync::Lazy;

/// Cached check for Flatpak environment
pub static IN_FLATPAK: Lazy<bool> = Lazy::new(|| {
    Path::new("/.flatpak-info").exists()
});

/// Check if running inside a Flatpak sandbox
pub fn is_flatpak() -> bool {
    *IN_FLATPAK
}

/// Execute a command on the host system.
/// If running inside Flatpak, automatically prefixes with `flatpak-spawn --host`.
/// Sets DISPLAY=:0 for GUI compatibility.
pub fn run_host_command(cmd: &str, args: &[&str]) -> Result<String> {
    let mut command = if is_flatpak() {
        let mut c = Command::new("flatpak-spawn");
        c.arg("--host").arg(cmd);
        c
    } else {
        Command::new(cmd)
    };

    command.args(args);
    command.env("DISPLAY", std::env::var("DISPLAY").unwrap_or_else(|_| ":0".to_string()));
    
    let output = command.output().context(format!("Failed to execute {}", cmd))?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        anyhow::bail!("Command failed: {} (stderr: {})", cmd, err);
    }
}

/// Execute a command and return success/failure without output
pub fn run_host_command_status(cmd: &str, args: &[&str]) -> bool {
    let mut command = if is_flatpak() {
        let mut c = Command::new("flatpak-spawn");
        c.arg("--host").arg(cmd);
        c
    } else {
        Command::new(cmd)
    };

    command.args(args);
    command.env("DISPLAY", std::env::var("DISPLAY").unwrap_or_else(|_| ":0".to_string()));
    
    command.output().map(|o| o.status.success()).unwrap_or(false)
}
