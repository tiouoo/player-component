<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";

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
      target.tagName === "IMG" ||
      target.closest("button") ||
      target.closest("input") ||
      target.closest("a") ||
      target.closest("form") ||
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

<template>
  <div class="drag-wrapper">
    <main class="container">
      <div class="player-card">
        <!-- 应用选择按钮 -->
        <button
          class="app-selector"
          @click="showSessionPicker = !showSessionPicker"
        >
          <svg
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <circle cx="12" cy="12" r="3" />
            <path
              d="M12 1v6m0 6v6m8.66-13.66l-4.24 4.24m-4.24 4.24l-4.24 4.24M23 12h-6m-6 0H1m20.66 8.66l-4.24-4.24m-4.24-4.24l-4.24-4.24"
            />
          </svg>
        </button>

        <!-- 应用选择器 -->
        <div v-if="showSessionPicker" class="session-picker">
          <div class="session-list">
            <div
              v-for="session in sessions"
              :key="session.id"
              class="session-item"
              @click="selectSession(session.id)"
            >
              {{ session.name }}
            </div>
            <div v-if="sessions.length === 0" class="session-item empty">
              没有找到正在播放的应用
            </div>
          </div>
        </div>

        <!-- 专辑封面和信息 -->
        <div class="media-header">
          <div class="album-art">
            <img
              v-if="mediaInfo?.thumbnail"
              :src="mediaInfo.thumbnail"
              alt="Album Art"
            />
            <div v-else class="album-placeholder">
              <svg viewBox="0 0 24 24" fill="currentColor">
                <path
                  d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"
                />
              </svg>
            </div>
          </div>
          <div class="media-info">
            <h2 class="song-name">{{ mediaInfo?.title || "Song Name" }}</h2>
            <p class="artist-name">
              {{ mediaInfo?.artist || "The Author of the Song" }}
            </p>
          </div>
        </div>

        <!-- 进度条 -->
        <div class="progress-section">
          <span class="time-label">{{ formatTime(currentPosition) }}</span>
          <div class="progress-bar">
            <div
              class="progress-fill"
              :style="{
                width: mediaInfo
                  ? `${(currentPosition / mediaInfo.duration) * 100}%`
                  : '0%',
              }"
            ></div>
          </div>
          <span class="time-label">{{
            mediaInfo ? formatTime(mediaInfo.duration) : "00:00"
          }}</span>
        </div>

        <!-- 控制按钮 -->
        <div class="controls">
          <button class="control-btn" @click="controlPlayback('previous')">
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M6 6h2v12H6zm3.5 6l8.5 6V6z" />
            </svg>
          </button>
          <button class="control-btn play-btn" @click="togglePlayPause">
            <svg v-if="!isPlaying" viewBox="0 0 24 24" fill="currentColor">
              <path d="M8 5v14l11-7z" />
            </svg>
            <svg v-else viewBox="0 0 24 24" fill="currentColor">
              <path d="M6 4h4v16H6V4zm8 0h4v16h-4V4z" />
            </svg>
          </button>
          <button class="control-btn" @click="controlPlayback('next')">
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z" />
            </svg>
          </button>
          <button class="control-btn cast-btn">
            <svg
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <circle cx="12" cy="12" r="10" />
              <circle cx="12" cy="12" r="3" />
              <path d="M12 2v4m0 12v4M2 12h4m12 0h4" />
            </svg>
          </button>
        </div>
      </div>
    </main>
  </div>
</template>

<style scoped>
:root {
  font-family:
    -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu,
    Cantarell, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  color: #ffffff;
  background-color: transparent;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
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
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
}

.container {
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.player-card {
  position: relative;
  width: 600px;
  padding: 40px;
  background: linear-gradient(
    135deg,
    rgba(100, 200, 255, 0.3) 0%,
    rgba(150, 100, 255, 0.3) 50%,
    rgba(255, 100, 150, 0.3) 100%
  );
  backdrop-filter: blur(40px);
  -webkit-backdrop-filter: blur(40px);
  border-radius: 40px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
}

.app-selector {
  position: absolute;
  top: 20px;
  right: 20px;
  width: 50px;
  height: 50px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.3);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
  pointer-events: auto;
  padding: 0;
}

.app-selector:hover {
  background: rgba(255, 255, 255, 0.3);
  transform: scale(1.05);
}

.app-selector svg {
  width: 24px;
  height: 24px;
  color: white;
}

.session-picker {
  position: absolute;
  top: 80px;
  right: 20px;
  background: rgba(30, 30, 40, 0.95);
  backdrop-filter: blur(20px);
  border-radius: 16px;
  padding: 8px;
  min-width: 200px;
  max-height: 300px;
  overflow-y: auto;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
  border: 1px solid rgba(255, 255, 255, 0.1);
  z-index: 1000;
  pointer-events: auto;
}

.session-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.session-item {
  padding: 12px 16px;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.2s ease;
  color: white;
  font-size: 14px;
}

.session-item:hover {
  background: rgba(255, 255, 255, 0.1);
}

.session-item.empty {
  cursor: default;
  color: rgba(255, 255, 255, 0.5);
}

.session-item.empty:hover {
  background: transparent;
}

.media-header {
  display: flex;
  align-items: center;
  gap: 24px;
  margin-bottom: 30px;
}

.album-art {
  width: 120px;
  height: 120px;
  border-radius: 20px;
  overflow: hidden;
  background: rgba(255, 255, 255, 0.1);
  flex-shrink: 0;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
}

.album-art img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.album-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(
    135deg,
    rgba(100, 100, 255, 0.3),
    rgba(255, 100, 200, 0.3)
  );
}

.album-placeholder svg {
  width: 50px;
  height: 50px;
  color: white;
  opacity: 0.6;
}

.media-info {
  flex: 1;
  min-width: 0;
}

.song-name {
  font-size: 32px;
  font-weight: 600;
  margin: 0 0 8px 0;
  color: white;
  text-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.artist-name {
  font-size: 18px;
  margin: 0;
  color: rgba(255, 255, 255, 0.8);
  text-shadow: 0 1px 4px rgba(0, 0, 0, 0.3);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.progress-section {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 30px;
}

.time-label {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.9);
  font-weight: 500;
  min-width: 45px;
  text-align: center;
}

.progress-bar {
  flex: 1;
  height: 6px;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 3px;
  overflow: hidden;
  position: relative;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #4ade80, #3b82f6);
  border-radius: 3px;
  transition: width 0.3s ease;
  box-shadow: 0 0 10px rgba(74, 222, 128, 0.5);
}

.controls {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 20px;
}

.control-btn {
  width: 50px;
  height: 50px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.15);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
  pointer-events: auto;
  padding: 0;
}

.control-btn:hover {
  background: rgba(255, 255, 255, 0.25);
  transform: scale(1.05);
}

.control-btn:active {
  transform: scale(0.95);
}

.control-btn svg {
  width: 24px;
  height: 24px;
  color: white;
}

.play-btn {
  width: 70px;
  height: 70px;
  background: rgba(255, 255, 255, 0.25);
}

.play-btn svg {
  width: 32px;
  height: 32px;
}

.cast-btn svg {
  width: 20px;
  height: 20px;
}
</style>
