use std::{fmt::DebugStruct, str::FromStr};
use strum_macros::{EnumIter};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};
use crate::dto::responses::fleet_dto::{EngineDTO, EngineRequirementsDTO};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EngineSymbol {
    EngineImpulseDriveI,
    EngineIonDriveI,
    EngineIonDriveII,
    EngineHyperDriveI
}

impl FromStr for EngineSymbol {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "ENGINE_IMPULSE_DRIVE_I"    => Ok(EngineSymbol::EngineImpulseDriveI),
            "ENGINE_ION_DRIVE_I"        => Ok(EngineSymbol::EngineIonDriveI),
            "ENGINE_ION_DRIVE_II"       => Ok(EngineSymbol::EngineIonDriveII),
            "ENGINE_HYPER_DRIVE_I"      => Ok(EngineSymbol::EngineHyperDriveI),
            other                 => Err(anyhow::anyhow!("Unknown Engine '{other}'")),
        }
    }
}


#[derive(Debug, Clone)]
pub struct EngineRequirements {
    pub power: i32,
    pub crew: i32,
    pub slots: Option<i32>
}

impl EngineRequirements {
    pub fn new(power: i32, crew: i32, slots: Option<i32>) -> anyhow::Result<Self> {
        Ok(Self {
            power,
            crew,
            slots
        })
    }
}

impl TryFrom<EngineRequirementsDTO> for EngineRequirements {
    type Error= anyhow::Error;

    fn try_from(dto: EngineRequirementsDTO) ->anyhow::Result<Self> {
        EngineRequirements::new(
            dto.power,
            dto.crew,
            dto.slots
        )
    }
}

#[derive(Debug, Clone)]
pub struct Engine {
    pub engine_symbol: EngineSymbol,
    pub name:   String,
    pub condition: i16,
    pub integrity: i16,
    pub description: String,
    pub speed: u16,
    pub requirements: EngineRequirements,
    pub quality: u8,
}

impl Engine {
    pub fn new<S1, S2>(
        engine_symbol: EngineSymbol,
        name: S1,
        condition: i16,
        integrity: i16,
        description: S2,
        speed: u16,
        requirements: EngineRequirements,
        quality: u8,
    ) -> anyhow::Result<Self> 
    where
        S1: Into<String>,
        S2: Into<String>
    {
        let name = name.into();
        let description = description.into();
        Ok(Self { engine_symbol, name, condition, integrity, description, speed, requirements, quality })
    }
}

impl TryFrom<EngineDTO> for Engine {
    type Error = anyhow::Error;

    fn try_from(dto: EngineDTO) -> anyhow::Result<Self> {
        Engine::new(
            EngineSymbol::from_str(&dto.engine_symbol)?,
            dto.name,
            dto.condition,
            dto.integrity,
            dto.description,
            dto.speed,
            dto.engine_requirements.try_into()?,
            dto.quality,
        )
    }
}
