use std::{process::Output, sync::Arc};
use serde::{Deserialize, Serialize};
use figment::{
    providers::{Format, Serialized, Toml, Json, Env },
    Figment,
    Error as FigmentError
};
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[clap(rename_all = "kebab_case")]        // accepts json / table (case-insens.)
pub enum OutputMode {
    Json,
    Table,
}

impl Default for OutputMode {
    fn default() -> Self { OutputMode::Json }   // <── fallback for Config::default()
}

#[derive(Debug, Deserialize, Default, Serialize, Parser)]
pub struct Config {
    #[clap(short = 'u', long, value_parser)]
    pub api_base_url: String,

    #[clap(short = 't', long, value_parser)]
    pub api_token: String,

    #[clap(short = 'm', long, value_enum, default_value_t)]
    pub output_mode: OutputMode,

    #[clap(short = 't', long, value_parser)]
    pub db_path: String
}


impl Config {
    // Note the `nested` option on both `file` providers. This makes each
    // top-level dictionary act as a profile.
    pub fn figment() -> Figment {
        return Figment::new()
            .merge(Toml::file("Settings.toml"))
            .merge(Env::prefixed("SPACETRADERS_"))
            .join(Json::file("Settings.json"));
    }
}

pub struct ConfigService {
    settings: Arc<Config>,
}

impl ConfigService {
    /// Build the service and load the config in one step.
    pub fn new() -> Result<Self, FigmentError> {
        let cfg: Config = Config::figment().extract()?;   // may fail
        Ok(Self { settings: Arc::new(cfg) })
    }

    /// Immutable access to the loaded config.
    pub fn settings(&self) -> Arc<Config> {
        Arc::clone(&self.settings)
    }
}