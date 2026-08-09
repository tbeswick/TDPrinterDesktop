


use std::time::Duration;

use reqwest::Client;
use crate::models::{PrinterStatus,VersionInfo, PrinterInfo, FileList};


const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);
const LONG_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);


pub async fn fetch_status(ip: &str, api_key: &str) -> Result<PrinterStatus, String> {
    let url = format!("http://{}/api/v1/status", ip);

    let client = Client::new();

    let res = client
        .get(&url)
        .header("X-Api-Key", api_key)      
        .timeout(CONNECT_TIMEOUT)
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
        .timeout(CONNECT_TIMEOUT)           
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
        .timeout(CONNECT_TIMEOUT)
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

pub async fn fetch_file_info(ip: &str, api_key: &str) -> Result<FileList, String> {
    let url = format!("http://{}/api/v1/files/usb", ip);

    let client = Client::new();

    let res = client
        .get(&url)
        .header("X-Api-Key", api_key)
        .timeout(LONG_CONNECT_TIMEOUT)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        println!("HTTP error: {}", res.status());
        return Err(format!("HTTP error: {}", res.status()));
    }

    let json = res
        .json::<FileList>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(json)
}


pub async fn fetch_file_image(
    ip: &str,
    api_key: &str,
    image_name: &str) -> Option<Vec<u8>> {
        
    let url = format!("http://{}/thumb/l/usb/{}", ip, image_name);

    println!("Fetching image from URL: {}", url);

    let client = Client::new();


    match client
        .get(url)
        .header("X-Api-Key", api_key)
        .timeout(Duration::from_secs(120))        
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.bytes().await {
                    Ok(bytes) => Some(bytes.to_vec()),
                    Err(_) => None,
                }
            } else {
                println!("HTTP error: {}", response.status());                
                None

            }
        }
        Err(_) => None,
    }



}


pub async fn delete_print_file(ip: &str, api_key: &str, filename: &str) -> Result<(),String> {
    let url = format!("http://{}/api/v1/files/usb/{}", ip, filename);

    let client = Client::new();

    let res = client
        .delete(&url)
        .header("X-Api-Key", api_key)
        .timeout(LONG_CONNECT_TIMEOUT)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        println!("HTTP error: {}", res.status());
        return Err(format!("HTTP error: {}", res.status()));
    }


    Ok(())  
}