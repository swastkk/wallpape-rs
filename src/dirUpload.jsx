import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

const dirUpload = () => {
  const [selectedFilePath, setSelectedFilePath] = useState('');

  const openFileDialog = async () => {
    try {
      const result = await invoke('open_file_dialog');
      if (result) {
        setSelectedFilePath(result);
      }
    } catch (error) {
      console.error('Error opening file dialog:', error);
    }
  };

  return (
    <div>
      <button onClick={openFileDialog}>Open File Dialog</button>
      {selectedFilePath && (
        <img src={`file://${selectedFilePath}`} alt="Selected Wallpaper" />
      )}
    </div>
  );
};

export default dirUpload;
