use serde::{Deserialize, Serialize};

use super::fleet_dto::{FrameDTO, ReactorDTO, EngineDTO, ModuleDTO, MountDTO, CrewDTO};


#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SystemDataDTO {
    Single(SystemEnvelopeDTO),
    List(SystemListEnvelopeDTO),
}

/// ------------  top-level envelope ------------
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemListEnvelopeDTO {
    pub data: Vec<SystemDTO>,
    pub meta: MetaDTO,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemEnvelopeDTO {
    pub data: SystemDTO,
}

/// ------------  meta pagination block ------------
#[derive(Debug, Serialize, Deserialize)]
pub struct MetaDTO {
    pub total: u32,
    pub page:  u32,
    pub limit: u32,
}

/// ------------  system (“sector/constellation”) ------------
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemDTO {
    pub symbol:         String,
    #[serde(rename = "sectorSymbol")]
    pub sector_symbol:  String,
    /// constellation identifier, string in payload
    pub constellation:  String,
    pub name:           String,
    #[serde(rename = "type")]
    pub system_type:    String,   // could be enum later
    pub x:              i32,
    pub y:              i32,
    pub waypoints:      Vec<WaypointBasicDTO>,
    pub factions:       Vec<FactionTagDTO>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaypointListEnvelopeDTO {
    pub data: Vec<WaypointDTO>,
    pub meta: MetaDTO,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaypointEnvelopeDTO {
    pub data: WaypointDTO,
}

/// ------------  waypoint inside a system ------------
#[derive(Debug, Serialize, Deserialize)]
pub struct WaypointBasicDTO {
    pub symbol: String,
    #[serde(rename = "type")]
    pub waypoint_type: String,    // PLANET, MOON, …
    pub x: i32,
    pub y: i32,
    pub orbitals: Vec<OrbitalDTO>,
    /// The parent object this waypoint orbits (may be null / absent)
    pub orbits: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaypointDTO {
    pub symbol: String,

    #[serde(rename = "type")]
    pub waypoint_type: String,            // e.g. "PLANET"

    #[serde(rename = "systemSymbol")]
    pub system_symbol: String,

    pub x: i32,
    pub y: i32,

    pub orbitals: Vec<OrbitalDTO>,

    /// Parent body this one orbits (may be null / missing)
    pub orbits: Option<String>,

    pub faction: Option<FactionTagDTO>,

    pub traits: Vec<TraitDTO>,

    pub modifiers: Vec<ModifierDTO>,

    pub chart: Option<ChartDTO>,

    #[serde(rename = "isUnderConstruction")]
    pub is_under_construction: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrbitalDTO {
    pub symbol: String,
}

/// ------------  faction tag inside a system ------------
#[derive(Debug, Serialize, Deserialize)]
pub struct FactionTagDTO {
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChartDTO {
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,
    #[serde(rename = "submittedBy")]
    pub submitted_by: Option<String>,
    #[serde(rename = "submittedOn")]
    pub submitted_on: Option<String>,     // ISO-8601; switch to DateTime if desired
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraitDTO {
    pub symbol: String,
    pub name:   String,
    pub description: String,
}

/// ---------------- waypoint modifier ----------------
#[derive(Debug, Serialize, Deserialize)]
pub struct ModifierDTO {
    pub symbol: String,
    pub name:   String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketEnvelopeDTO {
    pub data: MarketDTO,
}

/// ---------- market body ----------
#[derive(Debug, Serialize, Deserialize)]
pub struct MarketDTO {
    pub symbol: String,

    pub exports:      Vec<GoodInfoDTO>,
    pub imports:      Vec<GoodInfoDTO>,
    pub exchange:     Vec<GoodInfoDTO>,

    pub transactions: Vec<MarketTxDTO>,
    #[serde(rename = "tradeGoods")]
    pub trade_goods:  Vec<TradeGoodDTO>,
}

/// ---------- simple good descriptor (export / import / exchange) ----------
#[derive(Debug, Serialize, Deserialize)]
pub struct GoodInfoDTO {
    pub symbol: String,
    pub name:   String,
    pub description: String,
}

/// ---------- transaction record ----------
#[derive(Debug, Serialize, Deserialize)]
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

/// ---------- live market good snapshot ----------
#[derive(Debug, Serialize, Deserialize)]
pub struct TradeGoodDTO {
    pub symbol: String,

    #[serde(rename = "type")]
    pub good_type: String,        // EXPORT | IMPORT | EXCHANGE

    #[serde(rename = "tradeVolume")]
    pub trade_volume: u32,

    pub supply:   String,         // SCARCE | MODERATE | …
    pub activity: String,         // WEAK | STRONG | …

    #[serde(rename = "purchasePrice")]
    pub purchase_price: u32,
    #[serde(rename = "sellPrice")]
    pub sell_price: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipyardEnvelopeDTO {
    pub data: ShipyardDTO,
}

/// ------------- root shipyard object -------------
#[derive(Debug, Serialize, Deserialize)]
pub struct ShipyardDTO {
    pub symbol: String,

    #[serde(rename = "shipTypes")]
    pub ship_types: Vec<ShipTypeTagDTO>,

    pub transactions: Vec<ShipyardTxDTO>,

    pub ships: Vec<ShipOfferDTO>,

    #[serde(rename = "modificationsFee")]
    pub modifications_fee: u32,
}

/// ------------- simple `{ "type": "SHIP_PROBE" }` tag -------------
#[derive(Debug, Serialize, Deserialize)]
pub struct ShipTypeTagDTO {
    #[serde(rename = "type")]
    pub ship_type: String,
}

/// ------------- purchase / sell transaction record -------------
#[derive(Debug, Serialize, Deserialize)]
pub struct ShipyardTxDTO {
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,
    #[serde(rename = "shipType")]
    pub ship_type: String,

    pub price: u32,

    #[serde(rename = "agentSymbol")]
    pub agent_symbol: String,

    pub timestamp: String,                       // ISO-8601; switch to DateTime later
}

/// ------------- ship blueprint currently for sale -------------
#[derive(Debug, Serialize, Deserialize)]
pub struct ShipOfferDTO {
    #[serde(rename = "type")]
    pub ship_type: String,
    pub name:        String,
    pub description: String,

    pub supply:   String,                        // e.g. SCARCE
    pub activity: String,                        // e.g. WEAK

    #[serde(rename = "purchasePrice")]
    pub purchase_price: u32,

    pub frame:   FrameDTO,
    pub reactor: ReactorDTO,
    pub engine:  EngineDTO,
    pub modules: Vec<ModuleDTO>,
    pub mounts:  Vec<MountDTO>,
    pub crew:    CrewDTO,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JumpGateEnvelopeDTO {
    pub data: JumpGateDTO
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JumpGateDTO {
    pub symbol: String,
    pub connections: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConstructionSiteEnvelopeDTO {
    pub data:ConstructionSiteDTO
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConstructionSiteDTO {
    pub symbol: String,
    pub materials: Vec<ConstructionSiteMaterialsDTO>,
    #[serde(rename = "isComplete")]
    pub is_complete: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConstructionSiteMaterialsDTO {
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    pub required_units: i32,
    pub fulfilled: i32
}
