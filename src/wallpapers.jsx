import React, { useState, useEffect } from 'react';
import axios from 'axios';

const Wallpapers = () => {
  const [wallpapers, setWallpapers] = useState([]);

  useEffect(() => {
    // Fetch wallpapers from Wallhaven API
    axios.get('https://wallhaven.cc/api/v1/search?q=nature')
      .then(response => {
        setWallpapers(response.data.data);
      })
      .catch(error => {
        console.error('Error fetching wallpapers:', error);
      });
  }, []);

  return (
    <div>
      <div className="wallpaper-list">
        {wallpapers.map(wallpaper => (
          <img key={wallpaper.id} src={wallpaper.path} alt={wallpaper.id} />
        ))}
      </div>
    </div>
  );
};

export default Wallpapers;
