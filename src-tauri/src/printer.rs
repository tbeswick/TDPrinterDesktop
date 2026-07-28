
use crate::printer_api::*;
use crate::models::{PrinterStatus,FileList,FileItem}; 
use std::time::Duration;
use std::thread;
use tauri::Emitter;
use std::sync::{Mutex, OnceLock};

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

static FILE_LIST: OnceLock<Mutex<FileList>> = OnceLock::new();


fn sort_files() {
    if let Some(file_list) = FILE_LIST.get() {
        let mut file_list = file_list.lock().unwrap();

        if let Some(children) = &mut file_list.children {
            children.sort_by_key(|item| {
                item.display_name
                    .as_deref()
                    .unwrap_or("")
                    .to_ascii_lowercase()
            });
        }
    }
}


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
                        let file_list = file_list.unwrap();     

                        
                              
                        // FILE_LIST
                        //     .set(Mutex::new(file_list))
                        //     .unwrap();          
                        //     // Sort the files after fetching              
                        //     sort_files();

                        // print the sorted files to the console
                        // if let Some(file_list) = FILE_LIST.get() {
                        //     let file_list = file_list.lock().unwrap();                            
                             print_files(&file_list);                            
                        

                            if let Some(children) = file_list.children {
                                for child in children {
                                    process_file(child).await;
                                }
                            }            
                       // }            

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




