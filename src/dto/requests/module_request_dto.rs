use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, ensure};

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestModuleDTO {
    #[serde(rename = "symbol")]
    pub module_symbol: String,
}

impl RequestModuleDTO {
    pub fn new(
        module_symbol: String,
    ) -> anyhow::Result<Self>
    {
        Ok(Self
            {
                module_symbol
            }
        )
    }
}


