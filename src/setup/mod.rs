use anyhow::{Result, Context};

use crate::util::{run_host_command, run_host_command_status, is_flatpak};

const SUDOERS_FILE: &str = "/etc/sudoers.d/nvidia-wormhole";
const FLATPAK_APP_ID: &str = "com.github.doublepranks.nvidia-wormhole";

/// Check if sudo nvidia-settings works without password
/// We test this by running `sudo -n nvidia-settings --version` 
/// The -n flag means non-interactive (fail if password needed)
pub fn check_permissions() -> bool {
    let success = run_host_command_status("sudo", &["-n", "nvidia-settings", "--version"]);
    log::info!("sudo nvidia-settings test: success={}", success);
    success
}

/// Install the sudoers file using pkexec (Polkit)
/// This will prompt the user for their password via a graphical dialog
pub fn install_sudoers() -> Result<()> {
    let username = std::env::var("USER")
        .context("Could not get USER environment variable")?;
    
    // The sudoers rule: allow user to run nvidia-settings without password
    let sudoers_content = format!(
        "{} ALL=(ALL) NOPASSWD: /usr/bin/nvidia-settings\n",
        username
    );
    
    // Use pkexec to write the file with root privileges
    // We use a shell command because we need to write to a protected location
    let script = format!(
        "echo '{}' > {} && chmod 440 {}",
        sudoers_content.trim(),
        SUDOERS_FILE,
        SUDOERS_FILE
    );
    
    run_host_command("pkexec", &["sh", "-c", &script])?;
    log::info!("Successfully installed sudoers file");
    Ok(())
}

/// Create autostart desktop file for the daemon
/// In Flatpak, uses `flatpak run` command instead of binary path
pub fn create_autostart_entry(_binary_path: &str) -> Result<()> {
    let autostart_dir = dirs::config_dir()
        .context("Could not find config directory")?
        .join("autostart");
    
    std::fs::create_dir_all(&autostart_dir)
        .context("Failed to create autostart directory")?;
    
    // Determine the correct Exec command based on environment
    let exec_command = if is_flatpak() {
        format!("flatpak run {} --daemon", FLATPAK_APP_ID)
    } else {
        // For native installs, use the binary path
        let binary = std::env::current_exe()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "nvidia-wormhole".to_string());
        format!("{} --daemon", binary)
    };
    
    let desktop_content = format!(
r#"[Desktop Entry]
Type=Application
Name=Nvidia Wormhole Daemon
Comment=GPU Fan Control Daemon
Exec={}
Hidden=false
NoDisplay=true
X-GNOME-Autostart-enabled=true
"#,
        exec_command
    );
    
    let desktop_file = autostart_dir.join("nvidia-wormhole.desktop");
    std::fs::write(&desktop_file, desktop_content)
        .context("Failed to write desktop file")?;
    
    log::info!("Created autostart entry at {:?} with Exec={}", desktop_file, exec_command);
    Ok(())
}

/// Remove autostart desktop file
pub fn remove_autostart_entry() -> Result<()> {
    let desktop_file = dirs::config_dir()
        .context("Could not find config directory")?
        .join("autostart")
        .join("nvidia-wormhole.desktop");
    
    if desktop_file.exists() {
        std::fs::remove_file(&desktop_file)
            .context("Failed to remove desktop file")?;
        log::info!("Removed autostart entry");
    }
    Ok(())
}

/// Check if autostart is enabled
pub fn is_autostart_enabled() -> bool {
    dirs::config_dir()
        .map(|d| d.join("autostart").join("nvidia-wormhole.desktop").exists())
        .unwrap_or(false)
}
