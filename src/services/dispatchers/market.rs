use std::sync::Arc;
use crate::config::Config;
use crate::dto::responses::supply_chain_dto::MarketEnvelopeDTO;
use crate::{AgentService, SpaceTradersService};

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

    pub async fn get_market_supply_chain(&self, waypoint_symbol: &String) -> anyhow::Result<()> {
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        self._get_market_supply_chain(&agent_token, waypoint_symbol).await;
        Ok(())
    }
    async fn _get_market_supply_chain(&self, agent_token: &String, waypoint_symbol: &String) -> anyhow::Result<Option<MarketEnvelopeDTO>> {
        let system_symbol: String = self.st.split_waypoint_to_get_system_symbol(waypoint_symbol);
        let endpoint: String = format!("systems/{}/waypoints/{}/market", system_symbol, waypoint_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        let result = self.st.get::<MarketEnvelopeDTO>(&endpoint).await?;
        Ok(result)
    }
}