use std::sync::Arc;
use crate::config::Config;
use crate::dto::responses::agent_dto::{AgentDataDTO, AgentEnvelopeWithMetaDTO};
use crate::helpers::table_helpers::TableRow;
use crate::{RegisterDataDTO, SpaceTradersService};
use crate::{AgentDTO, AgentRequestDTO, RegisterEnvelopeDTO, AgentEnvelopeDTO};
use crate::model::agent_model::{self, Agent};
use comfy_table::{Row, Table};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use anyhow::{Result, bail};
use rusqlite::{params, Connection};


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
        self._register_new_agent(symbol, faction, email).await
    }

    pub async fn activate_agent(&self, agent_id: &String) -> anyhow::Result<()> {
        // TODO: This looks wayyy too much for a public function
        let agent = match self._find_agent_by_id(agent_id, true).await {
            Ok(a) => a,
            Err(e) => {
                println!("Error {e}");
                return Err(e);
            }
        };

        // JSON1 extension: json_extract(json,'$.active') → 0 / 1 / NULL
        // `filter` must *start with* WHERE because dump_table_from_tb
        // simply splices it after the table name.
        self._activate_agent(agent_id)
    }

    fn _activate_agent(&self, agent_id: &String) -> anyhow::Result<()> {
        let filter_get_agent = format!("WHERE id = '{}'", agent_id.to_uppercase());
        let mut db_agent = self.st.get_table_from_db::<Agent, Vec<&str>>(Some(filter_get_agent.as_str()), vec![])?;
        if (db_agent.len() > 1) {
            bail!("ID conflict, there's more than one agent with that ID, something is seriously wrong");
        } else if (db_agent.len() == 1) {
            let mut the_agent = db_agent.pop().expect("No Agent, serious bail").clone();
            if (the_agent.active) {
                println!("That agent is already active!");
            } else {
                self._deactivate_all_agents();
                the_agent.active = true;
                self.st.save_to_db(&the_agent, |a: &Agent| a.symbol.clone());
            }
    
            self.st.display_db_results_as_table(vec![the_agent]);
            Ok(())
        } else {
            bail!("No agent was found with that ID");
        }
    }

    fn _deactivate_all_agents(&self) -> anyhow::Result<()> {
        let filter_get_all_active_agents = format!("WHERE json_extract(json,'$.active') = 1");
        let mut db_agents = self.st.get_table_from_db::<Agent, Vec<&str>>(Some(filter_get_all_active_agents.as_str()), vec![])?;
        for mut each_db_agent in db_agents {
            each_db_agent.active = false;
            self.st.save_to_db(&each_db_agent, |a: &Agent| a.symbol.clone());
        }

        Ok(())
    }
    
    pub fn deactivate_agent(&self, show_secrets: &bool) -> anyhow::Result<()> {
        // let agent: Agent = self._get_current_selected_agent().await?;
        println!("Handling - Deactivating agent");
        self._deactivate_all_agents();
        self._list_all_agents_from_db(false, show_secrets);
        Ok(())
    }

    pub async fn delete_agent(&self, agent_id: &String) {
        let agent: anyhow::Result<Agent> = self._find_agent_by_id(agent_id, true).await;
    }

    pub async fn find_agent(
        &self,
        agent_id: &Option<String>,
        mine:     &bool,
        show_secrets: &bool
    ) -> anyhow::Result<()> {
        // Treat “mine: None” as false for simpler pattern-matching
        match (agent_id, mine) {
            // 1. agent_id provided, mine == false → specific agent
            (Some(id), false) => {
                self._find_agent_by_id(id, true).await;
            }
    
            // 2. mine == true, no agent_id        → show *all* my agents
            (None, true) => {
                self._list_all_agents_from_db(false, show_secrets);
                // let agent_token = self._get_current_selected_agent_token().await?;
                // self._find_agent_by_token(&agent_token).await?;
            }
    
            // 3. both mine && agent_id            → filter within my agents
            (Some(id), true) => {
                self._find_agent_by_id(id, true).await;
            }
    
            // 4. nothing to work with             → return error
            (None, false) => {
                self._list_all_agents().await;
            }
        }
    
        Ok(())
    }

    pub async fn get_token_by_agent_symbol(&self, agent_id: &String) -> anyhow::Result<String> {
        self._get_token_of_agent(agent_id).await
    }

    pub async fn find_current_agent(&self) -> anyhow::Result<Agent> {
        println!("Handling finding current agent");
        self._get_current_selected_agent()
    }

    pub async fn get_current_selected_agent_token(&self) -> anyhow::Result<String> {
        self._get_current_selected_agent_token().await
    }

    // TODO: This is just wrong, list_agents should return a Vec of Agents
    pub async fn list_agents(&self, symbol: &Option<String>, show_secrets: &bool) -> anyhow::Result<()> {
        match symbol {
            // ① a specific symbol was supplied → get just that agent
            Some(sym) => {
                let agent = self._find_agent_by_id(sym, true).await?;
            }
    
            // ② no symbol → list them all
            None => {
                let agents = self._list_all_agents_from_db(false, show_secrets);
            }
        }

        Ok(())   
    }

    pub async fn get_agent(&self, agent_id: &Option<String>) -> anyhow::Result<Agent> {
        match agent_id {
            Some(id) => {
                self._find_agent_by_id(id, true).await
            }

            None => {
                self._get_current_selected_agent()
            }
        }
    }

    pub async fn sync_db_agents_with_api(&self, display_results: bool) -> anyhow::Result<()> {
        self._sync_db_agents_with_api(display_results).await
    }

    async fn _sync_db_agents_with_api(&self, display_results: bool) -> anyhow::Result<()> {
        let filter = format!("WHERE json_extract(json,'$.is_archived') = 0");
        let results = self.st.get_table_from_db::<Agent, Vec<&str>>(Some(filter.as_str()), vec![])?;
        let mut output: Vec<Agent> = Vec::<Agent>::new();
        for mut each_agent in results {
            let token = match &each_agent.token {
                Some(t) => {t},
                None => { bail!("Failed to sync db agents with API") }
            };
            if (!self._test_agent_token(token.clone()).await) {
                each_agent.is_archived = true;
                each_agent.active = false;
                self.st.save_to_db(&each_agent, |a: &Agent| a.symbol.clone());
                output.push(each_agent.clone());
            }
        }

        println!("Successfully Synced our agents with the API\nThe following Agent was ARCHIVED!");
        self.st.display_db_results_as_table(output);
        // 1. Loop over agents in the db
        // 2. Get agent details of each agent from the API
        // 3. If token is bad then set the agent's "archived" attribute in the DB to true
        Ok(())
    }

    pub fn save_agent_to_db(&self, agent: &RegisterDataDTO, display_results: bool) -> anyhow::Result<()>
    {
        let agent_model = self._save_agent_to_db(agent)?;
        if (display_results) {
            self.st.display_db_results_as_table(vec![agent_model]);
        }
        Ok(())
    }

    fn _save_agent_to_db(&self, agent: &RegisterDataDTO) -> Result<Agent, anyhow::Error> {
        let a = agent.agent.clone();
        let mut agent_model: Agent = a.try_into()?;
        agent_model.token = Some(agent.token.clone());
        self.st.save_to_db(&agent_model, |a: &Agent| a.symbol.clone());
        Ok(agent_model)
    }
    
    async fn _register_new_agent(&self, symbol: &String, faction: &String, email: &Option<String>) -> anyhow::Result<()> {
        let endpoint: String = String::from("register");
        let agent_request: AgentRequestDTO = AgentRequestDTO { 
            symbol: symbol.clone(), 
            faction: faction.clone(), 
            email: email.clone().map(|e: String| e.to_owned())
        };
        let headers = self.st.get_account_headers()?;
        let dto: RegisterEnvelopeDTO = self.st.post_with_headers(
            &endpoint,
            Some(&AgentRequestDTO::from(agent_request)),
            Some(headers),
            false
        ).await?;
        let agent: RegisterDataDTO = dto.data.try_into()?;
        self._save_agent_to_db(&agent)?;
        self._activate_agent(&agent.agent.symbol);
        Ok(())
        
    }

    async fn _get_agents_from_api(&self) -> anyhow::Result<Option<Vec<AgentDTO>>> {
        let endpoint: String = String::from("agents");
        let result = self.st.get_with_headers_and_paging::<AgentDTO>(&endpoint, None, true).await?;
        Ok(result)
    }

    pub async fn _find_agent_by_token(&self, agent_token: &String) -> anyhow::Result<()> {
        let endpoint = String::from("my/agent");

        let mut hdr = HeaderMap::new();
        hdr.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", agent_token))?,
        );
        match self.st.get_with_headers::<AgentEnvelopeDTO>(&endpoint, Some(hdr), true).await? {
            Some(_) => Ok(()),
            None => bail!("Agent notfound")
        }
    }

    async fn _find_agent_by_id(&self, agent_id: &String, display_api_request: bool) -> anyhow::Result<Agent> {
        let endpoint: String = format!("agents/{}", agent_id.to_uppercase());
        let result = self.st.get::<AgentEnvelopeDTO>(&endpoint, display_api_request).await?;
        match result {
            Some(some_result) => match some_result.data {
                AgentDataDTO::Single(dto) => {
                    println!("Identified agent {}", dto.symbol);
                    Ok(dto.try_into()?)
                },
                AgentDataDTO::List(_) => bail!("Received a list for _find_agent_by_id, should've been a single")
            }

            None => {
                bail!("Failed to find agent by ID!")
            }
        }
        // Ok(String::from("Fake Agent - {agent_id}"))d
    }

    async fn _list_all_agents(&self) -> anyhow::Result<()> {
        self._get_agents_from_api().await;
        Ok(())
    }

    fn _list_all_agents_from_db(&self, include_archived_entries: bool, show_secrets: &bool) -> anyhow::Result<()> {
        self._sync_db_agents_with_api(false);
        // This feels nasty but whatever
        let include_flag = match include_archived_entries {
            true => {
                1
            },
            false => {
                0
            }
        };
        let filter = format!("WHERE json_extract(json,'$.is_archived') = {}", include_flag.to_string());
        if let Err(e) = self.st.dump_table_from_db::<Agent, Vec<&str>>(Some(filter.as_str()), vec![], show_secrets) {
            eprintln!("DB dump failed: {e:#}");
        }

        Ok(())
    }

    fn _get_current_selected_agent(&self) -> anyhow::Result<Agent> {
        let filter_get_active_agents = format!("WHERE json_extract(json,'$.active') = 1");
        let mut db_agent = self.st.get_table_from_db::<Agent, Vec<&str>>(Some(filter_get_active_agents.as_str()), vec![])?;
        if (db_agent.len() > 1) {
            bail!("ID conflict, there's more than one agent with that ID, something is seriously wrong");
        } else if (db_agent.len() == 1) {
            let mut the_agent = db_agent.pop().expect("No Agent, serious bail").clone();
            // self.st.display_results_as_table(vec![the_agent]);
            Ok(the_agent)
        } else {
            bail!("No agent was found with that ID");
        }
    }

    // TODO: This function should loop through our datastore, wherever that is (likely sqlite), and grab the currently selected agent
    async fn _get_current_selected_agent_token(&self) -> anyhow::Result<String> {
        match self._get_current_selected_agent()?.token {
            Some(token) => {
                Ok(token)
            },
            None => {
                bail!("Couldn't get the token for the current selected agent");
            }
        }
    }

    async fn _test_agent_token(&self, token: String) -> bool {
        self._find_agent_by_token(&token).await.is_ok()
    } 

    async fn _get_token_of_agent(&self, agent_id: &String) -> anyhow::Result<String> {
        let filter_get_agent = format!("WHERE id = '{}'", agent_id);
        let mut db_agent = self.st.get_table_from_db::<Agent, Vec<&str>>(Some(filter_get_agent.as_str()), vec![])?;
        if (db_agent.len() > 1) {
            bail!("ID conflict, there's more than one agent with that ID, something is seriously wrong");
        } else if (db_agent.len() == 1) {
            let mut the_agent = db_agent.pop().expect("No Agent, serious bail").clone();
            // self.st.display_results_as_table(vec![the_agent]);
            match the_agent.token {
                Some(token) => {
                    Ok(token)
                },
                None => {
                    let agent_symbol = the_agent.symbol;
                    bail!("Couldn't get the token for the selected agent {agent_symbol}");
                }
            }
        } else {
            bail!("No agent was found with that ID");
        }
    }
}
