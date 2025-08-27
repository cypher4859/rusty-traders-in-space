use comfy_table::{Row, Table};
use anyhow::{bail, Context, Result};
use owo_colors::OwoColorize;
use serde::Deserialize;
use strum::{IntoEnumIterator};
use strum_macros::{Display};
use tungstenite::http::request;
use std::any::type_name;
// cargo add owo-colors
use std::fmt::{Debug};
use std::fs;
use std::path::Path;
use reqwest::{Client, StatusCode};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Serialize, de::DeserializeOwned};   // ← blanket trait for any owned deserialisable type
use serde_json::from_str;
use std::sync::Arc;
use rusqlite::{params, Connection, OptionalExtension, params_from_iter, ToSql};
use std::time::Duration;
use rand::{thread_rng, Rng};
use tokio::time::sleep;
use crate::config::{Config, OutputMode};
use crate::dto::responses::error_dto::{ErrorEnvelope};
use crate::dto::responses::util_dto::PageEnvelopeDTO;
use crate::helpers::table_helpers::TableRow;
use crate::helpers::redact_helper::RedactableData;
use crate::{Agent, RegisterDataDTO};


pub struct SpaceTradersService {
    cfg: Arc<Config>,
    http: reqwest::Client,
    base: String,
    token: String,
    db_connection: Connection
}

pub enum SupportedHttpMethods {
    Get,
    Post
}

impl SpaceTradersService {
    pub async fn new(cfg: Arc<Config>) -> anyhow::Result<Self> {
        let mut headers= HeaderMap::new();
        // let bearer_value = format!("Bearer {}", cfg.api_token);
        // headers.insert(AUTHORIZATION, HeaderValue::from_str(&bearer_value)?);
        let http = Client::builder().default_headers(headers).build()?;
        let db_connection = SpaceTradersService::init_db(&cfg.db_path);
        Ok(Self {
            cfg: Arc::clone(&cfg),
            http,
            base: cfg.api_base_url.clone(),
            token: cfg.api_token.clone(),
            db_connection: db_connection?
        })
    }

    pub fn new_injected(
        cfg: std::sync::Arc<Config>,
        http: reqwest::Client,
        base: String,
        db_connection: rusqlite::Connection,
    ) -> Self {
        Self { cfg, http, base, token: String::new(), db_connection }
    }

    fn init_db(path: &str) -> Result<Connection> {
        let p = Path::new(path);
        let tables_to_create = vec![
            type_name::<Agent>().rsplit("::").next().unwrap()
        ];

        // make sure the parent directory exists
        if let Some(dir) = p.parent() {
            if !dir.exists() {
                fs::create_dir_all(dir)?;           // create db/
            }
        }
        let conn = Connection::open(path)?;
        for table_name in tables_to_create {
            conn.execute_batch(
                format!("CREATE TABLE IF NOT EXISTS {} (
                    type  TEXT NOT NULL,
                    id    TEXT NOT NULL PRIMARY KEY,
                    json  TEXT NOT NULL
                );", table_name).as_str(),
            )?;
        }
        Ok(conn)
    }

    pub fn get_db_connection(&self) -> &Connection {
        &self.db_connection
    }

    pub fn save_to_db<T, F>(
        &self,
        // conn: &Connection,
        model: &T,
        id_fn: F,
    ) -> Result<()>
    where
        T: Serialize,
        F: Fn(&T) -> String,
    {
        let conn = &self.db_connection;
        let bucket = type_name::<T>()
            .rsplit("::")
            .next()
            .unwrap_or("unknown");
    
        let json = serde_json::to_string_pretty(model)?;
        let mut sql = format!("INSERT OR REPLACE INTO \"{}\" (type, id, json) VALUES (?1, ?2, ?3)", bucket);
    
        conn.execute(
            &sql,
            params![bucket, id_fn(model), json],
        )?;
    
        Ok(())
    }

