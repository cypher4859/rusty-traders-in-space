use std::{fmt::DebugStruct, str::FromStr, vec};
use anyhow::{Result, anyhow, ensure};
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumIter, EnumString};
use crate::helpers::table_helpers::TableRow;


// pub trait TableRow {
//     fn headers() -> Vec<&'static str>;
//     fn to_row(&self) -> Vec<String>;
//     fn to_rows(&self) -> Vec<Vec<String>> {
//         vec![self.to_row()]
//     }
// }


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EngineSymbol {
    EngineImpulseDriveI,
    EngineIonDriveI,
    EngineIonDriveII,
    EngineHyperDriveI
}

impl FromStr for EngineSymbol {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "ENGINE_IMPULSE_DRIVE_I"    => Ok(EngineSymbol::EngineImpulseDriveI),
            "ENGINE_ION_DRIVE_I"        => Ok(EngineSymbol::EngineIonDriveI),
            "ENGINE_ION_DRIVE_II"       => Ok(EngineSymbol::EngineIonDriveII),
            "ENGINE_HYPER_DRIVE_I"      => Ok(EngineSymbol::EngineHyperDriveI),
            other                 => Err(anyhow::anyhow!("Unknown Engine '{other}'")),
        }
    }
}

impl TableRow for EngineSymbol {
    fn headers() -> Vec<&'static str> {
        vec!["Symbol", "Pretty"]
    }

    fn to_row(&self) -> Vec<String> {
        let sym = self.to_string();

        let pretty = sym
            .split("_")
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
                    None => String::new()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
    
        vec![sym, pretty]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TraitSymbol {
    Bureaucratic,
    Secretive,
    Capitalistic,
    Industrious,
    Peaceful,
    Distrustful,
    Welcoming,
    Smugglers,
    Scavengers,
    Rebellious,
    Exiles,
    Pirates,
    Raiders,
    Clan,
    Guild,
    Dominion,
    Fringe,
    Forsaken,
    Isolated,
    Localized,
    Established,
    Notable,
    Dominant,
    Inescapable,
    Innovative,
    Bold,
    Visionary,
    Curious,
    Daring,
    Exploratory,
    Resourceful,
    Flexible,
    Cooperative,
    United,
    Strategic,
    Intelligent,
    ResearchFocused,
    Collaborative,
    Progressive,
    Militaristic,
    TechnologicallyAdvanced,
    Aggressive,
    Imperialistic,
    TreasureHunters,
    Dexterous,
    Unpredictable,
    Brutal,
    Fleeting,
    Adaptable,
    SelfSufficient,
    Defensive,
    Proud,
    Diverse,
    Independent,
    SelfInterested,
    Fragmented,
    Commercial,
    FreeMarkets,
    Entrepreneurial
}

impl TableRow for TraitSymbol {
    fn headers() -> Vec<&'static str> {
        vec!["Symbol", "Pretty"]
    }

    fn to_row(&self) -> Vec<String> {
        let sym = self.to_string();

        let pretty = sym
            .split("_")
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
                    None => String::new()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
    
        vec![sym, pretty]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, Display, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FactionSymbol {
    Cosmic,
    Void,
    Galactic,
    Quantum,
    Dominion,
    Astro,
    Corsairs,
    Obsidian,
    Aegis,
    United,
    Solitary,
    Cobalt,
    Omega,
    Echo,
    Lords,
    Cult,
    Ancients,
    Shadow,
    Ethereal,
}

impl FromStr for FactionSymbol {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Case‑insensitive match so "cosmic", "COSMIC", "Cosmic" all work.
        match s.to_ascii_uppercase().as_str() {
            "COSMIC"   => Ok(FactionSymbol::Cosmic),
            "VOID"     => Ok(FactionSymbol::Void),
            "GALACTIC" => Ok(FactionSymbol::Galactic),
            "QUANTUM"  => Ok(FactionSymbol::Quantum),
            "DOMINION" => Ok(FactionSymbol::Dominion),
            "ASTRO"    => Ok(FactionSymbol::Astro),
            "CORSAIRS" => Ok(FactionSymbol::Corsairs),
            "OBSIDIAN" => Ok(FactionSymbol::Obsidian),
            "AEGIS"    => Ok(FactionSymbol::Aegis),
            "UNITED"   => Ok(FactionSymbol::United),
            "SOLITARY" => Ok(FactionSymbol::Solitary),
            "COBALT"   => Ok(FactionSymbol::Cobalt),
            "OMEGA"    => Ok(FactionSymbol::Omega),
            "ECHO"     => Ok(FactionSymbol::Echo),
            "LORDS"    => Ok(FactionSymbol::Lords),
            "CULT"     => Ok(FactionSymbol::Cult),
            "ANCIENTS" => Ok(FactionSymbol::Ancients),
            "SHADOW"   => Ok(FactionSymbol::Shadow),
            "ETHEREAL" => Ok(FactionSymbol::Ethereal),
            other => Err(anyhow::anyhow!("Unknown faction '{other}'")),
        }
    }
}

impl TableRow for FactionSymbol {
    fn headers() -> Vec<&'static str> {
        vec!["Symbol", "Pretty"]
    }

    fn to_row(&self) -> Vec<String> {
        let sym = self.to_string();

        let pretty = sym
            .split("_")
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
                    None => String::new()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
    
        vec![sym, pretty]
    }
}


