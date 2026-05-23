<template>
  <div
    class="drag-wrapper"
    :class="[
      `mode-${displayMode}`,
      { 'has-background': displayMode === 1 || displayMode === 3 },
    ]"
  >
    <main class="container">
      <div class="player-card">
        <div class="media-info">
          <div class="album-cover">
            <img
              class="img"
              v-if="mediaInfo?.thumbnail"
              :src="mediaInfo.thumbnail"
              alt="专辑封面"
            />
            <div v-else class="album-placeholder">
              <svg
                viewBox="0 0 24 24"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 14.5c-2.49 0-4.5-2.01-4.5-4.5S9.51 7.5 12 7.5s4.5 2.01 4.5 4.5-2.01 4.5-4.5 4.5zm0-5.5c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1z"
                  fill="rgba(255, 255, 255, 0.5)"
                />
              </svg>
            </div>
          </div>
          <div class="track-info">
            <div class="track-title">
              {{ mediaInfo?.title || "未知歌曲" }}
            </div>
            <div class="track-artist">
              {{ mediaInfo?.artist || "未知艺术家" }}
            </div>
          </div>
        </div>
        <div class="progress-container">
          <div class="progress-bar">
            <div
              class="progress-fill"
              :style="{ width: progressPercentage + '%' }"
            ></div>
          </div>
          <div class="time-display">
            <span class="time-current">{{ formatTime(currentPosition) }}</span>
            <span class="time-total">{{
              formatTime(mediaInfo?.duration || 0)
            }}</span>
          </div>
        </div>
        <Controls
          class="controls"
          :is-playing="isPlaying"
          :is-muted="isMuted"
          :display-mode="displayMode"
          @toggle="togglePlayPause"
          @prev="playPrevious"
          @next="playNext"
          @toggle-mute="toggleMute"
          @show-sessions="showSessionPicker = !showSessionPicker"
        />

        <!-- 会话选择弹出菜单 -->
        <div v-if="showSessionPicker" class="session-picker">
          <div class="session-list">
            <button
              v-for="session in sessions"
              :key="session.id"
              class="session-item"
              :class="{ active: session.id === selectedSessionId }"
              @click="selectSession(session.id)"
            >
              {{ session.name }}
            </button>
            <div v-if="sessions.length === 0" class="no-sessions">
              未找到正在播放的应用
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from "vue";
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
const isMuted = ref(false);
const savedVolume = ref(1.0);
const isDarkMode = ref(false);
// 显示模式: 0=黑色文字无背景, 1=白色文字有背景, 2=白色文字无背景, 3=黑色文字有背景
const displayMode = ref(2);
let updateInterval: number | null = null;

// 计算进度百分比
const progressPercentage = computed(() => {
  if (!mediaInfo.value || mediaInfo.value.duration === 0) {
    return 0;
  }
  return (currentPosition.value / mediaInfo.value.duration) * 100;
});

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

function playNext() {
  controlPlayback("next");
}

function playPrevious() {
  controlPlayback("previous");
}

function selectSession(sessionId: string) {
  selectedSessionId.value = sessionId;
  showSessionPicker.value = false;
  loadMediaInfo();
}

async function toggleMute() {
  try {
    const newMuteState = await invoke<boolean>("toggle_mute");
    isMuted.value = newMuteState;
  } catch (error) {
    console.error("Failed to toggle mute:", error);
  }
}

