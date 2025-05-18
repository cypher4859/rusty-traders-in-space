use serde::{Deserialize, Serialize};
use crate::model::faction_model::{Faction, Trait, TraitSymbol, FactionSymbol};
use anyhow::{Result, anyhow, ensure};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaDTO {
    pub total: u16,
    pub page: u16,
    pub limit: u16
}