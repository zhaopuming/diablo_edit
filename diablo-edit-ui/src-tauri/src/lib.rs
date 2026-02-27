use d2s_core::D2sSave;
use std::fs;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_save_file(path: String) -> Result<D2sSave, String> {
    let bytes = fs::read(&path).map_err(|e| e.to_string())?;
    let save = d2s_core::parse_d2s(&bytes).map_err(|e| e.to_string())?;
    Ok(save)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![greet, open_save_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
