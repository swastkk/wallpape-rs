use reqwest;
use wallpaper;
use serde_json;
use tempfile;
use log::info;
#[tauri::command]
async fn fetch_wallpapers() -> Result<Vec<String>, String> {
    let url: &str = "https://wallhaven.cc/api/v1/search?sorting=random";

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
    // Download the image using reqwest
    info!("Downloading image from: {}", image_url);
    let image_data = reqwest::get(&image_url)
        .await
        .map_err(|err| format!("Error downloading image: {:?}", err))?
        .bytes()
        .await
        .map_err(|err| format!("Error reading image bytes: {:?}", err))?;

    // Create a temporary file to store the downloaded image
    let temp_dir = tempfile::tempdir().map_err(|err| format!("Error creating temp dir: {:?}", err))?;
    let image_path = temp_dir.path().join("downloaded_image.png");
    info!("Setting wallpaper: {:?}", &image_path);
    // Write the image data to the temporary file
    std::fs::write(&image_path, &image_data)
        .map_err(|err| format!("Error writing image data to file: {:?}", err))?;
    info!("Temp file made!");
    // Set the downloaded image as the wallpaper
    println!("{:?}", wallpaper::get());
    wallpaper::set_from_path(&image_url)
        .map_err(|err| format!("Error setting wallpaper: {:?}", err))?;
    wallpaper::set_mode(wallpaper::Mode::Crop)
        .map_err(|err| format!("Error setting wallpaper mode: {:?}", err))?;
    println!("{:?}", wallpaper::get());

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_wallpapers, setwallpaper])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
