<script setup lang="ts">
import { ref, computed, onUnmounted } from 'vue';
import type { D2Item } from '../bindings/D2Item';
import ItemTooltip from './ItemTooltip.vue';
import { useCharacterStore } from '../stores/characterStore';

const props = defineProps<{
  items: D2Item[];
  rows: number;
  cols: number;
  title: string;
  containerId: number; // 1=inventory, 4=cube, 5=stash
}>();

// Note: itemDropped event available for future external handling
defineEmits<{
  itemDropped: [item: D2Item, container: number, col: number, row: number]
}>();

const store = useCharacterStore();

// Tooltip state
const hoveredItem = ref<D2Item | null>(null);
const tooltipX = ref(0);
const tooltipY = ref(0);

// Local drag preview state (current hover position in this grid)
const dragCurrentCol = ref(-1);
const dragCurrentRow = ref(-1);
const isHovering = ref(false); // Track if this grid is being hovered

// Track if drag is actually in progress (set on first drag event after dragstart)
const isDragInProgress = ref(false);

// Use global drag state from store
const draggedItem = computed(() => store.draggedItem);

// Current grid dimensions for passing to store functions
const gridDimensions = computed(() => ({ cols: props.cols, rows: props.rows }));

// Preview position during drag - only show for the grid being hovered
const previewPosition = computed(() => {
  if (!draggedItem.value || dragCurrentCol.value < 0 || !isHovering.value) return null;
  return {
    col: dragCurrentCol.value,
    row: dragCurrentRow.value,
    valid: store.canPlaceItem(
      draggedItem.value,
      props.containerId,
      dragCurrentCol.value,
      dragCurrentRow.value,
      store.findItemIndex(draggedItem.value),
      gridDimensions.value
    )
  };
});

// Calculate grid cells occupied during preview
const previewCells = computed(() => {
  if (!draggedItem.value || !previewPosition.value) return [];
  const cells: { col: number; row: number }[] = [];
  for (let r = 0; r < draggedItem.value.data.height; r++) {
    for (let c = 0; c < draggedItem.value.data.width; c++) {
      cells.push({
        col: previewPosition.value.col + c,
        row: previewPosition.value.row + r
      });
    }
  }
  return cells;
});

const showTooltip = (item: D2Item, event: MouseEvent) => {
  if (draggedItem.value) return; // Don't show tooltip while dragging
  hoveredItem.value = item;
  tooltipX.value = event.clientX;
  tooltipY.value = event.clientY;
};

const hideTooltip = () => {
  hoveredItem.value = null;
};

const getItemStyle = (item: D2Item) => {
  return {
    left: `${item.data.column * 40}px`,
    top: `${item.data.row * 40}px`,
    width: `${item.data.width * 40}px`,
    height: `${item.data.height * 40}px`,
  };
};

// Get display name for item
const getItemName = (item: D2Item) => {
  if (item.data.name) {
    return item.data.name;
  }
  // Fallback to type_id
  return String.fromCharCode(...item.data.type_id.filter(c => c !== 0));
};

// Get item quality class
const getItemClass = (item: D2Item) => {
  const classes = ['item-icon'];
  if (!item.data.identified) classes.push('unidentified');
  if (item.data.ethereal) classes.push('ethereal');
  if (item.data.runeword) classes.push('runeword');
  if (item.data.socketed && item.socketed_items.length > 0) classes.push('socketed');
  // Only apply .dragging class after drag is actually in progress (not on dragstart)
  if (item === draggedItem.value && isDragInProgress.value) classes.push('dragging');
  return classes.join(' ');
};

// Check if a cell is part of the preview
const isPreviewCell = (col: number, row: number) => {
  return previewCells.value.some(cell => cell.col === col && cell.row === row);
};

// Drag and drop handlers
const handleDragStart = (item: D2Item, event: DragEvent) => {
  if (!event.dataTransfer) return;

  // Set drag data
  event.dataTransfer.effectAllowed = 'move';
  event.dataTransfer.setData('text/plain', 'd2item'); // Simple data

  // Create a drag image from the element
  const target = event.target as HTMLElement;
  const rect = target.getBoundingClientRect();
  event.dataTransfer.setDragImage(target, rect.width / 2, rect.height / 2);

  // Use store's drag state
  store.startDrag(item, props.containerId);

  console.log('Drag started:', item.data.name, 'from container:', props.containerId);
};

