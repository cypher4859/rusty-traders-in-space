use std::sync::Arc;
use crate::config::Config;
use crate::dto::responses::supply_chain_dto::MarketEnvelopeDTO;
use crate::dto::responses::system_dto::{ConstructionSiteEnvelopeDTO, JumpGateEnvelopeDTO, ShipyardEnvelopeDTO, SystemDTO, SystemEnvelopeDTO, SystemListEnvelopeDTO, WaypointEnvelopeDTO, WaypointListEnvelopeDTO};
use crate::{RegisterEnvelopeDTO, RegisterDataDTO, SpaceTradersService, AgentService};
// use crate::model::system_model::System;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use anyhow::{Result, bail};


#[derive(Clone)]
pub struct SystemService {
    cfg: Arc<Config>,
    st: Arc<SpaceTradersService>,
    agent_svc: Arc<AgentService>
}

impl SystemService {
    pub fn new(cfg: Arc<Config>, st: Arc<SpaceTradersService>, agent_svc: Arc<AgentService>) -> Self {
        Self { cfg, st, agent_svc }
    }

    pub async fn show_location(&self, location: &String) -> anyhow::Result<()> {
        if (self._is_system_symbol(location)) {
            self.get_system(location).await;
        } else if (self._is_waypoint_symbol(location)) {
            self.get_waypoint(location).await;
        }

        format!("Symbol {} not found! Doesn't look like a system, doesn't look like a waypoint", location);
        Ok(())
    }

    pub async fn list_systems(&self) -> anyhow::Result<()> {
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        self._list_systems(&agent_token).await?;
        Ok(())
    }

    pub async fn get_system(&self, system_symbol: &String) -> anyhow::Result<()> {
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        self._get_single_system(&agent_token, system_symbol).await?;
        Ok(())
    }

    pub async fn list_waypoints_by_system(&self, system_symbol: &String) -> anyhow::Result<()> {
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        self._list_waypoints_by_system_symbol(&agent_token, system_symbol).await?;
        Ok(())
    }

    pub async fn get_waypoint(&self, waypoint_symbol: &String) -> anyhow::Result<()> {
        let system_symbol = self.st.split_waypoint_to_get_system_symbol(waypoint_symbol);
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        self._get_waypoint_by_symbol(&agent_token, &system_symbol, waypoint_symbol).await?;
        Ok(())
    }

    pub async fn get_market(&self, waypoint_symbol: &String) -> anyhow::Result<()> {
        let system_symbol = self.st.split_waypoint_to_get_system_symbol(waypoint_symbol);
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        self._get_market_by_system_waypiont(&agent_token, &system_symbol, waypoint_symbol).await?;
        Ok(())
    }

    pub async fn get_shipyard(&self, waypoint_symbol: &String) -> anyhow::Result<()> {
        let system_symbol = self.st.split_waypoint_to_get_system_symbol(waypoint_symbol);
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        self._get_shipyard_by_system_waypiont(&agent_token, &system_symbol, waypoint_symbol).await?;
        Ok(())
    }

    pub async fn get_jumpgate(&self, waypoint_symbol: &String) -> anyhow::Result<()> {
        let system_symbol = self.st.split_waypoint_to_get_system_symbol(waypoint_symbol);
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        self._get_jumpgate_by_system_waypiont(&agent_token, &system_symbol, waypoint_symbol).await?;
        Ok(())
    }

    pub async fn get_construction_site(&self, waypoint_symbol: &String) -> anyhow::Result<()> {
        let system_symbol = self.st.split_waypoint_to_get_system_symbol(waypoint_symbol);
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        self._get_construction_site_by_system_waypiont(&agent_token, &system_symbol, waypoint_symbol).await?;
        Ok(())
    }

