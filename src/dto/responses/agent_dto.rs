use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentEnvelopeDTO {
    pub data: AgentDataDTO,
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

impl TryFrom<AgentDTO> for Agent {
    type Error = anyhow::Error;

    fn try_from(dto: AgentDTO) -> anyhow::Result<Self> {
        let account_id = dto
            .account_id
            .ok_or_else(|| anyhow::anyhow!("account_id absent in this context"))?;

        Agent::new(
            account_id,
            dto.symbol, 
            dto.credits, 
            dto.hq,
            dto.starting_faction,
            dto.ship_count
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



