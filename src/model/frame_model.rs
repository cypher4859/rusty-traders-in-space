use std::{fmt::DebugStruct, str::FromStr};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString};
use anyhow::{Result, anyhow, ensure};
use crate::dto::ship_dto::{FrameDTO, FrameRequirementsDTO};



#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash,
    Serialize, Deserialize,
    EnumIter,            // → FrameSymbol::iter()
    EnumString           // → FromStr + parse::<FrameSymbol>()
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE", ascii_case_insensitive)]
pub enum FrameSymbol {
    Probe,
    Drone,
    Interceptor,
    Racer,
    Fighter,
    Frigate,
    Shuttle,
    Explorer,
    Miner,
    LightFreighter,
    HeavyFreighter,
    Transport,
    Destroyer,
    Cruiser,
    Carrier,
    BulkFreighter,
}

#[derive(Debug, Clone)]
pub struct FrameRequirements {
    pub power: i32,
    pub crew: i32,
    pub slots: i32
}

impl FrameRequirements {
    pub fn new(power: i32, crew: i32, slots: i32) -> anyhow::Result<Self> {
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

#[derive(Debug, Clone)]
pub struct Frame {
    pub frame_symbol: FrameSymbol,
    pub name:   String,
    pub condition: i16,
    pub integrity: i16,
    pub description: String,
    pub module_slots: u8,
    pub mounting_points: u8,
    pub fuel_capacity: u16,
    pub requirements: FrameRequirements,
    pub quality: u8,
}

impl Frame {
    pub fn new<S1, S2>(
        frame_symbol: FrameSymbol,
        name: S1,
        condition: i16,
        integrity: i16,
        description: S2,
        module_slots: u8,
        mounting_points: u8,
        fuel_capacity: u16,
        requirements: FrameRequirements,
        quality: u8,
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
        Frame::new(
            FrameSymbol::from_str(&dto.frame_symbol)?,
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