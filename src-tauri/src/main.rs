use tauri::{ Window, State};
use tauri_dialog::file::open_directory;

struct AppState {
    selected_directory: Option<String>,
    image_paths: Vec<String>,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            app.add_resource(AppState {
                selected_directory: None,
                image_paths: Vec::new(),
            });
        })
        .invoke_handler(tauri::generate_handler![open_directory_dialog])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn open_directory_dialog(app_state: State<AppState>, window: Window) {
    tauri::async_runtime::block_on(async {
        if let Some(result) = open_directory().await.unwrap() {
            let directory_path = result.path();
            app_state.selected_directory.replace(directory_path.to_string_lossy().to_string());
            let image_paths = // Retrieve image paths from the selected directory
            app_state.image_paths = image_paths;
            window.emit("directorySelected", app_state.image_paths.clone()).unwrap();
        }
    });
}