async function loadMuteState() {
  try {
    const muteState = await invoke<boolean>("get_mute_state");
    isMuted.value = muteState;
  } catch (error) {
    console.error("Failed to load mute state:", error);
  }
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

    // 只有点击控制按钮或会话选择器时才不触发拖动
    // 其他所有区域（图片、文字、背景等）都可以拖动
    if (target.closest(".control-btn") || target.closest(".session-picker")) {
      return;
    }

    // 触发窗口拖动
    try {
      await appWindow.startDragging();
    } catch (error) {
      console.error("Failed to start dragging:", error);
    }
  });

  // 监听 Ctrl+Q 切换显示模式
  document.addEventListener("keydown", (e) => {
    if (e.ctrlKey && e.key === "q") {
      e.preventDefault();
      displayMode.value = (displayMode.value + 1) % 4;
    }
  });

  // 初始加载
  loadSessions();
  loadMediaInfo();
  loadMuteState();

  // 定时更新
  updateInterval = window.setInterval(() => {
    loadMediaInfo();
    if (isPlaying.value && mediaInfo.value) {
      currentPosition.value = Math.min(
        currentPosition.value,
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
  transition: all 0.3s ease;
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
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  transition: all 0.3s ease;
}

/* 模式 0: 黑色文字无背景 */
.mode-0 .player-card {
  background: transparent;
  border: 1px solid rgba(0, 0, 0, 0.15);
}

/* 模式 1: 白色文字有背景 */
.mode-1 .player-card {
  background: rgba(20, 20, 20, 0.7);
  border: 1px solid rgba(80, 80, 80, 0.4);
}

/* 模式 2: 白色文字无背景 */
.mode-2 .player-card {
  background: transparent;
  border: 1px solid rgba(255, 255, 255, 0.25);
}

/* 模式 3: 黑色文字有背景 */
.mode-3 .player-card {
  background: rgba(255, 255, 255, 0.7);
  border: 1px solid rgba(200, 200, 200, 0.4);
}

.media-info {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 8px;
}

.img {
  opacity: 0.9;
}

.album-cover {
  width: 52px;
  height: 52px;
  border-radius: 12px;
  overflow: hidden;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.05);
  transition: background 0.3s ease;
}

.mode-0 .album-cover,
.mode-3 .album-cover {
  background: rgba(0, 0, 0, 0.05);
}

.mode-1 .album-cover,
.mode-2 .album-cover {
  background: rgba(60, 60, 60, 0.3);
}

.album-cover img {
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
}

.album-placeholder svg {
  width: 32px;
  height: 32px;
}

.mode-0 .album-placeholder svg path,
.mode-3 .album-placeholder svg path {
  fill: rgba(80, 80, 80, 0.5);
}

.mode-1 .album-placeholder svg path,
.mode-2 .album-placeholder svg path {
  fill: rgba(180, 180, 180, 0.5);
}

.track-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.track-title {
  font-size: 15px;
  font-weight: 100;
  color: white;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  transition: color 0.3s ease;
}

.mode-0 .track-title,
.mode-3 .track-title {
  color: rgba(20, 20, 20);
}

.mode-1 .track-title,
.mode-2 .track-title {
  color: rgb(255, 255, 255);
}

.track-artist {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.7);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  transition: color 0.3s ease;
}

.mode-0 .track-artist,
.mode-3 .track-artist {
  /* color: rgba(60, 60, 60, 1); */
  color: rgba(20, 20, 20);
}

.mode-1 .track-artist,
.mode-2 .track-artist {
  /* color: rgba(180, 180, 180, 1); */
  color: rgb(255, 255, 255);
}

.progress-container {
  width: 100%;
  position: relative;
  top: 3px;
  margin-bottom: -4px;
}

.progress-bar {
  height: 5px;
  background-color: rgba(200, 200, 200, 0.3);
  border-radius: 2px;
  overflow: hidden;
  margin-bottom: 4px;
  transition: background-color 0.3s ease;
}

.mode-0 .progress-bar,
.mode-3 .progress-bar {
  background-color: rgba(100, 100, 100, 0.3);
}

.mode-1 .progress-bar,
.mode-2 .progress-bar {
  background-color: rgba(80, 80, 80, 0.5);
}

.progress-fill {
  height: 100%;
  background-color: white;
  transition:
    width 0.3s ease,
    background-color 0.3s ease;
}

.mode-0 .progress-fill,
.mode-3 .progress-fill {
  background-color: rgba(70, 70, 70, 0.9);
}

.mode-1 .progress-fill,
.mode-2 .progress-fill {
  background-color: rgb(255, 255, 255);
}

.time-display {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 5px;
  font-size: 11px;
  color: rgba(255, 255, 255, 0.7);
  margin-bottom: 4px;
  transition: color 0.3s ease;
}

.mode-0 .time-display,
.mode-3 .time-display {
  /* color: rgba(80, 80, 80, 0.8); */
  color: rgba(20, 20, 20);
}

.mode-1 .time-display,
.mode-2 .time-display {
  /* color: rgba(160, 160, 160, 0.8); */
  color: rgb(255, 255, 255);
}

.controls {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
  margin-bottom: -12px;
}

.session-picker {
  position: absolute;
  bottom: 55px;
  right: 18px;
  background: rgba(40, 40, 40, 0.9);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-radius: 12px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  padding: 8px;
  min-width: 200px;
  max-width: 280px;
  max-height: 92px;
  overflow-y: auto;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
  z-index: 1000;
  animation: slideUp 0.2s ease;
  transition: all 0.3s ease;
}

.mode-0 .session-picker,
.mode-3 .session-picker {
  background: rgba(240, 240, 240, 0.95);
  border: 1px solid rgba(180, 180, 180, 0.3);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
}

.mode-1 .session-picker,
.mode-2 .session-picker {
  background: rgba(25, 25, 25, 0.95);
  border: 1px solid rgba(100, 100, 100, 0.3);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.6);
}

