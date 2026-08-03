
use crate::{AppState, printer_api::*};
use crate::models::{PrinterStatus,FileList,FileItem,VersionInfo,ThumbnailState}; 
use std::os::windows::process;
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


async fn process_file(child: FileItem) -> Option<Vec<u8>>   {
    println!("Processing {:?}", child.display_name);

    if let Some(image) = fetch_file_image(
        PRINTER_IP,
        APIKEY,
        &child.name.as_deref().unwrap_or(""),
    )
    .await
    {
        println!("Downloaded {} bytes for {}", image.len(), child.display_name);
        return Some(image);
    }
    else {
        println!("Failed to download image for {:?}", child.display_name);
        None
    }
}


async fn get_next_thumbnail_job(
    app_state: &Arc<AppState>,
) -> Option<FileItem> {

    let mut file_list = app_state.file_list.write().await;

    let file_list = file_list.as_mut()?;
    let children = file_list.children.as_mut()?;

    for child in children.iter_mut() {


        if child.file_type.as_deref() == Some("PRINT_FILE") && child.thumbnail_state == Some(ThumbnailState::NotStarted) {

            child.thumbnail_state = Some(ThumbnailState::Downloading);
            println!("Starting download for {:?}", child.display_name);
            return Some(child.clone());
        }
    }

    None
}


async fn download_thumbnail(
    file: &FileItem,
) -> Result<Vec<u8>, String> {

    let res = process_file(file.clone()).await;
    Ok(res.unwrap_or_else(|| Vec::new()))
}



async fn update_thumbnail(
    app_state: &Arc<AppState>,
    filename: &str,
    image: Result<Vec<u8>, String>,
) {

    let mut file_list = app_state.file_list.write().await;

    let Some(file_list) = file_list.as_mut() else {
        return;
    };

    let Some(children) = file_list.children.as_mut() else {
        return;
    };

    if let Some(child) =
        children.iter_mut()
                .find(|f| f.display_name == filename)
    {
        match image {

            Ok(bytes) => {

                child.thumbnail_image = Some(bytes);
                child.thumbnail_state = Some(ThumbnailState::Ready);
            }

            Err(_) => {

                child.thumbnail_state = Some(ThumbnailState::Failed);
            }
        }
    }
}


pub fn manage_file_thumbnails(
    app: tauri::AppHandle,
    app_state: Arc<AppState>,
) {

    tauri::async_runtime::spawn(async move {

        loop {

            if let Some(file) =
                get_next_thumbnail_job(&app_state).await
            {
                println!(
                    "Downloading thumbnail for {}",
                    file.display_name
                );

                let image =
                    download_thumbnail(&file).await;

                update_thumbnail(
                    &app_state,
                    &file.display_name,
                    image,
                ).await;

                let _ = app.emit(
                    "thumbnail_updated",
                    &file.display_name,
                );
            }
            else {

                tokio::time::sleep(
                    std::time::Duration::from_secs(1)
                ).await;
            }
        }
    });
}




pub fn start_background_thread(app: tauri::AppHandle, app_state: Arc<AppState>) {
    
   

    tauri::async_runtime::spawn(async move {

        let mut state = SmState::Connect;

        // settle delay - allows first version event to fire correctly
        tokio::time::sleep(Duration::from_secs(2)).await;        

        // Start a worker to manage file thumbnails
        manage_file_thumbnails(app.clone(), app_state.clone());        


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
                        println!("Printer Version: {:?}", &version);                        
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
                            for child in file_list.as_mut().unwrap().children.as_mut().unwrap() {
                                child.thumbnail_state = Some(ThumbnailState::NotStarted);
                            }
                        }
                        app.emit("file-list-updated", ()).unwrap();

                        SmState::Status // Transition to Status state on success
                    }                    
                }
                SmState::Idle => {
                    println!("Printer is idle.");
                    // Wait for a command or event to change state
                    SmState::Idle
                }
                SmState::Status => {
                 //   println!("Fetching printer status...");
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







