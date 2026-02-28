<script setup lang="ts">
import { ref, computed } from "vue";
import { useCharacterStore } from "../stores/characterStore";

const store = useCharacterStore();
const selectedDifficulty = ref(0); // 0=Normal, 1=Nightmare, 2=Hell

const difficulties = ["Normal", "Nightmare", "Hell"];

// Waypoint names organized by Act (39 waypoints total)
const waypointStructure = [
  {
    act: "Act I",
    waypoints: [
      "Rogue Encampment",
      "Cold Plains",
      "Stony Field",
      "Dark Wood",
      "Black Marsh",
      "Outer Cloister",
      "Jail Level 1",
      "Inner Cloister",
      "Catacombs Level 2",
    ],
  },
  {
    act: "Act II",
    waypoints: [
      "Lut Gholein",
      "Sewers Level 2",
      "Dry Hills",
      "Halls of the Dead Level 2",
      "Far Oasis",
      "Lost City",
      "Palace Cellar Level 1",
      "Arcane Sanctuary",
      "Canyon of the Magi",
    ],
  },
  {
    act: "Act III",
    waypoints: [
      "Kurast Docktown",
      "Spider Forest",
      "Great Marsh",
      "Flayer Jungle",
      "Lower Kurast",
      "Kurast Bazaar",
      "Upper Kurast",
      "Travincal",
      "Durance of Hate Level 2",
    ],
  },
  {
    act: "Act IV",
    waypoints: [
      "Pandemonium Fortress",
      "City of the Damned",
      "River of Flame",
    ],
  },
  {
    act: "Act V",
    waypoints: [
      "Harrogath",
      "Frigid Highlands",
      "Arreat Plateau",
      "Crystalline Passage",
      "Glacial Trail",
      "Halls of Anguish",
      "Worldstone Keep Level 2",
      "The Ancients",
      "Worldstone Chamber",
    ],
  },
];

// Parse waypoint status from raw byte data
// Waypoints are stored as a bitmask: each byte holds 8 waypoints
const parseWaypointStatus = (waypointData: number[]): boolean[] => {
  if (!waypointData || waypointData.length === 0) return [];

  const status: boolean[] = [];
  let mask = 1;
  let byteIndex = 0;

  // Parse 39 waypoints (indices 0-38)
  for (let i = 0; i < 39; i++) {
    status.push((waypointData[byteIndex] & mask) !== 0);
    if (mask === 0x80) {
      mask = 1;
      byteIndex++;
    } else {
      mask <<= 1;
    }
  }

  return status;
};

const waypointStatus = computed(() => {
  if (!store.saveData?.waypoints?.modes) return [];
  const modeData = store.saveData.waypoints.modes[selectedDifficulty.value];
  return parseWaypointStatus(modeData.waypoints);
});

const totalCompleted = computed(() => {
  if (!waypointStatus.value.length) return 0;
  return waypointStatus.value.filter(Boolean).length;
});

const isActComplete = (actIndex: number) => {
  if (!waypointStatus.value.length) return false;
  const startIndex = [0, 9, 18, 27, 30][actIndex];
  const count = [9, 9, 9, 3, 9][actIndex];
  return waypointStatus.value.slice(startIndex, startIndex + count).every(Boolean);
};

const getWaypointGlobalIndex = (actIndex: number, localIndex: number): number => {
  const offsets = [0, 9, 18, 27, 30];
  return offsets[actIndex] + localIndex;
};
</script>

<template>
  <div class="view-container">
    <h2>Waypoints</h2>

    <div v-if="store.saveData" class="waypoints-content">
      <!-- Difficulty Selector -->
      <div class="difficulty-tabs">
        <button
          v-for="(diff, idx) in difficulties"
          :key="idx"
          :class="['tab-btn', { active: selectedDifficulty === idx }]"
          @click="selectedDifficulty = idx"
        >
          {{ diff }}
        </button>
      </div>

      <!-- Waypoint Summary -->
      <div class="waypoint-summary">
        <span class="completed">{{ totalCompleted }} / 39</span> waypoints unlocked
      </div>

      <!-- Acts Grid -->
      <div class="acts-container">
        <div
          v-for="(actData, actIdx) in waypointStructure"
          :key="actIdx"
          class="act-panel"
          :class="{ complete: isActComplete(actIdx) }"
        >
          <h3 class="act-header">
            {{ actData.act }}
            <span v-if="isActComplete(actIdx)" class="check">✓</span>
          </h3>

          <div class="waypoint-list">
            <div
              v-for="(waypoint, wpIdx) in actData.waypoints"
              :key="wpIdx"
              class="waypoint-item"
              :class="{ unlocked: waypointStatus[getWaypointGlobalIndex(actIdx, wpIdx)] }"
            >
              <span class="waypoint-checkbox">
                {{ waypointStatus[getWaypointGlobalIndex(actIdx, wpIdx)] ? '☑' : '☐' }}
              </span>
              <span class="waypoint-name">{{ waypoint }}</span>
            </div>
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

.waypoints-content {
  max-width: 900px;
  margin: 0 auto;
}

.difficulty-tabs {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1.5rem;
}

.tab-btn {
  padding: 0.5rem 1.5rem;
  border: 1px solid #444;
  background: #1a1a1a;
  color: #888;
  cursor: pointer;
  transition: all 0.2s;
}

.tab-btn:hover {
  border-color: #666;
  color: #ccc;
}

.tab-btn.active {
  background: #2a2a4a;
  border-color: #646cff;
  color: #fff;
}

.waypoint-summary {
  margin-bottom: 1rem;
  font-size: 0.9rem;
  color: #888;
}

.waypoint-summary .completed {
  color: #4caf50;
  font-weight: bold;
}

.acts-container {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1rem;
}

.act-panel {
  background: #1a1a1a;
  border: 1px solid #333;
  border-radius: 8px;
  padding: 1rem;
  transition: border-color 0.2s;
}

.act-panel.complete {
  border-color: #4caf50;
}

.act-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin: 0 0 0.75rem 0;
  font-size: 1rem;
  color: #ccc;
  border-bottom: 1px solid #333;
  padding-bottom: 0.5rem;
}

.act-header .check {
  color: #4caf50;
  font-size: 1.2rem;
}

.waypoint-list {
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
}

.waypoint-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  transition: background 0.15s;
}

.waypoint-item:hover {
  background: #252525;
}

.waypoint-item.unlocked .waypoint-name {
  color: #4caf50;
}

.waypoint-checkbox {
  font-size: 1rem;
  width: 1.2rem;
}

.waypoint-name {
  font-size: 0.85rem;
  color: #aaa;
}
</style>
