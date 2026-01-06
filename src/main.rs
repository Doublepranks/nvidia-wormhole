mod gui;
mod hardware;
mod daemon;
mod config;
mod setup;
mod util;

use config::Config;
use daemon::r#loop::DaemonState;
use gui::app;

fn main() {
    env_logger::init();
    
    let args: Vec<String> = std::env::args().collect();
    let daemon_mode = args.contains(&String::from("--daemon"));
    
    if daemon_mode {
        run_daemon();
    } else {
        if let Err(e) = app::run() {
            log::error!("GUI error: {}", e);
        }
    }
}

fn run_daemon() {
    log::info!("Starting nvidia-wormhole in daemon mode");
    
    // Load config
    let config = Config::load().unwrap_or_else(|e| {
        log::warn!("Failed to load config: {}, using defaults", e);
        Config::default()
    });
    
    log::info!("Loaded curve: {:?}", config.curve);
    
    // Start daemon
    let daemon = DaemonState::new(config.curve);
    daemon.start(config.interval_ms);
    
    log::info!("Daemon running. Press Ctrl+C to stop.");
    
    // Keep main thread alive
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
