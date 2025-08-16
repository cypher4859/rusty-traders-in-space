use std::{fmt::DebugStruct, str::FromStr};
use anyhow::Context;
use anyhow::{Result, anyhow, ensure};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter};

use crate::helpers::enum_lookups::FactionSymbol;
use crate::helpers::enum_lookups::TraitSymbol;
use crate::helpers::table_helpers::TableRow;
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

impl TableRow for Faction {
    fn headers() -> Vec<&'static str> {
        vec!["Name", "Keyword", "HQ", "Recruiting", "Description"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.name.trim().to_string(),
            self.symbol.to_string(),
            self.headquarters.trim().to_string(),
            self.is_recruiting.to_string(),
            self.description.trim().to_string()
        ]
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

        // FIXME: This is broken, need to fix the `unwrap` shit
        let traits: Vec<Trait> = dto.traits
            .into_iter()
            .map(Trait::try_from)
            .collect::<Result<Vec<_>>>()
            .with_context(|| format!("Failed to get Traits"))?;

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
            "BUREAUCRATIC"            => TraitSymbol::Bureaucratic,
            "SECRETIVE"               => TraitSymbol::Secretive,
            "CAPITALISTIC"            => TraitSymbol::Capitalistic,
            "INDUSTRIOUS"             => TraitSymbol::Industrious,
            "PEACEFUL"                => TraitSymbol::Peaceful,
            "DISTRUSTFUL"             => TraitSymbol::Distrustful,
            "WELCOMING"               => TraitSymbol::Welcoming,
            "SMUGGLERS"               => TraitSymbol::Smugglers,
            "SCAVENGERS"              => TraitSymbol::Scavengers,
            "REBELLIOUS"              => TraitSymbol::Rebellious,
            "EXILES"                  => TraitSymbol::Exiles,
            "PIRATES"                 => TraitSymbol::Pirates,
            "RAIDERS"                 => TraitSymbol::Raiders,
            "CLAN"                    => TraitSymbol::Clan,
            "GUILD"                   => TraitSymbol::Guild,
            "DOMINION"                => TraitSymbol::Dominion,
            "FRINGE"                  => TraitSymbol::Fringe,
            "FORSAKEN"                => TraitSymbol::Forsaken,
            "ISOLATED"                => TraitSymbol::Isolated,
            "LOCALIZED"               => TraitSymbol::Localized,
            "ESTABLISHED"             => TraitSymbol::Established,
            "NOTABLE"                 => TraitSymbol::Notable,
            "DOMINANT"                => TraitSymbol::Dominant,
            "INESCAPABLE"             => TraitSymbol::Inescapable,
            "INNOVATIVE"              => TraitSymbol::Innovative,
            "BOLD"                    => TraitSymbol::Bold,
            "VISIONARY"               => TraitSymbol::Visionary,
            "CURIOUS"                 => TraitSymbol::Curious,
            "DARING"                  => TraitSymbol::Daring,
            "EXPLORATORY"             => TraitSymbol::Exploratory,
            "RESOURCEFUL"             => TraitSymbol::Resourceful,
            "FLEXIBLE"                => TraitSymbol::Flexible,
            "COOPERATIVE"             => TraitSymbol::Cooperative,
            "UNITED"                  => TraitSymbol::United,
            "STRATEGIC"               => TraitSymbol::Strategic,
            "INTELLIGENT"             => TraitSymbol::Intelligent,
            "RESEARCH_FOCUSED"        => TraitSymbol::ResearchFocused,
            "COLLABORATIVE"           => TraitSymbol::Collaborative,
            "PROGRESSIVE"             => TraitSymbol::Progressive,
            "MILITARISTIC"            => TraitSymbol::Militaristic,
            "TECHNOLOGICALLY_ADVANCED"=> TraitSymbol::TechnologicallyAdvanced,
            "AGGRESSIVE"              => TraitSymbol::Aggressive,
            "IMPERIALISTIC"           => TraitSymbol::Imperialistic,
            "TREASURE_HUNTERS"        => TraitSymbol::TreasureHunters,
            "DEXTEROUS"               => TraitSymbol::Dexterous,
            "UNPREDICTABLE"           => TraitSymbol::Unpredictable,
            "BRUTAL"                  => TraitSymbol::Brutal,
            "FLEETING"                => TraitSymbol::Fleeting,
            "ADAPTABLE"               => TraitSymbol::Adaptable,
            "SELF_SUFFICIENT"         => TraitSymbol::SelfSufficient,
            "DEFENSIVE"               => TraitSymbol::Defensive,
            "PROUD"                   => TraitSymbol::Proud,
            "DIVERSE"                 => TraitSymbol::Diverse,
            "INDEPENDENT"             => TraitSymbol::Independent,
            "SELF_INTERESTED"         => TraitSymbol::SelfInterested,
            "FRAGMENTED"              => TraitSymbol::Fragmented,
            "COMMERCIAL"              => TraitSymbol::Commercial,
            "FREE_MARKETS"            => TraitSymbol::FreeMarkets,
            "ENTREPRENEURIAL"         => TraitSymbol::Entrepreneurial,
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


