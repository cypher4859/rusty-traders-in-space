use std::sync::Arc;
use anyhow::bail;

use crate::config::Config;
use crate::dto::responses::supply_chain_dto::{MarketEnvelopeDTO, MarketSupplyChainDataEnvelopeDTO};
use crate::{AgentService, MarketSupplyChainDTO, SpaceTradersService};

pub struct MarketService {
    cfg: Arc<Config>,
    st: Arc<SpaceTradersService>,
    agent_svc: Arc<AgentService>
}

impl MarketService {
    pub fn new(st: Arc<SpaceTradersService>, agent_svc: Arc<AgentService>, cfg: Arc<Config>) -> Self {
        Self {
            cfg,
            st,
            agent_svc
        }
    }

    pub async fn get_market_supply_chain(&self) -> anyhow::Result<MarketSupplyChainDataEnvelopeDTO> {
        self._get_market_supply_chain().await
    }

    pub async fn get_market(&self, waypoint_symbol: &String) -> anyhow::Result<MarketEnvelopeDTO> {
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        let market = self._get_market(&agent_token, waypoint_symbol).await?;
        match market {
            Some(mar) => {
                Ok(mar)
            }

            None => {
                bail!("No market was found at {waypoint_symbol}");
            }
        }
    }

    async fn _get_market_supply_chain(&self) -> anyhow::Result<MarketSupplyChainDataEnvelopeDTO> {
        let endpoint: String = format!("market/supply-chain");
        let headers = self.st.get_account_headers()?;
        let result = self.st.get_with_headers::<MarketSupplyChainDataEnvelopeDTO>(&endpoint, Some(headers)).await?;
        match result {
            Some(res) => {
                Ok(res)
            }

            None => {
                bail!("No market supply was found!");
            }
        }
    }

    async fn _get_market(&self, agent_token: &String, waypoint_symbol: &String) -> anyhow::Result<Option<MarketEnvelopeDTO>> {
        let system_symbol: String = self.st.split_waypoint_to_get_system_symbol(waypoint_symbol);
        let endpoint: String = format!("systems/{}/waypoints/{}/market", system_symbol, waypoint_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        let result = self.st.get::<MarketEnvelopeDTO>(&endpoint).await?;
        Ok(result)
    }
}