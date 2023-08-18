import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [wallpapers, setWallpapers] = useState([]);
  const [selectedWallpaper, setSelectedWallpaper] = useState("");
  const [error, setError] = useState("");

  async function fetchWallpapers() {
    try {
      const result = await invoke("fetch_wallpapers");
      setWallpapers(result);
    } catch (error) {
      console.error("Error fetching wallpapers:", error);
      setError("Error fetching wallpapers.");
    }
  }

  async function setWallpaper(imagePath) {
    try {
      await invoke("setwallpaper", imagePath);
      setSelectedWallpaper(imagePath);
    } catch (error) {
      console.error("Error setting wallpaper:", error);
      setError("Error setting wallpaper.");
    }
  }

  return (
    <div className="container">
      <h1>Wallpaper Changer</h1>

      <button onClick={fetchWallpapers}>Fetch Wallpapers</button>

      <div className="wallpapers">
        {wallpapers.map((wallpaper, index) => (
          <img
            key={index}
            src={wallpaper}
            alt={`Wallpaper ${index}`}
            className={selectedWallpaper === wallpaper ? "selected" : ""}
            onClick={() => setWallpaper(wallpaper)}
          />
        ))}
      </div>
      {error && <p className="error">{error}</p>}
    </div>
  );
}

export default App;
