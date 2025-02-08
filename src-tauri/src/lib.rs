pub mod backend;
pub mod requests;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
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
