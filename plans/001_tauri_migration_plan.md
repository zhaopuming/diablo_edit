# Diablo Edit2 Tauri Migration Plan

## Feasibility Assessment
**Conclusion: Highly Feasible and Strongly Recommended.**

The current architecture relies heavily on legacy MFC, which tightly couples the application logic to Windows APIs and involves complex manual pixel calculations for UI layouts (as seen in recent grid adjustments).

Migrating to a Tauri + Rust + Vue3 (TypeScript) stack offers extreme advantages:
1. **Memory Safety & Parsing Speed**: Rust is arguably the best language today for parsing binary formats (like D2S files). Crates like `binrw` or `nom` make binary deserialization and checksum verification robust and secure.
2. **Modern UI/UX**: The grid layout, drag-and-drop mechanics, layered item icons, and dynamic property rendering we struggled to calculate manually in MFC are trivial in Vue3 using CSS Grid or Flexbox.
3. **Cross-Platform**: Tauri compiles to Windows, macOS, and Linux natively. You will no longer be restricted to Windows.
4. **Lightweight & Fast**: Unlike Electron, Tauri uses the OS native webview (WebView2 on Windows), keeping binary sizes small (<10MB) and RAM usage minimal.

---

## Architectural Split

### Backend (Rust Core)
* **Goal**: Handle pure data logic. Completely agnostic to UI.
* **Responsibilities**:
  * Parse raw `.d2s` binary files into structured Rust structs.
  * Validate and generate the correct D2S checksums.
  * Hold the game logic dictionaries (Item codes, Affixes, Runewords, Stats mappings).
  * Expose an API (Tauri Commands) to read/write state.

### Frontend (Vue3 + TypeScript)
* **Goal**: Modern, reactive representation of the character state.
* **Responsibilities**:
  * Route between tabs (Stats, Skills, Waypoints, Quests, Items).
  * State management (Pinia) to track character modifications locally before sending back to Rust.
  * Render grids dynamically (e.g., `<StashGrid rows="16" cols="16" />`).
  * Implement robust drag-and-drop for the inventory (e.g., using VueDraggable).

---

## Migration Roadmap

### Phase 1: Rust Backend Core (`d2s-core` crate) [COMPLETED]
Before touching UI, the binary parsing must be solid.
1. [x] **Initialize a pure Rust library** to parse/serialize Diablo 2 Resurrected `.d2s` strings, headers, and bit-level item streams.
2. [x] **Translate C++ Structs (`Diablo2Struct.h`)**: Ported all major structures (Stats, Waypoints, Quests, Skills, Items, Corpses, Mercenaries).
3. [x] **Verification**: Built a robust test suite (via `main.rs` and report generator) verifying all 18 items in the test character with perfect bitstream synchronization.
4. [x] **Port Data Dictionaries**: Implemented automated metadata generation (`gen_metadata.py` and `gen_properties.py`) from original game data files.

### Phase 2: Tauri Application Setup
1. **Scaffold the project**:
   ```bash
   pnpm create tauri-app
   # Select Vue, TypeScript, Vite
   ```
2. **Establish IPC (Inter-Process Communication)**:
   * Write Tauri commands in Rust (`open_file`, `save_file`, `get_character_state`, `set_character_state`).
   * Generate TypeScript types from Rust structs (using `ts-rs` crate) to ensure frontend/backend data consistency.

### Phase 3: Frontend Foundation (Vue3)
1. **State Management**: Set up Pinia to hold the current `CharacterData` loaded from Rust.
2. **Routing / Navigation**: Create the main App shell with tabs similar to the original app ( Basic Info, Skills, Quests, Waypoints, Items ).
3. **Reusable Components**:
   * Create generic form inputs tailored for character stats.
   * Create an `<ItemIcon>` component that can overlay sockets, gems, and ethereal states.

### Phase 4: Feature-by-Feature Porting
**Step 4.1: Basic Info & Quests**
* Render raw stats, level, name, class.
* Two-way bind them to the Pinia store.

**Step 4.2: Skills**
* Render skill trees visually based on the chosen character class (using CSS Grid/Flex).

**Step 4.3: The Items Engine (The Hardest Part)**
* Rebuild the `POSITION_INFO` logic via CSS CSS Grid layout.
   * E.g. grid-template-columns: repeat(16, 28px); grid-template-rows: repeat(16, 28px);
* Build a Drag-and-Drop system overlaying the grids.
* Port the popup item creation / modification window.
* Implement hover tooltips for item properties.

### Phase 5: Polish & Build
1. **Styling & Assets**: Reconstruct the "Diablo 2" vintage aesthetic using CSS/SCSS and extracting the original sprites to `public/` assets.
2. **Testing Edge Cases**: Ensure corrupted saves are caught gracefully in Rust and throw nice error dialogs in Vue3.
3. **Release**: Run `tauri build` to package standard `.msi` installers.
