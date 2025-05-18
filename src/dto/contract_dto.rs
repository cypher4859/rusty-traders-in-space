use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};


#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct ContractTermsDTO {
    pub deadline: String,
    pub payment:  PaymentDTO,
    pub deliver:  Vec<DeliverDTO>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentDTO {
    #[serde(rename = "onAccepted")]
    pub on_accepted: i64,
    #[serde(rename = "onFulfilled")]
    pub on_fulfilled: i64,
}

#[derive(Debug, Deserialize)]
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

