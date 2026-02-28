/**
 * Grid dimensions for a container
 */
export interface GridDimensions {
  cols: number;
  rows: number;
}

/**
 * Grid configuration for all containers
 */
export interface GridConfig {
  inventory: GridDimensions;
  stash: GridDimensions;
  cube: GridDimensions;
}

/**
 * A grid profile that defines container sizes
 */
export interface GridProfile {
  id: string;
  name: string;
  description?: string;
  isBuiltIn: boolean;
  grids: GridConfig;
}

/**
 * Built-in profile definitions
 */
export const BUILT_IN_PROFILES: GridProfile[] = [
  {
    id: 'original',
    name: 'Original',
    description: 'Standard Diablo 2 Resurrected grid sizes',
    isBuiltIn: true,
    grids: {
      inventory: { cols: 10, rows: 4 },
      stash: { cols: 10, rows: 10 },
      cube: { cols: 4, rows: 3 },
    },
  },
  {
    id: 'expanded',
    name: 'Expanded',
    description: 'Expanded inventory and stash (13×8 / 16×16)',
    isBuiltIn: true,
    grids: {
      inventory: { cols: 13, rows: 8 },
      stash: { cols: 16, rows: 16 },
      cube: { cols: 4, rows: 3 },
    },
  },
];
