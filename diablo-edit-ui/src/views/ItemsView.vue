<script setup lang="ts">
import { computed } from "vue";
import { useCharacterStore } from "../stores/characterStore";
import InventoryGrid from "../components/InventoryGrid.vue";

const store = useCharacterStore();

const inventoryItems = computed(() => {
  if (!store.saveData) return [];
  // location 0 = grid, container 1 = inventory
  return store.saveData.items.items.filter(item => 
    item.data.location === 0 && item.data.container === 1
  );
});

const stashItems = computed(() => {
  if (!store.saveData) return [];
  // location 0 = grid, container 5 = stash
  return store.saveData.items.items.filter(item => 
    item.data.location === 0 && item.data.container === 5
  );
});
</script>

<template>
  <div class="view-container">
    <h2>Items</h2>
    <div v-if="store.saveData">
      <div class="grids-wrapper">
        <InventoryGrid 
          title="Inventory" 
          :items="inventoryItems" 
          :rows="4" 
          :cols="10" 
        />
        <InventoryGrid 
          title="Stash" 
          :items="stashItems" 
          :rows="10" 
          :cols="10" 
        />
      </div>
    </div>
    <p v-else>No data loaded.</p>
  </div>
</template>

<style scoped>
.grids-wrapper {
  display: flex;
  flex-direction: column;
  gap: 20px;
  align-items: center;
}
</style>
