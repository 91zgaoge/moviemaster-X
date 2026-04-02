import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css";

// 初始化主题 - 在应用渲染前设置
const initializeTheme = () => {
  const savedTheme = localStorage.getItem('moviemaster-theme');
  if (savedTheme) {
    document.documentElement.setAttribute('data-theme', savedTheme);
  } else {
    // 默认春季主题
    document.documentElement.setAttribute('data-theme', 'spring');
  }
};

// 立即执行主题初始化
initializeTheme();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