impl FromStr for TraitSymbol {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "BUREAUCRATIC"   => Ok(TraitSymbol::Bureaucratic),
            other => Err(anyhow::anyhow!("Unknown faction '{other}'")),
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash,
    Serialize, Deserialize,
    EnumIter,
    EnumString,
    Display
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE", ascii_case_insensitive)]
pub enum FrameSymbol {
    #[strum(serialize = "FRAME_PROBE")]
    Probe,
    Drone,
    Interceptor,
    Racer,
    Fighter,
    #[strum(serialize = "FRAME_FRIGATE")]
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

impl TableRow for FrameSymbol {
    fn headers() -> Vec<&'static str> {
        vec!["Symbol", "Pretty"]
    }

    fn to_row(&self) -> Vec<String> {
        let sym = self.to_string();

        let pretty = sym
            .split("_")
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
                    None => String::new()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
    
        vec![sym, pretty]
    }
}


#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash,
    Serialize, Deserialize,
    EnumIter,
    EnumString,                    // <── derives `FromStr`
    Display
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE", ascii_case_insensitive)]
pub enum InventoryItemSymbol {
    /* -------- trade goods -------- */
    PreciousStones,
    QuartzSand,
    SiliconCrystals,
    AmmoniaIce,
    LiquidHydrogen,
    LiquidNitrogen,
    IceWater,
    ExoticMatter,
    AdvancedCircuitry,
    GravitonEmitters,
    /* -------- raw + refined ores -------- */
    Iron,
    IronOre,
    Copper,
    CopperOre,
    Aluminum,
    AluminumOre,
    Silver,
    SilverOre,
    Gold,
    GoldOre,
    Platinum,
    PlatinumOre,
    Diamonds,
    Uranite,
    UraniteOre,
    Meritium,
    MeritiumOre,
    Hydrocarbon,
    Antimatter,
    /* -------- commodities / consumables -------- */
    FabMats,
    Fertilizers,
    Fabrics,
    Food,
    Jewelry,
    Machinery,
    Firearms,
    AssaultRifles,
    MilitaryEquipment,
    Explosives,
    LabInstruments,
    Ammunition,
    Electronics,
    ShipPlating,
    ShipParts,
    Equipment,
    Fuel,
    Medicine,
    Drugs,
    Clothing,
    Microprocessors,
    Plastics,
    Polynucleotides,
    Biocomposites,
    QuantumStabilizers,
    Nanobots,
    AiMainframes,
    QuantumDrives,
    RoboticDrones,
    CyberImplants,
    GeneTherapeutics,
    NeuralChips,
    MoodRegulators,
    ViralAgents,
    MicroFusionGenerators,
    Supergrains,
    LaserRifles,
    Holographics,
    ShipSalvage,
    RelicTech,
    NovelLifeforms,
    BotanicalSpecimens,
    CulturalArtifacts,
    /* -------- frames, reactors, engines -------- */
    FrameProbe,
    FrameDrone,
    FrameInterceptor,
    FrameRacer,
    FrameFighter,
    FrameFrigate,
    FrameShuttle,
    FrameExplorer,
    FrameMiner,
    FrameLightFreighter,
    FrameHeavyFreighter,
    FrameTransport,
    FrameDestroyer,
    FrameCruiser,
    FrameCarrier,
    FrameBulkFreighter,
    ReactorSolarI,
    ReactorFusionI,
    ReactorFissionI,
    ReactorChemicalI,
    ReactorAntimatterI,
    EngineImpulseDriveI,
    EngineIonDriveI,
    EngineIonDriveII,
    EngineHyperDriveI,
    /* -------- modules -------- */
    ModuleMineralProcessorI,
    ModuleGasProcessorI,
    ModuleCargoHoldI,
    ModuleCargoHoldII,
    ModuleCargoHoldIII,
    ModuleCrewQuartersI,
    ModuleEnvoyQuartersI,
    ModulePassengerCabinI,
    ModuleMicroRefineryI,
    ModuleScienceLabI,
    ModuleJumpDriveI,
    ModuleJumpDriveII,
    ModuleJumpDriveIII,
    ModuleWarpDriveI,
    ModuleWarpDriveII,
    ModuleWarpDriveIII,
    ModuleShieldGeneratorI,
    ModuleShieldGeneratorII,
    ModuleOreRefineryI,
    ModuleFuelRefineryI,
    /* -------- mounts -------- */
    MountGasSiphonI,
    MountGasSiphonII,
    MountGasSiphonIII,
    MountSurveyorI,
    MountSurveyorII,
    MountSurveyorIII,
    MountSensorArrayI,
    MountSensorArrayII,
    MountSensorArrayIII,
    MountMiningLaserI,
    MountMiningLaserII,
    MountMiningLaserIII,
    MountLaserCannonI,
    MountMissileLauncherI,
    MountTurretI,
    /* -------- prefab ships -------- */
    ShipProbe,
    ShipMiningDrone,
    ShipSiphonDrone,
    ShipInterceptor,
    ShipLightHauler,
    ShipCommandFrigate,
    ShipExplorer,
    ShipHeavyFreighter,
    ShipLightShuttle,
    ShipOreHound,
    ShipRefiningFreighter,
    ShipSurveyor,
    ShipBulkFreighter,
}

impl TableRow for InventoryItemSymbol {
    fn headers() -> Vec<&'static str> {
        vec!["Symbol", "Pretty"]
    }

