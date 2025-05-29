use std::sync::Arc;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use crate::config::Config;
use crate::services::agent;
use crate::services::contract;
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
    
    pub async fn navigate_orbit(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn dock_at_station(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn get_navigation_status(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn set_flight_mode(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn warp_ship(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn jump_to_waypoint(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
