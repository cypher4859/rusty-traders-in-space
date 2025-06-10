use std::sync::Arc;
use crate::config::Config;
use crate::{ServerStatusDTO, SpaceTradersService};
use anyhow::bail;
use strum::IntoEnumIterator;

pub struct ServerService {
    cfg: Arc<Config>,
    st: Arc<SpaceTradersService>,
}

impl ServerService {
    pub fn new(st: Arc<SpaceTradersService>, cfg: Arc<Config>) -> Self {
        Self {
            cfg,
            st
        }
    }

    pub async fn get_status(&self) -> anyhow::Result<ServerStatusDTO> {
        self._get_status().await
    }
    async fn _get_status(&self) -> anyhow::Result<ServerStatusDTO> {
        let endpoint: String = String::from("");
        let result = self.st.get::<ServerStatusDTO>(&endpoint).await?;
        match result {
            Some(res) => {
                Ok(res)
            }

            None => {
                bail!("The server is unreachable!");
            }
        }
    }
}