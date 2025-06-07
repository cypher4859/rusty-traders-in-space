use serde::{Deserialize, Serialize};
use crate::model::ship_model::{Ship};
use crate::dto::responses::fleet_dto::ShipDTO;
use crate::dto::responses::util_dto::MetaDTO;
use crate::AgentDTO;
use super::fleet_dto::{CooldownDTO, FuelDTO, ShipStatusEventDTO};
use super::supply_chain_dto::MarketTxDTO;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigateStatusDataEnvelopeDTO {
    pub data: NavDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigateOrbitDataEnvelopeDTO {
    pub data: NavigateOrbitEnvelopeDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigateOrbitEnvelopeDTO {
    pub nav: NavDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigateWaypointDataEnvelopeDTO {
    pub data: NavigateWaypointEnvelopeDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigateWaypointEnvelopeDTO {
    pub fuel: FuelDTO,
    pub nav: NavDTO,
    pub events: Option<ShipStatusEventDTO>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigateDockDataEnvelopeDTO {
    pub data: NavigateDockEnvelopeDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigateDockEnvelopeDTO {
    pub nav: NavDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigateWarpDataEnvelopeDTO {
    pub data: NavigateWarpEnvelopeDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigateWarpEnvelopeDTO {
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigateJumpDataEnvelopeDTO {
    pub data: NavigateJumpEnvelopeDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigateJumpEnvelopeDTO {
    pub nav: NavDTO,
    pub cooldown: CooldownDTO,
    pub transaction: MarketTxDTO,
    pub agent: AgentDTO
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


