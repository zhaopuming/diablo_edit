<script setup lang="ts">
import { useCharacterStore } from "../stores/characterStore";
const store = useCharacterStore();
</script>
<template>
  <div class="view-container">
    <h2>Items</h2>
    <div v-if="store.saveData">
      <p>Parsed {{ store.saveData.items.items.length }} items.</p>
      <!-- Item grid will go here in Phase 4 -->
    </div>
    <p v-else>No data loaded.</p>
  </div>
</template>
