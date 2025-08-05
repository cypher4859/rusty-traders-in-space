use serde::{Deserialize, Serialize};

use crate::helpers::table_helpers::TableRow;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerStatusDTO {
    pub status:      String,
    pub version:     String,
    #[serde(rename = "resetDate")]
    pub reset_date:  String,
    pub description: String,

    pub stats:         StatsDTO,
    pub leaderboards:  LeaderboardsDTO,
    #[serde(rename = "serverResets")]
    pub server_resets: ServerResetsDTO,
    pub announcements: Vec<AnnouncementDTO>,
    pub links:         Vec<LinkDTO>,
}

impl TableRow for ServerStatusDTO {
    fn headers() -> Vec<&'static str> {
        vec!["Status", "Version", "Reset Date", "Description"]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.status.clone(),
            self.version.clone(),
            self.reset_date.clone(),
            self.description.clone()
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatsDTO {
    pub accounts:  u64,
    pub agents:    u64,
    pub ships:     u64,
    pub systems:   u64,
    pub waypoints: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeaderboardsDTO {
    #[serde(rename = "mostCredits")]
    pub most_credits:         Vec<CreditsEntryDTO>,
    #[serde(rename = "mostSubmittedCharts")]
    pub most_submitted_charts: Vec<ChartsEntryDTO>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreditsEntryDTO {
    #[serde(rename = "agentSymbol")]
    pub agent_symbol: String,
    pub credits:      i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChartsEntryDTO {
    #[serde(rename = "agentSymbol")]
    pub agent_symbol: String,
    #[serde(rename = "chartCount")]
    pub chart_count:  u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerResetsDTO {
    pub next:      String,
    pub frequency: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnnouncementDTO {
    pub title: String,
    pub body:  String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinkDTO {
    pub name: String,
    pub url:  String,
}