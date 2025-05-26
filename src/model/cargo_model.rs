use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};
use crate::dto::responses::fleet_dto::{CargoDTO};
use super::{InventoryItem};

#[derive(Debug, Clone)]
pub struct Cargo {
    pub capacity: u32,
    pub units:    u32,
    pub inventory: Vec<InventoryItem>,
}

impl Cargo {
    pub fn new(
        capacity: u32, 
        units: u32, 
        inventory: Vec<InventoryItem>
    ) -> anyhow::Result<Self>
    {
        ensure!(units <= capacity, "cargo units exceed capacity");
        Ok(Self { 
            capacity,
            units, 
            inventory 
        })
    }
}

impl TryFrom<CargoDTO> for Cargo {
    type Error = anyhow::Error;

    fn try_from(dto: CargoDTO) -> anyhow::Result<Self> {
        let inventory = dto
            .inventory
            .into_iter()
            .map(InventoryItem::try_from)
            .collect::<anyhow::Result<Vec<_>>>()?;
        Cargo::new(dto.capacity, dto.units, inventory)
    }
}