.session-picker::-webkit-scrollbar {
  width: 6px;
}

.session-picker::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 3px;
}

.mode-0 .session-picker::-webkit-scrollbar-track,
.mode-3 .session-picker::-webkit-scrollbar-track {
  background: rgba(180, 180, 180, 0.2);
}

.mode-1 .session-picker::-webkit-scrollbar-track,
.mode-2 .session-picker::-webkit-scrollbar-track {
  background: rgba(100, 100, 100, 0.2);
}

.session-picker::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.3);
  border-radius: 3px;
  transition: background 0.2s ease;
}

.mode-0 .session-picker::-webkit-scrollbar-thumb,
.mode-3 .session-picker::-webkit-scrollbar-thumb {
  background: rgba(120, 120, 120, 0.5);
}

.mode-1 .session-picker::-webkit-scrollbar-thumb,
.mode-2 .session-picker::-webkit-scrollbar-thumb {
  background: rgba(120, 120, 120, 0.5);
}

.session-picker::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.5);
}

.mode-0 .session-picker::-webkit-scrollbar-thumb:hover,
.mode-3 .session-picker::-webkit-scrollbar-thumb:hover {
  background: rgba(150, 150, 150, 0.7);
}

.mode-1 .session-picker::-webkit-scrollbar-thumb:hover,
.mode-2 .session-picker::-webkit-scrollbar-thumb:hover {
  background: rgba(150, 150, 150, 0.7);
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.session-picker-header {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.6);
  padding: 8px 12px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  margin-bottom: 4px;
}

.mode-0 .session-picker-header,
.mode-3 .session-picker-header {
  color: rgba(80, 80, 80, 0.7);
  border-bottom: 1px solid rgba(150, 150, 150, 0.3);
}

.mode-1 .session-picker-header,
.mode-2 .session-picker-header {
  color: rgba(180, 180, 180, 0.7);
  border-bottom: 1px solid rgba(100, 100, 100, 0.3);
}

.session-item {
  width: 100%;
  padding: 10px 12px;
  background: transparent;
  border: none;
  color: white;
  text-align: left;
  cursor: pointer;
  border-radius: 8px;
  font-size: 13px;
  transition: all 0.2s ease;
  display: block;
}

.session-item:hover {
  background: rgba(255, 255, 255, 0.1);
}

.mode-0 .session-item,
.mode-3 .session-item {
  color: rgba(40, 40, 40, 0.95);
}

.mode-0 .session-item:hover,
.mode-3 .session-item:hover {
  background: rgba(150, 150, 150, 0.3);
}

.mode-1 .session-item,
.mode-2 .session-item {
  color: rgba(220, 220, 220, 0.95);
}

.mode-1 .session-item:hover,
.mode-2 .session-item:hover {
  background: rgba(100, 100, 100, 0.3);
}

.session-item.active {
  background: rgba(255, 255, 255, 0.15);
  font-weight: 500;
}

.mode-0 .session-item.active,
.mode-3 .session-item.active {
  background: rgba(150, 150, 150, 0.4);
}

.mode-1 .session-item.active,
.mode-2 .session-item.active {
  background: rgba(120, 120, 120, 0.4);
}

.no-sessions {
  padding: 20px 12px;
  text-align: center;
  color: rgba(255, 255, 255, 0.5);
  font-size: 12px;
  transition: color 0.3s ease;
}

.mode-0 .no-sessions,
.mode-3 .no-sessions {
  color: rgba(100, 100, 100, 0.6);
}

.mode-1 .no-sessions,
.mode-2 .no-sessions {
  color: rgba(150, 150, 150, 0.6);
}
</style>
