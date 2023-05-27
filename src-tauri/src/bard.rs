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
        ("Host", "bard.google.com"),
        ("X-Same-Domain", "1"),
        ("Content-Type","application/x-www-form-urlencoded;charset=UTF-8"),
        ("Origin","https://bard.google.com"),
        ("Referer", "https://bard.google.com/"),
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

fn api_url(proxy_config: &ProxyConfig, api: &str) -> String {
    format!("{}{}", base_url(proxy_config), api)
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

fn new_headers(psid: &str) -> HeaderMap {
    let mut header = HeaderMap::new();
    header.insert(
        "Cookie",
        format!("__SecEAure-1PSID={}", psid).parse().unwrap(),
    );

    header
}

fn new_chat_headers(psid: &str) -> HeaderMap {
    let mut header = HeaderMap::new();

    for (name, value) in HEADER.clone().into_iter() {
        header.insert(name, value.parse().unwrap());
    }

    header.insert(
        "Cookie",
        format!("__Secure-1PSID={}", psid).parse().unwrap(),
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

#[tauri::command]
pub async fn get_snlm0e(proxy_config: ProxyConfig, psid: &str) -> Result<String> {
    let client = new_client(&proxy_config)?;
    let headers = new_headers(psid);

    let url = base_url(&proxy_config);

    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = response.text().await.map_err(|e| e.to_string())?;

    if let Some(captures) = SNLM0E_REGEX.captures(&text) {
        let snlm0e = captures.get(1).unwrap().as_str();
        debug!("got SNlM0e: {}", snlm0e);
        return Ok(snlm0e.into());
    }

    Err("SNlM0e value not found in response".to_string())
}

fn new_request_message(
    prompt: &str,
    conversation_id: &str,
    response_id: &str,
    choice_id: &str,
) -> String {
    let text = format!(
        r#"[null,"[["{}",null,null,[]],["en"],["{}","{}","{}"]]"]"#,
        prompt, conversation_id, response_id, choice_id
    );

    return text;
}

#[tauri::command]
pub async fn get_answer(
    proxy_config: ProxyConfig,
    snlm0e: &str,
    psid: &str,
    prompt: &str,
    request_id: u32,
    conversation_id: &str,
    response_id: &str,
    choice_id: &str,
) -> Result<()> {
    let params = [
        ("bl", "boq_assistant-bard-web-server_20230523.13_p1"),
        ("rt", "c"),
        ("_reqid", &request_id.to_string()),
    ];
    debug!("request params: {:?}", params);

    let msg = new_request_message(prompt, conversation_id, response_id, choice_id);
    let form = HashMap::from([("f.req", msg.as_str()), ("at", snlm0e)]);
    debug!("request form: {:?}", form);

    let url = api_url(
        &proxy_config,
        "_/BardChatUi/data/assistant.lamda.BardFrontendService/StreamGenerate",
    );

    let client = new_client(&proxy_config)?;

    let headers = new_chat_headers(psid);

    let response = client
        .post(url)
        .headers(headers)
        .query(&params)
        .form(&form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let content = response.text().await.unwrap();

    println!("{content}");

    Ok(())
}

fn parse_response(content: &str) {}
