use std::{fmt::DebugStruct, str::FromStr};
use strum_macros::{EnumIter};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};
use crate::{helpers::enum_lookups::MountSymbol, dto::responses::fleet_dto::{MountDTO, MountRequirementsDTO}};



#[derive(Debug, Clone, Serialize)]
pub struct MountRequirements {
    pub power: Option<i32>,
    pub crew: i32,
    pub slots: Option<i32>
}

impl MountRequirements {
    pub fn new(power: Option<i32>, crew: i32, slots: Option<i32>) -> anyhow::Result<Self> {
        Ok(Self {
            power,
            crew,
            slots
        })
    }
}

impl TryFrom<MountRequirementsDTO> for MountRequirements {
    type Error = anyhow::Error;

    fn try_from(dto: MountRequirementsDTO) -> anyhow::Result<Self> {
        MountRequirements::new(
            dto.power,
            dto.crew,
            dto.slots
        )
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Mount {
    pub mount_symbol: MountSymbol,
    pub name:   String,
    pub description: String,
    pub requirements: MountRequirements,
    pub strength: Option<u16>,
}

impl Mount {
    pub fn new<S1, S2>(
        mount_symbol: MountSymbol,
        name: S1,
        description: S2,
        requirements: MountRequirements,
        strength: Option<u16>,
    ) -> anyhow::Result<Self> 
    where 
        S1: Into<String>,
        S2: Into<String>
    {
        let name = name.into();
        let description = description.into();
        Ok(Self { mount_symbol, name, description, requirements, strength })
    }
}

impl TryFrom<MountDTO> for Mount {
    type Error = anyhow::Error;

    fn try_from(dto: MountDTO) -> anyhow::Result<Self> {
        Mount::new(
            MountSymbol::from_str(&dto.mount_symbol)?,
            dto.name,
            dto.description,
            dto.mount_requirements.try_into()?,
            dto.strength
        )
    }
}
