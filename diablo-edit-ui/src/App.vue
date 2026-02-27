<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { D2sSave } from "./bindings/D2sSave";

const saveData = ref<D2sSave | null>(null);
const error = ref("");

const CLASS_LABELS: Record<number, string> = {
  0: "Amazon",
  1: "Sorceress",
  2: "Necromancer",
  3: "Paladin",
  4: "Barbarian",
  5: "Druid",
  6: "Assassin"
};

const charName = computed(() => {
  if (!saveData.value) return "";
  const bytes = saveData.value.header.name;
  return new TextDecoder().decode(new Uint8Array(bytes)).replace(/\0/g, "");
});

const charClass = computed(() => {
  if (!saveData.value) return "Unknown";
  return CLASS_LABELS[saveData.value.header.char_class] || "Unknown";
});

async function pickAndOpenFile() {
  try {
    error.value = "";
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'Diablo 2 Save',
        extensions: ['d2s']
      }]
    });

    if (selected) {
      saveData.value = await invoke("open_save_file", { path: selected });
    }
  } catch (e: any) {
    error.value = e.toString();
  }
}
</script>

<template>
  <main class="container">
    <h1>Diablo Edit2 - Tauri Migration</h1>

    <div class="row">
      <button @click="pickAndOpenFile">Open .d2s File</button>
    </div>

    <div v-if="error" class="error">
      {{ error }}
    </div>

    <div v-if="saveData" class="data-view">
      <h3>Character: {{ charName }}</h3>
      <div class="char-info">
        <p><strong>Class:</strong> {{ charClass }}</p>
        <p><strong>Level:</strong> {{ saveData.header.char_level }}</p>
        <p><strong>Items:</strong> {{ saveData.items.items.length }}</p>
      </div>
      
      <h4>Stats</h4>
      <ul>
        <li v-for="(val, id) in saveData.stats.values" :key="id">
          Stat #{{ id }}: {{ val }}
        </li>
      </ul>

      <details>
        <summary>Raw JSON</summary>
        <pre>{{ JSON.stringify(saveData, null, 2) }}</pre>
      </details>
    </div>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.error {
  color: #ff4444;
  margin: 1em 0;
  font-weight: bold;
}

.data-view {
  text-align: left;
  margin-top: 2em;
  padding: 1em;
  background: #eee;
  border-radius: 8px;
  overflow: auto;
  max-width: 90vw;
}

pre {
  font-size: 0.8em;
  color: #333;
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>