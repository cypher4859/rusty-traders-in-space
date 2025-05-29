use std::sync::Arc;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use crate::config::Config;
use crate::services::agent;
use crate::services::contract;
use crate::SpaceTradersService;

#[derive(Clone)]
pub struct ModuleService {
    cfg: Arc<Config>,
    st: Arc<SpaceTradersService>,
}

impl ModuleService {
    pub fn new(cfg: Arc<Config>, st: Arc<SpaceTradersService>) -> Self {
        Self { 
            cfg,
            st
        }
    }

    pub async fn list_modules_by_ship(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn install_module_to_ship(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn remove_module_from_ship(&self) -> anyhow::Result<()> {
        Ok(())
    }
}