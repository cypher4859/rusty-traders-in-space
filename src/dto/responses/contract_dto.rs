use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};
use crate::{dto::responses::util_dto::MetaDTO, helpers::table_helpers::TableRow};

use super::fleet_dto::CargoDTO;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ContractDataDTO {
    Single(ContractDTO),
    List(Vec<ContractDTO>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractEnvelopeWithMetaDTO {
    pub data: ContractDataDTO,
    pub meta: MetaDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractEnvelopeDTO {
    pub data: ContractDataDTO,
}

impl TableRow for ContractEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "Type", "Faction", "Accepted", "Expiration"]
    }

    fn to_row(&self) -> Vec<String> {
        match &self.data {
            ContractDataDTO::Single(contract) => {
                vec![
                    contract.id.clone(),
                    contract.contract_type.clone(),
                    contract.faction_symbol.clone(),
                    contract.accepted.to_string(),
                    contract.expiration.clone(),
                ]
            },
            ContractDataDTO::List(contracts) => {
                vec![
                    format!("{} contracts", contracts.len()),
                    String::from("—"),
                    String::from("—"),
                    String::from("—"),
                    String::from("—"),
                    String::from("—"),
                ]
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractAndCargoEnvelopeDTO {
    pub data: ContractDataWithCargoDTO,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractDataWithCargoDTO {
    pub contract: ContractDTO,
    pub cargo: CargoDTO
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractDTO {
    pub id:            String,
    #[serde(rename = "factionSymbol")]
    pub faction_symbol: String,
    #[serde(rename = "type")]
    pub contract_type:  String,           // could enum later
    pub terms:         ContractTermsDTO,
    pub accepted:      bool,
    pub fulfilled:     bool,
    pub expiration:    String,            // or chrono::DateTime
    #[serde(rename = "deadlineToAccept")]
    pub deadline_to_accept: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractTermsDTO {
    pub deadline: String,
    pub payment:  PaymentDTO,
    pub deliver:  Vec<DeliverDTO>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentDTO {
    #[serde(rename = "onAccepted")]
    pub on_accepted: i64,
    #[serde(rename = "onFulfilled")]
    pub on_fulfilled: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeliverDTO {
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    #[serde(rename = "destinationSymbol")]
    pub destination_symbol: String,
    #[serde(rename = "unitsRequired")]
    pub units_required: i64,
    #[serde(rename = "unitsFulfilled")]
    pub units_fulfilled: i64,
}

