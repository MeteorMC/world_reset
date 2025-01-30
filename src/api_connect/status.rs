use core::panic;
use std::env;
use std::error::Error;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::blocking::Client;
use serde_json::Value;

pub fn server_status() -> Result<String, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Accept",
        HeaderValue::from_static("application/json"));
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!(
            "Bearer {}",
            env::var("Panel_token").expect("環境変数 Panel_server がありません")
        )).unwrap(),
    );
    let client = Client::builder().default_headers(headers).build().unwrap();
    let res = client.get(
        format!("https://{}/api/client/servers/{}/resources",
        env::var("Panel_domain").expect("環境変数 Panel_domain がありません"),
        env::var("Panel_server").expect("環境変数 Panel_server がありません")
    )).send().unwrap();
    if res.status().is_success() {
        let json:Value = res.json().unwrap();
        if let Some(current_state) = json["attributes"]["current_state"].as_str() {
            return Ok(current_state.to_string());
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}
