use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventoryItemDTO {
    #[serde(rename = "symbol")]
    pub item_symbol:    String, // TODO: Setup the Inventory enum of all the goods Symbols and plug it in here
    pub name:           String,
    pub description:    String,
    pub units:          u32
}
