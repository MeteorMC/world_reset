use std::env;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::blocking::Client;
use serde_json::{json, Value};

pub fn server_files_delete(root: String, files: Option<&[String]>) -> Result<bool, bool> {
    let data: Value = json!(
        {
            "root": root,
            "files": files
        }
    );
    let mut headers = HeaderMap::new();
    headers.insert("Accept",
    HeaderValue::from_static("application/json"));
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!(
            "Bearer {}",
            env::var("Panel_token").expect("環境変数 Panel_token がありません")
        )).unwrap(),
    );
    let client = Client::builder().default_headers(headers).build().unwrap();
    let res = client.post(
        format!("https://{}/api/client/servers/{}/files/delete",
        env::var("Panel_domain").expect("環境変数 Panel_domain がありません"),
        env::var("Panel_server").expect("環境変数 Panel_server がありません")
    )).json(&data).send().unwrap();
    if res.status().is_success() {
        return Ok(true);
    } else {
        return Err(false);
    }
}
