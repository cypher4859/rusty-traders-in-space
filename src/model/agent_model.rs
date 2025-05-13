use crate::AgentDTO;
use anyhow::{Result, anyhow, ensure};

#[derive(Debug, Clone)]
pub struct Agent {
    account_id: String,
    symbol:  String,
    hq:      String,
    credits: i32,
    starting_faction: String,
    ship_count: u32
}

impl Agent {
    pub fn new<S1, S2, S3, S4>(
        account_id: S1,
        symbol: S2, 
        credits: i32, 
        hq: S3, 
        starting_faction: S4, 
        ship_count: u32
    ) -> anyhow::Result<Self>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>
    {
        let account_id: String       = account_id.into();
        let symbol: String           = symbol.into();
        let hq: String               = hq.into();
        let starting_faction: String = starting_faction.into();
        
        ensure!(!symbol.is_empty(), "symbol cannot be empty");
        ensure!(
            symbol
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_'),
            "symbol must be ASCII alphanumeric or '_'"
        );

        ensure!(credits >= 0, "credits cannot be negative");
        ensure!(!hq.is_empty(), "headquarters cannot be empty");
        ensure!(
            !starting_faction.is_empty(),
            "starting faction cannot be empty"
        );

        Ok(Self
            {
                account_id,
                symbol,
                credits,
                hq,
                starting_faction,
                ship_count,
            }
        )
    }
}

impl From<Agent> for AgentDTO {
    fn from(model: Agent) -> Self {
        AgentDTO {
            account_id: String::new(), // not used outbound—leave blank or compute
            symbol:  model.symbol,
            hq: model.hq,
            credits: model.credits,
            starting_faction: model.starting_faction,
            ship_count: model.ship_count
        }
    }
}