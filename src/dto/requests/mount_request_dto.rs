use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestMountDTO {
    #[serde(rename = "symbol")]
    pub mount_symbol: String,
}

impl RequestMountDTO {
    pub fn new(
        mount_symbol: String,
    ) -> anyhow::Result<Self>
    {
        Ok(Self
            {
                mount_symbol
            }
        )
    }
}


