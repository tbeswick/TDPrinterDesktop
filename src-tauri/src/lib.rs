use crate::models::{FileList, FileItem, SmState};
use std::sync::Arc;
use tokio::sync::RwLock;
mod printer;
mod printer_api;
mod models;




pub struct AppState {
    pub file_list: Arc<RwLock<Option<FileList>>>,
    pub delete_filename: Arc<RwLock<Option<String>>>,
}


#[tauri::command]
async fn card_clicked(state: tauri::State<'_, Arc<AppState>>,  card_id: String) ->Result<Option<FileItem>, String> {

    let file_list = state.file_list.read().await;

    if let Some(file_list) = &*file_list {
        if let Some(file_item) = file_list.children.as_ref().unwrap().iter().find(|item| item.display_name == card_id) {
            return Ok(Some(file_item.clone()));
        }else{
            Err("File not found".into())
        }
    }else{
        Err("File not found".into())
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
        delete_filename: Arc::new(RwLock::new(None))
    };

    let background_state = Arc::new(app_state);


    tauri::Builder::default()
        .manage(background_state.clone())
        .setup(move |app|{
            printer::start_background_thread(app.handle().clone(),background_state.clone()); 
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_file_list, card_clicked, deletebutton_clicked])
        .run(tauri::generate_context!())
        .expect("error while running printer application");
}


#[tauri::command]
async fn deletebutton_clicked( state: tauri::State<'_, Arc<AppState>>,name: String) -> Result<(), String> {
    
    println!("Delete button clicked for file: {}", name);
    let Result =  printer::set_delete_file(&mut *state.delete_filename.write().await, &name).await;


    
    Ok(())
}