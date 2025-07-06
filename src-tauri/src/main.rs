use reqwest;
use wallpaper;
use serde_json;
use std::path::{Path, PathBuf};
use rand::Rng;
use rand::distr::Alphanumeric;
use std::env;

#[tauri::command]
async fn fetch_wallpapers() -> Result<Vec<String>, String> {
    let url: String = std::env::var("WALLPAPER_API_URL")
        .map_err(|e| format!("Missing or invalid WALLPAPER_API_URL: {:?}", e))?;


    let response = reqwest::get(url)
        .await
        .map_err(|err| format!("Error fetching wallpapers: {:?}", err))?;

    let text = response.text().await.map_err(|err| format!("Error reading response: {:?}", err))?;
    let json: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("Error parsing JSON: {:?}", err))?;

    let wallpapers = json["data"]
        .as_array()
        .map(|array| {
            array
                .iter()
                .filter_map(|item| item["path"].as_str().map(|path| format!("{}", path)))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    Ok(wallpapers)
}


#[tauri::command]
async fn setwallpaper(image_url: String) -> Result<(), String> {
    let wallpaper_dir = Path::new("/var/wallpapers");
    let image_data = reqwest::get(&image_url)
        .await
        .map_err(|err| format!("Error downloading image: {:?}", err))?
        .bytes()
        .await
        .map_err(|err| format!("Error reading image bytes: {:?}", err))?;


    let image_name: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect::<String>() + ".png";

    let image_path: PathBuf = wallpaper_dir.join(&image_name);
    std::fs::write(&image_path, &image_data)
        .map_err(|err| format!("Error writing image to file: {:?}", err))?;

    wallpaper::set_from_path(
        image_path
            .to_str()
            .ok_or("Error converting path to string")?,
    )
    .map_err(|err| format!("Error setting wallpaper: {:?}", err))?;

    Ok(())
}

fn main() {
    dotenvy::dotenv().ok();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_wallpapers, setwallpaper])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
