use std::sync::Arc;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use crate::config::Config;
use crate::dto::requests::contract_request_dto::RequestDeliverDTO;
use crate::services::agent;
use crate::services::contract;
use crate::AgentService;
use crate::SpaceTradersService;
use crate::Agent;
use crate::ContractEnvelopeDTO;

#[derive(Clone)]
pub struct ContractService {
    cfg: Arc<Config>,
    agent_svc: Arc<AgentService>,
    st: Arc<SpaceTradersService>,
}

impl ContractService {
    pub fn new(cfg: Arc<Config>, agent_svc: Arc<AgentService>, st: Arc<SpaceTradersService>) -> Self {
        Self { 
            cfg,
            agent_svc,
            st
        }
    }

    pub async fn accept_contract(&self, contract_id: &String) -> anyhow::Result<()> {
        println!("Handling Accepting contract {contract_id}");
        let agent_token: String = self.agent_svc.get_current_selected_agent_token().await?;
        self._accept_contract_by_id(contract_id, &agent_token).await?;
        Ok(())
    }

    pub async fn fulfill_contract(&self, contract_id: &String) -> anyhow::Result<()> {
        let agent_token: String = self.agent_svc.get_current_selected_agent_token().await?;
        self._fulfill_contract_by_id(contract_id, &agent_token).await?;
        Ok(())
    }

    pub async fn negotiate_contract(&self, contract_id: &String) -> anyhow::Result<()> {
        let agent_token: String = self.agent_svc.get_current_selected_agent_token().await?;
        // self._find_contract_by_id(contract_id, &agent_token).await?;
        self._negotiate_contract_by_id(contract_id, &agent_token).await?;
        Ok(())
    }

    pub async fn show_current_contracts(&self, agent_symbol: &String) -> anyhow::Result<()> {
        println!("Handling showing current contracts");
        let test_agent_symbol = String::from("Test");
        let agent_token: String = self.agent_svc.get_token_by_agent_symbol(&test_agent_symbol).await?;
        self._list_contracts_owned_by_agent(&agent_token).await?;
        Ok(())
    }

    pub async fn find_contract(&self, contract_id: &String) ->anyhow::Result<()> {
        let agent_token: String = self.agent_svc.get_current_selected_agent_token().await?;
        self._find_contract_by_id(contract_id, &agent_token).await?;
        Ok(())
    }

    pub async fn deliver_contract(&self, contract_id: &String) -> anyhow::Result<()> {
        let agent_token = self.agent_svc.get_current_selected_agent_token().await?;
        let ship: String = String::from("");
        self._deliver_contract_by_id(contract_id, &agent_token, &ship).await?;
        Ok(())
    }

    async fn _list_contracts_owned_by_agent(&self, agent_token: &String) -> anyhow::Result<()> {
        let endpoint: String = String::from("my/contracts");
        let mut hdr: HeaderMap =  HeaderMap::new();
        hdr.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", agent_token))?
        );
        self.st.get_with_headers::<ContractEnvelopeDTO>(&endpoint, Some(hdr)).await?;
        Ok(())
    }

    async fn _find_contract_by_id(&self, contract_id: &String, agent_token: &String) -> anyhow::Result<()> {
        println!("private - finding contract by id {contract_id}");
        let endpoint: String = format!("my/contracts/{}", contract_id);
        let mut hdr: HeaderMap =  HeaderMap::new();
        hdr.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", agent_token))?
        );
        self.st.get_with_headers::<ContractEnvelopeDTO>(&endpoint, Some(hdr)).await?;
        Ok(())
    }

    async fn _fulfill_contract_by_id(&self, contract_id: &String, agent_token: &String) -> anyhow::Result<()> {
        let endpoint: String = format!("my/contracts/{}/fulfill", contract_id);
        let mut hdr: HeaderMap =  HeaderMap::new();
        hdr.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", agent_token))?
        );
        self.st.post_with_headers::<ContractEnvelopeDTO, ()>(&endpoint, None, Some(hdr)).await?;
        Ok(())
    }

    async fn _deliver_contract_by_id(&self, contract_id: &String, agent_token: &String, ship_symbol: &String) -> anyhow::Result<()> {
        let endpoint: String = format!("my/contracts/{}/fulfill", contract_id);
        let mut hdr: HeaderMap =  HeaderMap::new();
        hdr.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", agent_token))?
        );

        let units = 0_i64;

        let body= RequestDeliverDTO::new(
            contract_id,
            units,
            ship_symbol
        )?;
        self.st.post_with_headers::<ContractEnvelopeDTO, RequestDeliverDTO>(&endpoint, Some(&body), Some(hdr)).await?;
        Ok(())
    }


    async fn _negotiate_contract_by_id(&self, contract_id: &String, agent_token: &String) -> anyhow::Result<()> {
        println!("private - negotiating contract by id {contract_id}");
        Ok(())
    }

    async fn _accept_contract_by_id(&self, contract_id: &String, agent_token: &String) -> anyhow::Result<()> {
        println!("private - accepting contract by id {contract_id}");
        let endpoint: String = format!("my/contracts/{}/accept", contract_id);
        let mut hdr: HeaderMap =  HeaderMap::new();
        hdr.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", agent_token))?
        );
        self.st.post_with_headers::<ContractEnvelopeDTO, ()>(&endpoint, None, Some(hdr)).await?;
        Ok(())
    }
}