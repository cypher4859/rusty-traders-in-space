use std::sync::Arc;
use crate::config::Config;
use crate::dto::requests::supply_request_dto::RequestSystemSupplyConstructionDTO;
use crate::dto::responses::supply_chain_dto::MarketEnvelopeDTO;
use crate::dto::responses::system_dto::{ConstructionSiteDTO, ConstructionSiteEnvelopeDTO, JumpGateEnvelopeDTO, ShipyardEnvelopeDTO, SystemDTO, SystemEnvelopeDTO, SystemListEnvelopeDTO, WaypointEnvelopeDTO, WaypointListEnvelopeDTO};
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

    pub async fn list_systems(&self) -> anyhow::Result<SystemListEnvelopeDTO> {
        Ok(self._list_systems().await?)
    }

    pub async fn get_system(&self, system_symbol: &String) -> anyhow::Result<SystemEnvelopeDTO> {
        Ok(self._get_single_system(system_symbol).await?)
    }

    pub async fn list_waypoints_by_system(&self, system_symbol: &String) -> anyhow::Result<WaypointListEnvelopeDTO> {
        Ok(self._list_waypoints_by_system_symbol(system_symbol).await?)
    }

    pub async fn get_waypoint(&self, waypoint_symbol: &String) -> anyhow::Result<WaypointEnvelopeDTO> {
        let system_symbol = self.st.split_waypoint_to_get_system_symbol(waypoint_symbol);
        Ok(self._get_waypoint_by_symbol(&system_symbol, waypoint_symbol).await?)
    }

    pub async fn get_market(&self, waypoint_symbol: &String, ship_symbol: &Option<String>) -> anyhow::Result<MarketEnvelopeDTO> {
        let system_symbol = self.st.split_waypoint_to_get_system_symbol(waypoint_symbol);
        Ok(self._get_market_by_system_waypoint(&system_symbol, waypoint_symbol, ship_symbol).await?)
    }

    pub async fn get_shipyard(&self, waypoint_symbol: &String) -> anyhow::Result<ShipyardEnvelopeDTO> {
        let system_symbol = self.st.split_waypoint_to_get_system_symbol(waypoint_symbol);
        Ok(self._get_shipyard_by_system_waypiont(&system_symbol, waypoint_symbol).await?)
    }

    pub async fn get_jumpgate(&self, waypoint_symbol: &String) -> anyhow::Result<JumpGateEnvelopeDTO> {
        let system_symbol = self.st.split_waypoint_to_get_system_symbol(waypoint_symbol);
        Ok(self._get_jumpgate_by_system_waypiont(&system_symbol, waypoint_symbol).await?)
    }

    pub async fn get_construction_site(&self, waypoint_symbol: &String) -> anyhow::Result<ConstructionSiteEnvelopeDTO> {
        let system_symbol = self.st.split_waypoint_to_get_system_symbol(waypoint_symbol);
        Ok(self._get_construction_site_by_system_waypiont(&system_symbol, waypoint_symbol).await?)
    }

    pub async fn supply_construction_site(&self, waypoint_symbol: &String, item_symbol: &String, units: &i64, ship_symbol: &String) -> anyhow::Result<ConstructionSiteDTO> {
        let system_symbol = self.st.split_waypoint_to_get_system_symbol(waypoint_symbol);
        Ok(self._supply_construction_site_by_system_waypiont(&system_symbol, waypoint_symbol, item_symbol, units, ship_symbol).await?)
    }

    async fn _list_systems(&self) -> anyhow::Result<SystemListEnvelopeDTO> {
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        let endpoint: String = String::from("systems");
        let headers = self.st.get_agent_headers(&agent_token);
        let response = self.st.get_with_headers::<SystemListEnvelopeDTO>(&endpoint, Some(headers?), true).await?;
        match response {
            Some (res) => {
                Ok(res)
            },
            None => {
                bail!("Failed to list systems");
            }
        }
    }

    async fn _get_single_system(&self, system_symbol: &String) -> anyhow::Result<SystemEnvelopeDTO> {
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        let endpoint: String = format!("systems/{}", system_symbol);
        let headers = self.st.get_agent_headers(&agent_token)?;
        let response = self.st.get_with_headers::<SystemEnvelopeDTO>(&endpoint, Some(headers), true).await?;
        match response {
            Some (res) => {
                Ok(res)
            },
            None => {
                bail!("Failed to get system {system_symbol}");
            }
        }
    }

    async fn _list_waypoints_by_system_symbol(&self, system_symbol: &String) -> anyhow::Result<WaypointListEnvelopeDTO> {
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        let endpoint: String = format!("systems/{}/waypoints", system_symbol);
        let headers = self.st.get_agent_headers(&agent_token)?;
        let response = self.st.get_with_headers::<WaypointListEnvelopeDTO>(&endpoint, Some(headers), true).await?;
        match response {
            Some (res) => {
                Ok(res)
            },
            None => {
                bail!("Failed to get system {system_symbol}");
            }
        }
    }

    async fn _get_waypoint_by_symbol(&self, system_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<WaypointEnvelopeDTO> {
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        let endpoint: String = format!("systems/{}/waypoints/{}", system_symbol, waypoint_symbol);
        let headers = self.st.get_agent_headers(&agent_token)?;
        let response = self.st.get_with_headers::<WaypointEnvelopeDTO>(&endpoint, Some(headers), true).await?;
        match response {
            Some (res) => {
                Ok(res)
            },
            None => {
                bail!("Failed to get system {system_symbol}");
            }
        }
    }

    async fn _get_market_by_system_waypoint(&self, system_symbol: &String, waypoint_symbol: &String, ship_symbol: &Option<String>) -> anyhow::Result<MarketEnvelopeDTO> {
        let endpoint: String = format!("systems/{}/waypoints/{}/market", system_symbol, waypoint_symbol);
        let &mut response;
        match ship_symbol {
            Some(ship) => {
                let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
                let headers = self.st.get_agent_headers(&agent_token)?;
                response = self.st.get_with_headers::<MarketEnvelopeDTO>(&endpoint, Some(headers), true).await?;                
            },
            None => {
                response = self.st.get::<MarketEnvelopeDTO>(&endpoint, true).await?;
            }
        }
        match response {
            Some (res) => {
                Ok(res)
            },
            None => {
                bail!("Failed to get system {system_symbol}");
            }
        }
    }

    async fn _get_shipyard_by_system_waypiont(&self, system_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<ShipyardEnvelopeDTO> {
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        let endpoint: String = format!("systems/{}/waypoints/{}/shipyard", system_symbol, waypoint_symbol);
        let headers = self.st.get_agent_headers(&agent_token)?;
        let response = self.st.get_with_headers::<ShipyardEnvelopeDTO>(&endpoint, Some(headers), true).await?;
        match response {
            Some (res) => {
                Ok(res)
            },
            None => {
                bail!("Failed to get system {system_symbol}");
            }
        }
    }

    async fn _get_jumpgate_by_system_waypiont(&self, system_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<JumpGateEnvelopeDTO> {
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        let endpoint: String = format!("systems/{}/waypoints/{}/jump-gate", system_symbol, waypoint_symbol);
        let headers = self.st.get_agent_headers(&agent_token)?;
        let response = self.st.get_with_headers::<JumpGateEnvelopeDTO>(&endpoint, Some(headers), true).await?;
        match response {
            Some (res) => {
                Ok(res)
            },
            None => {
                bail!("Failed to get system {system_symbol}");
            }
        }
    }

    async fn _get_construction_site_by_system_waypiont(&self, system_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<ConstructionSiteEnvelopeDTO> {
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        let endpoint: String = format!("systems/{}/waypoints/{}/construction", system_symbol, waypoint_symbol);
        let headers = self.st.get_agent_headers(&agent_token)?;
        let response = self.st.get_with_headers::<ConstructionSiteEnvelopeDTO>(&endpoint, Some(headers), true).await?;
        match response {
            Some(res) => {
                Ok(res)
            },
            None => {
                bail!("Failed to get the construction site at {waypoint_symbol}");
            }
        }
    }

    async fn _supply_construction_site_by_system_waypiont(&self, system_symbol: &String, waypoint_symbol: &String, item_symbol: &String, units: &i64, ship_symbol: &String) -> anyhow::Result<ConstructionSiteDTO> {
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        let endpoint: String = format!("systems/{}/waypoints/{}/construction", system_symbol, waypoint_symbol);
        let headers = self.st.get_agent_headers(&agent_token)?;
        let body = RequestSystemSupplyConstructionDTO::new(
            item_symbol.clone(),
            units.clone(),
            ship_symbol.clone()
        )?;
        let response = self.st.post_with_headers::<ConstructionSiteDTO, RequestSystemSupplyConstructionDTO>(&endpoint, Some(&body), Some(headers), true).await?;
        Ok(response)
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