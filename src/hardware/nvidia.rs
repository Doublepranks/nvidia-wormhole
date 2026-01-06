use anyhow::{Result, Context};

use crate::util::run_host_command;

pub fn set_fan_speed(gpu_id: u32, fan_id: u32, speed: u32) -> Result<()> {
    let assignment1 = format!("[gpu:{}]/GPUFanControlState=1", gpu_id);
    let assignment2 = format!("[fan:{}]/GPUTargetFanSpeed={}", fan_id, speed);
    
    let args = vec![
        "nvidia-settings",
        "-a", &assignment1,
        "-a", &assignment2
    ];
    
    run_host_command("sudo", &args).map(|_| ())
}

pub fn get_temp(gpu_id: u32) -> Result<u32> {
    let query = format!("[gpu:{}]/GPUCoreTemp", gpu_id);
    let args = vec![
        "-q", &query,
        "-t"
    ];
    
    let output = run_host_command("nvidia-settings", &args)?;
    let temp_str = output.trim();
    temp_str.parse::<u32>().context("Failed to parse temperature")
}

pub fn get_gpu_usage(gpu_id: u32) -> Result<u32> {
    // nvidia-smi --query-gpu=utilization.gpu --format=csv,noheader,nounits -i <id>
    // Output: just a number like "15"
    let gpu_idx = gpu_id.to_string();
    let args = vec![
        "--query-gpu=utilization.gpu",
        "--format=csv,noheader,nounits",
        "-i", &gpu_idx
    ];
    
    let output = run_host_command("nvidia-smi", &args)?;
    let usage_str = output.trim();
    usage_str.parse::<u32>().context("Failed to parse GPU usage from nvidia-smi")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fan_command_structure() {
        let gpu_id = 0;
        let fan_id = 1;
        let speed = 75;
        
        let assignment1 = format!("[gpu:{}]/GPUFanControlState=1", gpu_id);
        let assignment2 = format!("[fan:{}]/GPUTargetFanSpeed={}", fan_id, speed);
        
        assert_eq!(assignment1, "[gpu:0]/GPUFanControlState=1");
        assert_eq!(assignment2, "[fan:1]/GPUTargetFanSpeed=75");
    }
}
