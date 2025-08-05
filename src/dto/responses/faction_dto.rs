use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};

use crate::helpers::table_helpers::TableRow;

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

impl TableRow for FactionEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Name", "HQ", "Recruiting", "Description"]
    }

    fn to_row(&self) -> Vec<String> {
        match &self.data {
            FactionDataDTO::Single(faction) => {
                vec![
                    faction.name.clone(),
                    faction.headquarters.clone(),
                    faction.is_recruiting.to_string(),
                    faction.description.clone()
                ]
            },
            FactionDataDTO::List(factions) => {
                vec![
                    format!("{} factions", factions.len()),
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

impl TableRow for FactionDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Name", "HQ", "Recruiting", "Description"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.headquarters.clone(),
            self.is_recruiting.to_string(),
            self.description.clone()
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraitDTO {
    pub symbol: String,
    pub name: String,
    pub description: String
}


