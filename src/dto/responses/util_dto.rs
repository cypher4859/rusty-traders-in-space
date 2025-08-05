use serde::{Deserialize, de::DeserializeOwned, Serialize};
use anyhow::{Result, anyhow, ensure};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaDTO {
    pub total: u16,
    pub page: u16,
    pub limit: u16
}

/// Generic envelope: `D` can be `Vec<AgentDTO>`, `Vec<ShipDTO>`, a single
/// `FactionDTO`, or any custom struct.
#[derive(Debug, Deserialize)]
pub struct PageEnvelopeDTO<D> {
    pub data: Vec<D>,
    pub meta: MetaDTO,
}