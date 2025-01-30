use std::env;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::blocking::Client;
use serde_json::{json, Value};
use chrono::Local;

pub fn discord_webhook(title: String, content: String, success: bool) {
    if env::var("discord_webhook_url").is_err() {
        return;
    }
    let data:Value = json!(
        {
            "username": "Meteor",
            "embeds": [
                {
                    "title": title,
                    "description": content,
                    "timestamp": Local::now().format("%Y-%m-%dT%H:%M:%S%z").to_string(),
                    "color": if success {0x00FF00} else {0xFF0000}
                }
            ]
        }
    );
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/json"));
    let client = Client::builder().default_headers(headers).build().unwrap();
    client.post(env::var("discord_webhook_url").unwrap()).json(&data).send().unwrap();
}
