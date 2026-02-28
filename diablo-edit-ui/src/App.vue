<script setup lang="ts">
import { useCharacterStore } from "./stores/characterStore";
import { useProfileStore } from "./stores/profileStore";

const store = useCharacterStore();
const profileStore = useProfileStore();

const handleSave = async () => {
  const success = await store.saveFile();
  if (success) {
    // Save the file-profile association
    if (store.filePath) {
      profileStore.setProfileForFile(store.filePath);
    }
    alert("File saved successfully!");
  } else if (store.error) {
    alert("Save failed: " + store.error);
  }
};
</script>

<template>
  <div class="app-layout">
    <nav class="navbar" v-if="store.saveData">
      <router-link to="/">Home</router-link>
      <router-link to="/stats">Stats</router-link>
      <router-link to="/skills">Skills</router-link>
      <router-link to="/items">Items</router-link>
      <router-link to="/quests">Quests</router-link>
      <router-link to="/waypoints">Waypoints</router-link>
      <div class="spacer"></div>
      <div class="char-summary">
        {{ store.charName }} ({{ store.charClass }})
        <span v-if="store.isModified" class="modified-indicator">*</span>
      </div>
      <button
        class="save-btn"
        @click="handleSave"
        :disabled="!store.isModified"
        :title="store.isModified ? 'Save changes' : 'No changes to save'"
      >
        Save
      </button>
    </nav>
    <main class="content">
      <router-view />
    </main>
  </div>
</template>

<style>
/* Diablo 2 Inspired Theme */
:root {
  /* Color Palette - Dark Gothic */
  --bg-dark: #0a0a08;
  --bg-panel: #141410;
  --bg-card: #1c1c16;
  --bg-cell: #0f0f0c;

  /* Gold Accent Colors */
  --gold-primary: #c7a468;
  --gold-bright: #e8c776;
  --gold-dim: #8a7344;

  /* Text Colors */
  --text-primary: #d4c4a8;
  --text-secondary: #8a8070;
  --text-dim: #5a5448;

  /* Border Colors */
  --border-dark: #2a2a20;
  --border-light: #3a3a30;
  --border-gold: #4a4030;

  /* Item Quality Colors (D2 Standard) */
  --quality-normal: #c7c7c7;
  --quality-magic: #6888ff;
  --quality-rare: #e8c776;
  --quality-set: #0aff0a;
  --quality-unique: #c7a468;
  --quality-crafted: #fc8804;
  --quality-ethereal: #8a8aff;

  /* UI Colors */
  --accent-blue: #4a6a9a;
  --accent-red: #8a3030;
  --accent-green: #308030;

  /* Font Settings */
  font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
  font-size: 14px;
  line-height: 1.5;
  font-weight: 400;

  color: var(--text-primary);
  background-color: var(--bg-dark);

  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  min-width: 320px;
  min-height: 100vh;
  background: var(--bg-dark);
}

#app {
  width: 100%;
  margin: 0 auto;
}

