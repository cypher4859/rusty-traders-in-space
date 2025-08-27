use std::{fmt::DebugStruct, str::FromStr};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};
use strum_macros::{EnumIter, EnumString};
use crate::{helpers::enum_lookups::WaypointType, dto::responses::nav_dto::{NavDTO, NavRouteDTO, NavRouteLocationDTO}};

use super::{ShipStatus, ShipFlightMode};

#[derive(Debug, Clone, Serialize)]
pub struct Nav {
    pub system_symbol: String,
    pub waypoint_symbol: String,
    pub route: NavRoute,
    pub status: ShipStatus,
    pub flight_mode: ShipFlightMode,
}

impl Nav {
    pub fn new<S1, S2>(
        system_symbol: S1,
        waypoint_symbol: S2,
        route: NavRoute,
        status: ShipStatus,
        flight_mode: ShipFlightMode
    ) -> anyhow::Result<Self>
    where 
        S1: Into<String>,
        S2: Into<String>
    {
        let system_symbol = system_symbol.into();
        let waypoint_symbol = waypoint_symbol.into();
        Ok(Self {
            system_symbol,
            waypoint_symbol,
            route,
            status,
            flight_mode
        })
    }
}

impl TryFrom<NavDTO> for Nav {
    type Error = anyhow::Error;

    fn try_from(dto: NavDTO) -> anyhow::Result<Self> {
        Nav::new(
            dto.system_symbol,
            dto.waypoint_symbol,
            dto.route.try_into()?,
            ShipStatus::from_str(&dto.status.to_string())?,
            ShipFlightMode::from_str(&dto.flight_mode)?
        )
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct NavRoute {
    pub destination: NavRouteLocation,
    pub origin: NavRouteLocation,
    pub departure_time: String,
    pub arrival: String,
}

impl NavRoute {
    pub fn new<S1, S2>(
        destination: NavRouteLocation,
        origin: NavRouteLocation,
        departure_time: S1,
        arrival: S2,
    ) -> anyhow::Result<Self> 
    where 
        S1: Into<String>,
        S2: Into<String>
    {
        let departure_time = departure_time.into();
        let arrival = arrival.into();
        Ok(Self {
            destination,
            origin,
            departure_time,
            arrival,
        })
    }
}

impl TryFrom<NavRouteDTO> for NavRoute {
    type Error = anyhow::Error;

    fn try_from(dto: NavRouteDTO) -> anyhow::Result<Self> {
        let arrival: String = dto.arrival.try_into()?;
        let departure_time: String = dto.departure_time.try_into()?;

        NavRoute::new(dto.destination.try_into()?,
                      dto.origin.try_into()?,
                      departure_time,
                      arrival
                    )
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct NavRouteLocation {
    pub waypoint_symbol: String, //There are a shitload of these so not putting it into an enum
    pub waypoint_type: WaypointType,
    pub system_symbol: String,
    pub x: i32,
    pub y: i32,
}

impl NavRouteLocation {
    pub fn new<S1, S2>(
        waypoint_symbol: S1,
        waypoint_type: WaypointType,
        system_symbol: S2,
        x: i32,
        y: i32,
    ) -> anyhow::Result<Self> 
    where
        S1: Into<String>,
        S2: Into<String>
    {
        let waypoint_symbol = waypoint_symbol.into();
        let system_symbol = system_symbol.into();
        Ok(Self { 
            waypoint_symbol, 
            waypoint_type, 
            system_symbol, 
            x, 
            y 
        })
    }
}

impl TryFrom<NavRouteLocationDTO> for NavRouteLocation {
    type Error = anyhow::Error;

    fn try_from(dto: NavRouteLocationDTO) -> anyhow::Result<Self> {
        NavRouteLocation::new(
            dto.symbol,
            WaypointType::from_str(&dto.destination_type)?,
            dto.system_symbol,
            dto.location_coordinate_x,
            dto.location_coordinate_y,
        )
    }
}


