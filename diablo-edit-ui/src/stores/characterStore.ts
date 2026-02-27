import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { D2sSave } from "../bindings/D2sSave";

export const useCharacterStore = defineStore("character", () => {
  const saveData = ref<D2sSave | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);

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
      }
    } catch (e: any) {
      error.value = e.toString();
    } finally {
      loading.value = false;
    }
  }

  return {
    saveData,
    loading,
    error,
    charName,
    charClass,
    loadSaveFile
  };
});
