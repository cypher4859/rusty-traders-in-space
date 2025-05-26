use std::collections::HashMap;
use crate::ContractDTO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MarketSupplyChainDataDTO {
    Single(ContractDTO),
    List(Vec<ContractDTO>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarketSupplyChainEnvelopeDTO {
    pub data: MarketSupplyChainDataDTO,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketSupplyChainDTO {
    #[serde(rename = "exportToImportMap")]
    pub export_to_import_map: HashMap<String, Vec<String>>,
}

