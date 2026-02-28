<script setup lang="ts">
import { ref, computed } from "vue";
import { useCharacterStore } from "../stores/characterStore";

const store = useCharacterStore();
const selectedDifficulty = ref(0); // 0=Normal, 1=Nightmare, 2=Hell

const difficulties = ["Normal", "Nightmare", "Hell"];

// Quest names organized by Act
const questStructure = [
  {
    act: "Act I",
    quests: [
      "Den of Evil",
      "Sisters' Burial Grounds",
      "The Search for Cain",
      "The Forgotten Tower",
      "Tools of the Trade",
      "Sisters to the Slaughter",
    ],
  },
  {
    act: "Act II",
    quests: [
      "Radament's Lair",
      "The Horadric Staff",
      "Tainted Sun",
      "Arcane Sanctuary",
      "The Summoner",
      "The Seven Tombs",
    ],
  },
  {
    act: "Act III",
    quests: [
      "The Golden Bird",
      "Blade of the Old Religion",
      "Khalim's Will",
      "Lam Esen's Tome",
      "The Blackened Temple",
      "The Guardian",
    ],
  },
  {
    act: "Act IV",
    quests: [
      "The Fallen Angel",
      "Terror's End",
      "Hellforge",
    ],
  },
  {
    act: "Act V",
    quests: [
      "Siege on Harrogath",
      "Rescue on Mount Arreat",
      "Prison of Ice",
      "Betrayal of Harrogath",
      "Rite of Passage",
      "Eve of Destruction",
    ],
  },
];

// Parse quest completion status from raw byte data
// Based on C++ code: each difficulty uses 96 bytes (2 bytes per quest word)
// Quest completion is checked by (word & 1) != 0
const parseQuestStatus = (data: number[], difficulty: number): boolean[] => {
  if (!data || data.length < 288) return [];

  const status: boolean[] = [];
  const offset = difficulty * 96; // Each difficulty is 96 bytes

  // Helper to read a 16-bit word (little-endian)
  const readWord = (byteOffset: number): number => {
    return data[byteOffset] | (data[byteOffset + 1] << 8);
  };

  // Act I: 6 quests at offsets 0, 2, 4, 6, 8, 10 (words)
  // Note: word at offset 6 has special bits for cow quest
  for (let i = 0; i < 6; i++) {
    const word = readWord(offset + i * 2);
    status.push((word & 1) !== 0);
  }

  // Act II: 6 quests
  const act2Base = 16; // After Act I header
  for (let i = 0; i < 6; i++) {
    const word = readWord(offset + act2Base + i * 2);
    status.push((word & 1) !== 0);
  }

  // Act III: 6 quests
  const act3Base = 32; // After Act II header
  for (let i = 0; i < 6; i++) {
    const word = readWord(offset + act3Base + i * 2);
    status.push((word & 1) !== 0);
  }

  // Act IV: 3 quests
  const act4Base = 48; // After Act III header
  for (let i = 0; i < 3; i++) {
    const word = readWord(offset + act4Base + i * 2);
    status.push((word & 1) !== 0);
  }

  // Act V: 6 quests
  const act5Base = 60; // After Act IV header
  for (let i = 0; i < 6; i++) {
    const word = readWord(offset + act5Base + i * 2);
    status.push((word & 1) !== 0);
  }

  // Reset Stats available (special, from Act I quest completion)
  const resetStatsWord = readWord(offset + 12); // Special location
  status.push((resetStatsWord & 0x400) !== 0);

  return status;
};

const questStatus = computed(() => {
  if (!store.saveData?.quests?.data) return [];
  return parseQuestStatus(store.saveData.quests.data, selectedDifficulty.value);
});

const totalCompleted = computed(() => {
  if (!questStatus.value.length) return 0;
  // Don't count "Reset Stats" in total
  return questStatus.value.slice(0, 27).filter(Boolean).length;
});

const isActComplete = (actIndex: number) => {
  if (!questStatus.value.length) return false;
  const startIndex = [0, 6, 12, 18, 21][actIndex];
  const count = [6, 6, 6, 3, 6][actIndex];
  return questStatus.value.slice(startIndex, startIndex + count).every(Boolean);
};
</script>

<template>
  <div class="view-container">
    <h2>Quests</h2>

    <div v-if="store.saveData" class="quests-content">
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

      <!-- Quest Summary -->
      <div class="quest-summary">
        <span class="completed">{{ totalCompleted }} / 27</span> quests completed
      </div>

      <!-- Acts Grid -->
      <div class="acts-container">
        <div
          v-for="(actData, actIdx) in questStructure"
          :key="actIdx"
          class="act-panel"
          :class="{ complete: isActComplete(actIdx) }"
        >
          <h3 class="act-header">
            {{ actData.act }}
            <span v-if="isActComplete(actIdx)" class="check">✓</span>
          </h3>

          <div class="quest-list">
            <div
              v-for="(quest, questIdx) in actData.quests"
              :key="questIdx"
              class="quest-item"
              :class="{ completed: questStatus[actIdx * 6 + (actIdx > 3 ? actIdx - 3 : 0) + questIdx] }"
            >
              <span class="quest-checkbox">
                {{ questStatus[actIdx * 6 + (actIdx > 3 ? actIdx - 3 : 0) + questIdx] ? '☑' : '☐' }}
              </span>
              <span class="quest-name">{{ quest }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Reset Stats Option -->
      <div v-if="questStatus[27]" class="reset-stats">
        <span class="quest-checkbox">☑</span>
        <span class="quest-name">Reset Stats Available</span>
      </div>
    </div>

    <p v-else>No data loaded.</p>
  </div>
</template>

<style scoped>
.view-container {
  padding: 1rem;
}

.quests-content {
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

.quest-summary {
  margin-bottom: 1rem;
  font-size: 0.9rem;
  color: #888;
}

.quest-summary .completed {
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

.quest-list {
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
}

.quest-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.3rem 0.5rem;
  border-radius: 4px;
  transition: background 0.15s;
}

.quest-item:hover {
  background: #252525;
}

.quest-item.completed .quest-name {
  color: #4caf50;
}

.quest-checkbox {
  font-size: 1rem;
  width: 1.2rem;
}

.quest-name {
  font-size: 0.9rem;
  color: #aaa;
}

.reset-stats {
  margin-top: 1.5rem;
  padding: 0.75rem 1rem;
  background: #2a2a1a;
  border: 1px solid #ffc107;
  border-radius: 8px;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.reset-stats .quest-name {
  color: #ffc107;
}
</style>
