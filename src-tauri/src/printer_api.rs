


use reqwest::Client;
use crate::models::{PrinterStatus,VersionInfo, PrinterInfo};

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

pub async fn fetch_version(ip: &str, api_key: &str) -> Result<VersionInfo, String> {
    let url = format!("http://{}/api/version", ip);

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
        .json::<VersionInfo>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(json)
}

pub async fn fetch_info(ip: &str, api_key: &str) -> Result<PrinterInfo, String> {
    let url = format!("http://{}/api/v1/info", ip);

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
        .json::<PrinterInfo>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(json)
}

