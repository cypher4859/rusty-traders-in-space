#[derive(Clone)]

pub struct PlayerService {}

impl PlayerService {
    pub fn new(cfg: Arc<Config>, st: Arc<SpaceTradersService>) -> Self {
    
    }

    pub async fn show_agents() -> anyhow::Result<()> {}

    pub async fn create_new_agent() -> anyhow::Result<()> {}

    pub async fn show_factions() -> anyhow::Result<()> {}

    pub async fn find_factions() -> anyhow::Result<()> {}

    pub async fn show_contracts() -> anyhow::Result<()> {}

    pub async fn find_contract() -> anyhow::Result<()> {}

    pub async fn generate_contract() -> anyhow::Result<()> {}

    pub async fn list_ships() -> anyhow::Result<()> {}

    pub async fn find_ship() -> anyhow::Result<()> {}

    pub async fn repair_ship() -> anyhow::Result<()> {}

    pub async fn scrap_ship() -> anyhow::Result<()> {}



    pub async fn list_sectors() -> anyhow::Result<()> {}

    pub async fn list_waypoints() -> anyhow::Result<()> {}

    pub async fn list_systems() -> anyhow::Result<()> {}

    pub async fn get_waypoint() -> anyhow::Result<()> {}

    pub async fn get_system() -> anyhow::Result<()> {}

    /// Scan ships, systems, waypoints
    pub async fn scan() -> anyhow::Result<()> {}

    pub async fn move_to_waypoint() -> anyhow::Result<()> {}

    pub async fn jump_to_new_system() -> anyhow::Result<()> {}

    pub async fn warp_to_point() -> anyhow::Result<()> {}

    pub async fn dock() -> anyhow::Result<()> {}

    pub async fn orbit() -> anyhow::Result<()> {}

    pub async fn show() -> anyhow::Result<()> {}

    pub async fn extract_resources() -> anyhow::Result<()> {}

    pub async fn siphon_resources() -> anyhow::Result<()> {}

    pub async fn survey() -> anyhow::Result<()> {}


}