use std::{fmt::DebugStruct, str::FromStr};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};
use strum_macros::{EnumIter, EnumString};
use crate::{helpers::enum_lookups::InventoryItemSymbol, dto::responses::inventory_dto::InventoryItemDTO};

#[derive(Debug, Clone, Serialize)]
pub struct InventoryItem {
    pub item_symbol: InventoryItemSymbol,
    pub name: String,
    pub description: String,
    pub units:  u32,
}

impl InventoryItem {
    pub fn new<S1, S2>(
        item_symbol: InventoryItemSymbol,
        name: S1,
        description: S2, 
        units: u32
    ) -> anyhow::Result<Self> 
    where 
        S1: Into<String>,
        S2: Into<String>
    {
        let name = name.into();
        let description = description.into();
        Ok(Self { 
            item_symbol, 
            name,
            description,
            units 
        })
    }
}

impl TryFrom<InventoryItemDTO> for InventoryItem {
    type Error = anyhow::Error;

    fn try_from(dto: InventoryItemDTO) -> anyhow::Result<Self> {
        let item_symbol = InventoryItemSymbol::from_str(&dto.item_symbol)?;
        InventoryItem::new(
            item_symbol,
            dto.name,
            dto.description, 
            dto.units
        )
    }
}
