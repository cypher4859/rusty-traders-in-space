use std::sync::Arc;
use anyhow::bail;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use crate::config::Config;
use crate::dto::requests::fuel_request_dto::RequestRefuelShipDTO;
use crate::dto::responses::fleet_dto::CooldownEnvelopeDTO;
use crate::dto::responses::fleet_dto::ModuleDataEnvelopeEnumDTO;
use crate::dto::responses::fleet_dto::ModuleInstallDataEnvelopeDTO;
use crate::dto::responses::fleet_dto::ModuleRemoveDataEnvelopeDTO;
use crate::dto::responses::fleet_dto::MountDataEnvelopeEnumDTO;
use crate::dto::responses::fleet_dto::MountInstallDataEnvelopeDTO;
use crate::dto::responses::fleet_dto::MountRemoveDataEnvelopeDTO;
use crate::dto::responses::fleet_dto::ShipDataEnvelopeDTO;
use crate::dto::responses::fleet_dto::ShipRefuelDataEnvelopeDTO;
use crate::dto::responses::fleet_dto::ShipRepairDataEnvelopeDTO;
use crate::dto::responses::fleet_dto::ShipRepairStatusDataEnvelopeDTO;
use crate::dto::responses::fleet_dto::ShipScrapDataEnvelopeDTO;
use crate::dto::responses::fleet_dto::ShipScrapStatusDataEnvelopeDTO;
use crate::dto::responses::nav_dto::NavigateDockDataEnvelopeDTO;
use crate::dto::responses::nav_dto::NavigateJumpDataEnvelopeDTO;
use crate::dto::responses::nav_dto::NavigateOrbitDataEnvelopeDTO;
use crate::dto::responses::nav_dto::NavigateStatusDataEnvelopeDTO;
use crate::dto::responses::nav_dto::NavigateWarpDataEnvelopeDTO;
use crate::dto::responses::nav_dto::NavigateWaypointDataEnvelopeDTO;
use crate::dto::responses::system_dto::ChartDataEnvelopeDTO;
use crate::model::Cooldown;
use crate::model::Ship;
use crate::services::dispatchers::agent;
use crate::services::dispatchers::contract;
use crate::Agent;
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

    pub async fn show_ships(&self, agent_id: &Option<String>) -> anyhow::Result<Vec<Ship>> {
        // Get the number of ships and the ship names
        let agent: Agent = self.agent_svc.get_agent(agent_id).await?;
        let agent_symbol = agent.symbol;
        let mut ships: Vec<Ship> = vec![];
        for i in 0..agent.ship_count as usize {
            let mut num = i + 1;
            let mut ship_name = format!("{agent_symbol}-{num}");
            let mut ship = self.get_ship(&ship_name).await?;
            ships.push(ship.clone());
        }
        Ok(ships)
    }

    pub async fn get_ship(&self, ship_name: &String) -> anyhow::Result<Ship> {
        self._get_ship(ship_name).await
    }

    pub async fn list_cargo(&self, ship_symbol: &String) -> anyhow::Result<()> {
        self._list_cargo(ship_symbol).await
    }

    pub async fn purchase_cargo(&self, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        self._purchase_cargo(ship_symbol, cargo_symbol, cargo_units).await
    }

    pub async fn jettison_cargo(&self, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        self._jettison_cargo(ship_symbol, cargo_symbol, cargo_units).await
    }

    pub async fn sell_cargo(&self, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        self._sell_cargo(ship_symbol, cargo_symbol, cargo_units).await
    }

    pub async fn transfer_cargo(&self, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        self._transfer_cargo(ship_symbol, cargo_symbol, cargo_units).await
    }

    pub async fn navigate_orbit(&self, ship_symbol: &String) -> anyhow::Result<NavigateOrbitDataEnvelopeDTO> {
        self._navigate_orbit(ship_symbol).await
    }

    pub async fn navigate_to(&self, ship_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<NavigateWaypointDataEnvelopeDTO> {
        self._navigate_to(ship_symbol, waypoint_symbol).await
    }

    pub async fn dock_at_station(&self, ship_symbol: &String) -> anyhow::Result<NavigateDockDataEnvelopeDTO> {
        self._dock_at_station(ship_symbol).await
    }

    pub async fn get_navigation_status(&self, ship_symbol: &String) -> anyhow::Result<NavigateStatusDataEnvelopeDTO> {
        self._get_navigation_status(ship_symbol).await
    }

    pub async fn set_flight_mode(&self, ship_symbol: &String) -> anyhow::Result<()> {
        self._set_flight_mode(ship_symbol).await
    }

    pub async fn warp_ship(&self, ship_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<NavigateWarpDataEnvelopeDTO> {
        self._warp_ship(ship_symbol, waypoint_symbol).await
    }

    pub async fn jump_to_waypoint(&self, ship_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<NavigateJumpDataEnvelopeDTO> {
        self._jump_to_waypoint(ship_symbol, waypoint_symbol).await
    }

    pub async fn scan_systems(&self) -> anyhow::Result<()> {
        Ok(self.scanner.scan_systems().await?)
    }

    pub async fn scan_waypoints(&self) -> anyhow::Result<()> {
        Ok(self.scanner.scan_waypoints().await?)
    }

    pub async fn scan_ships(&self) -> anyhow::Result<()> {
        Ok(self.scanner.scan_ships().await?)
    }

    pub async fn list_mounts(&self, ship_symbol: &String) -> anyhow::Result<MountDataEnvelopeEnumDTO> {
        Ok(self.mount.list_mounts(ship_symbol).await?)
    }

    pub async fn install_mount(&self, ship_symbol: &String, mount_name: &String) -> anyhow::Result<MountInstallDataEnvelopeDTO> {
        Ok(self.mount.install_mount(ship_symbol, mount_name).await?)
    }

    pub async fn remove_mount(&self, ship_symbol: &String, mount_name: &String) -> anyhow::Result<MountRemoveDataEnvelopeDTO> {
        Ok(self.mount.remove_mount(ship_symbol, mount_name).await?)
    }

    pub async fn get_scrap_ship_status(&self, ship_symbol: &String) -> anyhow::Result<ShipScrapStatusDataEnvelopeDTO> {
        Ok(self._get_scrap_ship_status(ship_symbol).await?)
    }

    pub async fn initiate_scrap_ship(&self, ship_symbol: &String) -> anyhow::Result<ShipScrapDataEnvelopeDTO> {
        Ok(self._initiate_scrap_ship(ship_symbol).await?)
    }

    pub async fn list_modules_by_ship(&self, ship_symbol: &String) -> anyhow::Result<ModuleDataEnvelopeEnumDTO> {
        Ok(self.module.list_modules_by_ship(ship_symbol).await?)
    }

    pub async fn install_module_to_ship(&self, ship_symbol: &String, module_name: &String) -> anyhow::Result<ModuleInstallDataEnvelopeDTO> {
        Ok(self.module.install_module_to_ship(ship_symbol, module_name).await?)
    }

    pub async fn remove_module_from_ship(&self, ship_symbol: &String, module_name: &String) -> anyhow::Result<ModuleRemoveDataEnvelopeDTO> {
        Ok(self.module.remove_module_from_ship(ship_symbol, module_name).await?)
    }

    pub async fn refuel_ship(&self, ship_symbol: &String, use_cargo_fuel: &bool, units_of_fuel: &Option<u32>) -> anyhow::Result<ShipRefuelDataEnvelopeDTO> {
        Ok(self._refuel_ship(ship_symbol, use_cargo_fuel, units_of_fuel).await?)
    }

    pub async fn get_repair_status(&self, ship_symbol: &String) -> anyhow::Result<ShipRepairStatusDataEnvelopeDTO> {
        Ok(self._get_repair_status(ship_symbol).await?)
    }

    pub async fn initiate_repair(&self, ship_symbol: &String) -> anyhow::Result<ShipRepairDataEnvelopeDTO> {
        Ok(self._initiate_repair(ship_symbol).await?)
    }

    pub async fn create_chart(&self, ship_symbol: &String) -> anyhow::Result<ChartDataEnvelopeDTO> {
        Ok(self._create_chart(ship_symbol).await?)
    }

    pub async fn get_reactor_status(&self, ship_symbol: &String) -> anyhow::Result<CooldownEnvelopeDTO> {
        Ok(self._get_reactor_status(ship_symbol).await?)
    }







    fn _get_agent_symbol_by_ship_symbol(&self, ship_symbol: &String) -> String {
        // Ship Symbol ismade up for `[AGENT_SYMBOL]-[HEX_ID]` so we can split it out
        let mut parts = ship_symbol.splitn(2, "-");
        parts.next().unwrap().to_string()
    }

    async fn _create_chart(&self, ship_symbol: &String) -> anyhow::Result<ChartDataEnvelopeDTO> {
        let endpoint: String = format!("my/ships/{}/chart", ship_symbol);
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let headers = self.st.get_agent_headers(&agent_token)?;
        let response = self.st.post_with_headers::<ChartDataEnvelopeDTO, ()>(&endpoint, None, Some(headers)).await?;
        Ok(response)
    }

    async fn _get_reactor_status(&self, ship_symbol: &String) -> anyhow::Result<CooldownEnvelopeDTO> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let endpoint: String = format!("my/ships/{}/cooldown", ship_symbol);
        let headers = self.st.get_agent_headers(&agent_token)?;
        let response = self.st.get_with_headers::<CooldownEnvelopeDTO>(&endpoint, Some(headers)).await?;
        match response {
            Some(resp) => {
                Ok(resp)
            },
            None => {
                bail!("Could not get the status of the reactor!");
            }
        }
    }

    async fn _get_ship_by_symbol(&self, agent_token: &String, ship_symbol: &String) -> anyhow::Result<Ship> {
        let endpoint: String = format!("my/ships/{}", ship_symbol);
        let headers = self.st.get_agent_headers(agent_token)?;
        let ship_envelope: Option<ShipDataEnvelopeDTO> = self.st.get_with_headers::<ShipDataEnvelopeDTO>(&endpoint, Some(headers)).await?;
        match ship_envelope {
            Some(envelope) => {
                Ok(envelope.data.try_into()?)
            }

            None => {
                bail!("Ship wasn't found by symbol {}", ship_symbol)
            }
        }
    }

    async fn _refuel_ship(&self, ship_symbol: &String, use_cargo_fuel: &bool, units_of_fuel: &Option<u32>) -> anyhow::Result<ShipRefuelDataEnvelopeDTO> {
        let endpoint: String = format!("my/ships/{}/refuel", ship_symbol);
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let headers = self.st.get_agent_headers(&agent_token)?;
        let body = RequestRefuelShipDTO::new(
            use_cargo_fuel.clone(),
            units_of_fuel.clone()
        )?;
        let resp: ShipRefuelDataEnvelopeDTO = self.st.post_with_headers::<ShipRefuelDataEnvelopeDTO, RequestRefuelShipDTO>(&endpoint, Some(&body), Some(headers)).await?;
        Ok(resp)
    }

    async fn _get_ship(&self, ship_name: &String) -> anyhow::Result<Ship> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_name);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let ship = self._get_ship_by_symbol(&agent_token, ship_name).await?;
        Ok(ship)
    }

    async fn _list_cargo(&self, ship_symbol: &String) -> anyhow::Result<()> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        self.cargo.list_cargo(&agent_token, ship_symbol).await?;
        Ok(())
    }

    async fn _purchase_cargo(&self, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let result = self.cargo.purchase_cargo(&agent_token, ship_symbol, cargo_symbol, cargo_units).await?;
        Ok(result)
    }

    async fn _jettison_cargo(&self, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let result = self.cargo.jettison_cargo(&agent_token, ship_symbol, cargo_symbol, cargo_units).await?;
        Ok(result)
    }

    async fn _sell_cargo(&self, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let result = self.cargo.sell_cargo(&agent_token, ship_symbol, cargo_symbol, cargo_units).await?;
        Ok(result)
    }

    async fn _transfer_cargo(&self, ship_symbol: &String, cargo_symbol: &String, cargo_units: u32) -> anyhow::Result<()> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let result = self.cargo.transfer_cargo(&agent_token, ship_symbol, cargo_symbol, cargo_units).await?;
        Ok(result)
    }

    async fn _navigate_orbit(&self, ship_symbol: &String) -> anyhow::Result<NavigateOrbitDataEnvelopeDTO> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let result = self.navigator.navigate_orbit(&agent_token, ship_symbol).await?;
        Ok(result)
    }

    async fn _navigate_to(&self, ship_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<NavigateWaypointDataEnvelopeDTO> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let result = self.navigator.navigate_to_waypoint(&agent_token, ship_symbol, waypoint_symbol).await?;
        Ok(result)
    }

    async fn _dock_at_station(&self, ship_symbol: &String) -> anyhow::Result<NavigateDockDataEnvelopeDTO> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let result = self.navigator.dock_at_station(&agent_token, ship_symbol).await?;
        Ok(result)
    }

    async fn _get_navigation_status(&self, ship_symbol: &String) -> anyhow::Result<NavigateStatusDataEnvelopeDTO> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let result = self.navigator.get_navigation_status(&agent_token, ship_symbol).await?;
        Ok(result)
    }

    async fn _set_flight_mode(&self, ship_symbol: &String) -> anyhow::Result<()> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let result = self.navigator.set_flight_mode(&agent_token, ship_symbol).await?;
        Ok(result)
    }

    async fn _warp_ship(&self, ship_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<NavigateWarpDataEnvelopeDTO> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let result = self.navigator.warp_ship(&agent_token, ship_symbol, waypoint_symbol).await?;
        Ok(result)
    }

    async fn _jump_to_waypoint(&self, ship_symbol: &String, waypoint_symbol: &String) -> anyhow::Result<NavigateJumpDataEnvelopeDTO> {
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let result = self.navigator.jump_to_waypoint(&agent_token, ship_symbol, waypoint_symbol).await?;
        Ok(result)
    }

    async fn _get_repair_status(&self, ship_symbol: &String) -> anyhow::Result<ShipRepairStatusDataEnvelopeDTO> {
        let endpoint: String = format!("my/ships/{}/repair", ship_symbol);
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let headers = self.st.get_agent_headers(&agent_token)?;
        let response = self.st.get_with_headers::<ShipRepairStatusDataEnvelopeDTO>(&endpoint, Some(headers)).await?;
        match response {
            Some(resp) => {
                Ok(resp)
            },
            None => {
                bail!("Failed to get repair status");
            }
        }
    }

    async fn _initiate_repair(&self, ship_symbol: &String) -> anyhow::Result<ShipRepairDataEnvelopeDTO> {
        let endpoint: String = format!("my/ships/{}/repair", ship_symbol);
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let headers = self.st.get_agent_headers(&agent_token)?;
        let resp: ShipRepairDataEnvelopeDTO = self.st.post_with_headers::<ShipRepairDataEnvelopeDTO, ()>(&endpoint, None, Some(headers)).await?;
        Ok(resp)
    }

    async fn _get_scrap_ship_status(&self, ship_symbol: &String) -> anyhow::Result<ShipScrapStatusDataEnvelopeDTO> {
        let endpoint: String = format!("my/ships/{}/scrap", ship_symbol);
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let headers = self.st.get_agent_headers(&agent_token)?;
        let response = self.st.get_with_headers::<ShipScrapStatusDataEnvelopeDTO>(&endpoint, Some(headers)).await?;
        match response {
            Some(resp) => {
                Ok(resp)
            },
            None => {
                bail!("Failed to get repair status");
            }
        }
    }

    async fn _initiate_scrap_ship(&self, ship_symbol: &String) -> anyhow::Result<ShipScrapDataEnvelopeDTO> {
        let endpoint: String = format!("my/ships/{}/scrap", ship_symbol);
        let agent = self._get_agent_symbol_by_ship_symbol(ship_symbol);
        let agent_token = self.agent_svc.get_token_by_agent_symbol(&agent).await?;
        let headers = self.st.get_agent_headers(&agent_token)?;
        let resp: ShipScrapDataEnvelopeDTO = self.st.post_with_headers::<ShipScrapDataEnvelopeDTO, ()>(&endpoint, None, Some(headers)).await?;
        Ok(resp)
    }
}