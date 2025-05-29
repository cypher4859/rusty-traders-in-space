use std::sync::Arc;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use crate::config::Config;
use crate::dto::responses::fleet_dto::CooldownEnvelopeDTO;
use crate::dto::responses::system_dto::ChartDataEnvelopeDTO;
use crate::services::agent;
use crate::services::contract;
use crate::AgentService;
use crate::CargoService;
use crate::ModuleService;
use crate::MountService;
use crate::NavigateService;
use crate::ScanService;
use crate::SpaceTradersService;

#[derive(Clone)]
pub struct ShipService {
    cfg: Arc<Config>,
    st: Arc<SpaceTradersService>,
    agent_svc: Arc<AgentService>,
    cargo: Arc<CargoService>,
    scanner: Arc<ScanService>,
    navigator: Arc<NavigateService>,
    mount: Arc<MountService>,
    module: Arc<ModuleService>
}

impl ShipService {
    pub fn new(cfg: Arc<Config>, st: Arc<SpaceTradersService>, 
        agent_svc: Arc<AgentService>,
        cargo: Arc<CargoService>,
        scanner: Arc<ScanService>,
        navigator: Arc<NavigateService>,
        mount: Arc<MountService>,
        module: Arc<ModuleService>
    ) -> Self {
        Self { 
            cfg,
            st,
            agent_svc,
            cargo,
            scanner,
            navigator,
            mount,
            module
        }
    }

    pub async fn list_cargo(&self, ship_symbol: &String) -> anyhow::Result<()> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        self.cargo.list_cargo(&agent_token, ship_symbol).await?;
        Ok(())
    }

    pub async fn purchase_cargo(&self, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        self.cargo.purchase_cargo(&agent_token, ship_symbol, cargo_symbol, cargo_units).await?;
        Ok(())
    }

    pub async fn jettison_cargo(&self, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        self.cargo.jettison_cargo(&agent_token, ship_symbol, cargo_symbol, cargo_units).await?;
        Ok(())
    }

    pub async fn sell_cargo(&self, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        self.cargo.sell_cargo(&agent_token, ship_symbol, cargo_symbol, cargo_units).await?;
        Ok(())
    }

    pub async fn transfer_cargo(&self, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        self.cargo.transfer_cargo(&agent_token, ship_symbol, cargo_symbol, cargo_units).await?;
        Ok(())
    }

    pub async fn navigate_orbit(&self) -> anyhow::Result<()> {
        self.navigator.navigate_orbit().await?;
        Ok(())
    }

    pub async fn dock_at_station(&self) -> anyhow::Result<()> {
        self.navigator.dock_at_station().await?;
        Ok(())
    }

    pub async fn get_navigation_status(&self) -> anyhow::Result<()> {
        self.navigator.get_navigation_status().await?;
        Ok(())
    }

    pub async fn set_flight_mode(&self) -> anyhow::Result<()> {
        self.navigator.set_flight_mode().await?;
        Ok(())
    }

    pub async fn warp_ship(&self) -> anyhow::Result<()> {
        self.navigator.warp_ship().await?;
        Ok(())
    }

    pub async fn jump_to_waypoint(&self) -> anyhow::Result<()> {
        self.navigator.jump_to_waypoint().await?;
        Ok(())
    }

    pub async fn scan_systems(&self) -> anyhow::Result<()> {
        self.scanner.scan_systems().await?;
        Ok(())
    }

    pub async fn scan_waypoints(&self) -> anyhow::Result<()> {
        self.scanner.scan_waypoints().await?;
        Ok(())
    }

    pub async fn scan_ships(&self) -> anyhow::Result<()> {
        self.scanner.scan_ships().await?;
        Ok(())
    }

    pub async fn list_mounts(&self) -> anyhow::Result<()> {
        self.mount.list_mounts().await?;
        Ok(())
    }

    pub async fn install_mount(&self) -> anyhow::Result<()> {
        self.mount.install_mount().await?;
        Ok(())
    }

    pub async fn remove_mount(&self) -> anyhow::Result<()> {
        self.mount.remove_mount().await?;
        Ok(())
    }

    pub async fn get_scrap_ship_status(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn initiate_scrap_ship(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn list_modules_by_ship(&self) -> anyhow::Result<()> {
        self.module.list_modules_by_ship().await?;
        Ok(())
    }

    pub async fn install_module_to_ship(&self) -> anyhow::Result<()> {
        self.module.install_module_to_ship().await?;
        Ok(())
    }

    pub async fn remove_module_from_ship(&self) -> anyhow::Result<()> {
        self.module.remove_module_from_ship().await?;
        Ok(())
    }

    pub async fn refuel_ship(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn get_repair_status(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn initiate_repair(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn create_chart(&self, ship_symbol: &String) -> anyhow::Result<()> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        self._create_chart(&agent_token, ship_symbol).await?;
        Ok(())
    }

    pub async fn get_reactor_status(&self, ship_symbol: &String) -> anyhow::Result<()> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        self._get_reactor_status(&agent_token, ship_symbol).await?;
        Ok(())
    }

    pub fn _get_agent_symbol_by_ship_symbol(&self, ship_symbol: &String) -> String {
        // Ship Symbol ismade up for `[AGENT_SYMBOL]-[HEX_ID]` so we can split it out
        let mut parts = ship_symbol.splitn(2, "-");
        parts.next().unwrap().to_string()
    }

    pub async fn _create_chart(&self, agent_token: &String, ship_symbol: &String) -> anyhow::Result<()> {
        let endpoint: String = format!("my/ships/{}/chart", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        self.st.post_with_headers::<ChartDataEnvelopeDTO, ()>(&endpoint, None, Some(headers)).await?;
        Ok(())
    }

    pub async fn _get_reactor_status(&self, agent_token: &String, ship_symbol: &String) -> anyhow::Result<()> {
        let endpoint: String = format!("my/ships/{}/cooldown", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        self.st.get_with_headers::<CooldownEnvelopeDTO>(&endpoint, Some(headers)).await?;
        Ok(())
    }
}