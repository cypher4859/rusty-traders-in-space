use std::{fmt::DebugStruct, str::FromStr};
use strum_macros::{EnumIter, EnumString};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};
use crate::dto::ship_dto::{ReactorDTO, ReactorRequirementsDTO};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash,
    Serialize, Deserialize,
    EnumIter,            // ReactorSymbol::iter()
    EnumString           // "REACTOR_SOLAR_I".parse::<ReactorSymbol>()
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE", ascii_case_insensitive)]
pub enum ReactorSymbol {
    SolarI,        // "REACTOR_SOLAR_I"
    FusionI,       // "REACTOR_FUSION_I"
    FissionI,      // "REACTOR_FISSION_I"
    ChemicalI,     // "REACTOR_CHEMICAL_I"
    AntimatterI,   // "REACTOR_ANTIMATTER_I"
}

#[derive(Debug, Clone)]
pub struct Reactor {
    pub reactor_symbol: ReactorSymbol,
    pub name:   String,
    pub condition: i16,
    pub integrity: i16,
    pub description: String,
    pub power_output: u16,
    pub requirements: ReactorRequirements,
    pub quality: u8,
}

impl Reactor {
    pub fn new<S1, S2>(
        reactor_symbol: ReactorSymbol,
        name: S1,
        condition: i16,
        integrity: i16,
        description: S2,
        power_output: u16,
        requirements: ReactorRequirements,
        quality: u8,
    ) -> anyhow::Result<Self> 
    where
        S1: Into<String>,
        S2: Into<String>
    {
        let name = name.into();
        let description = description.into();
        Ok(Self { reactor_symbol, name, condition, integrity, description, power_output, requirements, quality })
    }
}

impl TryFrom<ReactorDTO> for Reactor {
    type Error = anyhow::Error;

    fn try_from(dto: ReactorDTO) -> anyhow::Result<Self> {
        Reactor::new(
            ReactorSymbol::from_str(&dto.reactor_symbol)?,
            dto.name,
            dto.condition,
            dto.integrity,
            dto.description,
            dto.power_output,
            dto.reactor_requirements.try_into()?,
            dto.quality as u8,
        )
    }
}

#[derive(Debug, Clone)]
pub struct ReactorRequirements {
    pub power: Option<i32>,
    pub crew: i32,
    pub slots: Option<i32>
}

impl ReactorRequirements {
    pub fn new(power: Option<i32>, crew: i32, slots: Option<i32>) -> anyhow::Result<Self> {
        Ok(Self {
            power,
            crew,
            slots
        })
    }
}

impl TryFrom<ReactorRequirementsDTO> for ReactorRequirements {
    type Error= anyhow::Error;

    fn try_from(dto: ReactorRequirementsDTO) ->anyhow::Result<Self> {
        ReactorRequirements::new(
            dto.power,
            dto.crew,
            dto.slots
        )
    }
}