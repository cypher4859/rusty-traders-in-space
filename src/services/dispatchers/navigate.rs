use std::result;
use std::sync::Arc;
use anyhow::bail;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use crate::config::Config;
use crate::dto::requests::nav_request_dto::RequestJumpToWaypointDTO;
use crate::dto::requests::nav_request_dto::RequestNavigateToWaypointDTO;
use crate::dto::requests::nav_request_dto::RequestWarpToWaypointDTO;
use crate::dto::responses::nav_dto::NavigateDockDataEnvelopeDTO;
use crate::dto::responses::nav_dto::NavigateJumpDataEnvelopeDTO;
use crate::dto::responses::nav_dto::NavigateOrbitDataEnvelopeDTO;
use crate::dto::responses::nav_dto::NavigateStatusDataEnvelopeDTO;
use crate::dto::responses::nav_dto::NavigateWarpDataEnvelopeDTO;
use crate::dto::responses::nav_dto::NavigateWaypointDataEnvelopeDTO;
use crate::services::dispatchers::agent;
use crate::services::dispatchers::contract;
use crate::SpaceTradersService;

#[derive(Clone)]
pub struct NavigateService {
    cfg: Arc<Config>,
    st: Arc<SpaceTradersService>,
}

impl NavigateService {
    pub fn new(cfg: Arc<Config>, st: Arc<SpaceTradersService>) -> Self {
        Self { 
            cfg,
            st
        }
    }
    
    // FIXME: The content of the pub functions needs to be abstracted away to private methods
    pub async fn navigate_orbit(&self, agent_token: &String, ship_symbol: &String,) -> anyhow::Result<NavigateOrbitDataEnvelopeDTO> {
        self._navigate_orbit(agent_token, ship_symbol).await
    }
    
    pub async fn navigate_to_waypoint(&self, agent_token: &String, ship_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<NavigateWaypointDataEnvelopeDTO> {
        self._navigate_to_waypoint(agent_token, ship_symbol, waypoint_symbol).await
    }    
    
    pub async fn dock_at_station(&self, agent_token: &String, ship_symbol: &String) -> anyhow::Result<NavigateDockDataEnvelopeDTO> {
        self._dock_at_station(agent_token, ship_symbol).await
    }
    
    pub async fn get_navigation_status(&self, agent_token: &String, ship_symbol: &String) -> anyhow::Result<NavigateStatusDataEnvelopeDTO> {
        self._get_navigation_status(agent_token, ship_symbol).await
    }
    
    pub async fn set_flight_mode(&self, agent_token: &String, ship_symbol: &String) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn warp_ship(&self, agent_token: &String, ship_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<NavigateWarpDataEnvelopeDTO> {
        self._warp_ship(agent_token, ship_symbol, waypoint_symbol).await
    }
    
    pub async fn jump_to_waypoint(&self, agent_token: &String, ship_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<NavigateJumpDataEnvelopeDTO> {
        self._jump_to_waypoint(agent_token, ship_symbol, waypoint_symbol).await
    }






    
    async fn _navigate_orbit(&self, agent_token: &String, ship_symbol: &String) -> anyhow::Result<NavigateOrbitDataEnvelopeDTO> {
        let endpoint: String = format!("my/ships/{}/orbit", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        let result = self.st.post_with_headers::<NavigateOrbitDataEnvelopeDTO, ()>(&endpoint, None, Some(headers), true).await?;
        Ok(result)
    }

    async fn _navigate_to_waypoint(&self, agent_token: &String, ship_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<NavigateWaypointDataEnvelopeDTO> {
        let endpoint: String = format!("my/ships/{}/navigate", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        let body = RequestNavigateToWaypointDTO::new(
            waypoint_symbol
        )?;
        let result = self.st.post_with_headers::<NavigateWaypointDataEnvelopeDTO, RequestNavigateToWaypointDTO>(&endpoint, Some(&body), Some(headers), true).await?;
        Ok(result)
    }

    async fn _dock_at_station(&self, agent_token: &String, ship_symbol: &String) -> anyhow::Result<NavigateDockDataEnvelopeDTO> {
        let endpoint: String = format!("my/ships/{}/dock", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        let result = self.st.post_with_headers::<NavigateDockDataEnvelopeDTO, ()>(&endpoint, None, Some(headers), true).await?;
        Ok(result)
    }

    async fn _get_navigation_status(&self, agent_token: &String, ship_symbol: &String) -> anyhow::Result<NavigateStatusDataEnvelopeDTO> {
        let endpoint: String = format!("my/ships/{}/nav", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        let result = self.st.get_with_headers::<NavigateStatusDataEnvelopeDTO>(&endpoint, Some(headers), false).await?;
        match result {
            Some(res) => {
                Ok(res)
            },
            None => {
                bail!("No Navigation status could be found");
            }
        }
    }

    async fn _warp_ship(&self, agent_token: &String, ship_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<NavigateWarpDataEnvelopeDTO> {
        let endpoint: String = format!("my/ships/{}/warp", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        let body = RequestWarpToWaypointDTO::new(
            waypoint_symbol
        )?;
        let result = self.st.post_with_headers::<NavigateWarpDataEnvelopeDTO, RequestWarpToWaypointDTO>(&endpoint, Some(&body), Some(headers), true).await?;
        Ok(result)
    }

    async fn _jump_to_waypoint(&self, agent_token: &String, ship_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<NavigateJumpDataEnvelopeDTO> {
        let endpoint: String = format!("my/ships/{}/jump", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        let body = RequestJumpToWaypointDTO::new(
            waypoint_symbol
        )?;
        let result = self.st.post_with_headers::<NavigateJumpDataEnvelopeDTO, RequestJumpToWaypointDTO>(&endpoint, Some(&body), Some(headers), true).await?;
        Ok(result)
    }
}
