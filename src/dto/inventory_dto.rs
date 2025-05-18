use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};


#[derive(Debug, Deserialize)]
pub struct InventoryItemDTO {
    pub item_symbol:    String,
    pub name:           String,
    pub description:    String,
    pub units:          u32
}

pub enum InventoryItemSymbol {
    
}