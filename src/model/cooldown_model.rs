use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};
use crate::dto::responses::fleet_dto::{CooldownDTO};


#[derive(Debug, Clone)]
pub struct Cooldown {
    pub ship_symbol: String,
    pub total_seconds: u32,
    pub remaining_seconds: u32,
}

impl Cooldown {
    pub fn new<S1>(
        ship_symbol: S1, 
        total_seconds: u32, 
        remaining_seconds: u32
    ) -> anyhow::Result<Self> 
    where 
        S1: Into<String>
    {
        let ship_symbol = ship_symbol.into();
        ensure!(remaining_seconds <= total_seconds, "remaining > total");
        Ok(Self { ship_symbol, total_seconds, remaining_seconds })
    }
}

impl TryFrom<CooldownDTO> for Cooldown {
    type Error = anyhow::Error;

    fn try_from(dto: CooldownDTO) -> anyhow::Result<Self> {
        Cooldown::new(
            dto.ship_symbol,
            dto.total_seconds,
            dto.remaining_seconds,
        )
    }
}