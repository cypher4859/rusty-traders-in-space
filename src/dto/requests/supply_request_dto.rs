use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestSystemSupplyConstructionDTO {
    pub trade_symbol: String,
    pub units_to_deliver: i64,
    pub ship_symbol: String
}


impl RequestSystemSupplyConstructionDTO {
    pub fn new<S1, S2>(
        trade_symbol: S1,
        units_to_deliver: i64,
        ship_symbol: S2
    ) -> anyhow::Result<Self>
    where
        S1: Into<String>,
        S2: Into<String>
    {
        let trade_symbol: String       = trade_symbol.into();
        let ship_symbol: String        = ship_symbol.into();
        Ok(Self
            {
                trade_symbol,
                units_to_deliver,
                ship_symbol
            }
        )
    }
}