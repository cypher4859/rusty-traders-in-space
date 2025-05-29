use std::sync::Arc;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use crate::config::Config;
use crate::services::agent;
use crate::services::contract;
use crate::SpaceTradersService;

#[derive(Clone)]
pub struct MountService {
    cfg: Arc<Config>,
    st: Arc<SpaceTradersService>,
}

impl MountService {
    pub fn new(cfg: Arc<Config>, st: Arc<SpaceTradersService>) -> Self {
        Self { 
            cfg,
            st
        }
    }

    pub async fn list_mounts(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn install_mount(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn remove_mount(&self) -> anyhow::Result<()> {
        Ok(())
    }
}