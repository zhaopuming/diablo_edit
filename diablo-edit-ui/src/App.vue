<script setup lang="ts">
import { useCharacterStore } from "./stores/characterStore";

const store = useCharacterStore();
</script>

<template>
  <div class="app-layout">
    <nav class="navbar" v-if="store.saveData">
      <router-link to="/">Home</router-link>
      <router-link to="/stats">Stats</router-link>
      <router-link to="/skills">Skills</router-link>
      <router-link to="/items">Items</router-link>
      <div class="spacer"></div>
      <div class="char-summary">
        {{ store.charName }} ({{ store.charClass }})
      </div>
    </nav>
    <main class="content">
      <router-view />
    </main>
  </div>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color-scheme: light dark;
  color: rgba(255, 255, 255, 0.87);
  background-color: #242424;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

body {
  margin: 0;
  display: flex;
  place-items: center;
  min-width: 320px;
  min-height: 100vh;
}

#app {
  width: 100%;
  margin: 0 auto;
}

.navbar {
  display: flex;
  gap: 1rem;
  padding: 1rem;
  background: #1a1a1a;
  border-bottom: 1px solid #333;
  align-items: center;
}

.navbar a {
  color: #888;
  text-decoration: none;
  font-weight: 500;
}

.navbar a.router-link-active {
  color: #646cff;
}

.spacer {
  flex: 1;
}

.char-summary {
  font-weight: bold;
  color: #ccc;
}

.content {
  padding: 1rem;
}

button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  background-color: #1a1a1a;
  cursor: pointer;
  transition: border-color 0.25s;
}
button:hover {
  border-color: #646cff;
}
button:focus,
button:focus-visible {
  outline: 4px auto -webkit-focus-ring-color;
}

@media (prefers-color-scheme: light) {
  :root {
    color: #213547;
    background-color: #ffffff;
  }
  .navbar {
    background-color: #f9f9f9;
    border-bottom: 1px solid #eee;
  }
  button {
    background-color: #f9f9f9;
  }
}
</style>