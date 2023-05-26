use crate::error::Result;
use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;
use reqwest::{header::HeaderMap, Client, Proxy};
use std::collections::HashMap;

use crate::config::ProxyConfig;

lazy_static! {
    static ref ALLOWED_LANGUAGES: [&'static str; 6] =
        ["en", "ko", "ja", "english", "korean", "japanese"];
    static ref DEFAULT_LANGUAGES: &'static str = "en";
    static ref HEADER: HashMap<&'static str, &'static str> = HashMap::from([
    ("User-Agent", "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36"),
    ]);
    static ref SNLM0E_REGEX: Regex = Regex::new(r#""SNlM0e":"(.*?)""#).unwrap();
}

const DIRECT_BASE_URL: &str = "https://bard.google.com/";

fn base_url(proxy_config: &ProxyConfig) -> String {
    if proxy_config.method == "reverse-proxy" {
        proxy_config.to_string()
    } else {
        DIRECT_BASE_URL.to_owned()
    }
}

fn new_request_id() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(100000..1000000)
}

fn new_http_client() -> Result<Client> {
    let client = Client::new();

    Ok(client)
}

fn new_http_client_with_proxy(proxy: &str) -> Result<Client> {
    debug!("proxy in use: {}", proxy);

    let proxy = Proxy::all(proxy).map_err(|e| {
        error!("failed to create reqwest::Proxy: {}", e);

        e.to_string()
    })?;

    let client = Client::builder().proxy(proxy).build().map_err(|e| {
        error!("failed to build a proxy-client: {}", e);

        e.to_string()
    })?;

    Ok(client)
}

fn new_headers(psid: &str, pdidts: &str) -> HeaderMap {
    let mut header = HeaderMap::new();
    header.insert(
        "Cookie",
        format!("__Secure-1PSID={}; __Secure-1PSIDTS={}", psid, pdidts)
            .parse()
            .unwrap(),
    );

    header
}

pub fn new_client(proxy_config: &ProxyConfig) -> Result<Client> {
    if proxy_config.method == "direct" {
        new_http_client()
    } else {
        new_http_client_with_proxy(&proxy_config.to_string())
    }
}

async fn get_snlm0e(proxy_config: &ProxyConfig, psid: &str, pdidts: &str) -> Result<String> {
    let client = new_client(proxy_config)?;
    let headers = new_headers(psid, pdidts);

    let url = base_url(proxy_config);

    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = response.text().await.map_err(|e| e.to_string())?;

    if let Some(mat) = SNLM0E_REGEX.find(&text) {
        return Ok(mat.as_str().into());
    }

    Err("SNlM0e value not found in response".to_string())
}
