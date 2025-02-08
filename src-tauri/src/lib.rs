pub mod backend;
pub mod requests;

use dirs::config_dir;
use std::fs;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn store_settings(settings: String) -> Result<(), String> {
    println!("Storing Settings...");
    let mut file_path = config_dir().unwrap();
    file_path.push("defttorrent");
    fs::create_dir_all(&file_path)
        .map_err(|e| format!("Failed to create configuration directory: {}", e))?;

    file_path.push("settings.dft");

    fs::write(file_path, settings)
        .map_err(|e| format!("Failed to write settings to file: {}", e))?;
    Ok(())
}

#[tauri::command]
async fn load_settings() -> Result<String, String> {
    println!("Loading Settings...");
    let mut file_path = config_dir().unwrap();
    file_path.push("defttorrent");
    file_path.push("settings.dft");

    let settings = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read settings from file: {}", e))?;
    Ok(settings)
}

#[tauri::command]
fn console_log(message: String) {
    println!("{}", message);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            store_settings,
            load_settings,
            console_log,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tokio::main]
async fn torrent_main() -> Result<(), Box<dyn std::error::Error>> {
    let data = backend::read_torrent_file("test2.torrent")?;
    let info_hash_str = String::from_utf8_lossy(&data.info_hash);
    println!("Torrent file: {:?}", data);
    println!("{}", info_hash_str);

    let announce_url = "http://tracker.opentrackr.org:1337/announce";
    println!("Announcing to tracker: {}", announce_url);
    println!("Tracker in torrent: {:?}", data.announce);
    let peers = requests::announce(&data.info_hash, &data.announce).await?;
    // println!("{:?}", peers);
    Ok(())
}

fn main() {}
