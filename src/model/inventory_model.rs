use std::{fmt::DebugStruct, str::FromStr};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};
use strum_macros::{EnumIter, EnumString};
use crate::dto::responses::inventory_dto::{InventoryItemDTO};

#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub item_symbol: InventoryItemSymbol,
    pub name: String,
    pub description: String,
    pub units:  u32,
}

impl InventoryItem {
    pub fn new<S1, S2>(
        item_symbol: InventoryItemSymbol,
        name: S1,
        description: S2, 
        units: u32
    ) -> anyhow::Result<Self> 
    where 
        S1: Into<String>,
        S2: Into<String>
    {
        let name = name.into();
        let description = description.into();
        Ok(Self { 
            item_symbol, 
            name,
            description,
            units 
        })
    }
}

impl TryFrom<InventoryItemDTO> for InventoryItem {
    type Error = anyhow::Error;

    fn try_from(dto: InventoryItemDTO) -> anyhow::Result<Self> {
        let item_symbol = InventoryItemSymbol::from_str(&dto.item_symbol)?;
        InventoryItem::new(
            item_symbol,
            dto.name,
            dto.description, 
            dto.units
        )
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash,
    Serialize, Deserialize,
    EnumIter,
    EnumString,                    // <── derives `FromStr`
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

