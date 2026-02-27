<script setup lang="ts">
import { computed } from 'vue';
import type { D2Item } from '../bindings/D2Item';

const props = defineProps<{
  items: D2Item[];
  rows: number;
  cols: number;
  title: string;
}>();

// Create a flat representation of the grid for rendering
const gridCells = computed(() => {
  const cells = Array.from({ length: props.rows }, () => 
    Array.from({ length: props.cols }, () => null as D2Item | null)
  );

  // Mark cells as occupied by items
  // Note: This is a simple implementation that doesn't handle overlapping items
  // (which shouldn't happen in a valid save)
  props.items.forEach(item => {
    const { row, column, width, height } = item.data;
    if (row < props.rows && column < props.cols) {
      cells[row][column] = item;
    }
  });

  return cells;
});

const getCellClass = (row: number, col: number) => {
  const item = gridCells.value[row][col];
  if (!item) return 'cell empty';
  return 'cell occupied';
};

const getItemStyle = (item: D2Item) => {
  return {
    gridColumn: `${item.data.column + 1} / span ${item.data.width}`,
    gridRow: `${item.data.row + 1} / span ${item.data.height}`,
  };
};

// Convert byte array to string for display (e.g. "buc ")
const getTypeName = (typeId: number[]) => {
  return String.fromCharCode(...typeId.filter(c => c !== 0));
};
</script>

<template>
  <div class="inventory-container">
    <h3>{{ title }}</h3>
    <div 
      class="grid" 
      :style="{ 
        gridTemplateColumns: `repeat(${cols}, 40px)`, 
        gridTemplateRows: `repeat(${rows}, 40px)` 
      }"
    >
      <!-- Background Grid -->
      <div 
        v-for="r in rows" :key="`r-${r}`"
        class="grid-row"
      >
        <div 
          v-for="c in cols" :key="`c-${c}`"
          class="grid-cell-bg"
        ></div>
      </div>

      <!-- Items Layer -->
      <div 
        v-for="(item, index) in items" 
        :key="index"
        class="item-icon"
        :style="getItemStyle(item)"
        :title="getTypeName(item.data.type_id)"
      >
        <div class="item-inner">
           {{ getTypeName(item.data.type_id) }}
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.inventory-container {
  margin: 20px;
  background: #1a1a1a;
  padding: 15px;
  border-radius: 8px;
  border: 1px solid #333;
}

h3 {
  margin-top: 0;
  color: #ccc;
  font-size: 1.1rem;
  margin-bottom: 10px;
}

.grid {
  display: grid;
  position: relative;
  background: #000;
  border: 1px solid #444;
  width: fit-content;
}

.grid-row {
  display: contents;
}

.grid-cell-bg {
  width: 40px;
  height: 40px;
  border: 1px solid #222;
  box-sizing: border-box;
}

.item-icon {
  position: relative;
  border: 1px solid #555;
  background: rgba(100, 100, 255, 0.2);
  display: flex;
  align-items: center;
  justify-content: center;
  box-sizing: border-box;
  z-index: 10;
  cursor: pointer;
  overflow: hidden;
}

.item-icon:hover {
  background: rgba(100, 100, 255, 0.4);
  border-color: #aaa;
}

.item-inner {
  font-size: 10px;
  color: #fff;
  text-align: center;
  word-break: break-all;
  padding: 2px;
}
</style>
