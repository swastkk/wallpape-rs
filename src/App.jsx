import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import logo from './assets/logo.png';
function App() {
  const [images, setImages] = useState([]);
  const [selectedImage, setSelectedImage] = useState("");
  const [wallpapers, setWallpapers] = useState([]); 

  useEffect(() => {
    async function fetchImages() {
      const fetchedImages = await invoke("fetch_wallpapers");
      setImages(fetchedImages);
      setWallpapers(fetchedImages);
    }
    fetchImages();
  }, []);


  async function setWallpaper(imagePath) {
    console.log("Setting wallpaper:", imagePath);
    setSelectedImage(imagePath);
    await invoke("setwallpaper", { imageUrl: imagePath });
  }
  async function refreshWallpapers() {
    console.log("Refresh btn clicked!");
    const newWallpapers = await invoke("fetch_wallpapers");
    console.log("New Wallpapers: ", newWallpapers);
    setWallpapers(newWallpapers);
  }
  return (
    <div className="container">
      <div>
        <img className="logo" src={logo} alt="" />
        <p className="para">Click on any wallpaper to set it as wallpaper!</p>
      </div>
      <div className="image-list">
      {wallpapers.map((image, index) => (
        <img
          key={index}
          src={image}
          alt={`Image ${index}`}
          onClick={() => setWallpaper(image)}
          className={selectedImage === image ? "selected" : ""} />
      ))}
      </div>
      <button className="refreshBtn" onClick={refreshWallpapers}>Refresh Wallpapers ↻</button>
    </div>
  );
}

export default App;
