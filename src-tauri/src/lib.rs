// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod printer;
mod printer_api;

#[tauri::command]
fn greet(name: &str) -> Result<String, String> {

    printer::print_document(name);

    if name == "error" {
        return Err("Error: Invalid name provided".to_string());             
    }else{   
        Ok(format!("Hello, {}! You've been greeted from Rust!", name))
    }
}


#[tauri::command]
fn greet2(name: &str) -> Result<String, String> {

    printer::print_document(name);

    if name == "error" {
        return Err("Error: Invalid name provided".to_string());             
    }else{   
        Ok(format!("Hello, {}! You've been greeted from Rust2!", name))
    }
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,greet2])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
