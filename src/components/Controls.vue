<template>
  <div class="container">
    <button @click="$emit('prev')">Previous</button>
    <button @click="$emit('toggle')">Play</button>
    <button @click="$emit('next')">Next</button>
  </div>
</template>
