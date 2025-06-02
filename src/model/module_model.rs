use std::{fmt::DebugStruct, str::FromStr};
use strum_macros::{EnumIter};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};
use crate::{constants::enum_lookups::ModuleSymbol, dto::responses::fleet_dto::{ModuleDTO, ModuleRequirementsDTO}};



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