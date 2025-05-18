pub mod declarations {
    use clap::Subcommand;
    #[derive(Subcommand)]
    pub enum ServerCmd {
        Health
    }

    #[derive(Subcommand)]
    pub enum ShowCmd {
        Location,
        Inventory,
        Systems,
        System,
        Waypoints,
        Waypoint,
        Contracts,
        Agents,
        Factions,

    }

    #[derive(Subcommand)]
    pub enum WhereamiCmd {
        Location,
        Inventory,
        Systems,
        System,
        Waypoints,
        Waypoint,
        Contracts,
        Agents,
        Factions,

    }

    #[derive(Subcommand)]
    pub enum ContractCmd {
        Current,
        Negotiate {
            #[arg(short, long)]
            contract_id: String
        },
        Accept {
            #[arg(short, long)]
            contract_id: String
        },
    }


    #[derive(Subcommand)]
    pub enum AgentCmd {
        Activate {
            #[arg(short, long)]
            agent_id: String
        },
        Deactivate,
        Show {
            #[arg(short, long)] // TODO: Add default vault to get currently selected Agent to show it
            agent_id: String
        },
        ShowAll,
        Delete {
            #[arg(short, long)]
            agent_id: String
        },
        New {
            #[arg(short, long)]
            symbol: String,
            #[arg(short, long)]
            faction: String,
            #[arg(short, long)]
            email: Option<String>
        }
    }

    #[derive(Subcommand)]
    pub enum NavigateCmd {
        Orbit,
        Dock,
        Navigate,
        SetFlightMode,
        Warp,
        Jump,
        Refuel
    }

    #[derive(Subcommand)]
    pub enum FactionCmd {
        ShowAll,
        Search {
            #[arg(short, long)]
            faction: String
        }
    }
}


pub mod definitions {
    use anyhow::Result;
    use crate::{model::faction_model::Faction, services::contract::ContractService};
    use crate::services::agent::AgentService;
    use crate::services::faction::{self, FactionService};
    use crate::{Config, ServerService};
    use super::declarations::{AgentCmd, ContractCmd, FactionCmd, NavigateCmd, ServerCmd, ShowCmd};

    pub async fn server(target: &ServerCmd, server_svc: &ServerService) -> Result<(), ()> {
        match target {
            ServerCmd::Health => server_svc.get_status().await?,
        }
        Ok(())
    }

    pub fn whereami(settings: &Config) -> Result<()> {
        println!("Handling show whereami");
        Ok(())
    }

    pub fn greet(name: &str) -> Result<()> {
        println!("Fuck you, {name}");
        Ok(())
    }

    pub fn show(target: &ShowCmd, contract_svc: &ContractService, agent_svc: &AgentService) -> Result<()> {
        match target {
            ShowCmd::Location => println!("Handling show location"),
            ShowCmd::Inventory => println!("Handling show inventory"),
            ShowCmd::Systems => println!("Handling show all systems"),
            ShowCmd::System => println!("Handling showing system"),
            ShowCmd::Waypoints => println!("Handling show waypoint"),
            ShowCmd::Waypoint => println!("Handling showing single waypoint"),
            ShowCmd::Contracts => println!("Handling show contracts"),
            ShowCmd::Agents => println!("Handling show agents"),
            ShowCmd::Factions => println!("Handling show factions")
        }
        Ok(())
    }

    pub fn contract_action(contract_svc: &ContractService, target: &ContractCmd) -> Result<()> {
        match target {
            ContractCmd::Accept { contract_id} => contract_svc.accept_contract(contract_id),
            ContractCmd::Negotiate { contract_id} => contract_svc.negotiate_contract(contract_id),
            ContractCmd::Current => {
                // let agent_id: String = find_current_agent();
                let current_contracts = contract_svc.show_current_contracts();
            }
        }

        Ok(())
    }

    pub async fn agent_actions(agent_svc: &AgentService, target: &AgentCmd) -> Result<(),()> {
        match target {
            AgentCmd::Activate { agent_id } => agent_svc.activate_agent(agent_id),
            AgentCmd::Deactivate => agent_svc.deactivate_agent(),
            AgentCmd::Delete { agent_id} => agent_svc.delete_agent(agent_id),
            AgentCmd::Show { agent_id } => agent_svc.find_agent_by_id(agent_id),
            AgentCmd::ShowAll => println!("Handling showing all agents"),
            AgentCmd::New { symbol, faction, email} => agent_svc.register_new_agent(symbol, faction, email).await?
        }
        Ok(())
    }

    pub fn navigate(target: &NavigateCmd) -> Result<()> {
        match target {
            NavigateCmd::Orbit => println!("Handling navigating to orbit"),
            NavigateCmd::Dock => println!("Handling navigating"),
            NavigateCmd::Jump => println!("Handling jump"),
            NavigateCmd::Navigate => println!("Handling navigation"),
            NavigateCmd::Refuel => println!("Handling refueling"),
            NavigateCmd::SetFlightMode => println!("Handling set flight mode"),
            NavigateCmd::Warp => println!("Handling Warp initialization")
        }
        Ok(())
    }

    pub async fn faction_action(faction_svc: &FactionService, target: &FactionCmd) -> Result<(),()> {
        match target {
            FactionCmd::ShowAll => faction_svc.show_all_factions().await?,
            FactionCmd::Search { faction } => faction_svc.search_factions(faction).await?
        }
        Ok(())
    }
}