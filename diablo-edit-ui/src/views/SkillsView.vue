<script setup lang="ts">
import { useCharacterStore } from "../stores/characterStore";
const store = useCharacterStore();
</script>
<template>
  <div class="view-container">
    <h2>Skills</h2>
    <div v-if="store.saveData">
      <div class="skills-grid">
        <div v-for="(level, idx) in store.saveData.skills.skills" :key="idx" class="skill-item">
          Skill {{ idx }}: {{ level }}
        </div>
      </div>
    </div>
    <p v-else>No data loaded.</p>
  </div>
</template>
<style scoped>
.skills-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
  gap: 0.5rem;
}
.skill-item {
  padding: 0.5rem;
  background: #eee;
  border-radius: 4px;
}
</style>
