use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};
use crate::dto::ship_dto::{FuelDTO};


#[derive(Debug, Clone)]
pub struct Fuel {
    pub current: u32,
    pub capacity: u32,
    pub consumed_amount: Option<u32>,
    pub consumed_timestamp: Option<String>,
}

impl Fuel {
    pub fn new<S1>(
        current: u32, 
        capacity: u32, 
        consumed_amount: Option<u32>, 
        consumed_timestamp: Option<S1>
    ) -> anyhow::Result<Self> 
    where 
        S1: Into<String>
    {
        ensure!(current <= capacity, "fuel exceeds capacity");
        let consumed_timestamp: Option<String> = consumed_timestamp.map(Into::into);
        let consumed_amount = consumed_amount.map(|s|s);
        Ok(Self { 
            current, 
            capacity, 
            consumed_amount, 
            consumed_timestamp 
        })
    }
}

impl TryFrom<FuelDTO> for Fuel {
    type Error = anyhow::Error;

    fn try_from(dto: FuelDTO) -> anyhow::Result<Self> {
        let (consumed_amount, consumed_timestamp) = match dto.fuel_consumed {
            Some(fc) => (Some(fc.amount), Some(fc.timestamp)),
            None     => (None,            None),
        };
        Fuel::new(
            dto.current,
            dto.capacity,
            consumed_amount,
            consumed_timestamp,
        )
    }
}