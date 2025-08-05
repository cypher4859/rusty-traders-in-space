use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};
use crate::{dto::responses::supply_chain_dto::{MarketTxDTO, RepairTransactionDTO, TransactionDTO}, helpers::table_helpers::TableRow, AgentDTO, InventoryItemDTO};
use super::nav_dto::{NavDTO, NavRouteDTO, NavRouteLocationDTO};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipDataEnvelopeDTO {
    pub data: ShipDTO
}

impl TableRow for ShipDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Name", "Faction", "Role", "Status", "Mode", "Location", "Fuel", "Cargo", "Power", "Crew (required)", "Morale", "Wages"]
    }

    fn to_row(&self) -> Vec<String> {
        let current_power: i32 = self.data.frame.frame_requirements.power + 
                                self.data.reactor.reactor_requirements.power.as_ref().unwrap_or(&0) +
                                self.data.engine.engine_requirements.power +
                                self.data.modules.iter().map(|module| { module.module_requirements.power.as_ref().unwrap_or(&0) }).sum::<i32>() +
                                self.data.mounts.iter().map(|mount| { mount.mount_requirements.power.as_ref().unwrap_or(&0) }).sum::<i32>();
        vec![
            self.data.symbol.clone(),
            self.data.registration.faction_symbol.clone(),
            self.data.registration.role.clone(),
            self.data.nav.get_status(),
            self.data.nav.flight_mode.clone(),
            self.data.nav.waypoint_symbol.clone(),
            format!("{}/{}", self.data.fuel.current.to_string(), self.data.fuel.capacity.to_string()),
            format!("{}/{}", self.data.cargo.units.as_ref().unwrap_or(&0).to_string(), self.data.cargo.capacity.as_ref().unwrap_or(&0).to_string()),
            format!("{}/{}", current_power.to_string(), self.data.reactor.power_output.to_string()),
            format!("{}/{} ({})", self.data.crew.current.to_string(), self.data.crew.capacity.to_string(), self.data.crew.required.to_string()),
            self.data.crew.morale.to_string(),
            self.data.crew.wages.to_string()
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipDTO {
    pub symbol: String,
    pub registration: RegistrationDTO,
    pub nav: NavDTO,
    pub crew: CrewDTO,
    pub frame: FrameDTO,
    pub reactor: ReactorDTO,
    pub engine: EngineDTO,
    pub modules: Vec<ModuleDTO>,
    pub mounts:  Vec<MountDTO>,
    pub cargo:   CargoDTO,
    pub fuel:    FuelDTO,
    pub cooldown: CooldownDTO,
}

impl TableRow for ShipDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Name", "Faction", "Role", "Status", "Mode", "Location"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.symbol.clone(),
            self.registration.faction_symbol.clone(),
            self.registration.role.clone(),
            self.nav.get_status(),
            self.nav.flight_mode.clone(),
            self.nav.waypoint_symbol.clone()
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipStatusEventDTO {

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipRefuelDataEnvelopeDTO {
    pub data: ShipRefuelEnvelopeDTO
}

impl TableRow for ShipRefuelDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Agent", "Credits", "Price", "Fuel"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.data.agent.symbol.clone(),
            self.data.agent.credits.to_string(),
            self.data.transaction.total_price.to_string(),
            format!("{}/{}", self.data.fuel.current.to_string(), self.data.fuel.capacity.to_string())
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipRefuelEnvelopeDTO {
    pub agent: AgentDTO,
    pub fuel: FuelDTO,
    pub transaction: MarketTxDTO,
    pub cargo: CargoDTO
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegistrationDTO {
    pub name:          String,
    #[serde(rename = "factionSymbol")]
    pub faction_symbol: String,
    pub role:          String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrewDTO { 
    pub current:    u16,
    pub required:   u16,
    pub capacity:   u16,
    pub rotation:   String,
    pub morale:     i16,
    pub wages:      i64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrameDTO { 
    #[serde(rename = "symbol")]
    pub frame_symbol:         String,
    pub name:           String,
    pub condition:      u16,
    pub integrity:      u16,
    pub description:    String,
    #[serde(rename = "moduleSlots")]
    pub module_slots:   u16,
    #[serde(rename = "mountingPoints")]
    pub mounting_points: u16,
    #[serde(rename = "fuelCapacity")]
    pub fuel_capacity: u16,
    #[serde(rename = "requirements")]
    pub frame_requirements: FrameRequirementsDTO,
    pub quality: u16
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrameRequirementsDTO {
    pub power:      i32,
    pub crew:       i32,
    pub slots:      Option<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReactorDTO { 
    #[serde(rename = "symbol")]
    pub reactor_symbol:         String,
    pub name:           String,
    pub condition:      i16,
    pub integrity:      i16,
    pub description:    String,
    #[serde(rename = "powerOutput")]
    pub power_output:   u16,
    #[serde(rename = "requirements")]
    pub reactor_requirements: ReactorRequirementsDTO,
    pub quality:        u8
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReactorRequirementsDTO {
    pub power: Option<i32>,
    pub crew: i32,
    pub slots: Option<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EngineDTO { 
    #[serde(rename = "symbol")]
    pub engine_symbol:         String,
    pub name:           String,
    pub condition:      i16,
    pub integrity:      i16,
    pub description:    String,
    pub speed:          u16,
    #[serde(rename = "requirements")]
    pub engine_requirements: EngineRequirementsDTO,
    pub quality:        u8
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EngineRequirementsDTO {
    pub power:          i32,
    pub crew:           i32,
    pub slots:          Option<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleDTO { 
    #[serde(rename = "symbol")]
    pub module_symbol:  String,
    pub name:           String,
    pub description:    String,
    #[serde(rename = "requirements")]
    pub module_requirements: ModuleRequirementsDTO,
    pub capacity:       Option<u16>,
    pub range:          Option<u32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleRequirementsDTO {
    pub power:          Option<i32>,
    pub crew:           i32,
    pub slots:          Option<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MountDTO {
    #[serde(rename = "symbol")]
    pub mount_symbol:   String,
    pub name:           String,
    pub description:    String,
    #[serde(rename = "requirements")]
    pub mount_requirements: MountRequirementsDTO,
    pub strength:       Option<u16>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MountRequirementsDTO {
    pub power:          Option<i32>,
    pub crew:           i32,
    pub slots:          Option<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CargoDataEnvelopeDTO {
    pub data: CargoDTO
}

impl TableRow for CargoDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Item", "Current", "Max Capacity", "Description"]
    }

    fn to_row(&self) -> Vec<String> {
        let capacity = self.data.capacity.as_ref().unwrap_or_else(|| &0).to_string();

        // Safely grab the first inventory item, if any
        let (name, units, desc) = self
            .data
            .inventory                       // Option<Vec<_>>
            .as_ref()                        // &Option<…>
            .and_then(|v| v.first())         // Option<&InventoryItem>
            .map(|item| (
                item.name.clone(),
                item.units.to_string(),
                item.description.clone(),
            ))
            .unwrap_or_else(|| (            // fallback when None or empty
                "—".into(),                 // name placeholder
                "0".into(),                 // units
                "—".into(),                 // description
            ));

        vec![name, units, capacity, desc]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CargoCargoDataEnvelopeDTO {
    pub data: CargoCargoEnvelopeDTO
}

impl TableRow for CargoCargoDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Item", "Current", "Max Capacity", "Description"]
    }

    fn to_row(&self) -> Vec<String> {
        let capacity = self.data.cargo.capacity.as_ref().unwrap_or_else(|| &0).to_string();

        // Safely grab the first inventory item, if any
        let (name, units, desc) = self
            .data
            .cargo
            .inventory                       // Option<Vec<_>>
            .as_ref()                        // &Option<…>
            .and_then(|v| v.first())         // Option<&InventoryItem>
            .map(|item| (
                item.name.clone(),
                item.units.to_string(),
                item.description.clone(),
            ))
            .unwrap_or_else(|| (            // fallback when None or empty
                "—".into(),                 // name placeholder
                "0".into(),                 // units
                "—".into(),                 // description
            ));

        vec![name, units, capacity, desc]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CargoCargoEnvelopeDTO {
    pub cargo: CargoDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CargoDTO { 
    pub capacity:       Option<u32>,
    pub units:          Option<u32>,
    pub inventory:      Option<Vec<InventoryItemDTO>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuelDTO { 
    pub current:        u32,
    pub capacity:       u32,
    #[serde(rename = "consumed")]
    pub fuel_consumed:       Option<FuelConsumedDTO>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuelConsumedDTO {
    pub amount:     u32,
    pub timestamp:  String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CooldownEnvelopeDTO { 
    pub data: Option<CooldownDTO>
}


impl TableRow for CooldownEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Ship", "Remaining", "Total Time", "Expiration"]
    }

    fn to_row(&self) -> Vec<String> {
        
        vec![
            self.data.as_ref().unwrap().ship_symbol.clone(),
            self.data.as_ref().unwrap().remaining_seconds.to_string(),
            self.data.as_ref().unwrap().total_seconds.to_string(),
            self.data.as_ref().unwrap().expiration.clone().unwrap_or(String::from("None")).clone()
        ]
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CooldownDTO { 
    #[serde(rename = "shipSymbol")]
    pub ship_symbol:        String,
    #[serde(rename = "totalSeconds")]
    pub total_seconds:      u32,
    #[serde(rename = "remainingSeconds")]
    pub remaining_seconds:  u32,
    pub expiration:         Option<String>
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ModuleDataEnvelopeEnumDTO {
    Single(ModuleDataEnvelopeDTO),
    List(Vec<ModuleDataEnvelopeDTO>)
}

impl TableRow for ModuleDataEnvelopeEnumDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Name", "Power", "Crew", "Slots", "Max", "Description"]
    }

    fn to_row(&self) -> Vec<String> {
        Vec::new()
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        let mut rows: Vec<Vec<String>> = Vec::new();
        match &self {
            ModuleDataEnvelopeEnumDTO::Single(module) => {
                rows.extend(vec![vec![
                    module.data.name.clone(),
                    module.data.module_requirements.power.unwrap_or(0).to_string(),
                    module.data.module_requirements.crew.to_string(),
                    module.data.module_requirements.slots.unwrap_or(0).to_string(),
                    module.data.capacity.unwrap_or(0).to_string(),
                    module.data.description.clone()
                ]]);
                rows
            },
            ModuleDataEnvelopeEnumDTO::List(modules) => {
                // let mut rows = Vec::new();
                rows.extend(modules.iter().map(|modu| {
                    vec![
                        modu.data.name.clone(),
                        modu.data.module_requirements.power.unwrap_or(0).to_string(),
                        modu.data.module_requirements.crew.to_string(),
                        modu.data.module_requirements.slots.unwrap_or(0).to_string(),
                        modu.data.capacity.unwrap_or(0).to_string(),
                        modu.data.description.clone()
                    ]
                }));
                rows
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleDataEnvelopeDTO {
    pub data: ModuleDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleInstallDataEnvelopeDTO {
    pub data: ModuleInstallEnvelopeDTO
}

impl TableRow for ModuleInstallDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Name", "Power", "Crew", "Slots", "Max", "Description"]
    }

    fn to_row(&self) -> Vec<String> {
        Vec::new()
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        let mut rows: Vec<Vec<String>> = Vec::new();
        match &self.data.modules {
            ModuleDataEnvelopeEnumDTO::Single(module) => {
                rows.extend(vec![vec![
                    module.data.name.clone(),
                    module.data.module_requirements.power.unwrap_or(0).to_string(),
                    module.data.module_requirements.crew.to_string(),
                    module.data.module_requirements.slots.unwrap_or(0).to_string(),
                    module.data.capacity.unwrap_or(0).to_string(),
                    module.data.description.clone()
                ]]);
                rows
            },
            ModuleDataEnvelopeEnumDTO::List(modules) => {
                // let mut rows = Vec::new();
                rows.extend(modules.iter().map(|modu| {
                    vec![
                        modu.data.name.clone(),
                        modu.data.module_requirements.power.unwrap_or(0).to_string(),
                        modu.data.module_requirements.crew.to_string(),
                        modu.data.module_requirements.slots.unwrap_or(0).to_string(),
                        modu.data.capacity.unwrap_or(0).to_string(),
                        modu.data.description.clone()
                    ]
                }));
                rows
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleInstallEnvelopeDTO {
    pub agent: AgentDTO,
    pub modules: ModuleDataEnvelopeEnumDTO,
    pub cargo: CargoDTO,
    pub transaction: TransactionDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleRemoveDataEnvelopeDTO {
    pub data: ModuleRemoveEnvelopeDTO
}

impl TableRow for ModuleRemoveDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Name", "Power", "Crew", "Slots", "Max", "Description"]
    }

    fn to_row(&self) -> Vec<String> {
        Vec::new()
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        let mut rows: Vec<Vec<String>> = Vec::new();
        match &self.data.modules {
            ModuleDataEnvelopeEnumDTO::Single(module) => {
                rows.extend(vec![vec![
                    module.data.name.clone(),
                    module.data.module_requirements.power.unwrap_or(0).to_string(),
                    module.data.module_requirements.crew.to_string(),
                    module.data.module_requirements.slots.unwrap_or(0).to_string(),
                    module.data.capacity.unwrap_or(0).to_string(),
                    module.data.description.clone()
                ]]);
                rows
            },
            ModuleDataEnvelopeEnumDTO::List(modules) => {
                // let mut rows = Vec::new();
                rows.extend(modules.iter().map(|modu| {
                    vec![
                        modu.data.name.clone(),
                        modu.data.module_requirements.power.unwrap_or(0).to_string(),
                        modu.data.module_requirements.crew.to_string(),
                        modu.data.module_requirements.slots.unwrap_or(0).to_string(),
                        modu.data.capacity.unwrap_or(0).to_string(),
                        modu.data.description.clone()
                    ]
                }));
                rows
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleRemoveEnvelopeDTO {
    pub agent: AgentDTO,
    pub modules: ModuleDataEnvelopeEnumDTO,
    pub cargo: CargoDTO,
    pub transaction: TransactionDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MountDataEnvelopeEnumDTO {
    Single(MountDataEnvelopeDTO),
    List(Vec<MountDataEnvelopeDTO>)
}

impl TableRow for MountDataEnvelopeEnumDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Name", "Power", "Crew", "Slots", "Max", "Description"]
    }

    fn to_row(&self) -> Vec<String> {
        Vec::new()
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        let mut rows: Vec<Vec<String>> = Vec::new();
        match &self {
            MountDataEnvelopeEnumDTO::Single(module) => {
                rows.extend(vec![vec![
                    module.data.name.clone(),
                    module.data.mount_requirements.power.unwrap_or(0).to_string(),
                    module.data.mount_requirements.crew.to_string(),
                    module.data.mount_requirements.slots.unwrap_or(0).to_string(),
                    module.data.strength.unwrap_or(0).to_string(),
                    module.data.description.clone()
                ]]);
                rows
            },
            MountDataEnvelopeEnumDTO::List(modules) => {
                // let mut rows = Vec::new();
                rows.extend(modules.iter().map(|module| {
                    vec![
                        module.data.name.clone(),
                        module.data.mount_requirements.power.unwrap_or(0).to_string(),
                        module.data.mount_requirements.crew.to_string(),
                        module.data.mount_requirements.slots.unwrap_or(0).to_string(),
                        module.data.strength.unwrap_or(0).to_string(),
                        module.data.description.clone()
                    ]
                }));
                rows
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MountDataEnvelopeDTO {
    pub data: MountDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MountInstallDataEnvelopeDTO {
    pub data: MountInstallEnvelopeDTO
}

impl TableRow for MountInstallDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Name", "Power", "Crew", "Slots", "Max", "Description"]
    }

    fn to_row(&self) -> Vec<String> {
        Vec::new()
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        let mut rows: Vec<Vec<String>> = Vec::new();
        match &self.data.mounts {
            MountDataEnvelopeEnumDTO::Single(mount) => {
                rows.extend(vec![vec![
                    mount.data.name.clone(),
                    mount.data.mount_requirements.power.unwrap_or(0).to_string(),
                    mount.data.mount_requirements.crew.to_string(),
                    mount.data.mount_requirements.slots.unwrap_or(0).to_string(),
                    mount.data.strength.unwrap_or(0).to_string(),
                    mount.data.description.clone()
                ]]);
                rows
            },
            MountDataEnvelopeEnumDTO::List(mounts) => {
                // let mut rows = Vec::new();
                rows.extend(mounts.iter().map(|mount| {
                    vec![
                        mount.data.name.clone(),
                        mount.data.mount_requirements.power.unwrap_or(0).to_string(),
                        mount.data.mount_requirements.crew.to_string(),
                        mount.data.mount_requirements.slots.unwrap_or(0).to_string(),
                        mount.data.strength.unwrap_or(0).to_string(),
                        mount.data.description.clone()
                    ]
                }));
                rows
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MountInstallEnvelopeDTO {
    pub agent: AgentDTO,
    pub mounts: MountDataEnvelopeEnumDTO,
    pub cargo: CargoDTO,
    pub transaction: TransactionDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MountRemoveDataEnvelopeDTO {
    pub data: MountRemoveEnvelopeDTO
}

impl TableRow for MountRemoveDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Name", "Power", "Crew", "Slots", "Max", "Description"]
    }

    fn to_row(&self) -> Vec<String> {
        Vec::new()
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        let mut rows: Vec<Vec<String>> = Vec::new();
        match &self.data.mounts {
            MountDataEnvelopeEnumDTO::Single(mount) => {
                rows.extend(vec![vec![
                    mount.data.name.clone(),
                    mount.data.mount_requirements.power.unwrap_or(0).to_string(),
                    mount.data.mount_requirements.crew.to_string(),
                    mount.data.mount_requirements.slots.unwrap_or(0).to_string(),
                    mount.data.strength.unwrap_or(0).to_string(),
                    mount.data.description.clone()
                ]]);
                rows
            },
            MountDataEnvelopeEnumDTO::List(mounts) => {
                // let mut rows = Vec::new();
                rows.extend(mounts.iter().map(|mount| {
                    vec![
                        mount.data.name.clone(),
                        mount.data.mount_requirements.power.unwrap_or(0).to_string(),
                        mount.data.mount_requirements.crew.to_string(),
                        mount.data.mount_requirements.slots.unwrap_or(0).to_string(),
                        mount.data.strength.unwrap_or(0).to_string(),
                        mount.data.description.clone()
                    ]
                }));
                rows
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MountRemoveEnvelopeDTO {
    pub agent: AgentDTO,
    pub mounts: MountDataEnvelopeEnumDTO,
    pub cargo: CargoDTO,
    pub transaction: TransactionDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipRepairStatusDataEnvelopeDTO {
    pub data: ShipRepairStatusEnvelopeDTO
}

impl TableRow for ShipRepairStatusDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Ship", "Waypoint", "Total Price", "Timestamp"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.data.transaction.ship_symbol.clone(),
            self.data.transaction.waypoint_symbol.clone(),
            self.data.transaction.total_price.to_string(),
            self.data.transaction.timestamp.clone()
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipRepairStatusEnvelopeDTO {
    pub transaction: RepairTransactionDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipRepairDataEnvelopeDTO {
    pub data: ShipRepairEnvelopeDTO
}

impl TableRow for ShipRepairDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Purhcase", "Ship", "Waypoint", "Total Price", "Timestamp"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            String::from("Repair"),
            self.data.transaction.ship_symbol.clone(),
            self.data.transaction.waypoint_symbol.clone(),
            self.data.transaction.total_price.to_string(),
            self.data.transaction.timestamp.clone()
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipRepairEnvelopeDTO {
    pub agent: AgentDTO,
    pub ship: ShipDTO,
    pub transaction: RepairTransactionDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipScrapStatusDataEnvelopeDTO {
    pub data: ShipScrapStatusEnvelopeDTO
}

impl TableRow for ShipScrapStatusDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Purchase", "Ship", "Waypoint", "Total Price", "Timestamp"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            String::from("Scrap"),
            self.data.transaction.ship_symbol.clone(),
            self.data.transaction.waypoint_symbol.clone(),
            self.data.transaction.total_price.to_string(),
            self.data.transaction.timestamp.clone()
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipScrapStatusEnvelopeDTO {
    pub agent: AgentDTO,
    pub transaction: RepairTransactionDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipScrapDataEnvelopeDTO {
    pub data: ShipScrapEnvelopeDTO
}

impl TableRow for ShipScrapDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Purchase", "Ship", "Waypoint", "Total Price", "Timestamp"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            String::from("Scrap"),
            self.data.transaction.ship_symbol.clone(),
            self.data.transaction.waypoint_symbol.clone(),
            self.data.transaction.total_price.to_string(),
            self.data.transaction.timestamp.clone()
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipScrapEnvelopeDTO {
    pub transaction: RepairTransactionDTO
}
