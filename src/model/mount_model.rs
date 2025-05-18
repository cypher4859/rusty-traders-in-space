use std::{fmt::DebugStruct, str::FromStr};
use strum_macros::{EnumIter};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};
use crate::dto::ship_dto::{MountDTO, MountRequirementsDTO};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MountSymbol {
    GasSiphonI,
    GasSiphonII,
    GasSiphonIII,
    SurveyorI,
    SurveyorII,
    SurveyorIII,
    SensorArrayI,
    SensorArrayII,
    SensorArrayIII,
    MiningLaserI,
    MiningLaserII,
    MiningLaserIII,
    LaserCannonI,
    MissileLauncherI,
    TurretI,
}

// impl fmt::Display for MountSymbol {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", self)   // SCREAMING_SNAKE style via Debug
//     }
// }

impl FromStr for MountSymbol {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "MOUNT_GAS_SIPHON_I"       => Ok(Self::GasSiphonI),
            "MOUNT_GAS_SIPHON_II"      => Ok(Self::GasSiphonII),
            "MOUNT_GAS_SIPHON_III"     => Ok(Self::GasSiphonIII),
            "MOUNT_SURVEYOR_I"         => Ok(Self::SurveyorI),
            "MOUNT_SURVEYOR_II"        => Ok(Self::SurveyorII),
            "MOUNT_SURVEYOR_III"       => Ok(Self::SurveyorIII),
            "MOUNT_SENSOR_ARRAY_I"     => Ok(Self::SensorArrayI),
            "MOUNT_SENSOR_ARRAY_II"    => Ok(Self::SensorArrayII),
            "MOUNT_SENSOR_ARRAY_III"   => Ok(Self::SensorArrayIII),
            "MOUNT_MINING_LASER_I"     => Ok(Self::MiningLaserI),
            "MOUNT_MINING_LASER_II"    => Ok(Self::MiningLaserII),
            "MOUNT_MINING_LASER_III"   => Ok(Self::MiningLaserIII),
            "MOUNT_LASER_CANNON_I"     => Ok(Self::LaserCannonI),
            "MOUNT_MISSILE_LAUNCHER_I" => Ok(Self::MissileLauncherI),
            "MOUNT_TURRET_I"           => Ok(Self::TurretI),
            other => Err(anyhow!("Unknown mount symbol '{other}'")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MountRequirements {
    pub power: i32,
    pub crew: i32,
    pub slots: i32
}

impl MountRequirements {
    pub fn new(power: i32, crew: i32, slots: i32) -> anyhow::Result<Self> {
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

#[derive(Debug, Clone)]
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
