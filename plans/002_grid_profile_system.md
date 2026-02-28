# Grid Profile System Plan

## Overview

Diablo 2 mods often change the inventory grid sizes (e.g., expanded stash, larger inventory). Since `.d2s` save files don't contain metadata about which mod created them, users need a way to configure grid dimensions to match their game setup.

This plan implements a **Profile System** that allows users to:
1. Select from pre-defined profiles (Original, popular mods)
2. Create and manage custom profiles
3. Switch profiles at any time
4. Persist their preferred profile across sessions

---

## Data Structure

### Profile Definition

```typescript
interface GridProfile {
  id: string;           // Unique identifier (e.g., "original", "plugy", "custom-1")
  name: string;         // Display name (e.g., "Original D2R", "PlugY")
  description?: string; // Optional description
  isBuiltIn: boolean;   // true for pre-defined profiles, false for user-created
  grids: {
    inventory: { cols: number; rows: number };
    stash: { cols: number; rows: number };
    cube: { cols: number; rows: number };
  };
}
```

### Built-in Profiles

| Profile ID | Name | Inventory | Stash | Cube |
|------------|------|-----------|-------|------|
| `original` | Original D2R | 10×4 | 10×10 | 4×3 |
| `plugy` | PlugY | 10×4 | 10×10* | 4×3 |
| `median-xl` | Median XL | 10×4 | 10×10 | 4×3 |
| `pd2` | Project Diablo 2 | 10×4 | 10×10 | 4×3 |

*Note: PlugY has shared stash pages, which is a separate feature to consider later.

---

## Storage

### Frontend Storage (Pinia Store)

Create a new `profileStore.ts`:

```typescript
// stores/profileStore.ts
export const useProfileStore = defineStore('profile', () => {
  const profiles = ref<GridProfile[]>([]);
  const activeProfileId = ref<string>('original');

  // Computed
  const activeProfile = computed(() =>
    profiles.value.find(p => p.id === activeProfileId.value)
  );

  // Actions
  function loadProfiles() { /* ... */ }
  function saveProfiles() { /* ... */ }
  function setActiveProfile(id: string) { /* ... */ }
  function addCustomProfile(profile: GridProfile) { /* ... */ }
  function updateCustomProfile(id: string, updates: Partial<GridProfile>) { /* ... */ }
  function deleteCustomProfile(id: string) { /* ... */ }

  return { /* ... */ };
});
```

### Persistence (Tauri Store Plugin)

Use `@tauri-apps/plugin-store` to persist profiles to a local JSON file:
- Location: `~/.config/diablo-edit/profiles.json` (or platform equivalent)
- Built-in profiles are always loaded first, custom profiles merged from file

---

## UI Components

### 1. Profile Selector (ItemsView)

A dropdown in the Items view header to quickly switch profiles:

```
[Items]                    Profile: [Original D2R ▼]
                           ┌─────────────────────┐
                           │ Original D2R        │
                           │ PlugY               │
                           │ Median XL           │
                           │ Project Diablo 2    │
                           │ ─────────────────── │
                           │ Manage Profiles...  │
                           └─────────────────────┘
```

### 2. Profile Manager Dialog

A modal for managing custom profiles:

```
┌─────────────────────────────────────────────────────────────┐
│  Manage Grid Profiles                                    [X] │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Built-in Profiles:                                         │
│  ┌───────────────────────────────────────────────────────┐ │
│  │ Original D2R    Inventory: 10×4  Stash: 10×10  Cube: 4×3│ │
│  │ PlugY           Inventory: 10×4  Stash: 10×10  Cube: 4×3│ │
│  │ Median XL       Inventory: 10×4  Stash: 10×10  Cube: 4×3│ │
│  └───────────────────────────────────────────────────────┘ │
│                                                             │
│  Custom Profiles:                                           │
│  ┌───────────────────────────────────────────────────────┐ │
│  │ My Mod Setup    Inventory: 13×8  Stash: 16×16  Cube: 4×3│ │
│  │                                    [Edit]  [Delete]     │ │
│  └───────────────────────────────────────────────────────┘ │
│                                                             │
│  [+ Add Custom Profile]                                     │
│                                                             │
│                                           [Close]           │
└─────────────────────────────────────────────────────────────┘
```

