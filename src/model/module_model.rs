use std::{fmt::DebugStruct, str::FromStr};
use strum_macros::{EnumIter};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};
use crate::dto::responses::fleet_dto::{ModuleDTO, ModuleRequirementsDTO};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModuleSymbol {
    MineralProcessorI,
    GasProcessorI,
    CargoHoldI,
    CargoHoldII,
    CargoHoldIII,
    CrewQuartersI,
    EnvoyQuartersI,
    PassengerCabinI,
    MicroRefineryI,
    OreRefineryI,
    FuelRefineryI,
    ScienceLabI,
    JumpDriveI,
    JumpDriveII,
    JumpDriveIII,
    WarpDriveI,
    WarpDriveII,
    WarpDriveIII,
    ShieldGeneratorI,
    ShieldGeneratorII,
}

impl FromStr for ModuleSymbol {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "MODULE_MINERAL_PROCESSOR_I" => Ok(Self::MineralProcessorI),
            "MODULE_GAS_PROCESSOR_I"     => Ok(Self::GasProcessorI),
            "MODULE_CARGO_HOLD_I"        => Ok(Self::CargoHoldI),
            "MODULE_CARGO_HOLD_II"       => Ok(Self::CargoHoldII),
            "MODULE_CARGO_HOLD_III"      => Ok(Self::CargoHoldIII),
            "MODULE_CREW_QUARTERS_I"     => Ok(Self::CrewQuartersI),
            "MODULE_ENVOY_QUARTERS_I"    => Ok(Self::EnvoyQuartersI),
            "MODULE_PASSENGER_CABIN_I"   => Ok(Self::PassengerCabinI),
            "MODULE_MICRO_REFINERY_I"    => Ok(Self::MicroRefineryI),
            "MODULE_ORE_REFINERY_I"      => Ok(Self::OreRefineryI),
            "MODULE_FUEL_REFINERY_I"     => Ok(Self::FuelRefineryI),
            "MODULE_SCIENCE_LAB_I"       => Ok(Self::ScienceLabI),
            "MODULE_JUMP_DRIVE_I"        => Ok(Self::JumpDriveI),
            "MODULE_JUMP_DRIVE_II"       => Ok(Self::JumpDriveII),
            "MODULE_JUMP_DRIVE_III"      => Ok(Self::JumpDriveIII),
            "MODULE_WARP_DRIVE_I"        => Ok(Self::WarpDriveI),
            "MODULE_WARP_DRIVE_II"       => Ok(Self::WarpDriveII),
            "MODULE_WARP_DRIVE_III"      => Ok(Self::WarpDriveIII),
            "MODULE_SHIELD_GENERATOR_I"  => Ok(Self::ShieldGeneratorI),
            "MODULE_SHIELD_GENERATOR_II" => Ok(Self::ShieldGeneratorII),
            other => Err(anyhow!("Unknown module symbol '{other}'")),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ModuleRequirements {
    pub power: Option<i32>,
    pub crew: i32,
    pub slots: Option<i32>
}

impl ModuleRequirements {
    pub fn new(power: Option<i32>, crew: i32, slots: Option<i32>) -> anyhow::Result<Self> {
        Ok(Self {
            power,
            crew,
            slots
        })
    }
}

impl TryFrom<ModuleRequirementsDTO> for ModuleRequirements {
    type Error = anyhow::Error;

    fn try_from(dto: ModuleRequirementsDTO) -> anyhow::Result<Self> {
        ModuleRequirements::new(
            dto.power,
            dto.crew,
            dto.slots
        )
    }
}


#[derive(Debug, Clone)]
pub struct Module {
    pub module_symbol: ModuleSymbol,
    pub name:   String,
    pub description: String,
    pub requirements: ModuleRequirements,
    pub capacity: Option<u16>,
}

impl Module {
    pub fn new<S1, S2>(
        module_symbol: ModuleSymbol,
        name: S1,
        description: S2,
        requirements: ModuleRequirements,
        capacity: Option<u16>,
    ) -> anyhow::Result<Self> 
    where
        S1: Into<String>,
        S2: Into<String>
    {
        let name = name.into();
        let description = description.into();
        Ok(Self { module_symbol, name, description, requirements, capacity })
    }
}

impl TryFrom<ModuleDTO> for Module {
    type Error = anyhow::Error;

    fn try_from(dto: ModuleDTO) -> anyhow::Result<Self> {
        Module::new(
            ModuleSymbol::from_str(&dto.module_symbol)?,
            dto.name,
            dto.description,
            dto.module_requirements.try_into()?,
            dto.capacity.map(|c| c as u16),
        )
    }
}