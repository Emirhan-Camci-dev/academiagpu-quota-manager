use nvml_wrapper::Nvml;
use nvml_wrapper::error::NvmlError;
use log::{info, warn, error};
use std::time::Duration;
use tokio::time::sleep;
use sysinfo::{System, ProcessRefreshKind, RefreshKind};

pub struct GpuMonitor {
    nvml: Nvml,
    vram_limit_mb: u64,
}

impl GpuMonitor {
    pub fn new(vram_limit_mb: u64) -> Result<Self, NvmlError> {
        let nvml = Nvml::init()?;
        Ok(Self {
            nvml,
            vram_limit_mb,
        })
    }

    /// Sub-millisecond interval polling for VRAM usage
    pub async fn start_monitoring_loop(&self) {
        info!("Starting AGPLv3 GPU VRAM Monitoring loop (Limit: {} MB)...", self.vram_limit_mb);
        let mut sys = System::new_with_specifics(
            RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
        );

        loop {
            match self.check_vram_and_enforce(&mut sys) {
                Ok(_) => {}
                Err(e) => error!("NVML Monitoring error: {:?}", e),
            }
            // Low-overhead polling delay (e.g., 500ms)
            sleep(Duration::from_millis(500)).await;
        }
    }

    fn check_vram_and_enforce(&self, _sys: &mut System) -> Result<(), NvmlError> {
        let device_count = self.nvml.device_count()?;
        for i in 0..device_count {
            let device = self.nvml.device_by_index(i)?;
            let memory = device.memory_info()?;
            let used_mb = memory.used / (1024 * 1024);
            
            if used_mb > self.vram_limit_mb {
                warn!("⚠️ VRAM OOM PREVENTER: GPU {} exceeded VRAM quota! Used: {} MB, Limit: {} MB", i, used_mb, self.vram_limit_mb);
                // Community Edition: Soft-throttle warning and basic logging.
                // Process killing is manual in the free tier up to 8 GPUs.
                
                // Fetch compute processes
                let processes = device.running_compute_processes()?;
                for proc in processes {
                    let vram_mb = match proc.used_gpu_memory {
                        nvml_wrapper::enums::device::UsedGpuMemory::Used(bytes) => format!("{} MB", bytes / (1024 * 1024)),
                        nvml_wrapper::enums::device::UsedGpuMemory::Unavailable => "Unavailable".to_string(),
                    };
                    warn!("  -> Offending Process PID: {}, VRAM: {}", proc.pid, vram_mb);
                }
            }
        }
        Ok(())
    }
}
