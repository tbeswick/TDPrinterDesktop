
mod printer;
mod printer_api;
mod models;

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



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    printer::start_background_thread();   

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,greet2])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
