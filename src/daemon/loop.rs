use std::sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex};
use std::thread;
use std::time::Duration;
use log::{info, error};

use crate::hardware::nvidia;
use super::math;

#[derive(Clone, Default)]
pub struct SharedStatus {
    pub current_temp: u32,
    pub current_speed: u32,
    pub gpu_usage: u32,
}

#[derive(Clone)]
pub struct DaemonState {
    pub running: Arc<AtomicBool>,
    pub curve: Arc<Mutex<Vec<(u32, u32)>>>,
    pub status: Arc<Mutex<SharedStatus>>,
}

impl DaemonState {
    pub fn new(curve: Vec<(u32, u32)>) -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            curve: Arc::new(Mutex::new(curve)),
            status: Arc::new(Mutex::new(SharedStatus::default())),
        }
    }

    pub fn start(&self, interval_ms: u64) {
        if self.running.load(Ordering::Relaxed) {
            info!("Daemon already running");
            return;
        }
        
        self.running.store(true, Ordering::Relaxed);
        let running = self.running.clone();
        let curve_lock = self.curve.clone();
        let status_lock = self.status.clone(); // Clone for thread

        thread::spawn(move || {
            info!("Daemon started");
            
            // Detect GPU and fans
            let gpu_id = 0;
            let fan_count = crate::hardware::probe::count_fans(gpu_id).unwrap_or_else(|e| {
                error!("Failed to detect fans, assuming 1: {}", e);
                1
            });
            info!("Detected {} fan(s) on GPU {}", fan_count, gpu_id);

            let mut last_speed: Option<u32> = None;

            while running.load(Ordering::Relaxed) {
                match nvidia::get_temp(gpu_id) {
                    Ok(temp) => {
                        let curve = curve_lock.lock().unwrap();
                        let target_speed = math::calculate_target_speed(temp, &curve);
                        let usage = nvidia::get_gpu_usage(gpu_id).unwrap_or(0);
                        
                        info!("Temp: {}°C, Usage: {}% -> Target Speed: {}%", temp, usage, target_speed);
                        
                        // Update Shared Status
                        if let Ok(mut status) = status_lock.lock() {
                            status.current_temp = temp;
                            status.current_speed = target_speed;
                            status.gpu_usage = usage;
                        }

                        // Only update if speed changed
                        let should_update = match last_speed {
                            Some(s) => s != target_speed,
                            None => true,
                        };

                        if should_update {
                            // Apply speed to ALL fans
                            let mut all_success = true;
                            for fan_id in 0..fan_count {
                                match nvidia::set_fan_speed(gpu_id, fan_id, target_speed) {
                                    Ok(_) => info!("Fan {} set to {}%", fan_id, target_speed),
                                    Err(e) => {
                                        error!("Failed to set fan {} speed: {}", fan_id, e);
                                        all_success = false;
                                    }
                                }
                            }
                            if all_success {
                                last_speed = Some(target_speed);
                            }
                        }
                    },
                    Err(e) => {
                        error!("Failed to read temp from GPU {}: {}", gpu_id, e);
                    }
                }
                
                thread::sleep(Duration::from_millis(interval_ms));
            }
            info!("Daemon stopped");
        });
    }

    #[allow(dead_code)]
    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}
