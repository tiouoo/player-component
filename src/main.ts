import { createApp } from "vue";
import App from "./App.vue";
import "./styles/fonts.css";

// 全局屏蔽右键菜单
document.addEventListener("contextmenu", (e) => {
  e.preventDefault();
});

createApp(App).mount("#app");
