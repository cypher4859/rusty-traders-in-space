use std::sync::Arc;
use std::{fmt::DebugStruct, str::FromStr};
use crate::config::Config;
use crate::constants::enum_lookups::InventoryItemSymbol;
use crate::dto::requests::cargo_request_dto::{RequestCargoBuyDTO, RequestCargoJettisonDTO, RequestCargoSellDTO, RequestCargoTransferDTO};
use crate::dto::responses::fleet_dto::{CargoCargoDataEnvelopeDTO, CargoDataEnvelopeDTO};
use crate::SpaceTradersService;

#[derive(Clone)]
pub struct CargoService {
    cfg: Arc<Config>,
    st: Arc<SpaceTradersService>,
}

impl CargoService {
    pub fn new(cfg: Arc<Config>, st: Arc<SpaceTradersService>) -> Self {
        Self { 
            cfg,
            st
        }
    }

    pub async fn list_cargo(&self, agent_token: &String, ship_symbol: &String) -> anyhow::Result<()> {
        self._list_cargo(agent_token, ship_symbol).await?;
        Ok(())
    }

    pub async fn purchase_cargo(&self, agent_token: &String, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        self._purchase_cargo(agent_token, ship_symbol, cargo_symbol, cargo_units).await?;
        Ok(())
    }

    pub async fn jettison_cargo(&self, agent_token: &String, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        self._jettison_cargo(agent_token, ship_symbol, cargo_symbol, cargo_units).await?;
        Ok(())
    }

    pub async fn sell_cargo(&self, agent_token: &String, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        self._sell_cargo(agent_token, ship_symbol, cargo_symbol, cargo_units).await?;
        Ok(())
    }

    pub async fn transfer_cargo(&self, agent_token: &String, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        self._transfer_cargo(agent_token, ship_symbol, cargo_symbol, cargo_units).await?;
        Ok(())
    }

    async fn _purchase_cargo(&self, agent_token: &String, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        let endpoint: String = format!("my/ships/{}/purchase", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        let inventory_item_symbol = InventoryItemSymbol::from_str(cargo_symbol)?;
        let body = RequestCargoBuyDTO::new(
            inventory_item_symbol,
            cargo_units
        )?;
        self.st.post_with_headers::<CargoCargoDataEnvelopeDTO, RequestCargoBuyDTO>(&endpoint, Some(&body), Some(headers)).await?;
        Ok(())
    }

    async fn _jettison_cargo(&self, agent_token: &String, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        let endpoint: String = format!("my/ships/{}/jettison", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        let inventory_item_symbol = InventoryItemSymbol::from_str(cargo_symbol)?;
        let body = RequestCargoJettisonDTO::new(
            inventory_item_symbol,
            cargo_units
        )?;
        self.st.post_with_headers::<CargoCargoDataEnvelopeDTO, RequestCargoJettisonDTO>(&endpoint, Some(&body), Some(headers)).await?;
        Ok(())
    }

    async fn _sell_cargo(&self, agent_token: &String, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        let endpoint: String = format!("my/ships/{}/sell", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        let inventory_item_symbol = InventoryItemSymbol::from_str(cargo_symbol)?;
        let body = RequestCargoSellDTO::new(
            inventory_item_symbol,
            cargo_units
        )?;
        self.st.post_with_headers::<CargoCargoDataEnvelopeDTO, RequestCargoSellDTO>(&endpoint, Some(&body), Some(headers)).await?;
        Ok(())
    }

    async fn _transfer_cargo(&self, agent_token: &String, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        let endpoint: String = format!("my/ships/{}/transfer", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        let inventory_item_symbol = InventoryItemSymbol::from_str(cargo_symbol)?;
        let body = RequestCargoTransferDTO::new(
            inventory_item_symbol,
            cargo_units,
            ship_symbol.clone()
        )?;
        self.st.post_with_headers::<CargoCargoDataEnvelopeDTO, RequestCargoTransferDTO>(&endpoint, Some(&body), Some(headers));
        Ok(())
    }

    async fn _list_cargo(&self, agent_token: &String, ship_symbol: &String) -> anyhow::Result<()> {
        let endpoint: String = format!("my/ships/{}/cargo", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        self.st.get_with_headers::<CargoDataEnvelopeDTO>(&endpoint, Some(headers)).await?;
        Ok(())
    }
}