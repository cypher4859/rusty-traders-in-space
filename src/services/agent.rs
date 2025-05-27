use std::sync::Arc;
use crate::config::Config;
use crate::dto::responses::agent_dto::{AgentDataDTO, AgentEnvelopeWithMetaDTO};
use crate::{RegisterDataDTO, SpaceTradersService};
use crate::{AgentDTO, AgentRequestDTO, RegisterEnvelopeDTO, AgentEnvelopeDTO};
use crate::model::agent_model::Agent;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use anyhow::{Result, bail};


#[derive(Clone)]
pub struct AgentService {
    cfg: Arc<Config>,
    st: Arc<SpaceTradersService>,
}

impl AgentService {
    pub fn new(cfg: Arc<Config>, st: Arc<SpaceTradersService>) -> Self {
        Self { cfg, st }
    }

    pub async fn register_new_agent(&self, symbol: &String, faction: &String, email: &Option<String>) -> anyhow::Result<()> {
        let new_agent: Result<RegisterDataDTO, anyhow::Error> = self._register_new_agent(symbol, faction, email).await;
        Ok(())
    }

    pub async fn activate_agent(&self, agent_id: &String) -> anyhow::Result<()> {
        self._find_agent_by_id(agent_id).await;
        Ok(())
    }

    pub async fn deactivate_agent(&self) -> anyhow::Result<()> {
        let agent: Agent = self._get_current_selected_agent().await?;
        println!("Handling - Activating agent {agent:?}");
        Ok(())
    }

    pub async fn delete_agent(&self, agent_id: &String) {
        let agent: anyhow::Result<String> = self._find_agent_by_id(agent_id).await;
    }

    pub async fn find_agent(
        &self,
        agent_id: &Option<String>,
        mine:     &bool,
    ) -> anyhow::Result<()> {
        // Treat “mine: None” as false for simpler pattern-matching
        match (agent_id, mine) {
            // 1. agent_id provided, mine == false → specific agent
            (Some(id), false) => {
                self._find_agent_by_id(id).await;
            }
    
            // 2. mine == true, no agent_id        → show *all* my agents
            (None, true) => {
                let agent_token = self._get_current_selected_agent_token().await?;
                self._find_agent_by_token(&agent_token).await?;
            }
    
            // 3. both mine && agent_id            → filter within my agents
            (Some(id), true) => {
                println!("This feature is in progress and requires implementing a local database");
            }
    
            // 4. nothing to work with             → return error
            (None, false) => {
                println!("provide an --agent-id or set --mine");
            }
        }
    
        Ok(())
    }

    pub async fn find_current_agent(&self) -> anyhow::Result<Agent> {
        println!("Handling finding current agent");
        self._get_current_selected_agent().await
    }

    pub async fn get_current_selected_agent_token(&self) -> anyhow::Result<String> {
        self._get_current_selected_agent_token().await
    }

    pub async fn list_agents(&self, symbol: &Option<String>) -> anyhow::Result<()> {
        match symbol {
            // ① a specific symbol was supplied → get just that agent
            Some(sym) => {
                let agent = self._find_agent_by_id(sym).await;
            }
    
            // ② no symbol → list them all
            None => {
                let agents = self._list_all_agents().await;
            }
        }

        Ok(())
        
    }

    pub async fn _show_all_of_my_agents(&self) -> anyhow::Result<()> {
        Ok(())
    }

    
    async fn _register_new_agent(&self, symbol: &String, faction: &String, email: &Option<String>) -> anyhow::Result<RegisterDataDTO> {
        let endpoint: String = String::from("register");
        let agent_request: AgentRequestDTO = AgentRequestDTO { 
            symbol: symbol.clone(), 
            faction: faction.clone(), 
            email: email.clone().map(|e: String| e.to_owned())
        };
        let dto: RegisterEnvelopeDTO = self.st.post(
            &endpoint,
            Some(&AgentRequestDTO::from(agent_request))
        ).await?;
        Ok(dto.data.try_into()?)
        
    }

    pub async fn _find_agent_by_token(&self, agent_token: &String) -> anyhow::Result<()> {
        let endpoint = String::from("my/agent");

        let mut hdr = HeaderMap::new();
        hdr.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", agent_token))?,
        );
        self.st.get_with_headers::<AgentEnvelopeDTO>(&endpoint, Some(hdr)).await;
        Ok(())
    }

    async fn _find_agent_by_id(&self, agent_id: &String) -> anyhow::Result<String> {
        let endpoint: String = format!("agents/{}", agent_id.to_uppercase());
        let result = self.st.get::<AgentEnvelopeDTO>(&endpoint).await;
        Ok(String::from("Fake Agent - {agent_id}"))
    }


    async fn _list_all_agents(&self) -> anyhow::Result<(), ()> {
        let endpoint: String = String::from("agents");
        let result = self.st.get::<AgentEnvelopeWithMetaDTO>(&endpoint).await;
        Ok(())
    }

    async fn _get_current_selected_agent(&self) -> anyhow::Result<Agent> {
        println!("private - getting currently selected agent");
        Ok(Agent::new(
            String::from(""),
            String::from("AELINDRACH"),
            175_000_i64,
            String::from("X1-AS18-A1"),
            String::from("SHADOW"),
            2_u32
        )?)
    }

    // TODO: This function should loop through our datastore, wherever that is (likely sqlite), and grab the currently selected agent
    async fn _get_current_selected_agent_token(&self) -> anyhow::Result<String> {
        let Agent = self._get_current_selected_agent().await?;
        Ok(String::from("eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZGVudGlmaWVyIjoiQUVMSU5EUkFDSCIsInZlcnNpb24iOiJ2Mi4zLjAiLCJyZXNldF9kYXRlIjoiMjAyNS0wNS0yNSIsImlhdCI6MTc0ODI3MDM0Niwic3ViIjoiYWdlbnQtdG9rZW4ifQ.VxPsqS6ImAQ46jlA8mJFotT6bPWvC8B2g0rxlVtCGPr0_wjATT7q6ajbctYjWk1q-w5nFsh_crNPIHzmU837cUUtbiZ9KsxxlWhFBQM4n3ynQHdBGfCWs4t6Aa_0Kj8atrDIijIpNcBdmiHd5n3ow0tP4pYjz0g6CODn6-QKgbFKZozERoPxRS0fSl_sutGcYbiz7MiBiQRyXCmzHAA3_jiTHj9X32pDBKmRj9wn7KJCm6vdkNs-9ES9h96PMgQrRlB1RhW2BS9fYnVOv-jUaAnaAuHCOJb5esgXPZN9cYKZGnlqKnl8BLwXxvZ4UkEh7wkXU-w-Q1UbOtlRAHPF3yZHdnE_Ymuxh-BWQGyOzgqvvCwldLaXoL9H_Ble3he6GrkJ_7T4tRYMp0r0nHeaTjL_1uvamHjzIcsgV5Ct7knoIurdTAVmqVopHurpOXuxmYglBlBBCEsYBGbeNez4NIZ3qv_IEG_JdBll3zuV3wfBcUcyfEB911mt0NOth_RJOWKcuYlsJszNrAVr0YDexRELbonpCQnfZiVxcYn0UhucVcU5tu0ai2I9oFYecQklqzyxCBDy4UA8DXsCKJnlZgIoq2TmMGqGFvhv-avE-WzX1DVtu3OnB0ky3p3oZlj8Lz9oz2u8uZyIL72G5a9ZHz4eWxGSh4RwmXPNLvW51H8"))
    }
}
