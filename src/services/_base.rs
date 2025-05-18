use reqwest::Client;
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

    pub async fn get<T>(&self, endpoint: &String) -> anyhow::Result<T> where T: DeserializeOwned {
        self.send_request::<T, ()>(endpoint, SupportedHttpMethods::Get, None).await
    }

    pub async fn post<T, B>(&self, endpoint: &String, body: Option<&B>) -> anyhow::Result<T> where T: DeserializeOwned, B: Serialize + ?Sized {
        self.send_request::<T, B>(endpoint, SupportedHttpMethods::Post, body).await
    }

    pub async fn send_request<T, B>(&self, endpoint: &String, http_method: SupportedHttpMethods, body: Option<&B>) -> anyhow::Result<T> where
        T: DeserializeOwned,
        B: Serialize + ?Sized
    {
        match http_method {
            SupportedHttpMethods::Get => self._get_request_by_http::<T>(endpoint).await,
            SupportedHttpMethods::Post => {
                let b = body.ok_or_else(|| anyhow::anyhow!("POST needs a body"))?;
                self._post_request_by_http::<T, B>(endpoint, b).await
            }
        }
    }
    
    async fn _get_request_by_http<T>(&self, endpoint: &String) -> anyhow::Result<T> where T: DeserializeOwned {
        let url = format!("{}/{}", self.base, endpoint);
        let value = self.http
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await?;
        Ok(value)
    }

    async fn _post_request_by_http<T, B>(&self, endpoint: &String, body: &B) -> anyhow::Result<T> where T: DeserializeOwned, B: Serialize + ?Sized {
        let url = format!("{}/{}", self.base, endpoint);
        let value = self.http
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await?;
        Ok(value)
    }

    fn _send_request_by_socket(&self) {

    }

}