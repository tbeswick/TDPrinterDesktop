
use crate::{AppState, printer_api::*};
use crate::models::{PrinterStatus,FileList,FileItem,VersionInfo,ThumbnailState}; 
use std::time::Duration;
use tauri::Emitter;
use std::sync::Arc;
use std::fs;
use crate::models::{SmState};



const PRINTER_IP:&str = "192.168.1.76";
const APIKEY:&str = "kpiTr8FC6WmrsJh";
const IMAGE_CACHE_DIR:&str = "../image_cache";



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
    app: &tauri::AppHandle,    
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

                // save the image to the cache directory
                let cache_path = format!("{}/{}.png", IMAGE_CACHE_DIR, filename);
                child.thumbnail_path = Some(cache_path.clone());
                if let Err(e) = fs::write(&cache_path, child.thumbnail_image.as_ref().unwrap()) {
                    println!("Failed to save image to cache: {}", e);
                }                
                child.thumbnail_state = Some(ThumbnailState::Ready);
                println!("Thumbnail for {} is ready and saved to {}", filename, child.thumbnail_path.as_ref().unwrap());

                // update app state
                app.emit("file-image-updated", &child).unwrap();

            }

            Err(_) => {

                child.thumbnail_state = Some(ThumbnailState::Failed);
            }
        }
    }
}


pub async fn set_delete_file(     
    delete_filename: &mut Option<String>, filename: &str) -> Result<(), String> {           
    println!("Delete file requested for {}", filename);
    delete_filename.replace(filename.to_string());
    println!("Delete filename set to: {:?}", delete_filename);
    Ok(())
}

pub async fn set_print_file(     
    print_filename: &mut Option<String>, filename: &str) -> Result<(), String> {           
    println!("Print file requested for {}", filename);
    print_filename.replace(filename.to_string());
    println!("Print filename set to: {:?}", print_filename);
    Ok(())
}

