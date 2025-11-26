use serde::Serialize;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State};

// Shared state to hold the graph in memory
struct AppState {
    current_path: Mutex<Option<String>>,
}

#[derive(Clone, Serialize)]
struct GraphStats {
    node_count: usize,
    path: String,
}

// Command: Open a folder and "scan" it
#[tauri::command]
async fn open_graph(app: AppHandle, state: State<'_, AppState>) -> Result<GraphStats, String> {
    use tauri::plugin::dialog::DialogExt;

    // Native file picker (works on Android too!)
    let folder_path = app.dialog().file().blocking_pick_folder();

    match folder_path {
        Some(path) => {
            let path_str = path.to_string();

            // In a real app, you would spawn a thread here to parse thousands of .md files
            // For now, we simulate a scan
            *state.current_path.lock().unwrap() = Some(path_str.clone());

            // Emit an event if needed, or just return stats
            Ok(GraphStats {
                node_count: 4200, // Simulated count
                path: path_str,
            })
        }
        None => Err("Operation cancelled".into()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            current_path: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![open_graph])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
