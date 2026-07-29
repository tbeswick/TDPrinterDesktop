use crate::models::{FileList};
use std::sync::Arc;
use tokio::sync::RwLock;
mod printer;
mod printer_api;
mod models;




pub struct AppState {
    pub file_list: Arc<RwLock<Option<FileList>>>,
}


#[tauri::command]
async fn greet(name: &str) -> Result<String, String> {
    
    if name == "error" {
        return Err("Error: Invalid name provided".to_string());             
    }else{   
        return Ok(format!("Hello, {}! You've been greeted from Rust!", name));
    }
}


#[tauri::command]
async fn greet2(name: &str) -> Result<String, String> {

    if name == "error" {
        return Err("Error: Invalid name provided".to_string());             
    }else{   
        Ok(format!("Hello, {}! You've been greeted from Rust2!", name))
    }
}


#[tauri::command]
async fn get_file_list(
    state: tauri::State<'_, Arc<AppState>>
) -> Result<Option<FileList>, String> {


    println!("Fetching file list from state...");

    let file_list = state.file_list.read().await;

    Ok(file_list.clone())
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

      
    let app_state = AppState {
        file_list: Arc::new(RwLock::new(None)),
    };

    let background_state = Arc::new(app_state);


    tauri::Builder::default()
        .manage(background_state.clone())
        .setup(move |app|{
            printer::start_background_thread(app.handle().clone(),background_state.clone()); 
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,greet2,get_file_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
