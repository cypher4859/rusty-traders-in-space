use std::sync::Arc;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use crate::config::Config;
use crate::services::agent;
use crate::services::contract;
use crate::SpaceTradersService;

#[derive(Clone)]
pub struct ScanService {
    cfg: Arc<Config>,
    st: Arc<SpaceTradersService>,
}

impl ScanService {
    pub fn new(cfg: Arc<Config>, st: Arc<SpaceTradersService>) -> Self {
        Self { 
            cfg,
            st
        }
    }
    
    pub async fn scan_systems(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn scan_waypoints(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn scan_ships(&self) -> anyhow::Result<()> {
        Ok(())
    }
}