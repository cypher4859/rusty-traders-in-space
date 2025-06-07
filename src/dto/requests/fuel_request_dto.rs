use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestRefuelShipDTO {
    #[serde(rename = "units")]
    pub units_of_fuel: Option<u32>,
    /// When set to false it will attempt to buy the fuel from Market or not
    #[serde(rename = "fromCargo")]
    pub use_fuel_from_cargo: bool
}

impl RequestRefuelShipDTO {
    pub fn new(
        use_fuel_from_cargo: bool,
        units_of_fuel: Option<u32>
    ) -> anyhow::Result<Self>
    {
        Ok(Self
            {
                use_fuel_from_cargo,
                units_of_fuel
            }
        )
    }
}