    pub async fn supply_construction_site(&self, waypoint_symbol: &String) -> anyhow::Result<()> {
        let system_symbol = self.st.split_waypoint_to_get_system_symbol(waypoint_symbol);
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        self._supply_construction_site_by_system_waypiont(&agent_token, &system_symbol, waypoint_symbol).await?;
        Ok(())
    }

    async fn _list_systems(&self, agent_token: &String) -> anyhow::Result<()> {
        let endpoint: String = String::from("systems");
        let headers = self.st.get_agent_headers(agent_token);
        self.st.get_with_headers::<SystemListEnvelopeDTO>(&endpoint, Some(headers?)).await?;
        Ok(())
    }

    async fn _get_single_system(&self, agent_token: &String, system_symbol: &String) -> anyhow::Result<()> {
        let endpoint: String = format!("systems/{}", system_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        self.st.get_with_headers::<SystemEnvelopeDTO>(&endpoint, Some(headers)).await?;
        Ok(())
    }

    async fn _list_waypoints_by_system_symbol(&self, agent_token: &String, system_symbol: &String) -> anyhow::Result<()> {
        let endpoint: String = format!("systems/{}/waypoints", system_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        self.st.get_with_headers::<WaypointListEnvelopeDTO>(&endpoint, Some(headers)).await?;
        Ok(())
    }

    async fn _get_waypoint_by_symbol(&self, agent_token: &String, system_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<()> {
        let endpoint: String = format!("systems/{}/waypoints/{}", system_symbol, waypoint_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        self.st.get_with_headers::<WaypointEnvelopeDTO>(&endpoint, Some(headers)).await?;
        Ok(())
    }

    async fn _get_market_by_system_waypiont(&self, agent_token: &String, system_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<()> {
        let endpoint: String = format!("systems/{}/waypoints/{}/market", system_symbol, waypoint_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        self.st.get_with_headers::<MarketEnvelopeDTO>(&endpoint, Some(headers)).await?;
        Ok(())
    }

    async fn _get_shipyard_by_system_waypiont(&self, agent_token: &String, system_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<()> {
        let endpoint: String = format!("systems/{}/waypoints/{}/shipyard", system_symbol, waypoint_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        self.st.get_with_headers::<ShipyardEnvelopeDTO>(&endpoint, Some(headers)).await?;
        Ok(())
    }

    async fn _get_jumpgate_by_system_waypiont(&self, agent_token: &String, system_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<()> {
        let endpoint: String = format!("systems/{}/waypoints/{}/jump-gate", system_symbol, waypoint_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        self.st.get_with_headers::<JumpGateEnvelopeDTO>(&endpoint, Some(headers)).await?;
        Ok(())
    }

    async fn _get_construction_site_by_system_waypiont(&self, agent_token: &String, system_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<()> {
        let endpoint: String = format!("systems/{}/waypoints/{}/construction", system_symbol, waypoint_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        self.st.get_with_headers::<ConstructionSiteEnvelopeDTO>(&endpoint, Some(headers)).await?;
        Ok(())
    }

    async fn _supply_construction_site_by_system_waypiont(&self, agent_token: &String, system_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<()> {
        // let endpoint: String = format!("systems/{}/waypoints/{}/construction", system_symbol, waypoint_symbol);
        // let headers = self.st.get_agent_headers(agent_token)?;
        // let body = RequestSystemSupplyConstructionDTO::new(

        // )?;
        // self.st.post_with_headers::<SystemEnvelopeWithMetaDTO, RequestSystemSupplyConstructionDTO>(&endpoint, body, Some(headers)).await?;
        todo!();
        Ok(())
    }

    

    fn _count_the_dashes(&self, symbol: &String) -> usize {
        symbol.chars().filter(|&c| c == '-').count()
    }

    fn _is_waypoint_symbol(&self, symbol: &String) -> bool {
        self._count_the_dashes(symbol) == 2
    }

    fn _is_system_symbol(&self, symbol: &String) -> bool {
        self._count_the_dashes(symbol) == 1
    }
}