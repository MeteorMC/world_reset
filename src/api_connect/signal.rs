use core::panic;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::blocking::Client;
use serde_json::json;

pub fn server_signal(signal: String) {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/json"));
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!(
            "Bearer {}",
            env::var("Panel_token").expect("環境変数 Panel_server がありません")
        )).unwrap(),
    );
    let client = Client::builder().default_headers(headers).build().unwrap();
    let res = client.post(
        format!("https://{}/api/client/servers/{}/power",
        env::var("Panel_domain").expect("環境変数 Panel_domain がありません"),
        env::var("Panel_server").expect("環境変数 Panel_server がありません")
    )).json(&json!({"signal": signal})).send().unwrap();
    if !res.status().is_success() {
        panic!();
    }
}
