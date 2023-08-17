import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

const App = () => {
  const [selectedDirectory, setSelectedDirectory] = useState('');
  const [imagePaths, setImagePaths] = useState([]);
  const [selectedImage, setSelectedImage] = useState('');

  const openDirectoryDialog = async () => {
    try {
      const result = await invoke('open_directory_dialog');
      if (result) {
        setSelectedDirectory(result);
        setImagePaths(result); // Set the image paths received from Rust
      }
    } catch (error) {
      console.error('Error opening directory dialog:', error);
    }
  };

  const changeWallpaper = (imagePath) => {
    setSelectedImage(imagePath); // Set the selected image as wallpaper
  };

  return (
    <div>
      <button onClick={openDirectoryDialog}>Open Directory Dialog</button>
      {imagePaths.map((imagePath, index) => (
        <img
          key={index}
          src={`file://${imagePath}`}
          alt={`Image ${index}`}
          onClick={() => changeWallpaper(imagePath)}
        />
      ))}
      {selectedImage && (
        <div>
          <h3>Selected Wallpaper</h3>
          <img src={`file://${selectedImage}`} alt="Selected Wallpaper" />
        </div>
      )}
    </div>
  );
};

export default App;
