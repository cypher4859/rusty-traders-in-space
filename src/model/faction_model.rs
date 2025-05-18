use std::{fmt::DebugStruct, str::FromStr};
use anyhow::{Result, anyhow, ensure};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter};

use crate::FactionDTO;
use crate::TraitDTO;

#[derive(Debug, Clone)]
pub struct Faction {
    symbol:        FactionSymbol,
    name:          String,
    description:   String,
    headquarters:  String,
    traits:        Vec<Trait>,
    is_recruiting: bool,
}

impl Faction {
    pub fn new<S1, S2, S3>(
        symbol: FactionSymbol,
        name: S1,
        description: S2,
        headquarters: S3,
        traits: Vec<Trait>,
        is_recruiting: bool
    ) -> anyhow::Result<Self>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>
    {
        let name: String = name.into();
        let description: String = description.into();
        let headquarters: String = headquarters.into();

        Ok(Self {
            symbol,
            name,
            description,
            headquarters,
            traits,
            is_recruiting
        })
    }
}

impl TryFrom<FactionDTO> for Faction {
    type Error = anyhow::Error;

    fn try_from(dto: FactionDTO) -> anyhow::Result<Self> {
        // let symbol = match dto.symbol.as_str() {
        //     "COSMIC" => FactionSymbol::Cosmic,
        //     "VOID"   => FactionSymbol::Void,
        //     other => return Err(anyhow!("unknown faction symbol {other}")),
        // };
        let symbol = FactionSymbol::from_str(&dto.symbol)?;

        ensure!(!dto.name.is_empty(), "name cannot be empty");

        let traits = dto
            .traits
            .into_iter()
            .map(Trait::try_from)
            .collect::<Result<Vec<_>>>()?;

        Faction::new(
            symbol,
            dto.name,
            dto.description,
            dto.headquarters,
            traits,
            dto.is_recruiting,
        )
    }
}


impl TryFrom<TraitDTO> for Trait {
    type Error = anyhow::Error;

    fn try_from(src: TraitDTO) -> Result<Self, anyhow::Error> {
        let symbol = match src.symbol.as_str() {
            "BUREAUCRATIC" => TraitSymbol::Bureaucratic,
            other => return Err(anyhow!("unknown trait symbol {other}")),
        };

        Ok(Self {
            symbol,
            name: src.name,
            description: src.description,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Trait {
    pub symbol:      TraitSymbol,
    pub name:        String,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TraitSymbol {
    Bureaucratic
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
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

impl FromStr for TraitSymbol {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "BUREAUCRATIC"   => Ok(TraitSymbol::Bureaucratic),
            other => Err(anyhow::anyhow!("Unknown faction '{other}'")),
        }
    }
}
