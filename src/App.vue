<template>
  <div class="drag-wrapper">
    <main class="container">
      <div class="player-card">
        <Controls @toggle="togglePlayPause" />
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import Controls from "./components/Controls.vue";

interface MediaInfo {
  title: string;
  artist: string;
  album: string;
  thumbnail: string | null;
  playback_status: string;
  position: number;
  duration: number;
}

interface MediaSession {
  id: string;
  name: string;
}

const mediaInfo = ref<MediaInfo | null>(null);
const sessions = ref<MediaSession[]>([]);
const selectedSessionId = ref<string | null>(null);
const showSessionPicker = ref(false);
const isPlaying = ref(false);
const currentPosition = ref(0);
let updateInterval: number | null = null;

async function loadSessions() {
  try {
    const result = await invoke<MediaSession[]>("get_media_sessions");
    sessions.value = result;
  } catch (error) {
    console.error("Failed to load sessions:", error);
  }
}

async function loadMediaInfo() {
  try {
    const info = await invoke<MediaInfo>("get_media_info", {
      sessionId: selectedSessionId.value,
    });
    mediaInfo.value = info;
    isPlaying.value = info.playback_status === "playing";
    currentPosition.value = info.position;
  } catch (error) {
    console.error("Failed to load media info:", error);
    mediaInfo.value = null;
  }
}

async function controlPlayback(action: string) {
  try {
    await invoke("control_playback", { action });
    setTimeout(loadMediaInfo, 200);
  } catch (error) {
    console.error("Failed to control playback:", error);
  }
}

function togglePlayPause() {
  if (isPlaying.value) {
    controlPlayback("pause");
  } else {
    controlPlayback("play");
  }
}

function selectSession(sessionId: string) {
  selectedSessionId.value = sessionId;
  showSessionPicker.value = false;
  loadMediaInfo();
}

function formatTime(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
}

onMounted(() => {
  const appWindow = getCurrentWindow();

  // 为整个文档添加拖动功能
  document.addEventListener("mousedown", async (e) => {
    const target = e.target as HTMLElement;

    // 如果点击的是交互元素,不触发拖动
    if (
      target.tagName === "INPUT" ||
      target.tagName === "BUTTON" ||
      target.tagName === "A" ||
      target.closest("button") ||
      target.closest(".session-picker")
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

  // 初始加载
  loadSessions();
  loadMediaInfo();

  // 定时更新
  updateInterval = window.setInterval(() => {
    loadMediaInfo();
    if (isPlaying.value && mediaInfo.value) {
      currentPosition.value = Math.min(
        currentPosition.value + 1,
        mediaInfo.value.duration,
      );
    }
  }, 1000);
});

onUnmounted(() => {
  if (updateInterval) {
    clearInterval(updateInterval);
  }
});
</script>

<style scoped>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  background-color: transparent;
  margin: 0;
  padding: 0;
  overflow: hidden;
}

.drag-wrapper {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  user-select: none;
  display: flex;
  align-items: center;
  justify-content: center;
}

.container {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.player-card {
  position: relative;
  width: 326px;
  height: 165px;
  padding: 18px;
  backdrop-filter: blur(30px);
  -webkit-backdrop-filter: blur(30px);
  border-radius: 26px;
  border: 1px solid rgba(255, 255, 255, 0.25);
  overflow: hidden;
}
</style>