    /// Return all rows from the table that corresponds to `T`.
    ///
    /// * `filter_sql` – optional **SQL WHERE fragment** (without the leading
    ///   "WHERE").
    /// * `params`     – parameters for that filter.
    pub fn dump_table_from_db<T, P>(
        &self,
        filter_sql: Option<&str>,
        params: P,
        show_secrets: &bool
    ) -> Result<()>
    where
        T: DeserializeOwned + Serialize + TableRow + Debug + RedactableData,
        P: IntoIterator,
        P::Item: ToSql,
    {
        let out = self.get_table_from_db::<T,P>(filter_sql, params);

        match self.cfg.output_mode {
            OutputMode::Table => self.display_db_results_as_table_and_redact(out?, show_secrets),
            OutputMode::Json => self.display_db_results_as_json_and_redact(out?, show_secrets)
        }

        Ok(())
    }


    pub fn get_table_from_db<T, P>(
        &self,
        filter_sql: Option<&str>,
        params: P,
    ) -> anyhow::Result<Vec<T>>
    where
        T: DeserializeOwned + Serialize + TableRow + Debug,
        P: IntoIterator,
        P::Item: ToSql, 
    {
        let conn: &Connection = &self.db_connection;

        /* ---------- derive bucket = table name ---------- */
        let raw_bucket = type_name::<T>()
            .rsplit("::")
            .next()
            .unwrap_or("unknown");

        // ✱ sanitise to [A-Z a-z 0-9 _] only
        let bucket: String = raw_bucket
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();

        if bucket.is_empty() {
            bail!("empty table name derived from type {}", raw_bucket);
        }

        /* ---------- build SQL (table name is inlined) ---- */
        let sql = match filter_sql {
            Some(f) => format!("SELECT json FROM {} {}", bucket, f),
            None          => format!("SELECT json FROM {}", bucket)
        };

        let mut stmt = conn.prepare(&sql)?;

        /* ---------- exec query ------------ */
        let rows = stmt.query_map([], |row| { row.get::<_, String>(0) }).with_context(||format!("Failed to execute query {sql}"))?;

        let mut out = Vec::<T>::new();
        for json_res in rows {
            let json = json_res?;
            out.push(from_str::<T>(&json)?);
        }

        Ok(out)
    }



    pub fn load_from_db<T>(
        &self,
        // conn: &Connection,
        id: &str,
    ) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let conn = &self.db_connection;
        let bucket = std::any::type_name::<T>().rsplit("::").next().unwrap_or("unknown");
    
        let json: Option<String> = conn.query_row(
            "SELECT json FROM storage WHERE type = ?1 AND id = ?2",
            params![bucket, id],
            |row| row.get(0),
        ).optional()?;
    
