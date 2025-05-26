use std::sync::Arc;
use crate::config::Config;
use crate::{SpaceTradersService};
use crate::{MarketSupplyChainDTO};

pub struct MarketService {
    cfg: Arc<Config>,
    st: Arc<SpaceTradersService>,
}

impl MarketService {
    pub fn new(st: Arc<SpaceTradersService>, cfg: Arc<Config>) -> Self {
        Self {
            cfg,
            st
        }
    }

    pub async fn get_market_supply_chain(&self) -> anyhow::Result<()> {
        self._get_market_supply_chain().await
    }
    async fn _get_market_supply_chain(&self) -> anyhow::Result<()> {
        let endpoint: String = String::from("market/supply-chain");
        let result: Result<MarketSupplyChainDTO, anyhow::Error> = self.st.get::<MarketSupplyChainDTO>(&endpoint).await;
        Ok(())
    }
}