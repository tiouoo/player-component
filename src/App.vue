<script setup lang="ts">
import { onMounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";

async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value });
}

onMounted(() => {
  const appWindow = getCurrentWindow();

  // 为整个文档添加拖动功能
  document.addEventListener("mousedown", async (e) => {
    const target = e.target as HTMLElement;

    // 如果点击的是交互元素，不触发拖动
    if (
      target.tagName === "INPUT" ||
      target.tagName === "BUTTON" ||
      target.tagName === "A" ||
      target.tagName === "IMG" ||
      target.closest("button") ||
      target.closest("input") ||
      target.closest("a") ||
      target.closest("form")
    ) {
      return;
    }

    // 触发窗口拖动
    try {
      await appWindow.startDragging();
    } catch (error) {
      console.error("Failed to start dragging:", error);
    }
  });
});
</script>

<template>
  <div class="drag-wrapper">
    <main class="container">
      <h1>Welcome to Tauri + Vue</h1>
    </main>
  </div>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  color: #0f0f0f;
  background-color: transparent;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

body {
  background-color: transparent;
  margin: 0;
  padding: 0;
}

.drag-wrapper {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  cursor: move;
  user-select: none;
}

.container {
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  height: 165px;
  width: 326px;
  background-color: rgba(255, 255, 255, 0.3);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border-radius: 26px;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
  pointer-events: auto;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
  pointer-events: auto;
  cursor: pointer;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
  user-select: none;
}

p {
  user-select: none;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  pointer-events: auto;
  user-select: auto;
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

input {
  cursor: text;
}

#greet-input {
  margin-right: 5px;
}
</style>
