use serde::{Deserialize, Serialize};
use crate::model::faction_model::{Faction, Trait, TraitSymbol, FactionSymbol};
use anyhow::{Result, anyhow, ensure};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FactionDataDTO {
    Single(FactionDTO),
    List(Vec<FactionDTO>),
}


#[derive(Debug, Deserialize)]
pub struct FactionEnvelopeDTO {
    pub data: FactionDataDTO
}


#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TraitDTO {
    pub symbol: String,
    pub name: String,
    pub description: String
}


