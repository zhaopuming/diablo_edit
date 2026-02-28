<script setup lang="ts">
import { ref } from 'vue';
import { useProfileStore } from '../stores/profileStore';

const profileStore = useProfileStore();

const isOpen = ref(false);

const toggleDropdown = () => {
  isOpen.value = !isOpen.value;
};

const closeDropdown = () => {
  isOpen.value = false;
};

const selectProfile = (profileId: string) => {
  profileStore.setActiveProfile(profileId);
  closeDropdown();
};

// Get profile display name with grid info
const getProfileLabel = (profile: { name: string; grids: { inventory: { cols: number; rows: number }; stash: { cols: number; rows: number } } }) => {
  return `${profile.name}`;
};
</script>

<template>
  <div class="profile-selector" v-click-outside="closeDropdown">
    <button class="profile-button" @click="toggleDropdown">
      <span class="profile-icon">📦</span>
      <span class="profile-name">{{ profileStore.activeProfile.name }}</span>
      <span class="arrow" :class="{ open: isOpen }">▼</span>
    </button>

    <div class="dropdown" v-if="isOpen">
      <div class="dropdown-header">Grid Profiles</div>

      <button
        v-for="profile in profileStore.profiles"
        :key="profile.id"
        class="dropdown-item"
        :class="{ active: profile.id === profileStore.activeProfileId }"
        @click="selectProfile(profile.id)"
      >
        <span class="item-name">{{ getProfileLabel(profile) }}</span>
        <span class="item-info">
          Inv: {{ profile.grids.inventory.cols }}×{{ profile.grids.inventory.rows }} |
          Stash: {{ profile.grids.stash.cols }}×{{ profile.grids.stash.rows }}
        </span>
      </button>

      <div class="dropdown-footer">
        <span class="hint">Select a profile matching your game/mod</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.profile-selector {
  position: relative;
  user-select: none;
}

.profile-button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px;
  background: #2a2a2a;
  border: 1px solid #444;
  border-radius: 4px;
  color: #ccc;
  font-size: 0.9rem;
  cursor: pointer;
  transition: all 0.15s;
}

.profile-button:hover {
  background: #333;
  border-color: #666;
}

.profile-icon {
  font-size: 1rem;
}

.profile-name {
  font-weight: 500;
}

.arrow {
  font-size: 0.7rem;
  color: #888;
  transition: transform 0.2s;
}

.arrow.open {
  transform: rotate(180deg);
}

.dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  margin-top: 4px;
  min-width: 280px;
  background: #1a1a1a;
  border: 1px solid #444;
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
  z-index: 100;
  overflow: hidden;
}

.dropdown-header {
  padding: 10px 14px;
  font-size: 0.75rem;
  font-weight: 600;
  color: #888;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  border-bottom: 1px solid #333;
}

.dropdown-item {
  display: flex;
  flex-direction: column;
  gap: 2px;
  width: 100%;
  padding: 10px 14px;
  background: none;
  border: none;
  text-align: left;
  cursor: pointer;
  transition: background 0.1s;
}

.dropdown-item:hover {
  background: #252525;
}

.dropdown-item.active {
  background: #2a3a2a;
  border-left: 3px solid #4caf50;
}

.item-name {
  font-size: 0.9rem;
  color: #fff;
  font-weight: 500;
}

.item-info {
  font-size: 0.75rem;
  color: #888;
}

.dropdown-footer {
  padding: 8px 14px;
  font-size: 0.7rem;
  color: #666;
  font-style: italic;
  border-top: 1px solid #333;
}

.hint {
  opacity: 0.8;
}
</style>