        Ok(json.map(|js| serde_json::from_str::<T>(js.as_str())).transpose()?)
    }

    pub fn display_enums<T>(&self)
    where
        T: IntoEnumIterator + Debug + Serialize + TableRow,
    {
        // for v in T::iter() {
        //     println!("{v:?}");
        // }
        let all: Vec<T> = T::iter().collect();
        // match self.cfg.output_mode {
        //     OutputMode::Json  => self._display_api_result_in_json("Name", &Ok(Some(all))),
        //     OutputMode::Table => self._display_api_result_in_table("Name", &Ok(Some(all))),
        // }
        self.display_api_result("Name", &Ok(Some(all)));
    }

    pub fn display_api_result<T>(&self, label: &str, outcome: &anyhow::Result<Option<T>>)
    where
        T: Serialize + Debug + TableRow,
    {
        match self.cfg.output_mode {
            OutputMode::Json  => self._display_api_result_in_json(label, outcome),
            OutputMode::Table => self._display_api_result_in_table(label, outcome),
        }
    }

    pub fn display_db_result<T>(&self, items: Vec<T>)
    where 
        T: Serialize + TableRow + Debug
    {
        match self.cfg.output_mode {
            OutputMode::Json  => self.display_db_results_as_json(items),
            OutputMode::Table => self.display_db_results_as_table(items),
        }
    }

    // pub fn save_agent_to_db(&self, agent: RegisterDataDTO) -> anyhow::Result<()>
    // {
    //     self.save_to_db::<RegisterDataDTO, String>(&self.db_connection, &agent, |a| a.agent.symbol.clone())
    // }

    /// Print an entire vector of models in table form.
    ///
    /// *If the vector is empty it prints a short notice instead of an empty table.*
    pub fn display_db_results_as_table<T>(&self, items: Vec<T>)
    where
        T: Debug + TableRow,
    {
        if items.is_empty() {
            println!("No data to show!");
            return;
        }

        let mut table = Table::new();
        table
            .load_preset(comfy_table::presets::UTF8_FULL)
            .set_header(Row::from(T::headers()));

        for item in items {
            for row in item.to_rows() {
                table.add_row(Row::from(row));
            }
        }

        println!("{}", table);
    }

    pub fn display_db_results_as_json_and_redact<T>(&self, items: Vec<T>, show_secrets: &bool)
    where 
        T: RedactableData + Debug
    {
        if items.is_empty() {
            println!("No data to show!");
            return;
        }

        let payload: Vec<_> = items.iter()
            .map(|it| it.to_redacted_json(show_secrets))
            .collect();

        println!("{}", serde_json::to_string_pretty(&payload)
            .unwrap_or_else(|_| "<failed to serialize JSON>".into()));
    }

    pub fn display_db_results_as_table_and_redact<T>(&self, items: Vec<T>, show_secrets: &bool)
    where 
        T: RedactableData + Debug + TableRow
    {
        self.display_db_results_as_table(items);
    }

    pub fn display_db_results_as_json<T>(&self, items: Vec<T>)
    where
        T: Serialize + Debug,     // <- add Serialize
    {
        if items.is_empty() {
            println!("No data to show!");
            return;
        }

        match serde_json::to_string_pretty(&items) {
            Ok(json) => println!("{}", json),
            Err(e) => {
                // Fallback: show a readable debug dump if JSON serialization fails
                eprintln!("Failed to serialize results as JSON: {e:?}");
                for (i, item) in items.iter().enumerate() {
                    println!("#{}: {:#?}", i + 1, item);
                }
            }
        }
    }

    fn _display_api_result_in_json<T>(&self, label: &str, outcome: &anyhow::Result<Option<T>>)
    where 
        T: Serialize + Debug
    {
        match outcome {
            Ok(Some(val)) => {
                println!(
                    "{}\n{}",
                    format!("✔ {label} OK").green().bold(),
                    serde_json::to_string_pretty(val).unwrap_or_else(|_| format!("{:#?}", val))
                );
            }
            Ok(None) => println!("{} (no content)", format!("✔ {label} OK").green().bold()),
            Err(e) => {
                println!("{}\n{e:?}", format!("✘ {label} FAILED").red().bold());
            }
        }
    }

    fn _display_api_result_in_table<T>(&self, label: &str, outcome: &anyhow::Result<Option<T>>)
    where
        T: TableRow + Serialize + Debug,
    {
        match outcome {
            Ok(val) => {
                match val {
                    Some(v) => {
                        let mut table = Table::new();
                        table
                            .load_preset(comfy_table::presets::UTF8_FULL)
                            .set_header(Row::from(T::headers()));
                            // .add_row(Row::from(v.to_row()));

                        for row in v.to_rows() {
                            table.add_row(Row::from(row));
                        }

                        println!(
                            "{}\n{}",
                            format!("✔ {label} OK").green().bold(),
                            table
                        );
                    },
                    None => {
                        println!("{}", format!("✘ {label} FAILED, there were no results to display").red().bold())
                    }
                }
            },
            Ok(None) => println!("{} (no content)", format!("✔ {label} OK").green().bold()),
            Err(e) => println!("{}\n{e:?}", format!("✘ {label} FAILED").red().bold()),
        }
    }

    pub async fn get<T>(&self, endpoint: &String, display_result: bool) -> Result<Option<T>>
    where
        T: DeserializeOwned + Serialize + Debug + TableRow,   // <- same bounds
    {
        self.get_with_headers::<T>(endpoint, None, display_result).await
    }

    pub async fn get_with_headers<T>(
        &self,
        endpoint: &str,
        extra: Option<HeaderMap>,
        display_result: bool
    ) -> anyhow::Result<Option<T>>
    where
        T: DeserializeOwned + Serialize + Debug + TableRow,
    {
        let result = self
            .send_request::<T, ()>(endpoint, SupportedHttpMethods::Get, None, extra)
            .await;

        if (display_result) {
            self.display_api_result(&format!("GET {endpoint}"), &result);
        }
        result
    }

    pub async fn get_with_headers_and_paging<D>(
        &self,
        endpoint: &str,
        extra: Option<HeaderMap>,
        display_result: bool
    ) -> anyhow::Result<Option<Vec<D>>>
    where
        D: DeserializeOwned + Serialize + Debug + TableRow + Clone,
    {
        let result = self
            .send_request_with_paging::<D, ()>(endpoint, SupportedHttpMethods::Get, None, extra)
            .await?;

        // let returned_result = result.iter().clone();

        if (display_result) {
            match &result {
                Some(res) => {
                    self.display_db_results_as_table(res.clone());
                },
                None => {
                    bail!("Something hardcore messed up with get_with_heders_and_paging")
                }
            }
        }
        Ok(result)
    }

    pub fn get_agent_headers(&self, agent_token: &String) -> anyhow::Result<HeaderMap> {
        let mut hdr: HeaderMap =  HeaderMap::new();
        hdr.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", agent_token))?
        );
        Ok((hdr))
    }

    pub fn get_account_headers(&self) -> anyhow::Result<HeaderMap> {
        let mut hdr: HeaderMap =  HeaderMap::new();
        hdr.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.cfg.api_token))?
        );
        Ok((hdr))
    }

    pub async fn post<T, B>(&self, endpoint: &String, body: Option<&B>, display_result: bool) -> anyhow::Result<T> where T: DeserializeOwned + Serialize + Debug + Clone + TableRow, B: Serialize + ?Sized {
        self.post_with_headers::<T, B>(endpoint, body, None, display_result).await
    }

    pub async fn post_with_headers<T, B>(
        &self,
        endpoint: &str,
        body: Option<&B>,
        extra: Option<HeaderMap>,
        display_result: bool
    ) -> anyhow::Result<T>
    where
        T: DeserializeOwned + Serialize + Debug + Clone + TableRow,
        B: Serialize + ?Sized,
    {
        let wrapped = self
            .send_request::<T, B>(endpoint, SupportedHttpMethods::Post, body, extra)
            .await?
            .expect("POST endpoints must return a body");
    
        if (display_result) {
            self.display_api_result(&format!("POST {endpoint}"), &Ok(Some(wrapped.clone())));
        }
        Ok(wrapped)
    }

    pub async fn send_request<T, B>(
        &self,
        endpoint: &str,
        http_method: SupportedHttpMethods,
        body: Option<&B>,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<Option<T>>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        match http_method {
            SupportedHttpMethods::Get => {
                self._get_request_by_http::<T>(endpoint, extra_headers).await
            }
    
            SupportedHttpMethods::Post => {
                let value = self
                    ._post_request_by_http::<T, B>(endpoint, body, extra_headers)
                    .await?;
                Ok(Some(value))         // ← no semicolon here
            }
        }
    }

    pub async fn send_request_with_paging<D, B>(
        &self,
        endpoint: &str,
        http_method: SupportedHttpMethods,
        body: Option<&B>,
        extra_headers: Option<HeaderMap>,
    ) -> anyhow::Result<Option<Vec<D>>>
    where
        // E: DeserializeOwned + Serialize,
        D: DeserializeOwned + Serialize,
        B: Serialize + ?Sized,
    {
        match http_method {
            SupportedHttpMethods::Get => {
                self._get_request_by_http_with_paging::<D>(endpoint, extra_headers).await
            }
    
            SupportedHttpMethods::Post => {
                let value = self
                    ._post_request_by_http::<D, B>(endpoint, body, extra_headers)
                    .await?;
                Ok(Some(vec![value]))         // ← no semicolon here
            }
        }
    }
    
    async fn _get_request_by_http<T>(
        &self,
        endpoint: &str,
        extra: Option<HeaderMap>,
    ) -> anyhow::Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}/{}", self.base, endpoint);
        let mut req = self.http.get(url);
    
        if let Some(h) = extra {
            req = req.headers(h);
        }

        let resp = req.send().await?;

        /* ========== 204 No Content ========== */
        if resp.status() == reqwest::StatusCode::NO_CONTENT {
            return Ok(None);
        }

        /* ========== error branch ========== */
        if !resp.status().is_success() {
            let status = resp.status();
            let text   = resp.text().await?;

            if let Ok(env) = serde_json::from_str::<ErrorEnvelope>(&text) {
                eprintln!(
                    "API error {} (code {}): {}",
                    status, env.error.code, env.error.message
                );
                bail!("API error {} – code {}: {}", status, env.error.code, env.error.message);
            } else {
                eprintln!("HTTP {} – raw body: {}", status, text);
                bail!("HTTP {}: {}", status, text);
            }
        }

        /* ========== success branch ========== */
        let value = resp.json::<T>().await?;
        Ok(Some(value))
    }

    async fn _get_request_by_http_with_paging<D>(
        &self,
        endpoint: &str,
        extra: Option<HeaderMap>,
    ) -> anyhow::Result<Option<Vec<D>>>
    where
        D: DeserializeOwned
    {
        const MAX_RETRIES: usize   = 5;
        const MAX_DELAY:   u64     = 30;            // seconds

        let mut page  = 1u32;
        let mut items = Vec::<D>::new();

        loop {
            /* -------- build request URL with ?page= -------- */
            let url = format!("{}/{endpoint}?page={page}", self.base);
            let mut req = self.http.get(url);
            if let Some(h) = &extra { req = req.headers(h.clone()); }

            /* -------- send with retry on 429 --------------- */
            let resp = {
                let mut tries = 0;
                loop {
                    let r = req.try_clone().expect("req is cloneable").send().await?;

                    if r.status() != StatusCode::TOO_MANY_REQUESTS {
                        break r;                                // success or other error
                    }

                    tries += 1;
                    if tries > MAX_RETRIES {
                        bail!("hit HTTP 429 too many times");
                    }

                    /* back-off: use Retry-After header if present */
                    let delay = r
                        .headers()
                        .get("Retry-After")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|s| s.parse::<u64>().ok())
                        .unwrap_or_else(|| {
                            // exponential 1,2,4… + jitter up to 200 ms
                            let base = 1 << (tries - 1);                         // 1,2,4…
                            let jitter: u64 = thread_rng().gen_range(0..200);    // ms
                            std::cmp::min(base, MAX_DELAY) * 1_000 + jitter
                        });

                    sleep(Duration::from_millis(delay)).await;
                }
            };

            if resp.status() == StatusCode::NO_CONTENT {
                break;                                    // nothing more
            }
            if !resp.status().is_success() {
                let status = resp.status();
                let text   = resp.text().await.unwrap_or_default();
                bail!("HTTP {}: {}", status, text);
            }

            /* -------- decode page envelope ---------------- */
            let env: PageEnvelopeDTO<D> = resp.json().await.context("json decode")?;
            items.extend(env.data);

            let total_pages =
                (env.meta.total + env.meta.limit - 1) / env.meta.limit;

            if env.meta.page >= total_pages { break; }
            page += 1;
        }

        Ok(Some(items))
    }
    
    async fn _post_request_by_http<T, B>(
        &self,
        endpoint: &str,
        body: Option<&B>,
        extra: Option<HeaderMap>,
    ) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let url = format!("{}/{}", self.base, endpoint);
        let mut req = self.http.post(url);
    
        if let Some(h) = extra {
            req = req.headers(h);
        }

        if let Some(payload) = body {
            req = req.json(payload);
        }
    
        let resp = req.send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text   = resp.text().await?;

            // try to decode the structured error
            if let Ok(env) = serde_json::from_str::<ErrorEnvelope>(&text) {
                eprintln!(
                    "API error {} (code {}): {}",
                    status, env.error.code, env.error.message
                );
                bail!("API error {} – code {}: {}", status, env.error.code, env.error.message);
            } else {
                // fallback: show raw body
                eprintln!("HTTP {} – raw body: {}", status, text);
                bail!("HTTP {}: {}", status, text);
            }
        }

        /* ----------- success branch ----------- */
        let value = resp.json::<T>().await?;
        Ok(value)
    }

    fn _send_request_by_socket(&self) {

    }

    pub fn split_waypoint_to_get_system_symbol(&self, waypoint_symbol: &String) -> String {
        self._split_waypoint_to_get_system_symbol(waypoint_symbol)
    }

    fn _split_waypoint_to_get_system_symbol(&self, waypoint_symbol: &String) -> String {
        let mut parts = waypoint_symbol.splitn(3, "-");
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();
        format!("{first}-{second}").clone()
    }
}