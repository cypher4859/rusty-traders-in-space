use std::sync::Arc;
use anyhow::bail;

use crate::config::Config;
use crate::{FactionEnvelopeDTO, SpaceTradersService};

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

    pub async fn show_all_factions(&self) -> anyhow::Result<FactionEnvelopeDTO> {
        self._show_all_factions().await
    }

    pub async fn search_factions(&self, faction_name: &String) -> anyhow::Result<FactionEnvelopeDTO> {
        self._search_factions(faction_name).await
    }

    async fn _search_factions(&self, faction_name: &String) -> anyhow::Result<FactionEnvelopeDTO> {
        let uppercase_faction_name = faction_name.to_uppercase();
        let endpoint: String = format!("factions/{uppercase_faction_name}");
        let result = self.st.get::<FactionEnvelopeDTO>(&endpoint).await?;
        match result {
            Some(res) => {
                Ok(res)
            }
            None => {
                bail!("No factions were acquired!");
            }
        }
    }

    async fn _show_all_factions(&self) -> anyhow::Result<FactionEnvelopeDTO> {
        let endpoint: String = String::from("factions");
        let result = self.st.get::<FactionEnvelopeDTO>(&endpoint).await?;
        match result {
            Some(res) => {
                Ok(res)
            }
            None => {
                bail!("No factions were acquired!");
            }
        }
    }
}
