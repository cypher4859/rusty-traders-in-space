use std::sync::Arc;
use anyhow::bail;

use crate::config::Config;
use crate::dto::responses::faction_dto::FactionDataDTO;
use crate::model::Faction;
use crate::{FactionDTO, FactionEnvelopeDTO, SpaceTradersService};
use async_trait::async_trait;


#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FactionServiceT: Send + Sync {
    async fn show_factions(&self, name: &Option<String>) -> anyhow::Result<()>;
    async fn show_all_factions(&self) -> anyhow::Result<()>;
    async fn search_factions(&self, name: &String) -> anyhow::Result<()>;
}

pub struct FactionService {
    cfg: Arc<Config>,
    st: Arc<SpaceTradersService>,
}

impl FactionService {
    pub fn new(cfg: Arc<Config>, st: Arc<SpaceTradersService>) -> Self {
        Self {
            cfg,
            st
        }
    }

    pub async fn show_factions(&self, faction_name: &Option<String>) -> anyhow::Result<()> {
        match faction_name {
            Some(name) => {
                self.search_factions(name).await;
            }
            None => {
                self.show_all_factions().await;
            }
        }

        Ok(())
    }

    pub async fn show_all_factions(&self) -> anyhow::Result<()> {
        self._show_all_factions().await
    }

    pub async fn search_factions(&self, faction_name: &String) -> anyhow::Result<FactionEnvelopeDTO> {
        self._search_factions(faction_name).await
    }

    async fn _search_factions(&self, faction_name: &String) -> anyhow::Result<FactionEnvelopeDTO> {
        let uppercase_faction_name = faction_name.to_uppercase();
        let endpoint: String = format!("factions/{uppercase_faction_name}");
        let result = self.st.get::<FactionEnvelopeDTO>(&endpoint, true).await?;
        match result {
            Some(res) => {
                Ok(res)
            }
            None => {
                bail!("No factions were acquired!");
            }
        }
    }

    async fn _show_all_factions(&self) -> anyhow::Result<()> {
        let endpoint: String = String::from("factions");
        let result = self.st.get_with_headers_and_paging::<FactionDTO>(&endpoint, None, false).await?;
        match(result) {
            Some(faction_dtos) => {
                let items = faction_dtos.into_iter().map(Faction::try_from).collect::<anyhow::Result<Vec<_>>>()?;
                self.st.display_db_results_as_table(items);
                Ok(())
            },
            None => {
                bail!("No factions pulled!")
            }
        }
    }
}

// #[async_trait]
// impl FactionServiceT for FactionService {
//     async fn show_factions(&self, name: &Option<String>) -> anyhow::Result<()> {
//         match name {
//             Some(n) => self.search_factions(n).await.map(|_| ()),
//             None    => self.show_all_factions().await,
//         }
//     }

//     async fn show_all_factions(&self) -> anyhow::Result<()> {
//         self._show_all_factions().await
//     }

//     async fn search_factions(&self, name: &String) -> anyhow::Result<()> {
//         let env = self._search_factions(name).await?;
//         // If your envelope contains a single faction, render it; otherwise adapt as needed
//         // Example: display one row
//         let faction = Faction::try_from(env.data)?;
//         self.st.display_results_as_table(vec![faction]);
//         Ok(())
//     }
// }
