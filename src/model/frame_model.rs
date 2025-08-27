use std::{fmt::DebugStruct, str::FromStr};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString};
use anyhow::{Result, anyhow, ensure};
use crate::{helpers::enum_lookups::FrameSymbol, dto::responses::fleet_dto::{FrameDTO, FrameRequirementsDTO}};


#[derive(Debug, Clone, Serialize)]
pub struct FrameRequirements {
    pub power: i32,
    pub crew: i32,
    pub slots: Option<i32>
}

impl FrameRequirements {
    pub fn new(power: i32, crew: i32, slots: Option<i32>) -> anyhow::Result<Self> {
        Ok(Self {
            power,
            crew,
            slots
        })
    }
}

impl TryFrom<FrameRequirementsDTO> for FrameRequirements {
    type Error = anyhow::Error;

    fn try_from(dto: FrameRequirementsDTO) -> anyhow::Result<Self> {
        FrameRequirements::new(
            dto.power,
            dto.crew,
            dto.slots
        )
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Frame {
    pub frame_symbol: FrameSymbol,
    pub name:   String,
    pub condition: u16,
    pub integrity: u16,
    pub description: String,
    pub module_slots: u16,
    pub mounting_points: u16,
    pub fuel_capacity: u16,
    pub requirements: FrameRequirements,
    pub quality: u16,
}

impl Frame {
    pub fn new<S1, S2>(
        frame_symbol: FrameSymbol,
        name: S1,
        condition: u16,
        integrity: u16,
        description: S2,
        module_slots: u16,
        mounting_points: u16,
        fuel_capacity: u16,
        requirements: FrameRequirements,
        quality: u16,
    ) -> anyhow::Result<Self> 
    where 
        S1: Into<String>,
        S2: Into<String>
    {
        let name = name.into();
        let description = description.into();
        Ok(Self {
            frame_symbol,
            name,
            condition,
            integrity,
            description,
            module_slots,
            mounting_points,
            fuel_capacity,
            requirements,
            quality,
        })
    }
}

impl TryFrom<FrameDTO> for Frame {
    type Error = anyhow::Error;

    fn try_from(dto: FrameDTO) -> anyhow::Result<Self> {
        let frame = FrameSymbol::from_str(&dto.frame_symbol)?;
        
        Frame::new(
            frame,
            dto.name,
            dto.condition,
            dto.integrity,
            dto.description,
            dto.module_slots,
            dto.mounting_points,
            dto.fuel_capacity,
            dto.frame_requirements.try_into()?,
            dto.quality,
        )
    }
}