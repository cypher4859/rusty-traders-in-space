use std::str::FromStr;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Duration, TimeDelta, Utc};
use tracing::warn;
use crate::helpers::enum_lookups::NavStatus;
use crate::helpers::table_helpers::TableRow;
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

impl TableRow for NavigateStatusDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Waypoint", "Status", "Mode", "Origin", "Destination", "Arrival"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.data.waypoint_symbol.clone(),
            self.data.get_status(),
            self.data.flight_mode.clone(),
            self.data.route.origin.symbol.clone(),
            self.data.route.destination.symbol.clone(),
            self.data.route.arrival.clone(),
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigateOrbitDataEnvelopeDTO {
    pub data: NavigateOrbitEnvelopeDTO
}

impl TableRow for NavigateOrbitDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Waypoint", "Status", "Mode", "Origin", "Destination", "Departure", "Arrival"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.data.nav.waypoint_symbol.clone(),
            self.data.nav.get_status(),
            self.data.nav.flight_mode.clone(),
            self.data.nav.route.origin.symbol.clone(),
            self.data.nav.route.destination.symbol.clone(),
            self.data.nav.route.departure_time.clone(),
            self.data.nav.route.arrival.clone()
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigateOrbitEnvelopeDTO {
    pub nav: NavDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigateWaypointDataEnvelopeDTO {
    pub data: NavigateWaypointEnvelopeDTO
}

impl TableRow for NavigateWaypointDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Waypoint", "Status", "Mode", "Origin", "Destination", "Arrival", "Fuel Used", "Fuel Remaining"]
    }

    fn to_row(&self) -> Vec<String> {
        let consumed_amount = self.data.fuel.fuel_consumed.as_ref().unwrap().amount.to_string();
        vec![
            self.data.nav.waypoint_symbol.clone(),
            self.data.nav.get_status(),
            self.data.nav.flight_mode.clone(),
            self.data.nav.route.origin.symbol.clone(),
            self.data.nav.route.destination.symbol.clone(),
            self.data.nav.route.arrival.clone(),
            consumed_amount.clone(),
            format!("{}/{}",self.data.fuel.current.to_string(), self.data.fuel.capacity.to_string())
        ]
    }
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

impl TableRow for NavigateDockDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Waypoint", "Status", "Mode", "Origin", "Destination", "Arrival"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.data.nav.waypoint_symbol.clone(),
            self.data.nav.get_status(),
            self.data.nav.flight_mode.clone(),
            self.data.nav.route.origin.symbol.clone(),
            self.data.nav.route.destination.symbol.clone(),
            self.data.nav.route.arrival.clone(),
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigateDockEnvelopeDTO {
    pub nav: NavDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigateWarpDataEnvelopeDTO {
    pub data: NavigateWarpEnvelopeDTO
}

impl TableRow for NavigateWarpDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Waypoint", "Status", "Mode", "Origin", "Destination", "Arrival", "Fuel"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.data.nav.waypoint_symbol.clone(),
            self.data.nav.get_status(),
            self.data.nav.flight_mode.clone(),
            self.data.nav.route.origin.symbol.clone(),
            self.data.nav.route.destination.symbol.clone(),
            self.data.nav.route.arrival.clone(),
            format!("{}/{}", self.data.fuel.current.to_string(), self.data.fuel.capacity.to_string())
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigateWarpEnvelopeDTO {
    pub fuel: FuelDTO,
    pub nav: NavDTO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigateJumpDataEnvelopeDTO {
    pub data: NavigateJumpEnvelopeDTO
}

impl TableRow for NavigateJumpDataEnvelopeDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Waypoint", "Status", "Mode", "Origin", "Destination", "Arrival", "Price", "Credits", "Cooldown"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.data.nav.waypoint_symbol.clone(),
            self.data.nav.get_status(),
            self.data.nav.flight_mode.clone(),
            self.data.nav.route.origin.symbol.clone(),
            self.data.nav.route.destination.symbol.clone(),
            self.data.nav.route.arrival.clone(),
            self.data.transaction.total_price.to_string(),
            self.data.agent.credits.to_string(),
            self.data.cooldown.remaining_seconds.to_string()
        ]
    }
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
    pub status:                 NavStatus,
    #[serde(rename = "flightMode")]
    pub flight_mode:            String
}

impl NavDTO {
    pub fn get_status(&self) -> String {
        let flight_time = self._remaining_from_now(&self.route.arrival);
        let flight_duration = match flight_time {
            Ok(s) => s,
            Err(e) => {
                warn!("nav status error for {}: {e}", self.waypoint_symbol);
                TimeDelta::new(0, 0).unwrap()
            }
        };
        format!("{} (ETA {})", self.status.to_string(), self.human(&flight_duration))
    }

    fn _remaining_until_arrival(&self, departure: &String, arrival: &String) -> anyhow::Result<Duration> {
        // RFC-3339 strings like "2025-06-20T16:14:30.753Z"
        let dep: DateTime<Utc> = DateTime::from_str(departure)?;
        let arr: DateTime<Utc> = DateTime::from_str(arrival)?;
    
        Ok(arr - dep)                 // chrono::Duration
    }
    
    fn _remaining_from_now(&self, arrival: &String) -> anyhow::Result<Duration> {
        let arr: DateTime<Utc> = DateTime::from_str(arrival)?;
        Ok(arr - Utc::now())
    }

    /// Quick helper to turn a chrono::Duration into "1h 14m 30s"
    fn human(&self, d: &Duration) -> String {

        let (neg, mut secs) = if d.num_seconds() < 0 {
            return "00h 00m 00s".into();
        } else {
            (false, d.num_seconds())
        };

        let h = secs / 3600;
        secs -= h * 3600;
        let m = secs / 60;
        secs -= m * 60;

        format!(
            "{}{:02}h {:02}m {:02}s",
            if neg { "-" } else { "" },
            h, m, secs
        )
    }
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


