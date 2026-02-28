<script setup lang="ts">
import { useCharacterStore } from "../stores/characterStore";

const store = useCharacterStore();

// Stat ID to name mapping (based on D2 save format)
const statNames: Record<number, string> = {
  0: "Strength",
  1: "Energy",
  2: "Dexterity",
  3: "Vitality",
  4: "Stat Points",
  5: "Skill Points",
  6: "Current Life",
  7: "Max Life",
  8: "Current Mana",
  9: "Max Mana",
  10: "Current Stamina",
  11: "Max Stamina",
  12: "Level",
  13: "Experience",
  14: "Gold (Inventory)",
  15: "Gold (Stash)",
};

// Core stats display order
const coreStats = [0, 1, 2, 3]; // Strength, Energy, Dexterity, Vitality
// Resources: [6, 7, 8, 9, 10, 11] - Life, Mana, Stamina (current & max) - for future expansion
const points = [4, 5]; // Stat Points, Skill Points
// Progression: [12, 13, 14, 15] - Level, Experience, Gold - for future expansion

const getStatValue = (id: number): number => {
  if (!store.saveData?.stats?.values) return 0;
  return store.saveData.stats.values[id] ?? 0;
};

const getStatName = (id: number): string => {
  return statNames[id] ?? `Unknown Stat (${id})`;
};

// Format large numbers with commas
const formatNumber = (val: number): string => {
  return val.toLocaleString();
};

// Future: Add derived stat calculations for life/mana regeneration
</script>

<template>
  <div class="view-container">
    <h2>Character Stats</h2>

    <div v-if="store.saveData" class="stats-content">
      <!-- Core Attributes -->
      <div class="stats-section">
        <h3>Core Attributes</h3>
        <div class="stats-grid core-stats">
          <div
            v-for="statId in coreStats"
            :key="statId"
            class="stat-item core-stat"
          >
            <span class="stat-name">{{ getStatName(statId) }}</span>
            <span class="stat-value">{{ getStatValue(statId) }}</span>
          </div>
        </div>
      </div>

      <!-- Resources (Life, Mana, Stamina) -->
      <div class="stats-section">
        <h3>Resources</h3>
        <div class="resources-grid">
          <div class="resource-item life">
            <span class="resource-name">Life</span>
            <span class="resource-value">
              {{ getStatValue(6) }} / {{ getStatValue(7) }}
            </span>
            <div class="resource-bar">
              <div
                class="resource-fill life-fill"
                :style="{ width: `${(getStatValue(6) / Math.max(getStatValue(7), 1)) * 100}%` }"
              ></div>
            </div>
          </div>

          <div class="resource-item mana">
            <span class="resource-name">Mana</span>
            <span class="resource-value">
              {{ getStatValue(8) }} / {{ getStatValue(9) }}
            </span>
            <div class="resource-bar">
              <div
                class="resource-fill mana-fill"
                :style="{ width: `${(getStatValue(8) / Math.max(getStatValue(9), 1)) * 100}%` }"
              ></div>
            </div>
          </div>

          <div class="resource-item stamina">
            <span class="resource-name">Stamina</span>
            <span class="resource-value">
              {{ getStatValue(10) }} / {{ getStatValue(11) }}
            </span>
            <div class="resource-bar">
              <div
                class="resource-fill stamina-fill"
                :style="{ width: `${(getStatValue(10) / Math.max(getStatValue(11), 1)) * 100}%` }"
              ></div>
            </div>
          </div>
        </div>
      </div>

      <!-- Points -->
      <div class="stats-section">
        <h3>Available Points</h3>
        <div class="stats-grid">
          <div
            v-for="statId in points"
            :key="statId"
            class="stat-item points-stat"
          >
            <span class="stat-name">{{ getStatName(statId) }}</span>
            <span class="stat-value highlight">{{ getStatValue(statId) }}</span>
          </div>
        </div>
      </div>

      <!-- Progression -->
      <div class="stats-section">
        <h3>Progression</h3>
        <div class="stats-grid progression-stats">
          <div class="stat-item">
            <span class="stat-name">Level</span>
            <span class="stat-value level">{{ getStatValue(12) }}</span>
          </div>
          <div class="stat-item wide">
            <span class="stat-name">Experience</span>
            <span class="stat-value">{{ formatNumber(getStatValue(13)) }}</span>
          </div>
          <div class="stat-item gold">
            <span class="stat-name">Gold (Inventory)</span>
            <span class="stat-value gold-value">{{ formatNumber(getStatValue(14)) }}</span>
          </div>
          <div class="stat-item gold">
            <span class="stat-name">Gold (Stash)</span>
            <span class="stat-value gold-value">{{ formatNumber(getStatValue(15)) }}</span>
          </div>
        </div>
      </div>
    </div>

    <p v-else>No data loaded.</p>
  </div>
</template>

<style scoped>
.view-container {
  padding: 1rem;
}

.stats-content {
  max-width: 600px;
  margin: 0 auto;
}

.stats-section {
  margin-bottom: 1.5rem;
}

.stats-section h3 {
  margin: 0 0 0.75rem 0;
  font-size: 0.9rem;
  color: #888;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  border-bottom: 1px solid #333;
  padding-bottom: 0.5rem;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 0.75rem;
}

.stats-grid.core-stats {
  grid-template-columns: repeat(4, 1fr);
}

.stat-item {
  display: flex;
  flex-direction: column;
  background: #1a1a1a;
  border: 1px solid #333;
  border-radius: 6px;
  padding: 0.75rem;
}

.stat-item.wide {
  grid-column: span 2;
}

.stat-name {
  font-size: 0.8rem;
  color: #888;
  margin-bottom: 0.25rem;
}

.stat-value {
  font-size: 1.25rem;
  font-weight: bold;
  color: #fff;
}

.stat-value.highlight {
  color: #646cff;
}

.stat-value.level {
  color: #ffc107;
}

.stat-value.gold-value {
  color: #ffd700;
}

/* Resources */
.resources-grid {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.resource-item {
  background: #1a1a1a;
  border: 1px solid #333;
  border-radius: 6px;
  padding: 0.75rem 1rem;
}

.resource-name {
  font-size: 0.85rem;
  color: #888;
  display: block;
  margin-bottom: 0.25rem;
}

.resource-value {
  font-size: 1rem;
  font-weight: bold;
  color: #fff;
  display: block;
  margin-bottom: 0.5rem;
}

.resource-bar {
  height: 6px;
  background: #333;
  border-radius: 3px;
  overflow: hidden;
}

.resource-fill {
  height: 100%;
  border-radius: 3px;
  transition: width 0.3s ease;
}

.life-fill {
  background: linear-gradient(90deg, #c62828, #ef5350);
}

.mana-fill {
  background: linear-gradient(90deg, #1565c0, #42a5f5);
}

.stamina-fill {
  background: linear-gradient(90deg, #2e7d32, #66bb6a);
}

/* Progression stats */
.progression-stats {
  grid-template-columns: repeat(2, 1fr);
}

/* Responsive */
@media (max-width: 500px) {
  .stats-grid.core-stats {
    grid-template-columns: repeat(2, 1fr);
  }
}
</style>
