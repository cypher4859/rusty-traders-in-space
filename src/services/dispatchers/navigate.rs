use std::sync::Arc;
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
    pub async fn navigate_orbit(&self, agent_token: &String, ship_symbol: &String,) -> anyhow::Result<()> {
        let endpoint: String = format!("my/ships/{}/orbit", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        self.st.post_with_headers::<NavigateOrbitDataEnvelopeDTO, ()>(&endpoint, None, Some(headers)).await?;
        Ok(())
    }

    pub async fn navigate_to_waypoint(&self, agent_token: &String, ship_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<()> {
        let endpoint: String = format!("my/ships/{}/navigate", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        let body = RequestNavigateToWaypointDTO::new(
            waypoint_symbol
        )?;
        self.st.post_with_headers::<NavigateWaypointDataEnvelopeDTO, RequestNavigateToWaypointDTO>(&endpoint, Some(&body), Some(headers)).await?;
        Ok(())
    }

    pub async fn dock_at_station(&self, agent_token: &String, ship_symbol: &String) -> anyhow::Result<()> {
        let endpoint: String = format!("my/ships/{}/dock", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        self.st.post_with_headers::<NavigateDockDataEnvelopeDTO, ()>(&endpoint, None, Some(headers)).await?;
        Ok(())
    }

    pub async fn get_navigation_status(&self, agent_token: &String, ship_symbol: &String) -> anyhow::Result<()> {
        let endpoint: String = format!("my/ships/{}/nav", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        self.st.get_with_headers::<NavigateStatusDataEnvelopeDTO>(&endpoint, Some(headers)).await?;
        Ok(())
    }

    pub async fn set_flight_mode(&self, agent_token: &String, ship_symbol: &String) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn warp_ship(&self, agent_token: &String, ship_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<()> {
        let endpoint: String = format!("my/ships/{}/warp", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        let body = RequestWarpToWaypointDTO::new(
            waypoint_symbol
        )?;
        self.st.post_with_headers::<NavigateWarpDataEnvelopeDTO, RequestWarpToWaypointDTO>(&endpoint, Some(&body), Some(headers)).await?;
        Ok(())
    }

    pub async fn jump_to_waypoint(&self, agent_token: &String, ship_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<()> {
        let endpoint: String = format!("my/ships/{}/jump", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        let body = RequestJumpToWaypointDTO::new(
            waypoint_symbol
        )?;
        self.st.post_with_headers::<NavigateJumpDataEnvelopeDTO, RequestJumpToWaypointDTO>(&endpoint, Some(&body), Some(headers)).await?;
        Ok(())
    }
}
