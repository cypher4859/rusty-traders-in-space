use std::collections::{HashMap, HashSet};
use crate::{helpers::table_helpers::TableRow, ContractDTO};
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

impl TableRow for MarketSupplyChainDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Export", "Imports"]
    }

    // we never use `to_row`, but must provide *some* body
    fn to_row(&self) -> Vec<String> {
        Vec::new()
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        self.data.export_to_import_map
            .iter()
            .map(|(export, imports)| {
                vec![
                    export.clone(),
                    imports.join(", "),
                ]
            })
            .collect()
    }
}

impl TableRow for MarketSupplyChainDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Export", "Imports"]
    }

    // we never use `to_row`, but must provide *some* body
    fn to_row(&self) -> Vec<String> {
        Vec::new()
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        self.export_to_import_map
            .iter()
            .map(|(export, imports)| {
                vec![
                    export.clone(),
                    imports.join(", "),
                ]
            })
            .collect()
    }
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

impl TableRow for MarketEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec![
            "Type",
            "Market",
            "Item",
            "Supply",
            "Purchase Cr",
            "Sell Cr",
            "Volume",
            "Description",
        ]
    }

    fn to_row(&self) -> Vec<String> {
        Vec::new()            // never used: multi-row printer below
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        let default = Vec::new();

        let market = &self.data.symbol;
        let mut rows = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();   // avoid duplicates
        let default_zero = String::from("N/A");

        /* ---------------- helper to build ONE row ---------------- */
        let mut make_row = |
                            kind: &str,
                            symbol: &str,
                            name: &str,
                            desc: &str,
                            tg: Option<&TradeGoodDTO>| 
            {
                let (supply, buy, sell, vol) = tg
                    .map(|g| (
                        g.supply.clone(),
                        g.purchase_price.to_string(),
                        g.sell_price.to_string(),
                        g.trade_volume.to_string(),
                    ))
                    .unwrap_or_else(|| ("—".into(), default_zero.clone(), default_zero.clone(), default_zero.clone()));

                vec![
                    kind.to_string(),             // EXPORT / IMPORT / EXCHANGE
                    market.clone(),
                    name.to_string(),
                    supply.clone(),
                    buy.clone(),
                    sell.clone(),
                    vol.clone(),
                    desc.to_string(),
                ]
            };

        /* ---------------- quick look-up tables ---------------- */
        let trade_map = self
            .data
            .trade_goods
            .as_ref()
            .unwrap_or(&default)
            .iter()
            .map(|tg| (tg.symbol.to_string(), tg))
            .collect::<std::collections::HashMap<_, _>>();

        /* ------- exports / imports / exchange (simple lists) ------- */
        let add_list = |
                        kind: &String,
                        list: &[GoodInfoDTO],
                        rows: &mut Vec<Vec<String>>,
                        seen: &mut HashSet<String>,
                        trade_map: HashMap<String, &TradeGoodDTO>| 
            {
                for g in list {
                    if seen.insert(g.symbol.to_string()) {
                        rows.push(make_row(
                            kind,
                            &g.symbol,
                            &g.name,
                            &g.description,
                            trade_map.get(&g.symbol.clone()).copied(),
                        ));
                    }
                }
            };

        add_list(&String::from("Export").clone(),   &self.data.exports, &mut rows, &mut seen, trade_map.clone());
        add_list(&String::from("Import").clone(),   &self.data.imports, &mut rows, &mut seen, trade_map.clone());
        add_list(&String::from("Exchange").clone(), &self.data.exchange, &mut rows, &mut seen, trade_map.clone());

        /* ------- any extra tradeGood not covered above -------- */
        if let Some(goods) = &self.data.trade_goods {
            for tg in goods {
                if seen.insert(tg.symbol.clone()) {
                    // No description available → placeholder
                    rows.push(make_row(
                        &tg.good_type,          // EXPORT / IMPORT / EXCHANGE
                        &tg.symbol,
                        &tg.symbol,             // fallback name = symbol
                        "—",
                        Some(tg),
                    ));
                }
            }
        }

        rows
    }
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

