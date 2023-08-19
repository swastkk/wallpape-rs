import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [images, setImages] = useState([]);
  const [selectedImage, setSelectedImage] = useState("");
  const [wallpapers, setWallpapers] = useState([]); 

  useEffect(() => {
    async function fetchImages() {
      const fetchedImages = await invoke("fetch_wallpapers");
      setImages(fetchedImages);
    }
    fetchImages();
  }, []);

  async function setWallpaper(imagePath) {
    console.log("Button clicked. Image path:", imagePath);
    setSelectedImage(imagePath);
    await invoke("setwallpaper", { imageUrl: imagePath }); 
  }
  async function refreshWallpapers() {
    console.log("Refresh btn clicked!");
    const newWallpapers = await invoke("fetch_wallpapers");
    setWallpapers(newWallpapers);
  }
  return (
    <div className="container">
      <h1>wallpape-rs</h1>
      <div className="image-list">
        {images.map((image, index) => (
          <img
            key={index}
            src={image}
            alt={`Image ${index}`}
            onClick={() => setWallpaper(image)}
            className={selectedImage === image ? "selected" : ""}
          />
        ))}
      </div>
      <button onClick={refreshWallpapers}>Refresh Wallpapers</button>
    </div>
  );
}

export default App;
