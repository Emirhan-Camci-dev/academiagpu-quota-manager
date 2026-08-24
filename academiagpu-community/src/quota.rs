use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuotaRule {
    pub max_vram_mb: u64,
    pub max_gpus: u32,
}

impl Default for QuotaRule {
    fn default() -> Self {
        Self {
            max_vram_mb: 8192, // 8GB default free tier limit
            max_gpus: 1,
        }
    }
}
