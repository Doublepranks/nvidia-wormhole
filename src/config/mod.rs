use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Fan curve points: (temperature_celsius, fan_speed_percent)
    pub curve: Vec<(u32, u32)>,
    /// Daemon loop interval in milliseconds
    pub interval_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            curve: vec![
                (30, 0),
                (50, 30),
                (70, 60),
                (85, 100),
            ],
            interval_ms: 2000,
        }
    }
}

impl Config {
    /// Get the config file path: ~/.config/nvidia-wormhole/config.json
    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Could not determine config directory")?
            .join("nvidia-wormhole");
        
        Ok(config_dir.join("config.json"))
    }

    /// Load config from file, or return default if not found
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        
        if !path.exists() {
            log::info!("Config file not found, using defaults");
            return Ok(Self::default());
        }
        
        let content = fs::read_to_string(&path)
            .context("Failed to read config file")?;
        
        let config: Config = serde_json::from_str(&content)
            .context("Failed to parse config file")?;
        
        log::info!("Loaded config from {:?}", path);
        Ok(config)
    }

    /// Save config to file
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        
        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create config directory")?;
        }
        
        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;
        
        fs::write(&path, content)
            .context("Failed to write config file")?;
        
        log::info!("Saved config to {:?}", path);
        Ok(())
    }

    /// Get curve as f64 array for GUI sliders (just speed values)
    pub fn curve_speeds_f64(&self) -> [f64; 4] {
        let mut speeds = [0.0; 4];
        for (i, (_, speed)) in self.curve.iter().enumerate().take(4) {
            speeds[i] = *speed as f64;
        }
        speeds
    }

    /// Update curve from GUI speed values
    pub fn set_curve_speeds(&mut self, speeds: &[f64; 4]) {
        let temps = [30, 50, 70, 85];
        self.curve.clear();
        for (i, &speed) in speeds.iter().enumerate() {
            self.curve.push((temps[i], speed as u32));
        }
    }
}
