<script setup lang="ts">
import { useCharacterStore } from "../stores/characterStore";
const store = useCharacterStore();
</script>
<template>
  <div class="view-container">
    <h2>Character Stats</h2>
    <div v-if="store.saveData">
      <ul>
        <li v-for="(val, id) in store.saveData.stats.values" :key="id">
          Stat #{{ id }}: {{ val }}
        </li>
      </ul>
    </div>
    <p v-else>No data loaded.</p>
  </div>
</template>
