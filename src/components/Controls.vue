<script setup lang="ts">
defineProps<{
  isPlaying: boolean;
}>();

defineEmits<{
  previous: [];
  play: [];
  pause: [];
  next: [];
  stop: [];
}>();
</script>

<template>
  <div class="controls">
    <button class="control-btn" @click="$emit('previous')" title="上一首">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="20"
        height="20"
        viewBox="0 0 24 24"
        fill="currentColor"
      >
        <path d="M6 6h2v12H6zm3.5 6l8.5 6V6z" />
      </svg>
    </button>

    <button
      class="control-btn play-btn"
      @click="isPlaying ? $emit('pause') : $emit('play')"
      :title="isPlaying ? '暂停' : '播放'"
    >
      <svg
        v-if="!isPlaying"
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="currentColor"
      >
        <path d="M8 5v14l11-7z" />
      </svg>
      <svg
        v-else
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="currentColor"
      >
        <path d="M6 4h4v16H6zm8 0h4v16h-4z" />
      </svg>
    </button>

    <button class="control-btn" @click="$emit('next')" title="下一首">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="20"
        height="20"
        viewBox="0 0 24 24"
        fill="currentColor"
      >
        <path d="M16 18h2V6h-2zm-11-7l8.5-6v12z" />
      </svg>
    </button>

    <button class="control-btn stop-btn" @click="$emit('stop')" title="停止">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="18"
        height="18"
        viewBox="0 0 24 24"
        fill="currentColor"
      >
        <path d="M6 6h12v12H6z" />
      </svg>
    </button>
  </div>
</template>

<style scoped>
.controls {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
}

.control-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border: none;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.15);
  color: rgba(255, 255, 255, 0.9);
  cursor: pointer;
  transition: all 0.2s ease;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

.control-btn:hover {
  background: rgba(255, 255, 255, 0.25);
  transform: scale(1.05);
}

.control-btn:active {
  transform: scale(0.95);
}

.play-btn {
  width: 44px;
  height: 44px;
  background: rgba(255, 255, 255, 0.25);
}

.play-btn:hover {
  background: rgba(255, 255, 255, 0.35);
}

.stop-btn {
  width: 32px;
  height: 32px;
  opacity: 0.8;
}

.stop-btn:hover {
  opacity: 1;
}
</style>
