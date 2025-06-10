use std::collections::HashMap;
use crate::ContractDTO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MarketSupplyChainDataEnumDTO {
    Single(MarketSupplyChainDataEnvelopeDTO),
    List(Vec<MarketSupplyChainDataEnvelopeDTO>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketSupplyChainDataEnvelopeDTO {
    pub data: MarketSupplyChainDTO,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketSupplyChainDTO {
    #[serde(rename = "exportToImportMap")]
    pub export_to_import_map: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketEnvelopeDTO {
    pub data: MarketDTO,
}

/// ---------- market body ----------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketDTO {
    pub symbol: String,

    pub exports:      Vec<GoodInfoDTO>,
    pub imports:      Vec<GoodInfoDTO>,
    pub exchange:     Vec<GoodInfoDTO>,

    pub transactions: Option<Vec<MarketTxDTO>>,
    #[serde(rename = "tradeGoods")]
    pub trade_goods:  Option<Vec<TradeGoodDTO>>,
}

/// ---------- simple good descriptor (export / import / exchange) ----------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoodInfoDTO {
    pub symbol: String,
    pub name:   String,
    pub description: String,
}

/// ---------- transaction record ----------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketTxDTO {
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,

    #[serde(rename = "type")]
    pub tx_type: String,          // PURCHASE | SELL — convert to enum later

    pub units: u32,

    #[serde(rename = "pricePerUnit")]
    pub price_per_unit: u32,
    #[serde(rename = "totalPrice")]
    pub total_price: u32,

    pub timestamp: String,        // ISO-8601; swap to chrono DateTime if desired
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionDTO {
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    #[serde(rename = "totalPrice")]
    pub total_price: u32,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepairTransactionDTO {
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,
    #[serde(rename = "totalPrice")]
    pub total_price: u32,
    pub timestamp: String,
}

/// ---------- live market good snapshot ----------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeGoodDTO {
    pub symbol: String,

    #[serde(rename = "type")]
    pub good_type: String,        // EXPORT | IMPORT | EXCHANGE

    #[serde(rename = "tradeVolume")]
    pub trade_volume: u32,

    pub supply:   String,         // SCARCE | MODERATE | …
    pub activity: Option<String>,         // WEAK | STRONG | …

    #[serde(rename = "purchasePrice")]
    pub purchase_price: u32,
    #[serde(rename = "sellPrice")]
    pub sell_price: u32,
}