.app-layout {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

/* Navbar Styling */
.navbar {
  display: flex;
  gap: 0;
  padding: 0;
  background: linear-gradient(180deg, #1e1e16 0%, #141410 100%);
  border-bottom: 2px solid var(--border-gold);
  align-items: stretch;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.5);
}

.navbar a {
  color: var(--text-dim);
  text-decoration: none;
  font-weight: 500;
  padding: 12px 20px;
  text-transform: uppercase;
  font-size: 12px;
  letter-spacing: 1px;
  border-right: 1px solid var(--border-dark);
  transition: all 0.2s ease;
  position: relative;
}

.navbar a:hover {
  color: var(--gold-primary);
  background: rgba(199, 164, 104, 0.1);
}

.navbar a.router-link-active {
  color: var(--gold-bright);
  background: linear-gradient(180deg, rgba(199, 164, 104, 0.15) 0%, rgba(199, 164, 104, 0.05) 100%);
  box-shadow: inset 0 -2px 0 var(--gold-primary);
}

.spacer {
  flex: 1;
}

.char-summary {
  font-weight: bold;
  color: var(--gold-primary);
  padding: 12px 20px;
  display: flex;
  align-items: center;
  font-size: 13px;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
}

.modified-indicator {
  color: var(--accent-red);
  margin-left: 4px;
  font-size: 16px;
  animation: pulse 1.5s infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.save-btn {
  background: linear-gradient(180deg, #3a3a30 0%, #2a2a20 100%);
  border: 1px solid var(--border-gold);
  color: var(--text-primary);
  padding: 8px 16px;
  margin: 6px 10px;
  cursor: pointer;
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 1px;
  transition: all 0.2s ease;
}

.save-btn:hover:not(:disabled) {
  background: linear-gradient(180deg, #4a4a40 0%, #3a3a30 100%);
  color: var(--gold-bright);
  border-color: var(--gold-primary);
}

.save-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Content Area */
.content {
  padding: 20px;
  flex: 1;
  background: var(--bg-dark);
}

/* Headings */
h1, h2, h3 {
  color: var(--gold-primary);
  text-transform: uppercase;
  letter-spacing: 2px;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
}

h2 {
  font-size: 1.4rem;
  margin-bottom: 1rem;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--border-gold);
}

h3 {
  font-size: 1rem;
  color: var(--text-primary);
}

/* View Container */
.view-container {
  max-width: 1200px;
  margin: 0 auto;
}

/* Button Styling */
button {
  border-radius: 4px;
  border: 1px solid var(--border-light);
  padding: 10px 24px;
  font-size: 14px;
  font-weight: 600;
  font-family: inherit;
  text-transform: uppercase;
  letter-spacing: 1px;
  background: linear-gradient(180deg, #2a2a20 0%, #1e1e16 100%);
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

button:hover {
  background: linear-gradient(180deg, #3a3a28 0%, #2a2a1e 100%);
  border-color: var(--gold-dim);
  color: var(--gold-bright);
}

button:active {
  transform: translateY(1px);
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
}

button.primary {
  background: linear-gradient(180deg, var(--gold-dim) 0%, #6a5434 100%);
  border-color: var(--gold-primary);
  color: #fff;
}

button.primary:hover {
  background: linear-gradient(180deg, var(--gold-primary) 0%, var(--gold-dim) 100%);
}

/* Input Styling */
input, select {
  background: var(--bg-cell);
  border: 1px solid var(--border-dark);
  color: var(--text-primary);
  padding: 8px 12px;
  border-radius: 4px;
  font-family: inherit;
  font-size: 14px;
}

input:focus, select:focus {
  outline: none;
  border-color: var(--gold-dim);
  box-shadow: 0 0 4px rgba(199, 164, 104, 0.3);
}

/* Panel Styling */
.panel {
  background: var(--bg-panel);
  border: 1px solid var(--border-dark);
  border-radius: 4px;
  padding: 16px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

/* Scrollbar Styling */
::-webkit-scrollbar {
  width: 12px;
  height: 12px;
}

::-webkit-scrollbar-track {
  background: var(--bg-dark);
}

::-webkit-scrollbar-thumb {
  background: var(--border-light);
  border-radius: 2px;
  border: 2px solid var(--bg-dark);
}

::-webkit-scrollbar-thumb:hover {
  background: var(--gold-dim);
}

/* Selection */
::selection {
  background: rgba(199, 164, 104, 0.3);
  color: var(--gold-bright);
}

/* Error Messages */
.error {
  color: #ff6666;
  background: rgba(138, 48, 48, 0.2);
  padding: 12px;
  border-radius: 4px;
  border: 1px solid #8a3030;
  margin-top: 1rem;
}

/* Success Messages */
.success {
  color: #66ff66;
  background: rgba(48, 128, 48, 0.2);
  padding: 12px;
  border-radius: 4px;
  border: 1px solid #308030;
  margin-top: 1rem;
}

/* Responsive */
@media (max-width: 768px) {
  .navbar {
    flex-wrap: wrap;
  }

  .navbar a {
    padding: 10px 14px;
    font-size: 11px;
  }

  .char-summary {
    width: 100%;
    justify-content: center;
    border-top: 1px solid var(--border-dark);
  }
}
</style>
