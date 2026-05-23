<template>
  <div class="controls-container">
    <button class="control-btn" @click="$emit('prev')" title="上一首">
      <svg viewBox="0 0 10 10" fill="none" xmlns="http://www.w3.org/2000/svg">
        <g filter="url(#filter0_i_1134_3223)">
          <path
            d="M3.27539 2.6001C2.94411 2.6001 2.67495 2.86846 2.6748 3.19971V6.80029C2.67498 7.13151 2.94413 7.3999 3.27539 7.3999C3.6065 7.39972 3.87482 7.1314 3.875 6.80029V6.59508C3.875 6.05769 3.875 5.789 3.95293 5.72895C3.97798 5.70965 3.99582 5.70101 4.0265 5.69335C4.12194 5.66949 4.34204 5.84362 4.78223 6.19189L5.2998 6.60205C5.92152 7.09394 6.23295 7.34055 6.49316 7.35107C6.75995 7.3617 7.01468 7.23779 7.17188 7.02197C7.3251 6.81143 7.3252 6.41514 7.3252 5.62256V4.37744C7.3252 3.58487 7.32509 3.18857 7.17188 2.97803C7.01468 2.76221 6.75995 2.6383 6.49316 2.64893C6.23295 2.65945 5.92152 2.90606 5.2998 3.39795L4.78223 3.80811C4.34189 4.15649 4.12172 4.33068 4.02607 4.30655C3.99575 4.2989 3.97809 4.29035 3.95328 4.27132C3.875 4.21128 3.875 3.9424 3.875 3.40463V3.19971C3.87486 2.86857 3.60652 2.60028 3.27539 2.6001Z"
            fill="white"
          />
        </g>
        <defs>
          <filter
            id="filter0_i_1134_3223"
            x="0"
            y="0"
            width="10"
            height="10"
            filterUnits="userSpaceOnUse"
            color-interpolation-filters="sRGB"
          >
            <feFlood flood-opacity="0" result="BackgroundImageFix" />
            <feBlend
              mode="normal"
              in="SourceGraphic"
              in2="BackgroundImageFix"
              result="shape"
            />
            <feColorMatrix
              in="SourceAlpha"
              type="matrix"
              values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
              result="hardAlpha"
            />
            <feOffset />
            <feGaussianBlur stdDeviation="1" />
            <feComposite in2="hardAlpha" operator="arithmetic" k2="-1" k3="1" />
            <feColorMatrix
              type="matrix"
              values="0 0 0 0 0.942 0 0 0 0 0.94 0 0 0 0 1 0 0 0 0.1 0"
            />
            <feBlend
              mode="hard-light"
              in2="shape"
              result="effect1_innerShadow_1134_3223"
            />
          </filter>
        </defs>
      </svg>
    </button>
    <button
      class="control-btn"
      @click="$emit('toggle')"
      :title="isPlaying ? '暂停' : '播放'"
    >
      <svg
        v-if="!isPlaying"
        viewBox="0 0 10 10"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <g filter="url(#filter0_i_1204_239)">
          <path
            d="M6.25925 3.6287C7.05468 4.22127 7.4524 4.51756 7.48962 4.91637C7.49481 4.972 7.49481 5.028 7.48962 5.08363C7.4524 5.48244 7.05468 5.77873 6.25925 6.3713L5.77501 6.73205C5.01262 7.3 4.63143 7.58398 4.31216 7.57055C4.05461 7.55971 3.81407 7.43895 3.65151 7.23889C3.45001 6.99089 3.45001 6.51554 3.45001 5.56486V5V4.43514C3.45001 3.48446 3.45001 3.00911 3.65151 2.76111C3.81407 2.56105 4.05461 2.44029 4.31216 2.42945C4.63143 2.41602 5.01262 2.7 5.77501 3.26795L6.25925 3.6287Z"
            fill="white"
          />
        </g>
        <defs>
          <filter
            id="filter0_i_1204_239"
            x="0"
            y="0"
            width="10"
            height="10"
            filterUnits="userSpaceOnUse"
            color-interpolation-filters="sRGB"
          >
            <feFlood flood-opacity="0" result="BackgroundImageFix" />
            <feBlend
              mode="normal"
              in="SourceGraphic"
              in2="BackgroundImageFix"
              result="shape"
            />
            <feColorMatrix
              in="SourceAlpha"
              type="matrix"
              values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
              result="hardAlpha"
            />
            <feOffset />
            <feGaussianBlur stdDeviation="1" />
            <feComposite in2="hardAlpha" operator="arithmetic" k2="-1" k3="1" />
            <feColorMatrix
              type="matrix"
              values="0 0 0 0 0.942 0 0 0 0 0.94 0 0 0 0 1 0 0 0 0.1 0"
            />
            <feBlend
              mode="hard-light"
              in2="shape"
              result="effect1_innerShadow_1204_239"
            />
          </filter>
        </defs>
      </svg>
      <svg
        v-else
        viewBox="0 0 10 10"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <g filter="url(#filter0_i_1134_3247)">
          <path
            d="M3.59961 2.2998C3.98621 2.2998 4.2998 2.6134 4.2998 3V7C4.2997 7.38651 3.98614 7.7002 3.59961 7.7002C3.21326 7.69998 2.9005 7.38638 2.90039 7V3C2.90039 2.61353 3.21319 2.30002 3.59961 2.2998ZM6.40039 2.2998C6.78681 2.30002 7.09961 2.61353 7.09961 3V7C7.0995 7.38638 6.78674 7.69998 6.40039 7.7002C6.01386 7.7002 5.7003 7.38651 5.7002 7V3C5.7002 2.6134 6.01379 2.2998 6.40039 2.2998Z"
            fill="white"
          />
        </g>
        <defs>
          <filter
            id="filter0_i_1134_3247"
            x="0"
            y="0"
            width="10"
            height="10"
            filterUnits="userSpaceOnUse"
            color-interpolation-filters="sRGB"
          >
            <feFlood flood-opacity="0" result="BackgroundImageFix" />
            <feBlend
              mode="normal"
              in="SourceGraphic"
              in2="BackgroundImageFix"
              result="shape"
            />
            <feColorMatrix
              in="SourceAlpha"
              type="matrix"
              values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
              result="hardAlpha"
            />
            <feOffset />
            <feGaussianBlur stdDeviation="1" />
            <feComposite in2="hardAlpha" operator="arithmetic" k2="-1" k3="1" />
            <feColorMatrix
              type="matrix"
              values="0 0 0 0 0.942 0 0 0 0 0.94 0 0 0 0 1 0 0 0 0.1 0"
            />
            <feBlend
              mode="hard-light"
              in2="shape"
              result="effect1_innerShadow_1134_3247"
            />
          </filter>
        </defs>
      </svg>
    </button>
    <button class="control-btn" @click="$emit('next')" title="下一首">
      <svg viewBox="0 0 10 10" fill="none" xmlns="http://www.w3.org/2000/svg">
        <g filter="url(#filter0_i_1134_3227)">
          <path
            d="M6.72461 2.6001C7.05589 2.6001 7.32505 2.86846 7.3252 3.19971V6.80029C7.32502 7.13151 7.05587 7.3999 6.72461 7.3999C6.3935 7.39972 6.12518 7.1314 6.125 6.80029V6.59508C6.125 6.05769 6.125 5.789 6.04707 5.72895C6.02202 5.70965 6.00418 5.70101 5.9735 5.69335C5.87806 5.66949 5.65796 5.84362 5.21777 6.19189L4.7002 6.60205C4.07848 7.09394 3.76705 7.34055 3.50684 7.35107C3.24005 7.3617 2.98532 7.23779 2.82812 7.02197C2.6749 6.81143 2.6748 6.41514 2.6748 5.62256V4.37744C2.6748 3.58487 2.67491 3.18857 2.82812 2.97803C2.98532 2.76221 3.24005 2.6383 3.50684 2.64893C3.76705 2.65945 4.07848 2.90606 4.7002 3.39795L5.21777 3.80811C5.65811 4.15649 5.87828 4.33068 5.97393 4.30655C6.00425 4.2989 6.02191 4.29035 6.04672 4.27132C6.125 4.21128 6.125 3.9424 6.125 3.40463V3.19971C6.12514 2.86857 6.39348 2.60028 6.72461 2.6001Z"
            fill="white"
          />
        </g>
        <defs>
          <filter
            id="filter0_i_1134_3227"
            x="0"
            y="0"
            width="10"
            height="10"
            filterUnits="userSpaceOnUse"
            color-interpolation-filters="sRGB"
          >
            <feFlood flood-opacity="0" result="BackgroundImageFix" />
            <feBlend
              mode="normal"
              in="SourceGraphic"
              in2="BackgroundImageFix"
              result="shape"
            />
            <feColorMatrix
              in="SourceAlpha"
              type="matrix"
              values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
              result="hardAlpha"
            />
            <feOffset />
            <feGaussianBlur stdDeviation="1" />
            <feComposite in2="hardAlpha" operator="arithmetic" k2="-1" k3="1" />
            <feColorMatrix
              type="matrix"
              values="0 0 0 0 0.942 0 0 0 0 0.94 0 0 0 0 1 0 0 0 0.1 0"
            />
            <feBlend
              mode="hard-light"
              in2="shape"
              result="effect1_innerShadow_1134_3227"
            />
          </filter>
        </defs>
      </svg>
    </button>
    <button class="control-btn" @click="$emit('showSessions')" title="切换应用">
      <svg
        width="28"
        height="28"
        viewBox="0 0 10 10"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <g filter="url(#filter0_i_1164_176)">
          <path
            d="M6.65039 5.25C6.92622 5.25021 7.15018 5.47417 7.15039 5.75C7.15039 5.84245 7.15039 5.88868 7.16049 5.92662C7.18816 6.0305 7.2693 6.11164 7.37319 6.13931C7.41113 6.14941 7.45735 6.14941 7.5498 6.14941C7.82569 6.14941 8.04938 6.37363 8.0498 6.64941C8.0498 6.92556 7.82595 7.14941 7.5498 7.14941C7.45735 7.14941 7.41113 7.14941 7.37319 7.15952C7.2693 7.18718 7.18816 7.26833 7.16049 7.37221C7.15039 7.41015 7.15039 7.4567 7.15039 7.5498C7.15039 7.82582 6.92635 8.04959 6.65039 8.0498C6.37435 8.04969 6.15039 7.82588 6.15039 7.5498C6.15039 7.45644 6.15039 7.40976 6.14009 7.37147C6.1123 7.26818 6.03162 7.1875 5.92833 7.15972C5.89004 7.14941 5.84336 7.14941 5.75 7.14941C5.47396 7.1493 5.25 6.92549 5.25 6.64941C5.25042 6.3737 5.47422 6.14953 5.75 6.14941C5.8431 6.14941 5.88965 6.14941 5.92759 6.13931C6.03148 6.11164 6.11262 6.0305 6.14029 5.92662C6.15039 5.88868 6.15039 5.84245 6.15039 5.75C6.1506 5.47411 6.37448 5.25012 6.65039 5.25ZM3.2002 5.4502C3.71547 5.4502 3.97334 5.45001 4.15527 5.5791C4.21939 5.62463 4.27578 5.68098 4.32129 5.74512C4.45009 5.92705 4.4502 6.18519 4.4502 6.7002C4.4502 7.21536 4.45037 7.47336 4.32129 7.65527C4.27582 7.71934 4.21932 7.77481 4.15527 7.82031C3.97334 7.9494 3.71547 7.9502 3.2002 7.9502C2.68479 7.9502 2.42708 7.94942 2.24512 7.82031C2.18102 7.77482 2.12461 7.71935 2.0791 7.65527C1.95002 7.47336 1.9502 7.21536 1.9502 6.7002C1.9502 6.18501 1.95016 5.92706 2.0791 5.74512C2.12466 5.68091 2.18091 5.62466 2.24512 5.5791C2.42707 5.45003 2.68484 5.4502 3.2002 5.4502ZM3.2002 1.9502C3.71547 1.9502 3.97334 1.95001 4.15527 2.0791C4.21939 2.12463 4.27578 2.18098 4.32129 2.24512C4.45009 2.42705 4.4502 2.68519 4.4502 3.2002C4.4502 3.71536 4.45037 3.97336 4.32129 4.15527C4.27582 4.21934 4.21932 4.27481 4.15527 4.32031C3.97334 4.4494 3.71547 4.4502 3.2002 4.4502C2.68479 4.4502 2.42708 4.44942 2.24512 4.32031C2.18102 4.27482 2.12461 4.21935 2.0791 4.15527C1.95002 3.97336 1.9502 3.71536 1.9502 3.2002C1.9502 2.68501 1.95016 2.42706 2.0791 2.24512C2.12466 2.18091 2.18091 2.12466 2.24512 2.0791C2.42707 1.95003 2.68483 1.9502 3.2002 1.9502ZM6.7002 1.9502C7.21547 1.9502 7.47334 1.95001 7.65527 2.0791C7.71939 2.12463 7.77578 2.18098 7.82129 2.24512C7.9501 2.42705 7.9502 2.68519 7.9502 3.2002C7.9502 3.71536 7.95037 3.97336 7.82129 4.15527C7.77582 4.21934 7.71932 4.27481 7.65527 4.32031C7.47334 4.4494 7.21547 4.4502 6.7002 4.4502C6.18479 4.4502 5.92708 4.44942 5.74512 4.32031C5.68102 4.27482 5.62461 4.21935 5.5791 4.15527C5.45002 3.97336 5.4502 3.71536 5.4502 3.2002C5.4502 2.68502 5.45017 2.42706 5.5791 2.24512C5.62466 2.18091 5.68091 2.12466 5.74512 2.0791C5.92707 1.95003 6.18483 1.9502 6.7002 1.9502Z"
            fill="white"
          />
        </g>
        <defs>
          <filter
            id="filter0_i_1164_176"
            x="0"
            y="0"
            width="10"
            height="10"
            filterUnits="userSpaceOnUse"
            color-interpolation-filters="sRGB"
          >
            <feFlood flood-opacity="0" result="BackgroundImageFix" />
            <feBlend
              mode="normal"
              in="SourceGraphic"
              in2="BackgroundImageFix"
              result="shape"
            />
            <feColorMatrix
              in="SourceAlpha"
              type="matrix"
              values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
              result="hardAlpha"
            />
            <feOffset />
            <feGaussianBlur stdDeviation="1" />
            <feComposite in2="hardAlpha" operator="arithmetic" k2="-1" k3="1" />
            <feColorMatrix
              type="matrix"
              values="0 0 0 0 0.942 0 0 0 0 0.94 0 0 0 0 1 0 0 0 0.1 0"
            />
            <feBlend
              mode="hard-light"
              in2="shape"
              result="effect1_innerShadow_1164_176"
            />
          </filter>
        </defs>
      </svg>
    </button>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  isPlaying: boolean;
}>();

defineEmits<{
  toggle: [];
  prev: [];
  next: [];
  showSessions: [];
}>();
</script>

<style scoped>
.controls-container {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
}

.control-btn {
  background: transparent;
  border: 0;
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  color: white;
  transition: all 0.2s ease;
}

.control-btn:hover {
  transform: scale(1.1);
}

.control-btn:active {
  transform: scale(0.95);
}
</style>
