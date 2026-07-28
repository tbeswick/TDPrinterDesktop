
use crate::printer_api::*;
use crate::models::{PrinterStatus}; 
use std::time::Duration;
use tauri::AppHandle;
use tauri::Emitter;

enum SmState{
    Connect,
    Info,
    Files,
    Idle,
    Status,
    // UploadFile,
    // DeleteFile,
    // NewJob,
    // SendPrintJob,
    // StopPrintJob,
    // PausePrintJob,
    // ResumePrintJob
}

const PRINTER_IP:&str = "192.168.1.76";
const APIKEY:&str = "kpiTr8FC6WmrsJh";

pub fn start_background_thread(app: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {

        let mut state = SmState::Connect;

        loop {

            state  = match state {
                SmState::Connect => {
                    println!("Connecting to printer...");
                    let printer_version  =  fetch_version(PRINTER_IP, APIKEY).await.map_err(|e| e.to_string());
                    if printer_version.is_err() {
                        println!("Error fetching printer version: {:?}", printer_version.err());
                        // Handle the error, maybe retry or transition to an error state
                        SmState::Connect // Retry connecting
                    } else {
                        println!("Printer Version: {:?}", printer_version);
                        SmState::Info // Transition to Info state on success
                    }
                }
                SmState::Info => {
                    println!("Fetching printer info...");
                    let printer_info = fetch_info(PRINTER_IP, APIKEY).await.map_err(|e| e.to_string());
                    if printer_info.is_err() {
                        println!("Error fetching printer info: {:?}", printer_info.err());
                        // Handle the error, maybe retry or transition to an error state
                        SmState::Connect // Retry connecting
                    } else {
                        println!("Printer Info: {:?}", printer_info);
                        SmState::Files // Transition to Status state on success
                    }
                }
                SmState::Files => {
                    println!("Fetching files...");
                    let file_list = fetch_file_info(PRINTER_IP, APIKEY).await.map_err(|e| e.to_string());
                    if file_list.is_err() {
                        println!("Error fetching file list: {:?}", file_list.err());
                        // Handle the error, maybe retry or transition to an error state
                        SmState::Connect // Retry connecting
                    } else {
                        let mut file_list = file_list.unwrap();                        
                        println!("File List: {:?}", file_list);
                        if let Some(children) = &mut file_list.children {
                            children.sort_by(|a, b| {
                                a.display_name
                                    .as_deref()
                                    .unwrap_or("")                                  
                                    .cmp(&b.display_name.as_deref().unwrap_or("").to_ascii_lowercase())                                
                            });

                            for child in children {
                                if child.file_type.as_deref() == Some("PRINT_FILE") {
                                    println!("Found G-code file: {:?}", child.display_name);
                                }
                            }
                        }

                        SmState::Status // Transition to Status state on success
                    }                    
                }
                SmState::Idle => {
                    println!("Printer is idle.");
                    // Wait for a command or event to change state
                    SmState::Status
                }
                SmState::Status => {
                    println!("Fetching printer status...");
                    let printer_status = fetch_status(PRINTER_IP, APIKEY).await.map_err(|e| e.to_string());                            
                    if printer_status.is_err() {
                        println!("Error fetching printer status: {:?}", printer_status.err());
                        // Handle the error, maybe retry or transition to an error state
                        SmState::Connect // Retry connecting
                    } else {
                        let status = printer_status.unwrap();
                        app.emit("printer-status-update", &status).unwrap();
                        SmState::Status // Transition to Status state on success
                    }
                    
                }

            };

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
}




