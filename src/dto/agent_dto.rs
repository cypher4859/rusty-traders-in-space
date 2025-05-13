use serde::{Deserialize, Serialize};
use crate::model::agent_model::Agent;

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentRequestDTO {
    pub symbol:  String,
    pub faction: String,
    // #[serde(rename = "faction")]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentDTO {
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub symbol: String,
    #[serde(rename = "headquarters")]
    pub hq: String,
    pub credits: i32,
    #[serde(rename = "startingFaction")]
    pub starting_faction: String,
    #[serde(rename = "shipCount")]
    pub ship_count: u32
}

impl TryFrom<AgentDTO> for Agent {
    type Error = anyhow::Error;

    fn try_from(dto: AgentDTO) -> anyhow::Result<Self> {
        Agent::new(
            dto.account_id,
            dto.symbol, 
            dto.credits, 
            dto.hq,
            dto.starting_faction,
            dto.ship_count
        )
    }
}

// impl TryFrom<AgentRequestDTO> for Agent {
//     type Error = anyhow::Error;

//     fn try_from(dto: AgentRequestDTO) -> anyhow::Result<Self> {
//         Agent::new(
//             dto.symbol, 
//             dto.faction, 
//             dto.email
//         )
//     }
// }





