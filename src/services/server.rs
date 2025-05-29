use std::sync::Arc;
use crate::config::Config;
use crate::model::FactionSymbol;
use crate::{ServerStatusDTO, SpaceTradersService};
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

    pub async fn get_status(&self) -> Result<(), ()> {
        self._get_status().await
    }
    async fn _get_status(&self) -> Result<(), ()> {
        let endpoint: String = String::from("");
        let result = self.st.get::<ServerStatusDTO>(&endpoint).await;
        Ok(())
    }
}