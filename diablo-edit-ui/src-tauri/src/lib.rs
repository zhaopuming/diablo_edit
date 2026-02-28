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

#[tauri::command]
async fn save_save_file(path: String, save: D2sSave) -> Result<(), String> {
    // Serialize the save data
    let mut bytes = d2s_core::serialize_d2s(&save).map_err(|e| e.to_string())?;

    // Update file_size in header (bytes 8-11, little endian)
    let file_size = bytes.len() as u32;
    bytes[8..12].copy_from_slice(&file_size.to_le_bytes());

    // Calculate and update checksum (bytes 12-15)
    let checksum = calculate_checksum(&bytes);
    bytes[12..16].copy_from_slice(&checksum.to_le_bytes());

    // Write to file
    fs::write(&path, &bytes).map_err(|e| e.to_string())?;
    Ok(())
}

/// D2S checksum algorithm
fn calculate_checksum(data: &[u8]) -> u32 {
    let mut checksum: u32 = 0;
    for (i, &byte) in data.iter().enumerate() {
        // Skip the checksum field itself (bytes 12-15)
        if i >= 12 && i < 16 {
            continue;
        }
        // Rotate left by 1, then XOR with byte
        checksum = ((checksum << 1) | (checksum >> 31)) ^ byte as u32;
    }
    checksum
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    log::info!("Diablo Edit2 Backend Initialized");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![greet, open_save_file, save_save_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
