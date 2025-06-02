use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FactionDataDTO {
    Single(FactionDTO),
    List(Vec<FactionDTO>),
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FactionEnvelopeDTO {
    pub data: FactionDataDTO
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FactionDTO {
    // #[serde]
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub headquarters: String,
    pub traits: Vec<TraitDTO>,
    #[serde(rename= "isRecruiting")]
    pub is_recruiting: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraitDTO {
    pub symbol: String,
    pub name: String,
    pub description: String
}


