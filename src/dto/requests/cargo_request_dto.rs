use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};

use crate::model::inventory_model::InventoryItemSymbol;

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestCargoJettisonDTO {
    pub symbol: InventoryItemSymbol, 
    pub units: u32
}

impl RequestCargoJettisonDTO {
    pub fn new(
        symbol: InventoryItemSymbol,
        units: u32
    ) -> anyhow::Result<Self>
    {
        Ok(Self
            {
                symbol,
                units
            }
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestCargoSellDTO {
    pub symbol: InventoryItemSymbol, 
    pub units: u32
}

impl RequestCargoSellDTO {
    pub fn new(
        symbol: InventoryItemSymbol,
        units: u32
    ) -> anyhow::Result<Self>
    {
        Ok(Self
            {
                symbol,
                units
            }
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestCargoBuyDTO {
    pub symbol: InventoryItemSymbol,
    pub units: u32
}

impl RequestCargoBuyDTO {
    pub fn new(
        symbol: InventoryItemSymbol,
        units: u32
    ) -> anyhow::Result<Self>
    {
        Ok(Self
            {
                symbol,
                units
            }
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestCargoTransferDTO {
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: InventoryItemSymbol, 
    pub units: u32,
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String
}

impl RequestCargoTransferDTO {
    pub fn new(
        trade_symbol: InventoryItemSymbol,
        units: u32,
        ship_symbol: String
    ) -> anyhow::Result<Self>
    {
        Ok(Self
            {
                trade_symbol,
                units,
                ship_symbol
            }
        )
    }
}
