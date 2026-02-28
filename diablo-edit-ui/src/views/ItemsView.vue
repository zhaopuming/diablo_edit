<script setup lang="ts">
import { computed } from "vue";
import { useCharacterStore, CONTAINER } from "../stores/characterStore";
import { useProfileStore } from "../stores/profileStore";
import InventoryGrid from "../components/InventoryGrid.vue";
import ProfileSelector from "../components/ProfileSelector.vue";

const store = useCharacterStore();
const profileStore = useProfileStore();

const inventoryItems = computed(() => {
  if (!store.saveData) return [];
  // location 0 = grid, container 1 = inventory
  return store.saveData.items.items.filter(item =>
    item.data.location === 0 && item.data.container === CONTAINER.INVENTORY
  );
});

const stashItems = computed(() => {
  if (!store.saveData) return [];
  // location 0 = grid, container 5 = stash
  return store.saveData.items.items.filter(item =>
    item.data.location === 0 && item.data.container === CONTAINER.STASH
  );
});

const cubeItems = computed(() => {
  if (!store.saveData) return [];
  // location 0 = grid, container 4 = cube
  return store.saveData.items.items.filter(item =>
    item.data.location === 0 && item.data.container === CONTAINER.CUBE
  );
});

// Item counts
const totalItems = computed(() => {
  if (!store.saveData) return 0;
  return store.saveData.items.items.length;
});

// Grid dimensions from profile
const inventoryGrid = computed(() => profileStore.getGridDimensions(CONTAINER.INVENTORY));
const stashGrid = computed(() => profileStore.getGridDimensions(CONTAINER.STASH));
const cubeGrid = computed(() => profileStore.getGridDimensions(CONTAINER.CUBE));
</script>

<template>
  <div class="view-container">
    <div class="view-header">
      <h2>Items</h2>
      <ProfileSelector />
    </div>

    <div v-if="store.saveData" class="items-content">
      <!-- Stats Bar -->
      <div class="items-stats">
        <span>Total Items: {{ totalItems }}</span>
        <span class="hint">Drag items to move between grids</span>
      </div>

      <!-- Main Layout -->
      <div class="grids-layout">
        <!-- Left Column: Inventory + Cube -->
        <div class="left-column">
          <InventoryGrid
            title="Inventory"
            :items="inventoryItems"
            :rows="inventoryGrid.rows"
            :cols="inventoryGrid.cols"
            :container-id="CONTAINER.INVENTORY"
          />

          <InventoryGrid
            title="Horadric Cube"
            :items="cubeItems"
            :rows="cubeGrid.rows"
            :cols="cubeGrid.cols"
            :container-id="CONTAINER.CUBE"
          />
        </div>

        <!-- Right Column: Stash -->
        <div class="right-column">
          <InventoryGrid
            title="Stash"
            :items="stashItems"
            :rows="stashGrid.rows"
            :cols="stashGrid.cols"
            :container-id="CONTAINER.STASH"
          />
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

.view-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.view-header h2 {
  margin: 0;
}

.items-content {
  max-width: 1400px;
  margin: 0 auto;
}

.items-stats {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 1rem;
  margin-bottom: 1rem;
  background: #1a1a1a;
  border-radius: 6px;
  font-size: 0.85rem;
  color: #888;
}

.items-stats .hint {
  font-style: italic;
}

.grids-layout {
  display: flex;
  gap: 20px;
  justify-content: center;
  flex-wrap: wrap;
}

.left-column {
  display: flex;
  flex-direction: column;
  gap: 10px;
  align-items: flex-start;
}

.right-column {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
}

/* Responsive layout */
@media (max-width: 900px) {
  .grids-layout {
    flex-direction: column;
    align-items: center;
  }
}
</style>
