use std::sync::Arc;
use crate::config::Config;
use crate::{RegisterDataDTO, SpaceTradersService};
use crate::{AgentDTO, AgentRequestDTO, RegisterEnvelopeDTO};
use crate::model::agent_model::Agent;


#[derive(Clone)]
pub struct AgentService {
    cfg: Arc<Config>,
    st: Arc<SpaceTradersService>,
}

impl AgentService {
    pub fn new(cfg: Arc<Config>, st: Arc<SpaceTradersService>) -> Self {
        Self { cfg, st }
    }

    pub async fn register_new_agent(&self, symbol: &String, faction: &String, email: &Option<String>) -> anyhow::Result<(), ()> {
        let new_agent: Result<RegisterDataDTO, anyhow::Error> = self._register_new_agent(symbol, faction, email).await;
        println!("Result: {new_agent:?}");
        Ok(())
    }

    pub fn activate_agent(&self, agent_id: &String) {
        let agent: String = self._find_agent_by_id(agent_id);
        println!("Handling - Activating agent {agent}");
    }

    pub fn deactivate_agent(&self) {
        let agent: String = self._get_current_selected_agent();
        println!("Handling - Activating agent {agent}");
    }

    pub fn delete_agent(&self, agent_id: &String) {
        let agent: String = self._find_agent_by_id(agent_id);
        println!("Handling - Deleting agent {agent} by id");
    }

    pub fn find_agent_by_id(&self, agent_id: &String) {
        let agent: String = self._find_agent_by_id(agent_id);
        println!("Handling - Find Agent {agent} by id");
    }

    pub fn find_current_agent(&self) -> String {
        println!("Handling finding current agent");
        self._get_current_selected_agent()
    }

    pub fn list_agents(&self) {
        println!("Handling listing allagents");
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

    fn _find_agent_by_id(&self, agent_id: &String) -> String {
        String::from("Fake Agent - {agent_id}")
    }


    fn _list_all_agents(&self) {
        println!("private - listing all agents");
    }

    fn _get_current_selected_agent(&self) -> String {
        println!("private - getting currently selected agent");
        String::from("Fake Agent")
    }
}
