use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};
use crate::InventoryItemDTO;


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


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegistrationDTO {
    pub name:          String,
    #[serde(rename = "factionSymbol")]
    pub faction_symbol: String,
    pub role:          String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavDTO { 
    #[serde(rename = "systemSymbol")]
    pub system_symbol:          String,
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol:        String,
    pub route:                  NavRouteDTO,
    pub status:                 String,
    #[serde(rename = "flightMode")]
    pub flight_mode:            String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavRouteDTO {
    pub destination:        NavRouteLocationDTO,
    pub origin:             NavRouteLocationDTO,
    #[serde(rename = "departureTime")]
    pub departure_time:     String,
    pub arrival:            String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavRouteLocationDTO {
    pub symbol:             String,
    #[serde(rename = "type")]
    pub destination_type:   String,
    #[serde(rename = "systemSymbol")]
    pub system_symbol:      String,
    #[serde(rename = "x")]
    pub location_coordinate_x: i32,
    #[serde(rename = "y")]
    pub location_coordinate_y: i32
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
    pub condition:      i16,
    pub integrity:      i16,
    pub description:    String,
    #[serde(rename = "moduleSlots")]
    pub module_slots:   u8,
    #[serde(rename = "mountingPoints")]
    pub mounting_points: u8,
    #[serde(rename = "fuelCapacity")]
    pub fuel_capacity: u16,
    #[serde(rename = "requirements")]
    pub frame_requirements: FrameRequirementsDTO,
    pub quality: u8
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
    pub capacity:       Option<u16>
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CargoCargoDataEnvelopeDTO {
    pub data: CargoCargoEnvelopeDTO
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CooldownDTO { 
    #[serde(rename = "shipSymbol")]
    pub ship_symbol:        String,
    #[serde(rename = "totalSeconds")]
    pub total_seconds:      u32,
    #[serde(rename = "remainingSeconds")]
    pub remaining_seconds:  u32
}
