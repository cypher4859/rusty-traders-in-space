use serde::{Deserialize, Serialize};

use crate::{dto::responses::fleet_dto::CargoDTO, helpers::table_helpers::TableRow, AgentDTO};

use super::fleet_dto::{FrameDTO, ReactorDTO, EngineDTO, ModuleDTO, MountDTO, CrewDTO};


#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SystemDataDTO {
    Single(SystemEnvelopeDTO),
    List(SystemListEnvelopeDTO),
}

/// ------------  top-level envelope ------------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemListEnvelopeDTO {
    pub data: Vec<SystemDTO>,
    pub meta: MetaDTO,
}

impl TableRow for SystemListEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["System", "Type", "Waypoints", "Constellation"]
    }

    fn to_row(&self) -> Vec<String> {
        Vec::new()
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        self.data
            .iter()
            .map(|system| {
                vec![
                    system.name.clone(),
                    system.system_type.clone(),
                    system.waypoints.iter().count().to_string(),
                    system.constellation.clone()
                ]
            }).collect()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemEnvelopeDTO {
    pub data: SystemDTO,
    // pub meta: MetaDTO,
}

impl TableRow for SystemEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["System", "Type", "Waypoint", "Waypoint Type", "# Orbitals"]
    }

    fn to_row(&self) -> Vec<String> {
        Vec::new()
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        let name = self.data.name.clone();
        let system_type = self.data.system_type.clone();
        self.data.waypoints
            .iter()
            .map(|waypoint| {
                vec![
                    name.clone(),
                    system_type.clone(),
                    waypoint.symbol.clone(),
                    waypoint.waypoint_type.clone(),
                    waypoint.orbitals.iter().count().to_string(),
                ]
            }).collect()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemSupplyConstructionDataEnvelopeDTO {
    pub data: SystemSupplyConstructionEnvelopeDTO,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemSupplyConstructionEnvelopeDTO {
    pub construction: ConstructionSiteDTO,
    pub cargo: CargoDTO
}



/// ------------  meta pagination block ------------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaDTO {
    pub total: u32,
    pub page:  u32,
    pub limit: u32,
}

/// ------------  system (“sector/constellation”) ------------
#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WaypointListEnvelopeDTO {
    pub data: Vec<WaypointDTO>,
    pub meta: MetaDTO,
}

impl TableRow for WaypointListEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Waypoint", "Waypoint Type", "Orbitals", "Traits", "Modifiers", "Chart", "Under Construction"]
    }

    fn to_row(&self) -> Vec<String> {
        Vec::new()
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        self.data
            .iter()
            .map(|wp| {
                let orbitals = wp
                    .orbitals
                    .iter()
                    .map(|o| o.symbol.clone())
                    .collect::<Vec<String>>()
                    .join(", ");

                let traits = wp
                    .traits
                    .iter()
                    .map(|t| t.symbol.clone())
                    .collect::<Vec<String>>()
                    .join(", ");

                let modifiers = wp
                    .modifiers
                    .iter()
                    .map(|m| m.symbol.clone())
                    .collect::<Vec<String>>()
                    .join(", ");

                let charted_by = wp
                    .chart
                    .as_ref()
                    .and_then(|c| c.submitted_by.clone())
                    .unwrap_or_else(|| "—".into());

                vec![
                    wp.symbol.clone(),
                    wp.waypoint_type.clone(),
                    orbitals,
                    traits,
                    modifiers,
                    charted_by,
                    wp.is_under_construction.to_string(),
                ]
            })
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WaypointEnvelopeDTO {
    pub data: WaypointDTO,
}

impl TableRow for WaypointEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec![
            "Waypoint",
            "Type",
            "Orbitals",
            "Traits",
            "Modifiers",
            "Charted By",
            "Under Construction",
        ]
    }

    fn to_row(&self) -> Vec<String> {
        let wp = &self.data;

        /* ---------- comma-joined helpers ---------- */
        let orbitals = wp
            .orbitals
            .iter()
            .map(|o| o.symbol.clone())
            .collect::<Vec<String>>()
            .join(", ");

        let traits = wp
            .traits
            .iter()
            .map(|t| t.symbol.clone())
            .collect::<Vec<String>>()
            .join(", ");

        let modifiers = wp
            .modifiers
            .iter()
            .map(|m| m.symbol.clone())
            .collect::<Vec<String>>()
            .join(", ");

        let charted_by = wp
            .chart
            .as_ref()
            .and_then(|c| c.submitted_by.clone())
            .unwrap_or_else(|| "—".into());

        /* ---------- one complete row ---------- */
        vec![
            wp.symbol.clone(),
            wp.waypoint_type.clone(),
            orbitals,
            traits,
            modifiers,
            charted_by,
            wp.is_under_construction.to_string(),
        ]
    }

    // fn to_rows(&self) -> Vec<Vec<String>>  ←  NOT NEEDED
    // trait default is already: vec![self.to_row()]
}

/// ------------  waypoint inside a system ------------
#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrbitalDTO {
    pub symbol: String,
}

/// ------------  faction tag inside a system ------------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FactionTagDTO {
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChartDataEnvelopeDTO {
    pub data: ChartEnvelopeDTO
}

