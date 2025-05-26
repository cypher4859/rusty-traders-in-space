use reqwest::Client;
use anyhow::Result;
use owo_colors::OwoColorize;          // cargo add owo-colors
use std::fmt::Debug;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Serialize, de::DeserializeOwned};   // ← blanket trait for any owned deserialisable type
use std::sync::Arc;
use crate::config::Config;

pub struct SpaceTradersService {
    cfg: Arc<Config>,
    http: reqwest::Client,
    base: String,
    token: String,
}

pub enum SupportedHttpMethods {
    Get,
    Post
}

impl SpaceTradersService {
    pub async fn new(cfg: Arc<Config>) -> anyhow::Result<Self> {
        let mut headers= HeaderMap::new();
        let bearer_value = format!("Bearer {}", cfg.api_token);
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&bearer_value)?);
        let http = Client::builder().default_headers(headers).build()?;
        Ok(Self {
            cfg: Arc::clone(&cfg),
            http,
            base: cfg.api_base_url.clone(),
            token: cfg.api_token.clone()
        })
    }

    pub fn display_api_result<T>(&self, label: &str, outcome: &Result<T>)
    where
        T: Serialize + Debug,
    {
        self._display_api_result_in_json::<T>(label, outcome);
    }

    pub fn _display_api_result_in_json<T>(&self, label: &str, outcome: &Result<T>)
    where 
        T: Serialize + Debug
    {
        match outcome {
            Ok(val) => {
                println!(
                    "{}\n{}",
                    format!("✔ {label} OK").green().bold(),
                    serde_json::to_string_pretty(val)
                        .unwrap_or_else(|_| format!("{:#?}", val))
                );
            }
            Err(e) => {
                println!("{}\n{e:?}", format!("✘ {label} FAILED").red().bold());
            }
        }
    }

    pub async fn get<T>(&self, endpoint: &String) -> Result<T>
    where
        T: DeserializeOwned + Serialize + Debug,   // <- same bounds
    {
        self.get_with_headers::<T>(endpoint, None).await
    }

    pub async fn get_with_headers<T>(
        &self,
        endpoint: &str,
        extra: Option<HeaderMap>,
    ) -> anyhow::Result<T>
    where
        T: DeserializeOwned + Serialize + Debug,
    {
        let result = self
            .send_request::<T, ()>(endpoint, SupportedHttpMethods::Get, None, extra)
            .await;

        self.display_api_result(&format!("GET {endpoint}"), &result);
        result
    }

    pub fn get_agent_headers(&self, agent_token: &String) -> anyhow::Result<HeaderMap> {
        let mut hdr: HeaderMap =  HeaderMap::new();
        hdr.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", agent_token))?
        );
        Ok((hdr))
    }

    pub async fn post<T, B>(&self, endpoint: &String, body: Option<&B>) -> anyhow::Result<T> where T: DeserializeOwned + Serialize + Debug, B: Serialize + ?Sized {
        self.post_with_headers::<T, B>(endpoint, body, None).await
    }

    pub async fn post_with_headers<T, B>(&self, endpoint: &String, body: Option<&B>, extra: Option<HeaderMap>) -> anyhow::Result<T> where T: DeserializeOwned + Serialize + Debug, B: Serialize + ?Sized {
        let result = self.send_request::<T, B>(endpoint, SupportedHttpMethods::Post, body, extra).await;
        self.display_api_result(&format!("POST {endpoint}"), &result);
        result

    }

    pub async fn send_request<T, B>(
        &self,
        endpoint: &str,
        http_method: SupportedHttpMethods,
        body: Option<&B>,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        match http_method {
            SupportedHttpMethods::Get => {
                self._get_request_by_http::<T>(endpoint, extra_headers).await
            }
            SupportedHttpMethods::Post => {
                let b = body.ok_or_else(|| anyhow::anyhow!("POST needs a body"))?;
                self._post_request_by_http::<T, B>(endpoint, b, extra_headers)
                    .await
            }
        }
    }
    
    async fn _get_request_by_http<T>(
        &self,
        endpoint: &str,
        extra: Option<HeaderMap>,
    ) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}/{}", self.base, endpoint);
        let mut req = self.http.get(url);
    
        if let Some(h) = extra {
            req = req.headers(h);
        }
    
        let value = req.send().await?.error_for_status()?.json::<T>().await?;
        Ok(value)
    }
    
    async fn _post_request_by_http<T, B>(
        &self,
        endpoint: &str,
        body: &B,
        extra: Option<HeaderMap>,
    ) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let url = format!("{}/{}", self.base, endpoint);
        let mut req = self.http.post(url).json(body);
    
        if let Some(h) = extra {
            req = req.headers(h);
        }
    
        let value = req.send().await?.error_for_status()?.json::<T>().await?;
        Ok(value)
    }

    fn _send_request_by_socket(&self) {

    }

}