    fn to_row(&self) -> Vec<String> {
        let sym = self.to_string();

        let pretty = sym
            .split("_")
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
                    None => String::new()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
    
        vec![sym, pretty]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, Display)]
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

impl TableRow for ModuleSymbol {
    fn headers() -> Vec<&'static str> {
        vec!["Symbol", "Pretty"]
    }

    fn to_row(&self) -> Vec<String> {
        let sym = self.to_string();

        let pretty = sym
            .split("_")
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
                    None => String::new()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
    
        vec![sym, pretty]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, Display)]
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

impl TableRow for MountSymbol {
    fn headers() -> Vec<&'static str> {
        vec!["Symbol", "Pretty"]
    }

    fn to_row(&self) -> Vec<String> {
        let sym = self.to_string();

        let pretty = sym
            .split("_")
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
                    None => String::new()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
    
        vec![sym, pretty]
    }
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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash,
    Serialize, Deserialize,
    EnumIter,
    EnumString,
    Display
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE", ascii_case_insensitive)]
pub enum WaypointType {
    Planet,
    GasGiant,
    Moon,
    OrbitalStation,
    JumpGate,
    AsteroidField,
    Asteroid,
    EngineeredAsteroid,
    AsteroidBase,
    Nebula,
    DebrisField,
    GravityWell,
    ArtificialGravityWell,
    FuelStation
}

impl TableRow for WaypointType {
    fn headers() -> Vec<&'static str> {
        vec!["Symbol", "Pretty"]
    }

    fn to_row(&self) -> Vec<String> {
        let sym = self.to_string();

        let pretty = sym
            .split("_")
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
                    None => String::new()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
    
        vec![sym, pretty]
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash,
    Serialize, Deserialize,
    EnumIter,
    EnumString,
    Display
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE", ascii_case_insensitive)]
pub enum ReactorSymbol {
    ReactorSolarI,        // "REACTOR_SOLAR_I"
    ReactorFusionI,       // "REACTOR_FUSION_I"
    ReactorFissionI,      // "REACTOR_FISSION_I"
    ReactorChemicalI,     // "REACTOR_CHEMICAL_I"
    ReactorAntimatterI,   // "REACTOR_ANTIMATTER_I"
}

impl TableRow for ReactorSymbol {
    fn headers() -> Vec<&'static str> {
        vec!["Symbol", "Pretty"]
    }

    fn to_row(&self) -> Vec<String> {
        let sym = self.to_string();

        let pretty = sym
            .split("_")
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
                    None => String::new()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
    
        vec![sym, pretty]
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash,
    Serialize, Deserialize,
    EnumIter,
    EnumString,
    Display
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE", ascii_case_insensitive)]
pub enum NavStatus {
    InTransit,
    InOrbit,
    Docked
}

impl TableRow for NavStatus {
    fn headers() -> Vec<&'static str> {
        vec!["Symbol", "Pretty"]
    }

    fn to_row(&self) -> Vec<String> {
        let sym = self.to_string();

        let pretty = sym
            .split("_")
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
                    None => String::new()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
    
        vec![sym, pretty]
    }
}