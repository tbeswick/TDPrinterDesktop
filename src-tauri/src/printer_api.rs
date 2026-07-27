


use reqwest::Client;
use crate::models::{PrinterStatus};

pub async fn fetch_status(ip: &str, api_key: &str) -> Result<PrinterStatus, String> {
    let url = format!("http://{}/api/v1/status", ip);

    let client = Client::new();

    let res = client
        .get(&url)
        .header("X-Api-Key", api_key)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        println!("HTTP error: {}", res.status());
        return Err(format!("HTTP error: {}", res.status()));
    }

    let json = res
        .json::<PrinterStatus>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(json)
}