const handleDragEnd = () => {
  store.endDrag();
  dragCurrentCol.value = -1;
  dragCurrentRow.value = -1;
  isDragInProgress.value = false;
  isHovering.value = false;
};

// Called on first drag event after dragstart to confirm drag is actually happening
const handleDrag = () => {
  if (!isDragInProgress.value && draggedItem.value) {
    isDragInProgress.value = true;
    console.log('Drag confirmed in progress');
  }
};

const handleDragEnter = (event: DragEvent) => {
  // .prevent modifier handles preventDefault
  event.stopPropagation();
  isHovering.value = true;
  console.log('Drag enter container:', props.containerId);
};

const handleDragLeave = (event: DragEvent) => {
  // Only reset if we're leaving the grid entirely
  const grid = event.currentTarget as HTMLElement;
  const rect = grid.getBoundingClientRect();
  const x = event.clientX;
  const y = event.clientY;

  if (x < rect.left || x >= rect.right || y < rect.top || y >= rect.bottom) {
    dragCurrentCol.value = -1;
    dragCurrentRow.value = -1;
    isHovering.value = false;
  }
};

const calculateGridPosition = (event: DragEvent): { col: number; row: number } | null => {
  const grid = event.currentTarget as HTMLElement;
  const rect = grid.getBoundingClientRect();

  // Fixed cell size as defined in CSS (40px × 40px)
  const cellSize = 40;

  // Grid border width (1px on each side)
  const gridBorder = 1;

  // Calculate position relative to grid content area (inside border)
  const x = event.clientX - rect.left - gridBorder;
  const y = event.clientY - rect.top - gridBorder;

  // Calculate column and row using fixed cell size
  const col = Math.floor(x / cellSize);
  const row = Math.floor(y / cellSize);

  return { col, row };
};

const handleDrop = (event: DragEvent) => {
  event.preventDefault();
  event.stopPropagation();

  console.log('Drop event on container:', props.containerId);

  const item = store.getDraggedItem();
  if (!item) {
    console.warn('No item being dragged');
    handleDragEnd();
    return;
  }

  const pos = calculateGridPosition(event);
  if (!pos) {
    console.warn('Could not calculate position');
    handleDragEnd();
    return;
  }

  // Adjust drop position to account for cursor offset
  // This assumes the user grabbed the item somewhere on the item
  // We snap to the closest valid grid position
  const targetCol = Math.max(0, Math.min(pos.col, props.cols - item.data.width));
  const targetRow = Math.max(0, Math.min(pos.row, props.rows - item.data.height));

  console.log('Dropping at position:', targetCol, targetRow);

  // Check if position changed
  if (store.dragStartContainer === props.containerId &&
      store.dragStartCol === targetCol &&
      store.dragStartRow === targetRow) {
    // Item dropped in same position
    console.log('Same position, ignoring');
    handleDragEnd();
    return;
  }

  // Try to move the item
  const itemIndex = store.findItemIndex(item);
  console.log('Moving item at index:', itemIndex, 'to container:', props.containerId);
  if (itemIndex >= 0) {
    const success = store.moveItem(itemIndex, props.containerId, targetCol, targetRow, gridDimensions.value);
    if (!success) {
      console.warn('Failed to move item - invalid position');
    } else {
      console.log('Item moved successfully');
    }
  }

  handleDragEnd();
};

const handleGridDragOver = (event: DragEvent) => {
  // Must call preventDefault to allow drop
  event.preventDefault();
  event.stopPropagation();

  const item = store.getDraggedItem();
  if (!item) {
    console.log('No dragged item in store');
    return;
  }

  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = 'move';
  }

  const pos = calculateGridPosition(event);
  if (!pos) return;

  const targetCol = Math.max(0, Math.min(pos.col, props.cols - item.data.width));
  const targetRow = Math.max(0, Math.min(pos.row, props.rows - item.data.height));

  if (targetCol !== dragCurrentCol.value || targetRow !== dragCurrentRow.value) {
    dragCurrentCol.value = targetCol;
    dragCurrentRow.value = targetRow;
    console.log('Drag over position:', targetCol, targetRow);
  }
};

// Clean up on unmount
onUnmounted(() => {
  // Clear local preview state
  dragCurrentCol.value = -1;
  dragCurrentRow.value = -1;
  isDragInProgress.value = false;
  isHovering.value = false;
});
</script>

