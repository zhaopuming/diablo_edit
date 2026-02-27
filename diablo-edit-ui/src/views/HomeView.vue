<script setup lang="ts">
import { useCharacterStore } from "../stores/characterStore";

const store = useCharacterStore();
</script>

<template>
  <div class="home">
    <h1>Diablo Edit2</h1>
    <p v-if="!store.saveData">Please open a .d2s file to begin.</p>
    <div v-else>
      <h2>Welcome back, {{ store.charName }}!</h2>
      <p>Class: {{ store.charClass }} | Level: {{ store.saveData.header.char_level }}</p>
    </div>
    
    <button @click="store.loadSaveFile" :disabled="store.loading">
      {{ store.loading ? "Loading..." : "Open .d2s File" }}
    </button>
    
    <div v-if="store.error" class="error">{{ store.error }}</div>
  </div>
</template>

<style scoped>
.home {
  padding: 2rem;
  text-align: center;
}
.error {
  color: red;
  margin-top: 1rem;
}
</style>
