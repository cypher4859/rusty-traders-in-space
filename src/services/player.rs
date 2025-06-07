#[derive(Clone)]

pub struct PlayerService {}

impl PlayerService {
    pub fn new(cfg: Arc<Config>, st: Arc<SpaceTradersService>) -> Self {
    
    }

    pub async fn list_ships() -> anyhow::Result<()> {

    }

    pub async fn get_ship() -> anyhow::Result<()> {}

    pub async fn list_sectors() -> anyhow::Result<()> {}

    pub async fn list_waypoints() -> anyhow::Result<()> {}

    pub async fn list_systems() -> anyhow::Result<()> {}

    pub async fn get_waypoint() -> anyhow::Result<()> {}

    pub async fn get_system() -> anyhow::Result<()> {}

    pub async fn scan() -> anyhow::Result<()> {}

    pub async fn move_to_waypoint() -> anyhow::Result<()> {}

    pub async fn jump_to_new_system() -> anyhow::Result<()> {}

    pub async fn warp_to_point() -> anyhow::Result<()> {}

    pub async fn dock() -> anyhow::Result<()> {}

    pub async fn orbit() -> anyhow::Result<()> {}

    pub async fn show() -> anyhow::Result<()> {}
}