use serde::{Deserialize, Serialize};
use crate::helpers::table_helpers::TableRow;
use crate::model::agent_model::{Agent, RegisterResult};
use crate::model::ship_model::{Ship};
use crate::dto::responses::faction_dto::{FactionDTO};
use crate::dto::responses::contract_dto::ContractDTO;
use crate::dto::responses::fleet_dto::ShipDTO;
use crate::dto::responses::util_dto::MetaDTO;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterEnvelopeDTO {
    pub data: RegisterDataDTO,
}

impl TableRow for RegisterEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Token", "Name", "HQ", "Faction", "Ships"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            // self.data.agent.account_id.clone(),
            self.data.token.clone(),
            self.data.agent.symbol.clone(),
            self.data.agent.hq.clone(),
            self.data.agent.starting_faction.clone(),
            self.data.agent.ship_count.to_string()
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterDataDTO {
    pub token:    String,
    pub agent:    AgentDTO,
    pub faction:  FactionDTO,
    pub contract: ContractDTO,
    pub ships:    Vec<ShipDTO>,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentRequestDTO {
    pub symbol:  String,
    pub faction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AgentDataDTO {
    Single(AgentDTO),
    List(Vec<AgentDTO>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentEnvelopeWithMetaDTO {
    pub data: AgentDataDTO,
    pub meta: MetaDTO
}

impl TableRow for AgentEnvelopeWithMetaDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Name", "Credits", "HQ", "Faction", "Ships"]
    }

    fn to_row(&self) -> Vec<String> {
        Vec::new()
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        let mut rows: Vec<Vec<String>> = Vec::new();
        match &self.data {
            AgentDataDTO::Single(agent) => {
                rows.extend(vec![vec![
                    agent.symbol.clone(),
                    agent.credits.to_string(),
                    agent.hq.clone(),
                    agent.starting_faction.clone(),
                    agent.ship_count.to_string()
                ]]);
                rows
            },
            AgentDataDTO::List(agents) => {
                rows.extend(agents.iter().map(|agent| {
                    vec![
                        agent.symbol.clone(),
                        agent.credits.to_string(),
                        agent.hq.clone(),
                        agent.starting_faction.clone(),
                        agent.ship_count.to_string()
                    ]
                }));
                rows
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentEnvelopeDTO {
    pub data: AgentDataDTO,
}

impl TableRow for AgentEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Name", "Credits", "HQ", "Faction", "Ships"]
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        let mut rows: Vec<Vec<String>> = Vec::new();
        match &self.data {
            AgentDataDTO::Single(agent) => {
                rows.extend(vec![vec![
                    agent.symbol.clone(),
                    agent.credits.to_string(),
                    agent.hq.clone(),
                    agent.starting_faction.clone(),
                    agent.ship_count.to_string()
                ]]);
                rows
            },
            AgentDataDTO::List(agents) => {
                rows.extend(agents.iter().map(|agent| {
                    vec![
                        agent.symbol.clone(),
                        agent.credits.to_string(),
                        agent.hq.clone(),
                        agent.starting_faction.clone(),
                        agent.ship_count.to_string()
                    ]
                }));
                rows
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentDTO {
    #[serde(rename = "accountId", default)]
    pub account_id: Option<String>,
    pub symbol: String,
    #[serde(rename = "headquarters")]
    pub hq: String,
    pub credits: i64,
    #[serde(rename = "startingFaction")]
    pub starting_faction: String,
    #[serde(rename = "shipCount")]
    pub ship_count: u32
}

impl TableRow for AgentDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Account", "Name", "HQ", "Credits", "Faction", "Ships"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            Some(self.account_id.clone()).expect("Missing Account ID").unwrap_or_else(|| String::from("Not Available")),
            self.symbol.clone(),
            self.hq.clone(),
            self.credits.to_string(),
            self.starting_faction.clone(),
            self.ship_count.to_string()
        ]
    }
}

impl TryFrom<AgentDTO> for Agent {
    type Error = anyhow::Error;

    fn try_from(dto: AgentDTO) -> anyhow::Result<Self> {
        let account_id = dto
            .account_id
            .unwrap_or(String::from("<Hidden>"));

        Agent::new(
            account_id,
            dto.symbol, 
            dto.credits, 
            dto.hq,
            dto.starting_faction,
            dto.ship_count,
            None
        )
    }
}


impl TryFrom<RegisterDataDTO> for RegisterResult {
    type Error = anyhow::Error;

    fn try_from(dto: RegisterDataDTO) -> anyhow::Result<Self> {
        let ships: Vec<Ship> = dto
                                .ships
                                .into_iter()
                                .map(Ship::try_from)
                                .collect::<anyhow::Result<Vec<_>>>()?;
        
        RegisterResult::new(
            dto.token,
            dto.agent.try_into()?,
            dto.faction.try_into()?,
            dto.contract.try_into()?,
            ships
        )
    }
}

impl TryFrom<RegisterEnvelopeDTO> for RegisterResult {
    type Error = anyhow::Error;

    fn try_from(env: RegisterEnvelopeDTO) -> anyhow::Result<Self> {
        env.data.try_into()                // re-use the step above
    }
}