<template>
  <div class="inventory-container">
    <h3>{{ title }}</h3>
    <div
      class="grid"
      :class="{ 'drag-active': !!draggedItem }"
      :style="{
        width: `${cols * 40}px`,
        height: `${rows * 40}px`
      }"
      @dragover.prevent="handleGridDragOver"
      @dragenter.prevent="handleDragEnter"
      @dragleave="handleDragLeave"
      @drop.prevent="handleDrop"
    >
      <!-- Background Grid Cells -->
      <div
        class="grid-background"
        :style="{
          gridTemplateColumns: `repeat(${cols}, 40px)`,
          gridTemplateRows: `repeat(${rows}, 40px)`
        }"
      >
        <div
          v-for="rowIdx in rows"
          :key="`bg-row-${rowIdx}`"
          class="grid-row"
        >
          <div
            v-for="colIdx in cols"
            :key="`bg-${rowIdx}-${colIdx}`"
            class="grid-cell-bg"
            :class="{
              'preview-valid': previewPosition?.valid && isPreviewCell(colIdx - 1, rowIdx - 1),
              'preview-invalid': previewPosition && !previewPosition.valid && isPreviewCell(colIdx - 1, rowIdx - 1)
            }"
          ></div>
        </div>
      </div>

      <!-- Items Layer -->
      <div class="items-layer">
        <div
          v-for="(item, index) in items"
          :key="index"
          :class="getItemClass(item)"
          :style="getItemStyle(item)"
          draggable="true"
          @dragstart="handleDragStart(item, $event)"
          @drag="handleDrag"
          @dragend="handleDragEnd"
          @mouseenter="showTooltip(item, $event)"
          @mousemove="showTooltip(item, $event)"
          @mouseleave="hideTooltip"
        >
          <div class="item-inner">
            {{ getItemName(item) }}
          </div>
        </div>
      </div>
    </div>

    <!-- Tooltip -->
    <ItemTooltip
      v-if="hoveredItem"
      :item="hoveredItem"
      :visible="!!hoveredItem"
      :x="tooltipX"
      :y="tooltipY"
    />
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
  position: relative;
  background: #000;
  border: 1px solid #444;
  box-sizing: content-box;
}

.grid.drag-active {
  border-color: #646cff;
}

.grid-background {
  display: grid;
  gap: 0;
}

.grid-row {
  display: contents; /* Make rows transparent for grid layout */
}

.grid-cell-bg {
  width: 40px;
  height: 40px;
  border: 1px solid #222;
  box-sizing: border-box;
  transition: background-color 0.15s;
  pointer-events: none;
}

.grid-cell-bg.preview-valid {
  background: rgba(76, 175, 80, 0.3);
  border-color: #4caf50;
}

.grid-cell-bg.preview-invalid {
  background: rgba(244, 67, 54, 0.3);
  border-color: #f44336;
}

.items-layer {
  position: absolute;
  top: 0;
  left: 0;
  pointer-events: none; /* Allow drag events to pass through to grid */
}

.item-icon {
  position: absolute;
  border: 1px solid #555;
  background: rgba(100, 100, 255, 0.2);
  display: flex;
  align-items: center;
  justify-content: center;
  box-sizing: border-box;
  z-index: 10;
  cursor: grab;
  overflow: hidden;
  transition: opacity 0.15s, transform 0.15s, background 0.15s, border-color 0.15s;
  pointer-events: auto; /* Re-enable pointer events for items */
}

.item-icon:hover:not(.dragging) {
  background: rgba(100, 100, 255, 0.4);
  border-color: #aaa;
}

.item-icon.dragging {
  opacity: 0.4;
  cursor: grabbing;
  transform: scale(0.95);
  pointer-events: none;
}

.item-icon:active {
  cursor: grabbing;
}

/* Item quality variations */
.item-icon.unidentified {
  background: rgba(136, 136, 136, 0.3);
}

.item-icon.ethereal {
  border-color: #87ceeb;
  background: rgba(135, 206, 235, 0.2);
}

.item-icon.runeword {
  border-color: #ffd700;
  background: rgba(255, 215, 0, 0.15);
}

.item-icon.socketed {
  border-color: #4caf50;
}

.item-inner {
  font-size: 9px;
  color: #fff;
  text-align: center;
  word-break: break-all;
  padding: 2px;
  line-height: 1.2;
  pointer-events: none;
}
</style>
