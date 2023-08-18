use tauri::api::http::{Request, Method, ResponseType};
// use tauri::{AppBuilder, Manager, State};

#[tauri::command]
fn fetch_wallpapers() -> Result<Vec<String>, String> {
    let url = "https://wallhaven.cc/api/v1/search?sorting=random";

    let request = Request::new(Method::Get, url)
        .header("User-Agent", "Tauri")
        .response_type(ResponseType::Text);

    let response = match tauri::api::http::fetch(request) {
        Ok(response) => response,
        Err(err) => return Err(format!("Error fetching wallpapers: {:?}", err)),
    };

    let wallpapers: Vec<String> = match response.text() {
        Ok(text) => {
            let json: serde_json::Value = match serde_json::from_str(&text) {
                Ok(json) => json,
                Err(err) => return Err(format!("Error parsing JSON: {:?}", err)),
            };

            let mut result = Vec::new();
            if let Some(array) = json["data"].as_array() {
                for item in array {
                    if let Some(path) = item["path"].as_str() {
                        result.push(format!("https://wallhaven.cc{}", path));
                    }
                }
            }
            result
        }
        Err(err) => return Err(format!("Error reading response: {:?}", err)),
    };

    Ok(wallpapers)
}

#[tauri::command]
async fn setwallpaper(image_path: String) -> Result<(), String> {
    println!("{:?}", wallpaper::get());
    wallpaper::set_from_path(&image_path).unwrap();
    wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
    println!("{:?}", wallpaper::get());
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_wallpapers, setwallpaper])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
