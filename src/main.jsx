import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import Wallpapers from "./wallpapers";
import dirUpload from "./dirUpload";

import "./styles.css";

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <App />
    <Wallpapers />
  </React.StrictMode>,
);