impl TableRow for ChartDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Waypoint Charted", "By", "Date", "Faction", "Traits", "Mods"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.data.chart.waypoint_symbol.clone(),
            self.data.chart.submitted_by.as_ref().unwrap_or(&String::from("None")).clone(),
            self.data.chart.submitted_on.as_ref().unwrap_or(&String::from("None")).clone(),
            self.data.waypoint.faction.as_ref().unwrap().symbol.clone(),
            self.data.waypoint.traits.iter().map(|item| item.name.clone()).collect(),
            self.data.waypoint.modifiers.iter().map(|item| item.name.clone()).collect()
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChartEnvelopeDTO {
    pub chart: ChartDTO,
    pub waypoint: WaypointDTO,
    pub agent: AgentDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChartDTO {
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,
    #[serde(rename = "submittedBy")]
    pub submitted_by: Option<String>,
    #[serde(rename = "submittedOn")]
    pub submitted_on: Option<String>,     // ISO-8601; switch to DateTime if desired
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraitDTO {
    pub symbol: String,
    pub name:   String,
    pub description: String,
}

/// ---------------- waypoint modifier ----------------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModifierDTO {
    pub symbol: String,
    pub name:   String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipyardEnvelopeDTO {
    pub data: ShipyardDTO,
}

impl TableRow for ShipyardEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec![
            "Waypoint Charted",
            "By",
            "Date",
            "Faction",
            "Traits",
            "Mods",
        ]
    }

    fn to_row(&self) -> Vec<String> {
        let yard   = &self.data;

        /* --- most-recent transaction, if any --- */
        let (by, date) = yard
            .transactions
            .last()                          // newest because API is append-only
            .map(|tx| (tx.agent_symbol.clone(), tx.timestamp.clone()))
            .unwrap_or_else(|| ("—".into(), "—".into()));

        /* --- all supported ship types in one cell --- */
        let traits = yard
            .ship_types
            .iter()
            .map(|t| t.ship_type.clone())
            .collect::<Vec<String>>()
            .join(", ");

        vec![
            yard.symbol.clone(),              // Waypoint
            by,                               // By
            date,                             // Date
            "—".into(),                       // Faction (not in payload)
            traits,                           // Traits
            yard.modifications_fee.to_string(), // Mods
        ]
    }
}


/// ------------- root shipyard object -------------
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipTypeTagDTO {
    #[serde(rename = "type")]
    pub ship_type: String,
}

/// ------------- purchase / sell transaction record -------------
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JumpGateEnvelopeDTO {
    pub data: JumpGateDTO
}

impl TableRow for JumpGateEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Jump-Gate", "Connected To"]
    }

    fn to_row(&self) -> Vec<String> {
        Vec::new()
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        let gate = &self.data.symbol;

        if self.data.connections.is_empty() {
            return vec![vec![gate.clone(), "—".into()]];
        }

        self.data
            .connections
            .iter()
            .map(|dest| vec![gate.clone(), dest.clone()])
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JumpGateDTO {
    pub symbol: String,
    pub connections: Vec<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstructionSiteEnvelopeDTO {
    pub data:ConstructionSiteDTO
}

impl TableRow for ConstructionSiteEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec![
            "Site",
            "Material",
            "Required",
            "Fulfilled",
            "Remaining",
            "Complete?",
        ]
    }

    fn to_row(&self) -> Vec<String> {
        Vec::new()
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        let site   = &self.data.symbol;
        let status = self.data.is_complete.to_string();

        if self.data.materials.is_empty() {
            return vec![vec![site.clone(), "—".into(), "0".into(), "0".into(), "0".into(), status]];
        }

        self.data
            .materials
            .iter()
            .map(|m| {
                let remaining = m.required_units - m.fulfilled;
                vec![
                    site.clone(),
                    m.trade_symbol.clone(),
                    m.required_units.to_string(),
                    m.fulfilled.to_string(),
                    remaining.to_string(),
                    status.clone(),
                ]
            })
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstructionSiteDTO {
    pub symbol: String,
    pub materials: Vec<ConstructionSiteMaterialsDTO>,
    #[serde(rename = "isComplete")]
    pub is_complete: bool
}

impl TableRow for ConstructionSiteDTO {
    fn headers() -> Vec<&'static str> {
        vec![
            "Site",
            "Material",
            "Required",
            "Fulfilled",
            "Remaining",
            "Complete?",
        ]
    }

    /*  not used, because we override `to_rows`   */
    fn to_row(&self) -> Vec<String> {
        Vec::new()
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        let status   = self.is_complete.to_string();
        let site_sym = &self.symbol;

        if self.materials.is_empty() {
            // show the site even when it lists no materials
            return vec![vec![
                site_sym.clone(),
                "—".into(),
                "0".into(),
                "0".into(),
                "0".into(),
                status,
            ]];
        }

        self.materials
            .iter()
            .map(|m| {
                let remaining = m.required_units - m.fulfilled;
                vec![
                    site_sym.clone(),                 // Site
                    m.trade_symbol.clone(),           // Material
                    m.required_units.to_string(),     // Required
                    m.fulfilled.to_string(),          // Fulfilled
                    remaining.to_string(),            // Remaining
                    status.clone(),                   // Complete?
                ]
            })
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstructionSiteMaterialsDTO {
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    pub required_units: i32,
    pub fulfilled: i32
}


