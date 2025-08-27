use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};
use crate::dto::responses::fleet_dto::{CargoDTO};
use super::{InventoryItem};

#[derive(Debug, Clone, Serialize)]
pub struct Cargo {
    pub capacity: Option<u32>,
    pub units:    Option<u32>,
    pub inventory: Option<Vec<InventoryItem>>,
}

impl Cargo {
    pub fn new(
        capacity: Option<u32>, 
        units: Option<u32>, 
        inventory: Option<Vec<InventoryItem>>
    ) -> anyhow::Result<Self>
    {
        ensure!(Some(units) <= Some(capacity), "cargo units exceed capacity");
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
        // Option<Vec<InventoryItemDTO>> ─► Option<Vec<InventoryItem>>
        let inventory: Option<Vec<InventoryItem>> = match dto.inventory {
            Some(list) => Some(
                list.into_iter()
                    .map(InventoryItem::try_from)
                    .collect::<anyhow::Result<Vec<_>>>()?,
            ),
            None => None,
        };

        Cargo::new(dto.capacity, dto.units, inventory)
    }
}