### 3. Profile Editor Dialog

For creating/editing custom profiles:

```
┌─────────────────────────────────────────────────────────────┐
│  Edit Profile                                             [X] │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Profile Name:    [My Mod Setup              ]              │
│  Description:     [Expanded grids for...     ]              │
│                                                             │
│  Grid Sizes:                                                 │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ Inventory:  Cols [13]▼  Rows [8]▼                       ││
│  │ Stash:      Cols [16]▼  Rows [16]▼                      ││
│  │ Cube:       Cols [4]▼   Rows [3]▼                       ││
│  └─────────────────────────────────────────────────────────┘│
│                                                             │
│                              [Cancel]  [Save]               │
└─────────────────────────────────────────────────────────────┘
```

---

## Auto-Detection Hint

When loading a save file, check if any items are positioned outside the active profile's grid bounds:

```typescript
function checkGridBounds(saveData: D2sSave, profile: GridProfile): string[] {
  const warnings: string[] = [];

  for (const item of saveData.items.items) {
    if (item.data.location !== 0) continue; // Skip equipped items

    const gridConfig = getGridConfig(item.data.container, profile);
    if (!gridConfig) continue;

    if (item.data.column + item.data.width > gridConfig.cols ||
        item.data.row + item.data.height > gridConfig.rows) {
      warnings.push(
        `Item "${item.data.name}" at position (${item.data.column}, ${item.data.row}) ` +
        `exceeds ${getContainerName(item.data.container)} bounds`
      );
    }
  }

  return warnings;
}
```

If warnings are found, show a toast/notification:
> ⚠️ **Grid Size Mismatch**: Some items are outside the current profile's grid bounds. Consider switching to a larger profile.

---

## Implementation Phases

### Phase 1: Core Infrastructure
1. [ ] Create `GridProfile` interface in `src/types/profile.ts`
2. [ ] Create `profileStore.ts` with built-in profiles
3. [ ] Integrate profile grid sizes into `characterStore.ts`
4. [ ] Update `ItemsView.vue` to use profile-based grid sizes

### Phase 2: Profile Selection UI
1. [ ] Create `ProfileSelector.vue` dropdown component
2. [ ] Add selector to `ItemsView.vue` header
3. [ ] Wire up profile switching

### Phase 3: Persistence
1. [ ] Add `@tauri-apps/plugin-store` dependency
2. [ ] Implement profile persistence (save/load custom profiles)
3. [ ] Persist active profile ID

### Phase 4: Profile Management
1. [ ] Create `ProfileManager.vue` dialog component
2. [ ] Create `ProfileEditor.vue` form component
3. [ ] Add CRUD operations for custom profiles

### Phase 5: Auto-Detection
1. [ ] Implement `checkGridBounds()` function
2. [ ] Show warning notification when items exceed bounds
3. [ ] Add "Switch Profile" quick action in notification

---

## File Structure

```
diablo-edit-ui/
├── src/
│   ├── types/
│   │   └── profile.ts          # GridProfile interface
│   ├── stores/
│   │   ├── characterStore.ts   # Updated to use profile grids
│   │   └── profileStore.ts     # New: profile management
│   ├── components/
│   │   ├── ProfileSelector.vue # New: dropdown selector
│   │   ├── ProfileManager.vue  # New: management dialog
│   │   └── ProfileEditor.vue   # New: editor form
│   └── views/
│       └── ItemsView.vue       # Updated: include selector
```

---

## Future Considerations

1. **Shared Stash Pages**: PlugY and some mods have multiple stash pages. This would require:
   - Extending the profile to include `stashPages: number`
   - UI for switching between pages
   - Understanding how pages are stored in the save file

2. **Profile Import/Export**: Allow users to share profiles via JSON files

3. **Mod Detection Heuristics**: If specific mods have unique signatures, attempt auto-detection
