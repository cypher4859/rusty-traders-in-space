use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};


#[derive(Debug, Deserialize, Serialize)]
pub struct InventoryItemDTO {
    pub item_symbol:    String,
    pub name:           String,
    pub description:    String,
    pub units:          u32
}
