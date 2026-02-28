import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { LazyStore } from "@tauri-apps/plugin-store";
import type { GridProfile, GridConfig, GridDimensions } from "../types/profile";
import { BUILT_IN_PROFILES } from "../types/profile";

// Container type constants (same as characterStore for consistency)
export const CONTAINER = {
  INVENTORY: 1,
  STASH: 5,
  CUBE: 4,
} as const;

// Store file and keys
const STORE_FILE = "profile-store.json";
const KEY_FILE_PROFILE_MAP = "file-profile-map";
const KEY_CUSTOM_PROFILES = "custom-profiles";
const KEY_ACTIVE_PROFILE = "active-profile";

// Lazy-loaded store instance
const store = new LazyStore(STORE_FILE);

export const useProfileStore = defineStore("profile", () => {
  // All profiles (built-in + custom)
  const profiles = ref<GridProfile[]>([...BUILT_IN_PROFILES]);

  // Currently active profile ID
  const activeProfileId = ref<string>("original");

  // Custom profiles (persisted)
  const customProfiles = ref<GridProfile[]>([]);

  // Mapping of file paths to profile IDs
  const fileProfileMap = ref<Map<string, string>>(new Map());

  // Initialization flag
  const initialized = ref(false);

  // Computed: Get the active profile
  const activeProfile = computed(() => {
    return profiles.value.find((p) => p.id === activeProfileId.value) || BUILT_IN_PROFILES[0];
  });

  // Computed: Get grid config for active profile
  const gridConfig = computed((): GridConfig => {
    return activeProfile.value.grids;
  });

  // Set active profile by ID
  function setActiveProfile(id: string) {
    const profile = profiles.value.find((p) => p.id === id);
    if (profile) {
      activeProfileId.value = id;
      console.log("Switched to profile:", profile.name);
    } else {
      console.warn("Profile not found:", id);
    }
  }

  // Get grid dimensions for a specific container
  function getGridDimensions(container: number): GridDimensions {
    const config = gridConfig.value;
    switch (container) {
      case CONTAINER.INVENTORY:
        return config.inventory;
      case CONTAINER.STASH:
        return config.stash;
      case CONTAINER.CUBE:
        return config.cube;
      default:
        // Default to inventory size
        return config.inventory;
    }
  }

  // ===== Persistence Functions =====

  // Load all persisted data from Tauri store
  async function loadFromStore() {
    if (initialized.value) return;

    try {
      // Init the lazy store
      await store.init();

      // Load file-profile map
      const fileMapData = await store.get<Record<string, string>>(KEY_FILE_PROFILE_MAP);
      if (fileMapData) {
        fileProfileMap.value = new Map(Object.entries(fileMapData));
        console.log("Loaded file-profile map:", fileProfileMap.value.size, "entries");
      }

      // Load custom profiles
      const customProfilesData = await store.get<GridProfile[]>(KEY_CUSTOM_PROFILES);
      if (customProfilesData && customProfilesData.length > 0) {
        customProfiles.value = customProfilesData;
        profiles.value = [...BUILT_IN_PROFILES, ...customProfilesData];
        console.log("Loaded custom profiles:", customProfilesData.length);
      }

      // Load active profile
      const activeProfileData = await store.get<string>(KEY_ACTIVE_PROFILE);
      if (activeProfileData && profiles.value.some(p => p.id === activeProfileData)) {
        activeProfileId.value = activeProfileData;
        console.log("Restored active profile:", activeProfileData);
      }

      initialized.value = true;
    } catch (e) {
      console.warn("Failed to load from store:", e);
    }
  }

  // Save file-profile map to Tauri store
  async function saveFileProfileMap() {
    try {
      const obj = Object.fromEntries(fileProfileMap.value);
      await store.set(KEY_FILE_PROFILE_MAP, obj);
    } catch (e) {
      console.warn("Failed to save file-profile map:", e);
    }
  }

  // Save custom profiles to Tauri store
  async function saveCustomProfiles() {
    try {
      await store.set(KEY_CUSTOM_PROFILES, customProfiles.value);
    } catch (e) {
      console.warn("Failed to save custom profiles:", e);
    }
  }

  // Save active profile to Tauri store
  async function saveActiveProfile() {
    try {
      await store.set(KEY_ACTIVE_PROFILE, activeProfileId.value);
    } catch (e) {
      console.warn("Failed to save active profile:", e);
    }
  }

  // ===== Profile Management =====

  // Add a custom profile
  async function addCustomProfile(profile: GridProfile) {
    // Ensure ID is unique
    const existingIds = new Set(profiles.value.map((p) => p.id));
    let finalId = profile.id;
    let counter = 1;
    while (existingIds.has(finalId)) {
      finalId = `${profile.id}-${counter}`;
      counter++;
    }

    const newProfile: GridProfile = {
      ...profile,
      id: finalId,
      isBuiltIn: false,
    };

    customProfiles.value.push(newProfile);
    profiles.value.push(newProfile);
    await saveCustomProfiles();
    return newProfile;
  }

  // Update a custom profile
  async function updateCustomProfile(id: string, updates: Partial<GridProfile>) {
    const index = customProfiles.value.findIndex((p) => p.id === id);
    if (index >= 0) {
      customProfiles.value[index] = {
        ...customProfiles.value[index],
        ...updates,
      };
      // Also update in the main profiles array
      const mainIndex = profiles.value.findIndex((p) => p.id === id);
      if (mainIndex >= 0) {
        profiles.value[mainIndex] = customProfiles.value[index];
      }
      await saveCustomProfiles();
    }
  }

  // Delete a custom profile
  async function deleteCustomProfile(id: string) {
    const index = customProfiles.value.findIndex((p) => p.id === id);
    if (index >= 0) {
      customProfiles.value.splice(index, 1);
      // Also remove from main profiles array
      const mainIndex = profiles.value.findIndex((p) => p.id === id);
      if (mainIndex >= 0) {
        profiles.value.splice(mainIndex, 1);
      }
      // If this was the active profile, switch to default
      if (activeProfileId.value === id) {
        setActiveProfile("original");
      }
      await saveCustomProfiles();
    }
  }

  // ===== File-Profile Mapping Functions =====

  // Get the profile ID associated with a file path
  function getProfileForFile(filePath: string): string | null {
    return fileProfileMap.value.get(filePath) || null;
  }

  // Associate a file path with the current profile
  async function setProfileForFile(filePath: string, profileId?: string) {
    const id = profileId || activeProfileId.value;
    fileProfileMap.value.set(filePath, id);
    await saveFileProfileMap();
    console.log(`Associated file "${filePath}" with profile "${id}"`);
  }

  // Remove a file from the mapping
  async function removeFileFromMap(filePath: string) {
    if (fileProfileMap.value.has(filePath)) {
      fileProfileMap.value.delete(filePath);
      await saveFileProfileMap();
    }
  }

  // Load profile for a specific file (call when opening a file)
  async function loadProfileForFile(filePath: string): Promise<boolean> {
    // Ensure store is loaded first
    if (!initialized.value) {
      await loadFromStore();
    }

    const profileId = getProfileForFile(filePath);
    if (profileId) {
      // Check if the profile still exists
      const profileExists = profiles.value.some(p => p.id === profileId);
      if (profileExists) {
        setActiveProfile(profileId);
        console.log(`Auto-loaded profile "${profileId}" for file "${filePath}"`);
        return true;
      } else {
        // Profile no longer exists, remove the stale mapping
        await removeFileFromMap(filePath);
      }
    }
    return false;
  }

  // Initialize: load persisted data (async, called on app start)
  loadFromStore();

  return {
    profiles,
    activeProfileId,
    activeProfile,
    gridConfig,
    initialized,
    setActiveProfile,
    getGridDimensions,
    // Persistence
    loadFromStore,
    saveActiveProfile,
    // Custom profiles
    addCustomProfile,
    updateCustomProfile,
    deleteCustomProfile,
    // File-profile mapping
    fileProfileMap,
    getProfileForFile,
    setProfileForFile,
    removeFileFromMap,
    loadProfileForFile,
    CONTAINER,
  };
});
