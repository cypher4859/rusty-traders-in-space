use std::collections::HashMap;
use crate::ContractDTO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MarketSupplyChainDataDTO {
    Single(ContractDTO),
    List(Vec<ContractDTO>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketSupplyChainEnvelopeDTO {
    pub data: MarketSupplyChainDataDTO,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketSupplyChainDTO {
    #[serde(rename = "exportToImportMap")]
    pub export_to_import_map: HashMap<String, Vec<String>>,
}

