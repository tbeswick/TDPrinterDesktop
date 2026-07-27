
use crate::printer_api::*;
use crate::models::{PrinterStatus}; 
use std::time::Duration;



async fn fetch_printer_status(ip: &str, api_key: &str) -> Result<PrinterStatus, String> {
    fetch_status(ip, api_key).await
}


pub fn start_background_thread() {
    tauri::async_runtime::spawn(async {
        loop {
             let printer_status  =  fetch_printer_status("192.168.1.76", "kpiTr8FC6WmrsJh").await.map_err(|e| e.to_string());
            println!("Printer Status: {:?}", printer_status);

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
}




