use std::{fmt::DebugStruct, str::FromStr};
use anyhow::{Result, anyhow, ensure};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter};

use crate::constants::enum_lookups::FactionSymbol;
use crate::constants::enum_lookups::TraitSymbol;
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


