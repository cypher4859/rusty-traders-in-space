use std::{fmt::DebugStruct, str::FromStr};
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumIter, EnumString};
use crate::helpers::enum_lookups::FactionSymbol;
use crate::dto::responses::inventory_dto::{InventoryItemDTO};
use crate::dto::responses::fleet_dto::{CrewDTO, EngineDTO, FrameDTO, ReactorDTO, RegistrationDTO, ShipDTO, ModuleDTO, ModuleRequirementsDTO, MountDTO, MountRequirementsDTO, CooldownDTO, FuelDTO, CargoDTO};
use crate::dto::responses::nav_dto::{NavRouteLocationDTO};
use crate::helpers::table_helpers::TableRow;
use anyhow::{Result, ensure};
use std::convert::TryFrom;

use super::{
    Nav, 
    Engine, 
    Frame, 
    Reactor,
    Crew,
    Module,
    Mount,
    Cargo,
    Fuel,
    Cooldown
};


#[derive(Debug, Clone, Serialize)]
pub struct Ship {
    pub symbol: String,
    pub registration: Registration,
    pub nav: Nav,
    pub crew: Crew,
    pub frame: Frame,
    pub reactor: Reactor,
    pub engine: Engine,
    pub modules: Vec<Module>,
    pub mounts:  Vec<Mount>,
    pub cargo:   Cargo,
    pub fuel:    Fuel,
    pub cooldown: Cooldown,
}

impl Ship {
    pub fn new(
        symbol: String,
        registration: Registration,
        nav: Nav,
        crew: Crew,
        frame: Frame,
        reactor: Reactor,
        engine: Engine,
        modules: Vec<Module>,
        mounts: Vec<Mount>,
        cargo: Cargo,
        fuel: Fuel,
        cooldown: Cooldown,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            symbol,
            registration,
            nav,
            crew,
            frame,
            reactor,
            engine,
            modules,
            mounts,
            cargo,
            fuel,
            cooldown,
        })
    }
}

impl TryFrom<ShipDTO> for Ship {
    type Error = anyhow::Error;

    fn try_from(dto: ShipDTO) -> anyhow::Result<Self> {
        let modules = dto
                                        .modules
                                        .into_iter()
                                        .map(Module::try_from)
                                        .collect::<anyhow::Result<Vec<_>>>()?;
        let mounts = dto
                                .mounts
                                .into_iter()
                                .map(Mount::try_from)
                                .collect::<anyhow::Result<Vec<_>>>()?;
        Ship::new(
            dto.symbol.try_into()?,
            dto.registration.try_into()?,
            dto.nav.try_into()?,
            dto.crew.try_into()?,
            dto.frame.try_into()?,
            dto.reactor.try_into()?,
            dto.engine.try_into()?,
            modules,
            mounts,
            dto.cargo.try_into()?,
            dto.fuel.try_into()?,
            dto.cooldown.try_into()?
        )
    }
}

impl TableRow for Ship {
    fn headers() -> Vec<&'static str> {
        vec!["Names", "Faction", "Location", "Status", "Mode", "Role", "Crew", "Fuel", "Cargo", "CoolDown"]
    }

    fn to_row(&self) -> Vec<String> {
        let cargo_amount = match self.cargo.units {
            Some(u) => { u.to_string()},
            None => String::from("0")
        };
        vec![
            self.symbol.clone(),
            self.registration.faction_symbol.to_string(),
            self.nav.waypoint_symbol.clone(),
            self.nav.status.to_string(),
            self.nav.flight_mode.to_string(),
            self.registration.role.clone(),
            format!("{}/{}", self.crew.current.to_string(), self.crew.capacity.to_string()),
            format!("{}/{}", self.fuel.current.to_string(), self.fuel.capacity.to_string()),
            cargo_amount,
            self.cooldown.remaining_seconds.to_string()
        ]
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Registration {
    pub name: String,
    pub faction_symbol: FactionSymbol,
    pub role: String,
}

impl Registration {
    pub fn new<S1, S2>(
        name: S1,
        faction_symbol: FactionSymbol,
        role: S2
    ) -> anyhow::Result<Self> where
        S1: Into<String>,
        S2: Into<String>,
    {
        let name: String = name.into();
        let role: String = role.into();

        Ok(Self {
            name,
            faction_symbol,
            role
        })
    }
}

impl TryFrom<RegistrationDTO> for Registration {
    type Error = anyhow::Error;

    fn try_from(dto: RegistrationDTO) -> anyhow::Result<Self> {
        Registration::new(
            dto.name,
            FactionSymbol::from_str(&dto.faction_symbol)?,
            dto.role
        )
    }
}


#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash,
    Serialize, Deserialize,           // JSON ↔ enum
    EnumIter,                         // ShipStatus::iter()
    EnumString,                        // "IN_TRANSIT".parse::<ShipStatus>(),
    AsRefStr,
    Display
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE", ascii_case_insensitive)]
pub enum ShipStatus {
    InTransit,
    InOrbit,
    Docked,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash,
    Serialize, Deserialize,           // JSON ↔ enum
    EnumIter,                         // ShipStatus::iter()
    EnumString,                        // "IN_TRANSIT".parse::<ShipStatus>()
    AsRefStr,
    Display
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE", ascii_case_insensitive)]
pub enum ShipFlightMode {
    Drift,
    Stealth,
    Cruise,
    Burn
}

