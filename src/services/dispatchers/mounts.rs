use std::sync::Arc;
use anyhow::bail;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use crate::config::Config;
use crate::dto::requests::mount_request_dto::RequestMountDTO;
use crate::dto::responses::fleet_dto::MountDataEnvelopeEnumDTO;
use crate::dto::responses::fleet_dto::MountInstallDataEnvelopeDTO;
use crate::dto::responses::fleet_dto::MountRemoveDataEnvelopeDTO;
use crate::services::dispatchers::agent;
use crate::services::dispatchers::contract;
use crate::AgentService;
use crate::SpaceTradersService;

#[derive(Clone)]
pub struct MountService {
    cfg: Arc<Config>,
    st: Arc<SpaceTradersService>,
    agent_svc: Arc<AgentService>
}

impl MountService {
    pub fn new(cfg: Arc<Config>, st: Arc<SpaceTradersService>, agent_svc: Arc<AgentService>) -> Self {
        Self { 
            cfg,
            st,
            agent_svc
        }
    }

    pub async fn list_mounts(&self, ship_symbol: &String) -> anyhow::Result<MountDataEnvelopeEnumDTO> {
        self._list_mounts_by_ship(ship_symbol).await
    }

    pub async fn install_mount(&self, ship_symbol: &String, mount_name: &String) -> anyhow::Result<MountInstallDataEnvelopeDTO> {
        self._install_mount_to_ship(ship_symbol, mount_name).await
    }

    pub async fn remove_mount(&self, ship_symbol: &String, mount_name: &String) -> anyhow::Result<MountRemoveDataEnvelopeDTO> {
        self._remove_mount_from_ship(ship_symbol, mount_name).await
    }

    async fn _list_mounts_by_ship(&self, ship_symbol: &String) -> anyhow::Result<MountDataEnvelopeEnumDTO> {
        let agent_token: String = self.agent_svc.get_current_selected_agent_token().await?;
        let endpoint: String = format!("my/ships/{}/mounts/remove", ship_symbol);
        let headers = self.st.get_agent_headers(&agent_token)?;
        let response = self.st.get_with_headers::<MountDataEnvelopeEnumDTO>(&endpoint, Some(headers)).await?;
        match response {
            Some(res) => {
                Ok(res)
            }
            None => {
                bail!("Failed to list mounts for ship {ship_symbol}!");
            }
        }
    }

    async fn _install_mount_to_ship(&self, ship_symbol: &String, mount_name: &String) -> anyhow::Result<MountInstallDataEnvelopeDTO> {
        let agent_token: String = self.agent_svc.get_current_selected_agent_token().await?;
        let endpoint: String = format!("my/ships/{}/mounts/install", ship_symbol);
        let headers = self.st.get_agent_headers(&agent_token)?;
        let body = RequestMountDTO::new(
            mount_name.clone()
        )?;
        let response = self.st.post_with_headers::<MountInstallDataEnvelopeDTO, RequestMountDTO>(&endpoint, Some(&body), Some(headers)).await?;
        Ok(response)
    }

    async fn _remove_mount_from_ship(&self, ship_symbol: &String, mount_name: &String) -> anyhow::Result<MountRemoveDataEnvelopeDTO> {
        let agent_token: String = self.agent_svc.get_current_selected_agent_token().await?;
        let endpoint: String = format!("my/ships/{}/mounts/remove", ship_symbol);
        let headers = self.st.get_agent_headers(&agent_token)?;
        let body = RequestMountDTO::new(
            mount_name.clone()
        )?;
        let response = self.st.post_with_headers::<MountRemoveDataEnvelopeDTO, RequestMountDTO>(&endpoint, Some(&body), Some(headers)).await?;
        Ok(response)
    }
}