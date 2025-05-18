use std::sync::Arc;
use anyhow::Result;
use clap::{Parser, Subcommand};
use lib::ServerService;
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
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config_service = ConfigService::new()?;
    let cfg: Arc<Config> = config_service.settings();

    let spacetraders_service: Arc<SpaceTradersService> = Arc::new(SpaceTradersService::new(cfg.clone()).await?);
    let server_service: Arc<ServerService>= Arc::new(ServerService::new(spacetraders_service.clone(), cfg.clone()));
    let agent_service = Arc::new(AgentService::new(cfg.clone(), spacetraders_service.clone()));
    let contract_service = Arc::new(ContractService::new(cfg.clone(), agent_service.clone(), spacetraders_service.clone()));
    let faction_service = Arc::new(FactionService::new(cfg.clone(), spacetraders_service.clone()));


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
            subcommands::definitions::contract_action(&contract_service, &target);
        }

        Commands::Agent { target } => {
            subcommands::definitions::agent_actions(&agent_service, &target).await;
        }

        Commands::Faction { target } => {
            subcommands::definitions::faction_action(&faction_service, &target).await;
        }

        Commands::Navigate { target } => {
            subcommands::definitions::navigate(&target);
        }
    }

    Ok(())
}
