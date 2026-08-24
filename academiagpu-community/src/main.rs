use academiagpu_community::monitor::GpuMonitor;
use clap::Parser;
use log::info;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 8192)]
    vram_limit_mb: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();

    info!("Initializing AcademiaGPU Core (Community Edition)...");
    
    match GpuMonitor::new(args.vram_limit_mb) {
        Ok(monitor) => {
            monitor.start_monitoring_loop().await;
        }
        Err(e) => {
            log::error!("Failed to initialize NVML. Are NVIDIA drivers installed? Error: {:?}", e);
        }
    }

    Ok(())
}
