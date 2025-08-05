use std::sync::Arc;
use anyhow::bail;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use crate::config::Config;
use crate::dto::requests::module_request_dto::RequestModuleDTO;
use crate::dto::responses::fleet_dto::{ModuleDataEnvelopeEnumDTO, ModuleInstallDataEnvelopeDTO, ModuleRemoveDataEnvelopeDTO};
use crate::services::dispatchers::agent;
use crate::services::dispatchers::contract;
use crate::AgentService;
use crate::SpaceTradersService;

#[derive(Clone)]
pub struct ModuleService {
    cfg: Arc<Config>,
    st: Arc<SpaceTradersService>,
    agent_svc: Arc<AgentService>
}

impl ModuleService {
    pub fn new(cfg: Arc<Config>, st: Arc<SpaceTradersService>, agent_svc: Arc<AgentService>) -> Self {
        Self { 
            cfg,
            st,
            agent_svc
        }
    }

    pub async fn list_modules_by_ship(&self, ship_symbol: &String) -> anyhow::Result<ModuleDataEnvelopeEnumDTO> {
        self._list_modules_by_ship(ship_symbol).await
    }

    pub async fn install_module_to_ship(&self, ship_symbol: &String, module_name: &String) -> anyhow::Result<ModuleInstallDataEnvelopeDTO> {
        self._install_module_to_ship(ship_symbol, module_name).await
    }

    pub async fn remove_module_from_ship(&self, ship_symbol: &String, module_name: &String) -> anyhow::Result<ModuleRemoveDataEnvelopeDTO> {
        self._remove_module_from_ship(ship_symbol, module_name).await
    }

    async fn _list_modules_by_ship(&self, ship_symbol: &String) -> anyhow::Result<ModuleDataEnvelopeEnumDTO> {
        let agent_token: String = self.agent_svc.get_current_selected_agent_token().await?;
        let endpoint: String = format!("my/ships/{}/modules/remove", ship_symbol);
        let headers = self.st.get_agent_headers(&agent_token)?;
        let response = self.st.get_with_headers::<ModuleDataEnvelopeEnumDTO>(&endpoint, Some(headers), true).await?;
        match response {
            Some(res) => {
                Ok(res)
            }
            None => {
                bail!("Failed to list modules for ship {ship_symbol}!");
            }
        }
    }

    async fn _install_module_to_ship(&self, ship_symbol: &String, module_name: &String) -> anyhow::Result<ModuleInstallDataEnvelopeDTO> {
        let agent_token: String = self.agent_svc.get_current_selected_agent_token().await?;
        let endpoint: String = format!("my/ships/{}/modules/install", ship_symbol);
        let headers = self.st.get_agent_headers(&agent_token)?;
        let body = RequestModuleDTO::new(
            module_name.clone()
        )?;
        let response = self.st.post_with_headers::<ModuleInstallDataEnvelopeDTO, RequestModuleDTO>(&endpoint, Some(&body), Some(headers), true).await?;
        Ok(response)
    }

    async fn _remove_module_from_ship(&self, ship_symbol: &String, module_name: &String) -> anyhow::Result<ModuleRemoveDataEnvelopeDTO> {
        let agent_token: String = self.agent_svc.get_current_selected_agent_token().await?;
        let endpoint: String = format!("my/ships/{}/modules/remove", ship_symbol);
        let headers = self.st.get_agent_headers(&agent_token)?;
        let body = RequestModuleDTO::new(
            module_name.clone()
        )?;
        let response = self.st.post_with_headers::<ModuleRemoveDataEnvelopeDTO, RequestModuleDTO>(&endpoint, Some(&body), Some(headers), true).await?;
        Ok(response)
    }
}