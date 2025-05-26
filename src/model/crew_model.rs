use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};
use crate::dto::responses::fleet_dto::{CrewDTO};

#[derive(Debug, Clone)]
pub struct Crew {
    pub current: u16,
    pub required: u16,
    pub capacity: u16,
    pub rotation: String,
    pub morale: i16,
    pub wages: i64,
}

impl Crew {
    pub fn new<S1>(
        current: u16,
        required: u16,
        capacity: u16,
        rotation: S1,
        morale: i16,
        wages: i64,
    ) -> anyhow::Result<Self> 
    where
        S1: Into<String>
    {
        let rotation = rotation.into();
        ensure!(current <= capacity, "current crew exceeds capacity");
        ensure!(required <= capacity, "required crew exceeds capacity");
        Ok(Self { current, required, capacity, rotation, morale, wages })
    }
}

impl TryFrom<CrewDTO> for Crew {
    type Error = anyhow::Error;

    fn try_from(dto: CrewDTO) -> anyhow::Result<Self> {
        Crew::new(
            dto.current,
            dto.required,
            dto.capacity,
            dto.rotation,
            dto.morale,
            dto.wages,
        )
    }
}