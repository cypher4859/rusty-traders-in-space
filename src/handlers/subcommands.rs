pub mod declarations {
    use clap::Subcommand;
    #[derive(Subcommand)]
    pub enum ServerCmd {
        Health
    }

    #[derive(Subcommand)]
    pub enum ShowCmd {
        /// Show a system or waypoint details
        Location {
            #[arg(short, long)]
            location: String,
        },
        /// Show the market details at a waypoint
        Market {
            #[arg(short, long)]
            waypoint: String,
            #[arg(short, long, required = false)]
            ship: Option<String>
        },
        /// Show cargo of a ship
        Cargo {
            #[arg(short, long)]
            ship: String
        },
        /// Show all systems
        Systems,
        /// Show system details
        System {
            #[arg(short, long)]
            system: String
        },
        /// Show the waypoints of a system
        Waypoints {
            #[arg(short, long)]
            system: String,
            #[arg(short, long, required = false)]
            waypoint_type: String
        },
        /// Show waypoint details
        Waypoint {
            #[arg(short, long)]
            waypoint: String
        },
        /// Show contracts of an agent
        Contracts {
            #[arg(short, long, required = false)]
            agent: Option<String>
        },
        /// Show agents
        Agents {
            #[arg(short, long, required = false)]
            name: Option<String>,
            #[arg(short, long, action = clap::ArgAction::SetTrue)]
            show_secrets: bool
        },
        /// Show currently selected agent
        CurrentAgent {
            #[arg(short, long, action = clap::ArgAction::SetTrue)]
            show_secrets: bool
        },
        /// Show all factions
        Factions {
            #[arg(short, long, required = false)]
            name: Option<String>
        },
        Ships {
            #[arg(short, long, required = false)]
            agent: Option<String>
        },
        /// Show all the game items
        Items,
        /// Show all the possible traits
        Traits,
        /// Show all the possible ship frames
        Engines,
        Frames,
        Modules,
        Mounts,
        Reactors,
        WaypointTypes
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
        /// Show current contract
        Current,
        /// Negotiate a contract
        Negotiate {
            #[arg(short, long)]
            contract_id: String
        },
        /// Accept a contract
        Accept {
            #[arg(short, long)]
            contract_id: String
        },
        /// Find a contract by ID
        Find {
            #[arg(short, long)]
            contract_id: String
        },
        /// Fulfill a contract
        Fulfill {
            #[arg(short, long)]
            contract_id: String,
        },
        /// Deliver goods for the contract
        Deliver {
            #[arg(short, long)]
            contract_id: String,
            #[arg(short, long)]
            ship: String,
        }
    }


    #[derive(Subcommand)]
    pub enum AgentCmd {
        /// Sync DB agent list with API by testing their tokens. Useful with server resets
        Sync,
        /// Activate an agent as the default agent for commands
        Activate {
            #[arg(short, long)]
            agent_id: String
        },
        /// Deactivate the currently selected agent
        Deactivate {
            #[arg(short, long, action = clap::ArgAction::SetTrue)]
            show_secrets: bool
        },
        Show {
            #[arg(short, long, required = false)]
            agent_id: Option<String>,
            #[arg(short, long, action = clap::ArgAction::SetTrue)]
            mine: bool,
            #[arg(short, long, action = clap::ArgAction::SetTrue)]
            show_secrets: bool

        },
        /// Search for an agent
        Search {
            #[arg(short, long)]
            symbol: Option<String>,
            #[arg(short, long, action = clap::ArgAction::SetTrue)]
            show_secrets: bool
        },
        /// In-progress
        Delete {
            #[arg(short, long)]
            agent_id: String
        },
        /// Register a new agent
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
        /// List all systems
        List,
        /// Get details of a system
        Details {
            #[arg(short, long)]
            system: String
        },
        /// List all waypoints in a system
        Waypoints {
            #[arg(short, long)]
            system: String
        },
        /// Get waypoint details
        Waypoint {
            #[arg(short, long)]
            waypoint: String
        }
    }

    #[derive(Subcommand)]
    pub enum WaypointCmd {
        /// List all waypoints in a system
        List {
            #[arg(short, long)]
            system: String
        },
        /// Get waypoint details
        Details {
            #[arg(short, long)]
            waypoint: String
        },
        /// hmm...
        Jumpgate {
            #[arg(short, long)]
            waypoint: String
        },
        /// hmm...
        Market {
            #[arg(short, long)]
            waypoint: String,
            #[arg(short, long, required = false)]
            ship: Option<String>
        },
        /// hmm...
        Construction {
            #[arg(short, long)]
            waypoint: String
        }
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
        Details {
            #[arg(short, long)]
            waypoint: String,
            #[arg(short, long, required = false)]
            ship: Option<String>,
        }
    }

    #[derive(Subcommand)]
    pub enum FleetCmd {
        Ships,
        Purchase {
            #[arg(short, long)]
            ship_type: String,
            #[arg(short, long)]
            waypoint: String,
        },
        #[command(subcommand)]
        Ship(ShipCmd),
        // Orbit,
        // Refine,
        // Chart,
        // GetCooldown,
        // Dock,
        // Survey,
        // Extract,
        // Siphon,
        // Jettison,
        // Jump,
        // Warp,
        // Navigate,
        // PatchNavigation,
        // Navigation,
        // #[command(subcommand)]
        // Scan(ScanCmd),
        // Refuel,
        // #[command(subcommand)]
        // Mounts(MountCmd),
        // #[command(subcommand)]
        // Scrap(ScrapCmd),
        // #[command(subcommand)]
        // Repair(RepairCmd),
        // #[command(subcommand)]
        // Modules(ModuleCmd)
    }

    #[derive(Subcommand)]
    pub enum CargoCmd {
        List {
            #[arg(short, long)]
            ship: String,
        },
        Purchase {
            #[arg(short, long)]
            ship: String,
            #[arg(short, long)]
            item: String,
            #[arg(short, long)]
            units: u32
        },
        Sell {
            #[arg(short, long)]
            ship: String,
            #[arg(short, long)]
            item: String,
            #[arg(short, long)]
            units: u32
        },
        Transfer {
            #[arg(short, long)]
            ship: String,
            #[arg(short, long)]
            item: String,
            #[arg(short, long)]
            units: u32
        },
        Jettison {
            #[arg(short, long)]
            ship: String,
            #[arg(short, long)]
            item: String,
            #[arg(short, long)]
            units: u32
        },
    }

    #[derive(Subcommand)]
    pub enum ScanCmd {
        Market {
            #[arg(short, long)]
            ship: String,
            #[arg(short, long)]
            waypoint: String,
        },
        Systems {
            #[arg(short, long)]
            ship: String,
            #[arg(short, long)]
            system: String,
        },
        Waypoints {
            #[arg(short, long)]
            ship: String,
            #[arg(short, long)]
            waypoint: String,
        },
        Ships {
            #[arg(short, long)]
            ship: String,
            #[arg(short, long)]
            target: String,
        }
    }

    #[derive(Subcommand)]
    pub enum ShipCmd {
        Status {
            #[arg(short, long)]
            ship: String,
        },
        #[command(subcommand)]
        Cargo(CargoCmd),
        Chart {
            #[arg(short, long)]
            ship: String,
        },
        #[command(subcommand)]
        Reactor(ReactorCmd),
        #[command(subcommand)]
        Navigate(NavigateCmd),
        #[command(subcommand)]
        Scan(ScanCmd),
        Refuel {
            #[arg(short, long)]
            ship: String,
            #[arg(short, long, action = clap::ArgAction::SetFalse)]
            from_cargo: bool,
            #[arg(short, long, required = false)]
            units: Option<u32>,
        },
        #[command(subcommand)]
        Mounts(MountCmd),
        #[command(subcommand)]
        Scrap(ScrapCmd),
        #[command(subcommand)]
        Repair(RepairCmd),
        #[command(subcommand)]
        Modules(ModuleCmd),
        #[command(subcommand)]
        Resources(ResourcesCmd)
    }

    #[derive(Subcommand)]
    pub enum ReactorCmd {
        Status {
            #[arg(short, long)]
            ship: String,
        }
    }

    #[derive(Subcommand)]
    pub enum ResourcesCmd {
        Refine,
        Survey,
        Extract,
        Siphon,
    }

    #[derive(Subcommand)]
    pub enum NavigateCmd {
        Orbit {
            #[arg(short, long)]
            ship: String,
        },
        Dock {
            #[arg(short, long)]
            ship: String,
        },
        Status {
            #[arg(short, long)]
            ship: String,
        },
        To {
            #[arg(short, long)]
            ship: String,
            #[arg(short, long)]
            waypoint: String,
        },
        SetFlightMode {
            #[arg(short, long)]
            ship: String,
        },
        Warp {
            #[arg(short, long)]
            ship: String,
            #[arg(short, long)]
            waypoint: String,
        },
        Jump {
            #[arg(short, long)]
            ship: String,
            #[arg(short, long)]
            waypoint: String,
        },
    }

    #[derive(Subcommand)]
    pub enum MountCmd {
        List {
            #[arg(short, long)]
            ship: String,
        },
        Install {
            #[arg(short, long)]
            ship: String,
            #[arg(short, long)]
            item: String,
        },
        Remove {
            #[arg(short, long)]
            ship: String,
            #[arg(short, long)]
            item: String,
        }
    }

    #[derive(Subcommand)]
    pub enum ModuleCmd {
        List {
            #[arg(short, long)]
            ship: String,
        },
        Install {
            #[arg(short, long)]
            ship: String,
            #[arg(short, long)]
            item: String,
        },
        Remove {
            #[arg(short, long)]
            ship: String,
            #[arg(short, long)]
            item: String,
        }
    }

    #[derive(Subcommand)]
    pub enum RepairCmd {
        Status {
            #[arg(short, long)]
            ship: String,
        },
        Initiate {
            #[arg(short, long)]
            ship: String
        }
    }

    #[derive(Subcommand)]
    pub enum ScrapCmd {
        Status {
            #[arg(short, long)]
            ship: String,
        },
        Initiate {
            #[arg(short, long)]
            ship: String
        }
    }
}


