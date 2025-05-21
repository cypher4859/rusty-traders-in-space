use std::sync::Arc;
use crate::config::Config;
use crate::AgentService;
use crate::SpaceTradersService;
use crate::Agent;

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

    pub fn accept_contract(&self, contract_id: &String) {
        println!("Handling Accepting contract {contract_id}");
    }

    pub fn negotiate_contract(&self, contract_id: &String) {
        println!("Handling Negotiating contract {contract_id}");
    }

    pub async fn show_current_contracts(&self) -> anyhow::Result<()> {
        println!("Handling showing current contracts");
        let agent: Agent = self.agent_svc.find_current_agent().await?;
        self._list_contracts_owned_by_agent(&agent.get_symbol());
        Ok(())
    }

    fn _list_contracts_owned_by_agent(&self, agent_id: &String) {
        println!("private - Listing contracts owned by {agent_id}");
    }

    fn _find_contract_by_id(&self, contract_id: &String) {
        println!("private - finding contract by id {contract_id}");
    }


    fn _negotiate_contract_by_id(&self, contract_id: &String) {
        println!("private - negotiating contract by id {contract_id}");
    }

    fn _accept_contract_by_id(&self, contract_id: &String) {
        println!("private - accepting contract by id {contract_id}");
    }
}