#![allow(dead_code)]
use anyhow::Result;
use crate::util::run_host_command;
use regex::Regex;

#[derive(Debug)]
pub struct Gpu {
    pub id: u32,
    pub name: String,
    pub fans: u32,
}

pub fn detect_gpus() -> Result<Vec<Gpu>> {
    // 1. Detect GPUs first
    // nvidia-settings -q gpus
    // But easier might be nvidia-smi -L or parsing nvidia-settings output
    
    // For simplicty/robustness, let's assume single GPU for now or parse `nvidia-settings -q gpus`
    // Sample output: "  [0] my_machine:0[gpu:0] (NVIDIA GeForce RTX 3080)"
    
    // Let's implement fan detection for GPU 0 first as requested "Detectar quantas fans a GPU tem"
    
    let gpu_id = 0; // Scanning for GPU 0
    let fan_count = count_fans(gpu_id)?;
    
    Ok(vec![Gpu {
        id: gpu_id,
        name: "Generic NVIDIA GPU".to_string(), // TODO: Get real name
        fans: fan_count,
    }])
}

pub fn count_fans(_gpu_id: u32) -> Result<u32> {
    // run: nvidia-settings -q fans
    // output detects: [fan:0], [fan:1]...
    
    // Alternatively query `[gpu:0]/TargetFanSpeed` entries?
    // Reliable way: `nvidia-settings -q consumers` or listing targets.
    
    // Let's try `nvidia-settings -q fans`
    let output = run_host_command("nvidia-settings", &["-q", "fans"])?;
    
    // Count occurrences of "fan:N" identifiers in the output
    // Output format often has lines starting with properties.
    // We can count unique IDs.
    
    // A more robust way might be checking for `[fan:0]`, `[fan:1]` presence.
    // Let's count unique `\[fan:(\d+)\]` matches.
    
    let re = Regex::new(r"\[fan:(\d+)\]").unwrap();
    let mut max_fan_id = -1;
    
    for cap in re.captures_iter(&output) {
        if let Some(m) = cap.get(1) {
            if let Ok(id) = m.as_str().parse::<i32>() {
                if id > max_fan_id {
                    max_fan_id = id;
                }
            }
        }
    }
    
    if max_fan_id >= 0 {
        Ok((max_fan_id + 1) as u32)
    } else {
        Ok(0)
    }
}

pub fn get_gpu_name(gpu_id: u32) -> Result<String> {
    // nvidia-settings -q gpus
    // output: "  [0] hostname:0[gpu:0] (NVIDIA GeForce RTX 3080)"
    let output = run_host_command("nvidia-settings", &["-q", "gpus"])?;
    
    // Simple regex to capture content inside parentheses
    // This assumes the last parentheses pair contains the name
    let re = Regex::new(r"\(([^)]+)\)$").unwrap();
    
    for line in output.lines() {
        if line.contains(&format!("[gpu:{}]", gpu_id)) {
            if let Some(cap) = re.captures(line) {
                if let Some(name) = cap.get(1) {
                    return Ok(name.as_str().to_string());
                }
            }
        }
    }
    
    Ok(format!("Nvidia GPU {}", gpu_id))
}
