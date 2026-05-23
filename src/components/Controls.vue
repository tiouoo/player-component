<template>
  <div class="controls-container">
    <button
      class="control-btn"
      @click="$emit('toggleMute')"
      :title="isMuted ? '取消静音' : '静音'"
    >
      <!-- 音量图标 (未静音) -->
      <svg
        v-if="!isMuted"
        viewBox="0 0 10 10"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <g filter="url(#filter0_i_1079_176)">
          <path
            d="M5.00391 2.60779C5.08137 2.61978 5.15657 2.6463 5.22363 2.68689C5.51262 2.8622 5.5127 3.43688 5.5127 4.58533V5.41541C5.5127 6.56395 5.51214 7.13876 5.22266 7.31384C5.15594 7.35413 5.08189 7.38093 5.00488 7.39294C4.71678 7.43781 4.40248 7.11746 3.84807 6.46503C3.71341 6.30657 3.64608 6.22734 3.55835 6.18205C3.54544 6.17538 3.53249 6.16939 3.51905 6.16387C3.42773 6.12634 3.32511 6.12634 3.11988 6.12634H3.0623C2.63933 6.12634 2.42784 6.12634 2.28758 6.00655C2.26766 5.98953 2.24911 5.97098 2.2321 5.95107C2.1123 5.8108 2.1123 5.59932 2.1123 5.17634V4.82634C2.1123 4.40337 2.1123 4.19188 2.2321 4.05162C2.24911 4.0317 2.26766 4.01315 2.28758 3.99614C2.42784 3.87634 2.63933 3.87634 3.0623 3.87634H3.11938C3.32483 3.87634 3.42756 3.87634 3.51896 3.83874C3.53242 3.83321 3.54537 3.82721 3.5583 3.82053C3.6461 3.77515 3.71343 3.69577 3.8481 3.53702C4.40196 2.88409 4.71593 2.56347 5.00391 2.60779ZM6.61914 3.17908C6.79471 3.00351 7.07957 3.00334 7.25535 3.17857C7.25569 3.1789 7.25597 3.17929 7.25618 3.17972L7.25622 3.17979C7.25662 3.1806 7.25727 3.18125 7.25807 3.18165C7.25855 3.18189 7.25898 3.18221 7.25929 3.18263C7.26003 3.18364 7.26077 3.18494 7.26172 3.18591C7.26433 3.18859 7.26803 3.19199 7.27148 3.19568C7.27851 3.20318 7.28767 3.21333 7.29785 3.22498C7.31844 3.24853 7.34559 3.28114 7.37598 3.32166C7.4371 3.40323 7.51417 3.51992 7.58984 3.67126C7.7423 3.97628 7.8877 4.41988 7.8877 4.99744C7.88763 5.575 7.74233 6.01864 7.58984 6.32361C7.51415 6.47495 7.43709 6.59169 7.37598 6.67322C7.34557 6.71374 7.31842 6.74637 7.29785 6.7699C7.28765 6.78155 7.2785 6.79171 7.27148 6.79919C7.26801 6.80288 7.2643 6.80629 7.26172 6.80896C7.26088 6.80989 7.26019 6.81107 7.2595 6.81202C7.25906 6.81261 7.25845 6.81303 7.2578 6.81336C7.25716 6.81368 7.25658 6.8141 7.25607 6.81461L7.25586 6.81482C7.08017 6.99031 6.79483 6.99128 6.61914 6.8158C6.44347 6.64012 6.44359 6.35483 6.61914 6.17908L6.62012 6.1781C6.62674 6.17054 6.63928 6.15544 6.65527 6.13416C6.68778 6.09081 6.7361 6.01933 6.78516 5.92126C6.88261 5.72635 6.98724 5.4195 6.9873 4.99744C6.9873 4.57539 6.8826 4.26859 6.78516 4.07361C6.73613 3.97557 6.68782 3.90413 6.65527 3.86072C6.63934 3.83951 6.62681 3.82443 6.62012 3.81677L6.61914 3.8158L6.56152 3.74548C6.44606 3.57086 6.46547 3.33293 6.61914 3.17908Z"
            fill="white"
          />
        </g>
        <defs>
          <filter
            id="filter0_i_1079_176"
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
              result="effect1_innerShadow_1079_176"
            />
          </filter>
        </defs>
      </svg>
      <!-- 静音图标 -->
      <svg
        v-else
        viewBox="0 0 10 10"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <g filter="url(#filter0_i_1079_181)">
          <path
            fill-rule="evenodd"
            clip-rule="evenodd"
            d="M4.34207 2.60761C4.41939 2.61963 4.49388 2.64619 4.56082 2.68671C4.84999 2.86188 4.84988 3.43648 4.84988 4.58515V5.41522C4.84988 6.56349 4.8501 7.13843 4.56082 7.31366C4.49402 7.35407 4.41921 7.38072 4.34207 7.39276C4.05401 7.43743 3.73962 7.11714 3.18526 6.46485C3.05059 6.30638 2.98326 6.22715 2.89553 6.18186C2.88261 6.17519 2.86967 6.16921 2.85623 6.16368C2.76491 6.12616 2.6623 6.12616 2.45707 6.12616H2.39949C1.97652 6.12616 1.76503 6.12616 1.62477 6.00636C1.60485 5.98935 1.5863 5.9708 1.56929 5.95089C1.44949 5.81062 1.44949 5.59913 1.44949 5.17616V4.82616C1.44949 4.40319 1.44949 4.1917 1.56929 4.05144C1.5863 4.03152 1.60485 4.01297 1.62477 3.99596C1.76503 3.87616 1.97652 3.87616 2.39949 3.87616H2.45656C2.66201 3.87616 2.76474 3.87616 2.85613 3.83857C2.86958 3.83304 2.88254 3.82704 2.89546 3.82036C2.98325 3.775 3.0506 3.69562 3.18531 3.53685C3.73951 2.88367 4.05401 2.56301 4.34207 2.60761ZM7.76492 3.65839C7.94457 3.47914 8.23571 3.47906 8.41531 3.65839C8.59502 3.83809 8.59502 4.13005 8.41531 4.30976C8.17091 4.55377 8.04872 4.67577 8.01812 4.82165C8.00394 4.88921 8.00392 4.95898 8.01803 5.02655C8.04852 5.17245 8.17078 5.29472 8.41531 5.53925C8.59481 5.71897 8.59495 6.01001 8.41531 6.18964C8.23567 6.36918 7.94461 6.36911 7.76492 6.18964C7.52039 5.94511 7.39813 5.82284 7.25223 5.79236C7.18465 5.77824 7.11489 5.77827 7.04733 5.79244C6.90145 5.82304 6.77944 5.94524 6.53543 6.18964C6.35581 6.36922 6.06475 6.36909 5.88504 6.18964C5.70535 6.00995 5.70538 5.71895 5.88504 5.53925C6.12949 5.29441 6.25171 5.17199 6.28204 5.02592C6.29598 4.95875 6.29595 4.88944 6.28195 4.82228C6.25152 4.67623 6.12936 4.55407 5.88504 4.30976C5.70534 4.13005 5.70534 3.83809 5.88504 3.65839C6.06471 3.47916 6.35585 3.47902 6.53543 3.65839C6.77975 3.90271 6.90191 4.02486 7.04796 4.0553C7.11511 4.0693 7.18443 4.06933 7.25159 4.05538C7.39767 4.02506 7.52009 3.90284 7.76492 3.65839Z"
            fill="white"
          />
        </g>
        <defs>
          <filter
            id="filter0_i_1079_181"
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
              result="effect1_innerShadow_1079_181"
            />
          </filter>
        </defs>
      </svg>
    </button>
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
      <svg viewBox="0 0 10 10" fill="none" xmlns="http://www.w3.org/2000/svg">
        <g filter="url(#filter0_i_1145_2787)">
          <path
            fill-rule="evenodd"
            clip-rule="evenodd"
            d="M4.40076 2.74331C4.40125 2.18468 5.02366 1.85145 5.48865 2.16128L7.13318 3.25796C7.40867 3.44176 7.48277 3.81435 7.29919 4.08999C7.11528 4.36533 6.74273 4.43971 6.46716 4.256L6.06658 3.9885C5.86723 3.85537 5.59998 3.99827 5.59998 4.23798V6.45717C5.59994 7.28557 4.92838 7.95717 4.09998 7.95717H3.80017C3.13752 7.95707 2.59998 7.41965 2.59998 6.75698C2.60005 6.09437 3.13757 5.55689 3.80017 5.55678H4.10076C4.26644 5.55678 4.40076 5.42247 4.40076 5.25678V2.74331Z"
            fill="white"
          />
        </g>
        <defs>
          <filter
            id="filter0_i_1145_2787"
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
              result="effect1_innerShadow_1145_2787"
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
  isMuted: boolean;
}>();

defineEmits<{
  toggle: [];
  prev: [];
  next: [];
  toggleMute: [];
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
