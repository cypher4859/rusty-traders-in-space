use std::sync::Arc;
use anyhow::Result;
use clap::{Parser, Subcommand};
use lib::CargoService;
use lib::ModuleService;
use lib::MountService;
use lib::ScanService;
use lib::ServerService;
use lib::ShipService;
use lib::SystemService;
use lib::NavigateService;
use tracing::{info, error};
use lib::SpaceTradersService;
use lib::AgentService;
use lib::ConfigService;
use lib::ContractService;
use lib::FactionService;
use lib::subcommands;
use lib::Config;


#[derive(Parser)]
#[command(name = "spacetraders", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Server {
        #[command(subcommand)]
        target: subcommands::declarations::ServerCmd
    },
    Whereami,
    Greet {
        #[arg(short, long, default_value = "world")]
        name: String
    },
    Show {
        #[command(subcommand)]
        target: subcommands::declarations::ShowCmd
    },
    Contract {
        #[command(subcommand)]
        target: subcommands::declarations::ContractCmd
    },
    Agent {
        #[command(subcommand)]
        target: subcommands::declarations::AgentCmd
    },
    Faction {
        #[command(subcommand)]
        target: subcommands::declarations::FactionCmd
    },
    Navigate {
        #[command(subcommand)]
        target: subcommands::declarations::NavigateCmd
    },
    System {
        #[command(subcommand)]
        target: subcommands::declarations::SystemCmd
    },
    Waypoint {
        #[command(subcommand)]
        target: subcommands::declarations::WaypointCmd
    },
    Ship {
        #[command(subcommand)]
        target: subcommands::declarations::ShipCmd
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config_service = ConfigService::new()?;
    let cfg: Arc<Config> = config_service.settings();

    let spacetraders_service: Arc<SpaceTradersService> = Arc::new(SpaceTradersService::new(cfg.clone()).await?);
    let server_service: Arc<ServerService> = Arc::new(ServerService::new(spacetraders_service.clone(), cfg.clone()));
    let agent_service = Arc::new(AgentService::new(cfg.clone(), spacetraders_service.clone()));
    let contract_service = Arc::new(ContractService::new(cfg.clone(), agent_service.clone(), spacetraders_service.clone()));
    let faction_service = Arc::new(FactionService::new(cfg.clone(), spacetraders_service.clone()));
    let system_service = Arc::new(SystemService::new(cfg.clone(), spacetraders_service.clone(), agent_service.clone()));
    let cargo_service = Arc::new(CargoService::new(cfg.clone(), spacetraders_service.clone()));
    let scanner_service = Arc::new(ScanService::new(cfg.clone(), spacetraders_service.clone()));
    let navigator_service = Arc::new(NavigateService::new(cfg.clone(), spacetraders_service.clone()));
    let mount_service = Arc::new(MountService::new(cfg.clone(), spacetraders_service.clone()));
    let module_service = Arc::new(ModuleService::new(cfg.clone(), spacetraders_service.clone()));
    let ship_service = Arc::new(ShipService::new(
        cfg.clone(), 
        spacetraders_service.clone(), 
        agent_service.clone(),
        cargo_service.clone(),
        scanner_service.clone(),
        navigator_service.clone(),
        mount_service.clone(),
        module_service.clone()
    ));

    // Match the CLI  command
    match cli.command {
        Commands::Server { target } => {
            subcommands::definitions::server(&target, &server_service).await;
        }

        Commands::Whereami => {
            subcommands::definitions::whereami(&cfg);
        }

        Commands::Greet { name } => {
            info!("Greeting {name}");
            subcommands::definitions::greet(&name);
        }

        Commands::Show { target } => {
            // info!("Showing {target}");
            subcommands::definitions::show(&target, &contract_service, &agent_service);
        }

        Commands::Contract { target } => {
            subcommands::definitions::contract_action(&contract_service, &target).await;
        }

        Commands::Agent { target } => {
            subcommands::definitions::agent_actions(&agent_service, &target).await;
        }

        Commands::Faction { target } => {
            subcommands::definitions::faction_action(&faction_service, &target).await;
        }

        Commands::Navigate { target } => {
            subcommands::definitions::navigate(&ship_service, &target);
        }

        Commands::System { target } => {
            subcommands::definitions::system_actions(&system_service, &target).await;
        }

        Commands::Waypoint { target } => {
            subcommands::definitions::waypoint_actions(&system_service, &target).await;
        }

        Commands::Ship { target } => {
            subcommands::definitions::ship_actions(&ship_service, &target).await;
        }
    }

    Ok(())
}
