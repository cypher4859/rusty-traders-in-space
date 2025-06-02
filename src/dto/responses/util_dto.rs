use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaDTO {
    pub total: u16,
    pub page: u16,
    pub limit: u16
}