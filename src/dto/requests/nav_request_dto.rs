use serde::{Deserialize, Serialize};
use crate::model::ship_model::{Ship};
use crate::dto::responses::fleet_dto::ShipDTO;
use crate::dto::responses::util_dto::MetaDTO;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestWarpToWaypointDTO {
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String
}

impl RequestWarpToWaypointDTO {
    pub fn new<S1>(
        waypoint_symbol: S1,
    ) -> anyhow::Result<Self>
    where
        S1: Into<String>,
    {
        let waypoint_symbol: String       = waypoint_symbol.into();

        Ok(Self
            {
                waypoint_symbol,
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestJumpToWaypointDTO {
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String
}

impl RequestJumpToWaypointDTO {
    pub fn new<S1>(
        waypoint_symbol: S1,
    ) -> anyhow::Result<Self>
    where
        S1: Into<String>,
    {
        let waypoint_symbol: String       = waypoint_symbol.into();

        Ok(Self
            {
                waypoint_symbol,
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestNavigateToWaypointDTO {
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String
}

impl RequestNavigateToWaypointDTO {
    pub fn new<S1>(
        waypoint_symbol: S1,
    ) -> anyhow::Result<Self>
    where
        S1: Into<String>,
    {
        let waypoint_symbol: String       = waypoint_symbol.into();

        Ok(Self
            {
                waypoint_symbol,
            }
        )
    }
}