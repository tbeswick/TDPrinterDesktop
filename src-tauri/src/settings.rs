use keyring::Entry;
use serde::Serialize;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const SETTINGS_STORE: &str = "settings.json";

const API_URL_KEY: &str = "api_url";

const KEYRING_SERVICE: &str = "3d-printer-manager";
const KEYRING_USERNAME: &str = "printer-api";


/// Settings returned to the React frontend.
///
/// Notice that the password is deliberately NOT included.
#[derive(Debug, Serialize)]
pub struct PrinterSettings {
    pub api_url: String,
    pub has_password: bool,
}


/// Get the Windows Credential Manager entry used by this application.
fn password_entry() -> Result<Entry, String> {
    Entry::new(
        KEYRING_SERVICE,
        KEYRING_USERNAME,
    )
    .map_err(|e| e.to_string())
}


/// Return the currently configured printer settings.
///
/// The password itself is never returned to React.
#[tauri::command]
pub fn get_printer_settings(
    app: AppHandle,
) -> Result<PrinterSettings, String> {

    let store = app
        .store(SETTINGS_STORE)
        .map_err(|e| e.to_string())?;

    let api_url = store
        .get(API_URL_KEY)
        .and_then(|value| value.as_str().map(String::from))
        .unwrap_or_default();

    let entry = password_entry()?;

    let has_password = entry.get_password().is_ok();

    Ok(PrinterSettings {
        api_url,
        has_password,
    })
}


/// Save the printer API URL and, optionally, a new password.
///
/// An empty password means "leave the existing password unchanged".
#[tauri::command]
pub fn save_printer_settings(
    app: AppHandle,
    api_url: String,
    password: String,
) -> Result<(), String> {

    // -------------------------------
    // Save API URL
    // -------------------------------

    let store = app
        .store(SETTINGS_STORE)
        .map_err(|e| e.to_string())?;

    store.set(
        API_URL_KEY,
        serde_json::Value::String(api_url),
    );

    store
        .save()
        .map_err(|e| e.to_string())?;


    // -------------------------------
    // Save password
    // -------------------------------

    if !password.is_empty() {

        let entry = password_entry()?;

        entry
            .set_password(&password)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}


/// Remove the stored printer API password.
#[tauri::command]
pub fn delete_printer_password() -> Result<(), String> {

    let entry = password_entry()?;

    match entry.delete_credential() {
        Ok(_) => Ok(()),

        // Treat "not found" as already deleted.
        Err(_) => Ok(()),
    }
}


/// Get the configured API URL for use by the Rust printer client.
pub fn get_api_url(
    app: &AppHandle,
) -> Result<String, String> {

    let store = app
        .store(SETTINGS_STORE)
        .map_err(|e| e.to_string())?;

    let api_url = store
        .get(API_URL_KEY)
        .and_then(|value| value.as_str().map(String::from))
        .unwrap_or_default();

    if api_url.is_empty() {
        return Err("Printer API URL has not been configured".to_string());
    }

    Ok(api_url)
}


/// Get the printer API password.
///
/// This function should only be used by Rust code that needs
/// to communicate with the printer.
///
/// The password is never sent back to React.
pub fn get_api_password() -> Result<String, String> {

    let entry = password_entry()?;

    entry
        .get_password()
        .map_err(|e| e.to_string())
}