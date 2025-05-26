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
        Find {
            #[arg(short, long)]
            contract_id: String
        },
        Fulfill {
            #[arg(short, long)]
            contract_id: String
        },
        Deliver {
            #[arg(short, long)]
            contract_id: String
        }
    }


    #[derive(Subcommand)]
    pub enum AgentCmd {
        Activate {
            #[arg(short, long)]
            agent_id: String
        },
        Deactivate,
        Show {
            #[arg(short, long, required = false)]
            agent_id: Option<String>,
            #[arg(short, long, action = clap::ArgAction::SetTrue)]
            mine: bool

        },
        Search {
            #[arg(short, long)]
            symbol: Option<String>
        },
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
    pub enum SystemCmd {
        List,
        Details {
            #[arg(short, long)]
            system: String
        },
        Waypoints {
            #[arg(short, long)]
            system: String
        },
        Waypoint {
            #[arg(short, long)]
            waypoint: String
        }
    }

    #[derive(Subcommand)]
    pub enum WaypointCmd {
        List {
            #[arg(short, long)]
            system: String
        },
        Details {
            #[arg(short, long)]
            waypoint: String
        },
        Jumpgate {
            #[arg(short, long)]
            waypoint: String
        },
        Market {
            #[arg(short, long)]
            waypoint: String
        },
        Construction {
            #[arg(short, long)]
            waypoint: String
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

    #[derive(Subcommand)]
    pub enum MarketCmd {
        Supply
    }
}


pub mod definitions {
    use anyhow::Result;
    use tracing_subscriber::field::MakeExt;
    use crate::{model::faction_model::Faction, services::contract::ContractService};
    use crate::services::agent::AgentService;
    use crate::services::faction::{self, FactionService};
    use crate::{Config, MarketService, ServerService, SystemService};
    use super::declarations::{AgentCmd, ContractCmd, FactionCmd, MarketCmd, NavigateCmd, ServerCmd, ShowCmd, SystemCmd};

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

    pub async fn contract_action(contract_svc: &ContractService, target: &ContractCmd) -> Result<()> {
        match target {
            ContractCmd::Accept { contract_id} => contract_svc.accept_contract(contract_id).await?,
            ContractCmd::Negotiate { contract_id } => contract_svc.negotiate_contract(contract_id).await?,
            ContractCmd::Current => contract_svc.show_current_contracts().await?,
            ContractCmd::Find { contract_id} => contract_svc.find_contract(contract_id).await?,
            ContractCmd::Fulfill { contract_id } => contract_svc.fulfill_contract(contract_id).await?,
            ContractCmd::Deliver { contract_id } => contract_svc.deliver_contract(contract_id).await?
        }

        Ok(())
    }

    pub async fn agent_actions(agent_svc: &AgentService, target: &AgentCmd) -> Result<()> {
        match target {
            AgentCmd::Activate { agent_id } => agent_svc.activate_agent(agent_id).await?,
            AgentCmd::Deactivate => agent_svc.deactivate_agent().await?,
            AgentCmd::Delete { agent_id} => agent_svc.delete_agent(agent_id).await,
            AgentCmd::Show { agent_id, mine } => agent_svc.find_agent(agent_id, mine).await?,
            AgentCmd::Search { symbol} => agent_svc.list_agents(symbol).await?,
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

    pub async fn market_action(market_svc: &MarketService, target: &MarketCmd) -> anyhow::Result<()> {
        match target {
            MarketCmd::Supply => market_svc.get_market_supply_chain().await
        }
    }

    pub async fn system_actions(system_svc: &SystemService, target: &SystemCmd) ->anyhow::Result<()> {
        match target {
            SystemCmd::List => system_svc.list_systems().await,
            SystemCmd::Details { system } => system_svc.get_system(system).await,
            SystemCmd::Waypoints { system, } => system_svc.list_waypoints_by_system(system).await,
            SystemCmd::Waypoint { waypoint } => system_svc.get_waypoint(waypoint).await
        }
    }
}