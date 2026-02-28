import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { D2sSave } from "../bindings/D2sSave";
import type { D2Item } from "../bindings/D2Item";
import type { GridDimensions } from "../types/profile";
import { useProfileStore } from "./profileStore";

// Container type constants
export const CONTAINER = {
  INVENTORY: 1,
  STASH: 5,
  CUBE: 4,
} as const;

// Default grid dimensions (used as fallback)
export const DEFAULT_GRID_SIZE = {
  INVENTORY: { rows: 4, cols: 10 },
  STASH: { rows: 10, cols: 10 },
  CUBE: { rows: 3, cols: 4 },
} as const;

// Legacy export for backwards compatibility (deprecated - use profileStore instead)
export const GRID_SIZE = DEFAULT_GRID_SIZE;

export const useCharacterStore = defineStore("character", () => {
  const saveData = ref<D2sSave | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const filePath = ref<string | null>(null);
  const isModified = ref(false);

  // Global drag state - shared across all InventoryGrid instances
  const draggedItem = ref<D2Item | null>(null);
  const dragStartContainer = ref<number | null>(null);
  const dragStartCol = ref(0);
  const dragStartRow = ref(0);

  const charName = computed(() => {
    if (!saveData.value) return "";
    const bytes = saveData.value.header.name;
    return new TextDecoder().decode(new Uint8Array(bytes)).replace(/\0/g, "");
  });

  const CLASS_LABELS: Record<number, string> = {
    0: "Amazon",
    1: "Sorceress",
    2: "Necromancer",
    3: "Paladin",
    4: "Barbarian",
    5: "Druid",
    6: "Assassin"
  };

  const charClass = computed(() => {
    if (!saveData.value) return "Unknown";
    return CLASS_LABELS[saveData.value.header.char_class] || "Unknown";
  });

  async function loadSaveFile() {
    try {
      error.value = null;
      loading.value = true;
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Diablo 2 Save',
          extensions: ['d2s']
        }]
      });

      if (selected) {
        saveData.value = await invoke("open_save_file", { path: selected });
        filePath.value = selected as string;
        isModified.value = false;

        // Auto-load the profile associated with this file
        const profileStore = useProfileStore();
        profileStore.loadProfileForFile(selected as string);
      }
    } catch (e: any) {
      error.value = e.toString();
    } finally {
      loading.value = false;
    }
  }

  // Save the current file
  async function saveFile(): Promise<boolean> {
    if (!saveData.value || !filePath.value) {
      error.value = "No file to save";
      return false;
    }

    try {
      error.value = null;
      await invoke("save_save_file", { path: filePath.value, save: saveData.value });
      isModified.value = false;
      return true;
    } catch (e: any) {
      error.value = e.toString();
      return false;
    }
  }

  // Mark data as modified
  function setModified(value: boolean) {
    isModified.value = value;
  }

  // Close current file
  function closeFile() {
    saveData.value = null;
    filePath.value = null;
    isModified.value = false;
    error.value = null;
  }

  // Get items for a specific container
  function getItemsByContainer(container: number): D2Item[] {
    if (!saveData.value) return [];
    // location 0 = grid position
    return saveData.value.items.items.filter(
      item => item.data.location === 0 && item.data.container === container
    );
  }

  // Check if a position is valid and free for an item
  function canPlaceItem(
    item: D2Item,
    container: number,
    col: number,
    row: number,
    excludeItemIndex: number = -1,
    gridDimensions?: GridDimensions
  ): boolean {
    if (!saveData.value) return false;

    // Use provided grid dimensions or fall back to defaults
    const gridConfig = gridDimensions || (
      container === CONTAINER.INVENTORY
        ? DEFAULT_GRID_SIZE.INVENTORY
        : container === CONTAINER.STASH
          ? DEFAULT_GRID_SIZE.STASH
          : DEFAULT_GRID_SIZE.CUBE
    );

    // Check bounds
    if (col < 0 || row < 0 ||
        col + item.data.width > gridConfig.cols ||
        row + item.data.height > gridConfig.rows) {
      return false;
    }

    // Check for collision with other items
    const items = saveData.value.items.items;
    for (let i = 0; i < items.length; i++) {
      if (i === excludeItemIndex) continue; // Skip the item being moved

      const otherItem = items[i];
      // Only check items in the same container at grid location
      if (otherItem.data.location !== 0 || otherItem.data.container !== container) continue;

      // Check for overlap
      const otherEndCol = otherItem.data.column + otherItem.data.width;
      const otherEndRow = otherItem.data.row + otherItem.data.height;
      const newEndCol = col + item.data.width;
      const newEndRow = row + item.data.height;

      // Check if rectangles overlap
      if (col < otherEndCol && newEndCol > otherItem.data.column &&
          row < otherEndRow && newEndRow > otherItem.data.row) {
        return false;
      }
    }

    return true;
  }

  // Move an item to a new position
  function moveItem(
    itemIndex: number,
    newContainer: number,
    newCol: number,
    newRow: number,
    gridDimensions?: GridDimensions
  ): boolean {
    if (!saveData.value) return false;

    const items = saveData.value.items.items;
    if (itemIndex < 0 || itemIndex >= items.length) return false;

    const item = items[itemIndex];

    // Validate the new position
    if (!canPlaceItem(item, newContainer, newCol, newRow, itemIndex, gridDimensions)) {
      return false;
    }

    // Update the item's position data
    // Note: This modifies the reactive state directly
    item.data.container = newContainer;
    item.data.column = newCol;
    item.data.row = newRow;
    item.data.location = 0; // Grid location

    // Mark as modified
    isModified.value = true;

    return true;
  }

  // Find item index by reference
  function findItemIndex(item: D2Item): number {
    if (!saveData.value) return -1;
    return saveData.value.items.items.indexOf(item);
  }

  // Drag state management
  function startDrag(item: D2Item, containerId: number) {
    console.log('Store: startDrag', item.data.name, 'container:', containerId);
    draggedItem.value = item;
    dragStartContainer.value = containerId;
    dragStartCol.value = item.data.column;
    dragStartRow.value = item.data.row;
  }

  function endDrag() {
    console.log('Store: endDrag');
    draggedItem.value = null;
    dragStartContainer.value = null;
  }

  function getDraggedItem(): D2Item | null {
    return draggedItem.value;
  }

  return {
    saveData,
    loading,
    error,
    filePath,
    isModified,
    charName,
    charClass,
    loadSaveFile,
    saveFile,
    closeFile,
    setModified,
    getItemsByContainer,
    canPlaceItem,
    moveItem,
    findItemIndex,
    // Drag state
    draggedItem,
    dragStartContainer,
    dragStartCol,
    dragStartRow,
    startDrag,
    endDrag,
    getDraggedItem,
    CONTAINER,
    GRID_SIZE,
  };
});