pub mod definitions {
    use anyhow::{Ok, Result};
    use tracing_subscriber::field::MakeExt;
    use crate::helpers::enum_lookups::{EngineSymbol, FrameSymbol, InventoryItemSymbol, ModuleSymbol, MountSymbol, ReactorSymbol, TraitSymbol, WaypointType};
    use crate::services::dispatchers::ship;
    use crate::{model::faction_model::Faction, services::dispatchers::contract::ContractService};
    use crate::services::dispatchers::agent::AgentService;
    use crate::services::dispatchers::faction::{self, FactionService};
    use crate::{Config, MarketService, NavigateService, ServerService, ShipService, SpaceTradersService, SystemService};

    use super::declarations::{AgentCmd, CargoCmd, ContractCmd, FactionCmd, FleetCmd, MarketCmd, ModuleCmd, MountCmd, NavigateCmd, ReactorCmd, RepairCmd, ResourcesCmd, ScanCmd, ScrapCmd, ServerCmd, ShipCmd, ShowCmd, SystemCmd, WaypointCmd};

    pub async fn server(target: &ServerCmd, server_svc: &ServerService) -> anyhow::Result<()> {
        match target {
            ServerCmd::Health => {server_svc.get_status().await?;},
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

    pub async fn show(
        target: &ShowCmd,
        spacetraders_svc: &SpaceTradersService,
        contract_svc: &ContractService, 
        agent_svc: &AgentService,
        server_svc: &ServerService,
        navigator_svc: &NavigateService,
        faction_svc: &FactionService,
        market_svc: &MarketService,
        system_svc: &SystemService,
        ship_svc: &ShipService
    ) -> anyhow::Result<()> {
        match target {
            ShowCmd::Location { location} => {system_svc.show_location(location).await?;},
            ShowCmd::Market { waypoint, ship } => {system_svc.get_market(waypoint, ship).await?;},
            ShowCmd::Cargo {ship} => {ship_svc.list_cargo(ship).await?;},
            ShowCmd::Systems => {system_svc.list_systems().await?;},
            ShowCmd::System { system} => {system_svc.get_system(system).await?;},
            ShowCmd::Waypoints { system, waypoint_type } => {system_svc.list_waypoints_by_system(system).await?;},
            ShowCmd::Waypoint { waypoint } => {system_svc.get_waypoint(waypoint).await?;},
            ShowCmd::Contracts { agent } => {contract_svc.show_current_contracts(agent).await?;},
            ShowCmd::Agents { name, show_secrets } => {agent_svc.list_agents(name, show_secrets).await?;},
            ShowCmd::CurrentAgent { show_secrets } => {agent_svc.find_agent(&None, &true, show_secrets).await?;},
            ShowCmd::Factions { name } => {faction_svc.show_factions(name).await?;},
            ShowCmd::Ships { agent } => {
                ship_svc.show_ships(agent).await?;
            },
            ShowCmd::Items => { 
                spacetraders_svc.display_enums::<InventoryItemSymbol>(); 
            },
            ShowCmd::Frames => {
                spacetraders_svc.display_enums::<FrameSymbol>();
            },
            ShowCmd::Modules => {
                spacetraders_svc.display_enums::<ModuleSymbol>();
            },
            ShowCmd::Mounts => {
                spacetraders_svc.display_enums::<MountSymbol>();
            },
            ShowCmd::Reactors => {
                spacetraders_svc.display_enums::<ReactorSymbol>();
            },
            ShowCmd::Engines => {
                spacetraders_svc.display_enums::<EngineSymbol>();
            },
            ShowCmd::WaypointTypes => {
                spacetraders_svc.display_enums::<WaypointType>();
            },
            ShowCmd::Traits => {
                spacetraders_svc.display_enums::<TraitSymbol>();
            },
        }
        Ok(())
    }

    pub async fn contract_action(contract_svc: &ContractService, agent_svc: &AgentService, target: &ContractCmd) -> Result<()> {
        match target {
            ContractCmd::Accept { contract_id} => contract_svc.accept_contract(contract_id).await?,
            ContractCmd::Negotiate { contract_id } => contract_svc.negotiate_contract(contract_id).await?,
            ContractCmd::Current => {
                let agent_symbol = agent_svc.get_current_selected_agent_token().await?;
                contract_svc.show_current_contracts(&Some(agent_symbol)).await?
            },
            ContractCmd::Find { contract_id} => contract_svc.find_contract(contract_id).await?,
            ContractCmd::Fulfill { contract_id } => contract_svc.fulfill_contract(contract_id).await?,
            ContractCmd::Deliver { contract_id, ship } => contract_svc.deliver_contract(contract_id, ship).await?
        }

        Ok(())
    }

    pub async fn agent_actions(agent_svc: &AgentService, target: &AgentCmd) -> Result<()> {
        match target {
            AgentCmd::Sync => agent_svc.sync_db_agents_with_api(true).await?,
            AgentCmd::Activate { agent_id } => agent_svc.activate_agent(agent_id).await?,
            AgentCmd::Deactivate { show_secrets } => agent_svc.deactivate_agent(show_secrets)?,
            AgentCmd::Delete { agent_id} => agent_svc.delete_agent(agent_id).await,
            AgentCmd::Show { agent_id, mine, show_secrets} => agent_svc.find_agent(agent_id, mine, show_secrets).await?,
            AgentCmd::Search { symbol, show_secrets} => agent_svc.list_agents(symbol, show_secrets).await?,
            AgentCmd::New { symbol, faction, email} => agent_svc.register_new_agent(symbol, faction, email).await?
        }
        Ok(())
    }

    pub async fn navigate(ship_svc: &ShipService, target: &NavigateCmd) -> anyhow::Result<()> {
        match target {
            NavigateCmd::Orbit { ship } => ship_svc.navigate_orbit(ship).await.map(|_| ()),
            NavigateCmd::Dock { ship } => ship_svc.dock_at_station(ship).await.map(|_| ()),
            NavigateCmd::Status { ship } => ship_svc.get_navigation_status(ship).await.map(|_| ()),
            NavigateCmd::To { ship, waypoint } => ship_svc.navigate_to(ship, waypoint).await.map(|_| ()),
            NavigateCmd::SetFlightMode { ship } => ship_svc.set_flight_mode(ship).await.map(|_| ()),
            NavigateCmd::Warp { ship, waypoint } => ship_svc.warp_ship(ship, waypoint).await.map(|_| ()),
            NavigateCmd::Jump { ship, waypoint } => ship_svc.jump_to_waypoint(ship, waypoint).await.map(|_| ()),
        }
    }

    pub async fn faction_action(faction_svc: &FactionService, target: &FactionCmd) -> anyhow::Result<()> {
        match target {
            FactionCmd::ShowAll => {
                // This should also show the keyword for the faction we can use for other commands
                faction_svc.show_all_factions().await;
            },
            FactionCmd::Search { faction } => { 
                faction_svc.search_factions(faction).await;
            }
        }
        Ok(())
    }

    pub async fn market_action(market_svc: &MarketService, target: &MarketCmd) -> anyhow::Result<()> {
        match target {
            MarketCmd::Details { waypoint, ship  } => market_svc.get_market(waypoint, ship).await.map(|_| ())
        }
    }

    pub async fn system_actions(system_svc: &SystemService, target: &SystemCmd) ->anyhow::Result<()> {
        match target {
            SystemCmd::List => system_svc.list_systems().await.map(|_| ()),
            SystemCmd::Details { system } => system_svc.get_system(system).await.map(|_| ()),
            SystemCmd::Waypoints { system, } => system_svc.list_waypoints_by_system(system).await.map(|_| ()),
            SystemCmd::Waypoint { waypoint } => system_svc.get_waypoint(waypoint).await.map(|_| ())
        }
    }

    pub async fn waypoint_actions(system_svc: &SystemService, target: &WaypointCmd) -> anyhow::Result<()> {
        match target {
            WaypointCmd::List { system } => system_svc.list_waypoints_by_system(system).await.map(|_| ()),
            WaypointCmd::Details { waypoint } => system_svc.get_waypoint(waypoint).await.map(|_| ()),
            WaypointCmd::Jumpgate { waypoint } => system_svc.get_jumpgate(waypoint).await.map(|_| ()),
            WaypointCmd::Market { waypoint , ship } => system_svc.get_market(waypoint, ship).await.map(|_| ()),
            WaypointCmd::Construction { waypoint } => system_svc.get_construction_site(waypoint).await.map(|_| ())
        }
    }

    // TODO: It would be useful to "activate" a specific ship ot save from having to type the name out all the time
    pub async fn ship_actions(ship_svc: &ShipService, target: &ShipCmd) -> anyhow::Result<()> {
        match target {
            ShipCmd::Status { ship } => ship_svc.get_ship(ship).await.map(|_| ()),
            ShipCmd::Cargo(cmd) => match cmd {
                CargoCmd::List { ship } => ship_svc.list_cargo(ship).await.map(|_| ()),
                CargoCmd::Purchase { ship, item, units} => ship_svc.purchase_cargo(ship, item, *units).await.map(|_| ()),
                CargoCmd::Sell { ship, item, units }  => ship_svc.sell_cargo(ship, item, *units).await.map(|_| ()),
                CargoCmd::Transfer { ship, item, units } => ship_svc.transfer_cargo(ship, item, *units).await.map(|_| ()),
                CargoCmd::Jettison { ship, item, units } => ship_svc.jettison_cargo(ship, item, *units).await.map(|_| ())
            },
            ShipCmd::Chart { ship } => ship_svc.create_chart(ship).await.map(|_| ()),
            ShipCmd::Reactor(reactor_cmd) => match reactor_cmd {
                ReactorCmd::Status { ship } => ship_svc.get_reactor_status(ship).await.map(|_| ()),
            },
            // FIXME: This appears broken. Ran a navigate status and it failed silently. Ran a navigate to and it also failed silently
            // Need tests bad
            ShipCmd::Navigate(navigate_cmd) => match navigate_cmd {
                NavigateCmd::Orbit { ship } => ship_svc.navigate_orbit(ship).await.map(|_| ()),
                NavigateCmd::Dock { ship } => ship_svc.dock_at_station(ship).await.map(|_| ()),
                NavigateCmd::Status { ship } => ship_svc.get_navigation_status(ship).await.map(|_| ()),
                NavigateCmd::To { ship, waypoint } => ship_svc.navigate_to(ship, waypoint).await.map(|_| ()),
                NavigateCmd::SetFlightMode { ship } => ship_svc.set_flight_mode(ship).await.map(|_| ()),
                NavigateCmd::Warp { ship, waypoint } => ship_svc.warp_ship(ship, waypoint).await.map(|_| ()),
                NavigateCmd::Jump { ship, waypoint } => ship_svc.jump_to_waypoint(ship, waypoint).await.map(|_| ())
            },
            ShipCmd::Scan(scan_cmd) => match scan_cmd {
                ScanCmd::Market { ship, waypoint } => ship_svc.scan_market(ship, waypoint).await.map(|_| ()),
                ScanCmd::Systems { ship, system } => ship_svc.scan_systems(ship, system).await.map(|_| ()),
                ScanCmd::Waypoints {ship, waypoint } => ship_svc.scan_waypoints(ship, waypoint).await.map(|_| ()),
                ScanCmd::Ships {ship, target } => ship_svc.scan_ships(ship, target).await.map(|_| ()),
            },
            ShipCmd::Refuel { ship, from_cargo, units } => ship_svc.refuel_ship(ship, from_cargo, units).await.map(|_| ()),
            ShipCmd::Mounts(mount_cmd) => match mount_cmd {
                MountCmd::List { ship } => ship_svc.list_mounts(ship).await.map(|_| ()),
                MountCmd::Install { ship, item } => ship_svc.install_mount(ship, item).await.map(|_| ()),
                MountCmd::Remove { ship, item } => ship_svc.remove_mount(ship, item).await.map(|_| ()),
            },
            ShipCmd::Modules(module_cmd) => match module_cmd {
                ModuleCmd::List { ship} => ship_svc.list_modules_by_ship(ship).await.map(|_| ()),
                ModuleCmd::Install { ship, item } => ship_svc.install_module_to_ship(ship, item).await.map(|_| ()),
                ModuleCmd::Remove { ship, item } => ship_svc.remove_module_from_ship(ship, item).await.map(|_| ()),
            },
            ShipCmd::Scrap(scrap_cmd) => match scrap_cmd {
                ScrapCmd::Status { ship} => ship_svc.get_scrap_ship_status(ship).await.map(|_| ()),
                ScrapCmd::Initiate { ship } => ship_svc.initiate_scrap_ship(ship).await.map(|_| ()),
            },
            ShipCmd::Repair(repair_cmd) => match repair_cmd {
                RepairCmd::Status { ship } => ship_svc.get_repair_status(ship).await.map(|_| ()),
                RepairCmd::Initiate { ship} => ship_svc.initiate_repair(ship).await.map(|_| ()),
            },
            ShipCmd::Resources(material_cmd) => match material_cmd {
                ResourcesCmd::Refine => todo!(),
                ResourcesCmd::Survey => todo!(),
                ResourcesCmd::Extract => todo!(),
                ResourcesCmd::Siphon => todo!()
            }
        }
    }
}