use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestDeliverDTO {
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    #[serde(rename = "units")]
    pub units_to_deliver: i64,
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,
}

impl RequestDeliverDTO {
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
        // ensure!(
        //     !starting_faction.is_empty(),
        //     "starting faction cannot be empty"
        // );

        Ok(Self
            {
                trade_symbol,
                units_to_deliver,
                ship_symbol
            }
        )
    }
}