pub async fn set_upload_file(     
    upload_filename: &mut Option<String>, filename: &str) -> Result<(), String> {           
    println!("Upload file requested for {}", filename);
    upload_filename.replace(filename.to_string());
    println!("Upload filename set to: {:?}", upload_filename);
    Ok(())
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
                    &app,
                    &app_state,
                    &file.display_name,
                    image,
                ).await;

                println!(
                    "Thumbnail p for {} is now ready",
                    file.thumbnail_path.as_ref().unwrap_or(&"unknown".to_string())
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

        // Create the image cache directory if it doesn't exist
        if !std::path::Path::new(IMAGE_CACHE_DIR).exists() {
            if let Err(e) = fs::create_dir_all(IMAGE_CACHE_DIR) {
                println!("Failed to create image cache directory: {}", e);
            }
        }


        // settle delay - allows first version event to fire correctly
        tokio::time::sleep(Duration::from_secs(3)).await;        

        // Start a worker to manage file thumbnails
        manage_file_thumbnails(app.clone(), app_state.clone());        


        loop {

            // check for delete file request
            let mut delete_filename = app_state.delete_filename.write().await;
            if delete_filename.is_some() && !delete_filename.as_ref().unwrap().is_empty() {
                state = SmState::DeleteFile;
            }

            let mut print_filename = app_state.print_filename.write().await;
            if print_filename.is_some() && !print_filename.as_ref().unwrap().is_empty() {
                state = SmState::SendPrintJob;
            }

            let mut upload_filename = app_state.upload_filename.write().await;
            if upload_filename.is_some() && !upload_filename.as_ref().unwrap().is_empty() {
                state = SmState::UploadFile;
            }



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
                                child.thumbnail_image = None;
                                child.thumbnail_path = check_for_cache_image(&child.display_name); 
                                child.thumbnail_state = check_for_cache_state(&child.display_name);
                            }
                        }
                        app.emit("file-list-updated", ()).unwrap();

                        // clean the cache directory, loop files and delete any png files that are not in the current file list
                        if let Ok(entries) = fs::read_dir(IMAGE_CACHE_DIR) {
                            for entry in entries {
                                if let Ok(entry) = entry {
                                    let path = entry.path();
                                    if path.is_file() && path.extension().map_or(false, |ext| ext == "png") {
                                        let filename = path.file_stem().unwrap().to_string_lossy().to_string();
                                        let file_list = app_state.file_list.read().await;
                                        let file_list = file_list.as_ref();
                                        let children = file_list.and_then(|fl| fl.children.as_ref());
                                        let exists_in_file_list = children.map_or(false, |children| {
                                            children.iter().any(|child| child.display_name == filename)
                                        });
                                        if !exists_in_file_list {
                                            if let Err(e) = fs::remove_file(&path) {
                                                println!("Failed to delete cached image {}: {}", path.display(), e);
                                            } else {
                                                println!("Deleted cached image not in file list: {}", path.display());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        
                        SmState::Status // Transition to Status state on success
                    }                    
                }
                SmState::DeleteFile => {
                    println!("Deleting file...{}", delete_filename.as_ref().unwrap());
                    let delete_result = delete_print_file(PRINTER_IP, APIKEY, delete_filename.as_ref().unwrap()).await.map_err(|e| e.to_string());                
                    if let Err(error) = delete_result {
                        if error.contains("409 Conflict") {
                            let cp_delete_filename = delete_filename.as_ref().unwrap().clone();                          
                            // Handle the contention
                            println!("File is currently in use");
                            app.emit("delete-file-contention", cp_delete_filename).unwrap();    
                            delete_filename.replace("".to_string());// Clear the delete filename after processing                                                       
                            SmState::Status
                        } else {
                            println!("Delete failed: {}", error);
                            delete_filename.replace("".to_string());// Clear the delete filename after processing                               
                            SmState::Status // Transition to Status state on error, maybe retry or transition to an error state
                        }
                    }else{
                        println!("File deleted successfully: {}", delete_filename.as_ref().unwrap());
                        delete_filename.replace("".to_string());// Clear the delete filename after processing                           
                        SmState::Files // Transition to Files state on success                     
                    }
                            
                }
                SmState::UploadFile => {
                    println!("Uploading file...{}", upload_filename.as_ref().unwrap());
                    let upload_file = upload_printer_file(PRINTER_IP, APIKEY, upload_filename.as_ref().unwrap()).await.map_err(|e| e.to_string());
                    if upload_file.is_err() {
                        println!("Error uploading file: {:?}", upload_file.err());
                        // Handle the error, maybe retry or transition to an error state
                        SmState::Connect // Retry connecting
                    } else {
                        upload_filename.replace("".to_string());// Clear the upload filename after processing                           
                        println!("File uploaded successfully: {}", upload_filename.as_ref().unwrap());
                        SmState::Files // Transition to Files state on success                     
                    }
                }
                SmState::SendPrintJob => {
                    println!("Sending print job...{}", print_filename.as_ref().unwrap());
                    let print_result = send_print_job(PRINTER_IP, APIKEY, print_filename.as_ref().unwrap()).await.map_err(|e| e.to_string());
                    if let Err(error) = print_result {
                        println!("Print job failed: {}", error);
                        print_filename.replace("".to_string());// Clear the print filename after processing                               
                        SmState::Status // Transition to Status state on error, maybe retry or transition to an error state
                    } else {
                        println!("Print job sent successfully: {}", print_filename.as_ref().unwrap());
                        print_filename.replace("".to_string());// Clear the print filename after processing                           
                        SmState::Files // Transition to Files state on success                     
                    }
                }
                
                SmState::NewJob => {
                    let njb = app_state.new_print_job.read().await;
                    println!("New job data requested. {}",*njb);
                    // Wait for a command or event to change state
                    SmState::Status
                }
                SmState::Idle => {
                    // Wait for a command or event to change state
                    SmState::Status
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
                        println!("Printer Status: {:?}", status);
                        app.emit("printer-status-updated", &status).unwrap();
                        let sts = status.printer.unwrap().state;
                        let mut new_job_flag = app_state.new_print_job.write().await;
                        println!("new_job {:?}",new_job_flag);
                        if !*new_job_flag  &&  sts == Some("PRINTING".to_string()) {     
                            *new_job_flag = true;
                            println!("nj {}",new_job_flag);
                            SmState::NewJob
                        }else if Some("PRINTING".to_string()) != sts{
                            // reset the new job flag whn not printing
                            *new_job_flag = false;
                            SmState::Status
                        } else{
                            SmState::Status
                        }                       
                    }
                    
                }

            };

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
}



fn check_for_cache_image(filename: &str) -> Option<String> {
    let cache_path = format!("{}/{}.png", IMAGE_CACHE_DIR, filename);
    if std::path::Path::new(&cache_path).exists() {
        Some(cache_path)
    } else {
        Some("tauri.svg".to_string()) // Return a default image path if not found
    }
}

fn check_for_cache_state(filename: &str) -> Option<ThumbnailState> {
    let cache_path = format!("{}/{}.png", IMAGE_CACHE_DIR, filename);
    if std::path::Path::new(&cache_path).exists() {
        Some(ThumbnailState::Ready)
    } else {
        Some(ThumbnailState::NotStarted) // Return a default state if not found
    }
}



