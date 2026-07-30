
use crate::{AppState, printer_api::*};
use crate::models::{PrinterStatus,FileList,FileItem}; 
use std::time::Duration;
use std::thread;
use tauri::Emitter;
use std::sync::{Mutex, OnceLock};
use std::sync::Arc;
use tokio::sync::RwLock;

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



 fn print_files(file_list: &FileList) {
    if let Some(children) = &file_list.children {
        for child in children {
            if child.file_type.as_deref() == Some("FOLDER") {
                println!("Folder: {:?}", child.display_name);
            } else {
                println!("File: {:?}", child.display_name);
            }
            println!("{:?}", child.display_name);
        }
    }
}


async fn process_file(child: FileItem) {
    println!("Processing {:?}", child.display_name);

    if let Some(image) = fetch_file_image(
        PRINTER_IP,
        APIKEY,
        &child.name.as_deref().unwrap_or(""),
    )
    .await
    {
        println!("Downloaded {} bytes", image.len());
    }
}


pub fn start_background_thread(app: tauri::AppHandle, app_state: Arc<AppState>) {
    
   

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
                        let version = printer_version.unwrap();
                        println!("Printer Version: {:?}", version);                        
                        app.emit("printer-version-updated", &version).unwrap();
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
                    let file_list_response = fetch_file_info(PRINTER_IP, APIKEY).await.map_err(|e| e.to_string());
                    if file_list_response.is_err() {
                        println!("Error fetching file list: {:?}", file_list_response.err());
                        // Handle the error, maybe retry or transition to an error state
                        SmState::Connect // Retry connecting
                    } else {
                        let file_list_remote = file_list_response.unwrap();     
                        print_files(&file_list_remote);
                        {
                            let mut file_list = app_state.file_list.write().await;
                            *file_list = Some(file_list_remote);
                        }
                        app.emit("file-list-updated", ()).unwrap();

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
                        app.emit("printer-status-updated", &status).unwrap();
                        SmState::Status // Transition to Status state on success
                    }
                    
                }

            };

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
}




