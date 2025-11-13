# TMX Loading Implementation Guide for Dominion Earth

## Overview

This guide documents how to integrate `bevy_ecs_tiled` to load TMX map files in Dominion Earth, replacing the current hardcoded map loading logic.

## Current Implementation (game.rs)

The current `load_map_from_tiled()` function in `game.rs` hardcodes a 10x10 grassland map and doesn't actually parse TMX files. We need to replace this with proper TMX loading using `bevy_ecs_tiled`.

## bevy_ecs_tiled Architecture

### Core Components

1. **TiledPlugin** - Main plugin that handles TMX/TSX asset loading

   - Automatically adds `bevy_ecs_tilemap::TilemapPlugin`
   - Handles asset loading and entity spawning

2. **TiledMap** - Component to spawn a map entity

   ```rust
   commands.spawn(TiledMap(asset_server.load("map.tmx")));
   ```

3. **TiledMapAsset** - The loaded TMX asset

   - Contains the raw `tiled::Map` data
   - Accessed via `Assets<TiledMapAsset>`

4. **TiledMapStorage** - Component that stores references to spawned entities
   - Can query tiles, objects, layers via this component
   - Example: `storage.tiles()`, `storage.objects()`

### Entity Hierarchy

When a TMX map loads, bevy_ecs_tiled creates this hierarchy:

```
TiledMap (top-level entity)
├── TiledLayer::Tiles (tile layers)
│   ├── TiledTilemap (one per tileset used)
│   │   └── TiledTile entities
├── TiledLayer::Objects (object layers)
│   └── TiledObject entities
└── TiledLayer::Image (image layers)
```

### Key Events

`bevy_ecs_tiled` fires several events during map loading:

1. **TiledEvent<MapCreated>** - Fired when map finishes loading

   - Access via `EventReader<TiledEvent<MapCreated>>`
   - Or use `.observe()` on the map entity

2. **TiledEvent<LayerCreated>** - Fired for each layer

3. **TiledEvent<TileCreated>** - Fired for each tile (only if tile has custom properties)

4. **TiledEvent<ObjectCreated>** - Fired for each object

## Implementation Plan for Dominion Earth

### Step 1: Add Dependencies

Add to `dominion_earth/Cargo.toml`:

```toml
[dependencies]
bevy_ecs_tiled = "0.9"
bevy_ecs_tilemap = "0.16"  # Will be auto-added by bevy_ecs_tiled
```

### Step 2: Add TiledPlugin

In `dominion_earth/src/main.rs` or the game plugin setup:

```rust
use bevy_ecs_tiled::prelude::*;

App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(TiledPlugin::default())
    // ... rest of plugins
```

### Step 3: Replace load_map_from_tiled() Function

Current function location: `dominion_earth/src/game.rs` lines ~169-193

**Current approach:**

- Called in `setup_game()`
- Hardcodes 10x10 grassland
- Returns `WorldMap` directly

**New approach:**

- Spawn a `TiledMap` entity in `setup_game()`
- Use `TiledEvent<MapCreated>` to convert to `WorldMap`
- Handle async loading (map loads over multiple frames)

### Step 4: Convert TMX Data to WorldMap

We need a system that:

1. Listens for `TiledEvent<MapCreated>`
2. Queries the loaded `TiledMapAsset`
3. Extracts tile data and converts to our `WorldMap` format

**Example conversion system:**

```rust
fn convert_tiled_to_world_map(
    mut events: EventReader<TiledEvent<MapCreated>>,
    tiled_assets: Res<Assets<TiledMapAsset>>,
    mut world_map: ResMut<WorldMap>,
) {
    for event in events.read() {
        // Get the TiledMapAsset
        let Some(tiled_map_asset) = event.get_map(&tiled_assets) else {
            continue;
        };

        // Access the underlying tiled::Map
        let tiled_map: &tiled::Map = &tiled_map_asset.map;

        // Extract dimensions
        let width = tiled_map.width;
        let height = tiled_map.height;

        // Initialize WorldMap
        *world_map = WorldMap::new(width, height);

        // Iterate through tile layers
        for layer in tiled_map.layers() {
            if let Some(tile_layer) = layer.as_tile_layer() {
                for x in 0..width {
                    for y in 0..height {
                        if let Some(tile_data) = tile_layer.get_tile(x as i32, y as i32) {
                            // Convert tile GID to TerrainType
                            let terrain = map_tile_id_to_terrain(tile_data.id());

                            world_map.tiles[x as usize][y as usize] = MapTile {
                                terrain,
                                owner: None,
                                city: None,
                                resource: None,
                            };
                        }
                    }
                }
            }
        }
    }
}

fn map_tile_id_to_terrain(tile_id: u32) -> TerrainType {
    match tile_id {
        1 => TerrainType::Plains,
        2 => TerrainType::Grassland,
        3 => TerrainType::Desert,
        4 => TerrainType::Tundra,
        5 => TerrainType::Ocean,
        6 => TerrainType::Mountain,
        7 => TerrainType::Hills,
        8 => TerrainType::Forest,
        _ => TerrainType::Plains, // default
    }
}
```

### Step 5: Handle Timing Issues

The TMX map loads asynchronously, so we need to:

1. **Split setup_game() into stages:**

   - Stage 1: Spawn TiledMap entity
   - Stage 2: Wait for MapCreated event
   - Stage 3: Convert to WorldMap
   - Stage 4: Spawn civilizations

2. **Use Bevy States or run conditions:**

   ```rust
   #[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
   enum GameLoadState {
       #[default]
       LoadingMap,
       MapLoaded,
       SpawningCivs,
       Ready,
   }

   // In plugin:
   .add_systems(OnEnter(GameLoadState::LoadingMap), spawn_tiled_map)
   .add_systems(Update,
       convert_tiled_to_world_map
           .run_if(in_state(GameLoadState::LoadingMap))
   )
   .add_systems(OnEnter(GameLoadState::MapLoaded), spawn_civilizations)
   ```

3. **Or use a Local flag:**
   ```rust
   fn spawn_civilizations_when_ready(
       mut world_map: ResMut<WorldMap>,
       mut initialized: Local<bool>,
       // ... other params
   ) {
       if *initialized || world_map.width == 0 {
           return;
       }

       spawn_initial_civilizations(...);
       *initialized = true;
   }
   ```

### Step 6: Map TMX Tiles to TerrainTypes

**Option A: Use tile IDs directly**

- Define tile ID → TerrainType mapping in constants
- Example: tiles/grass-land.tmx tile ID 1 = Plains

**Option B: Use Tiled Custom Properties**

- Enable `user_properties` feature in bevy_ecs_tiled
- Define TerrainType as a Component with `#[derive(Reflect)]`
- Export types to .json and import in Tiled
- Add TerrainType as custom property to tiles in Tiled
- Read from `TiledTile` components

**Recommendation:** Start with Option A (simpler), can upgrade to Option B later for more flexibility.

### Step 7: Position TiledMap vs WorldMap

Important consideration:

- `TiledMap` spawns visual entities (sprites/tilemaps) for rendering
- `WorldMap` is our simulation data structure
- They can coexist - TiledMap for visuals, WorldMap for game logic

**Integration options:**

1. **Keep both separate** (Recommended)

   - TiledMap handles rendering
   - WorldMap handles simulation
   - Conversion system syncs them once at startup

2. **Use only TiledMap**
   - Query TiledTile entities for simulation
   - More Bevy-native but requires refactoring all simulation code

## Example Implementation Flow

```rust
// 1. Setup - spawn the TiledMap
fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfig>,
) {
    // Spawn the TiledMap entity
    commands.spawn((
        TiledMap(asset_server.load("tiles/grass-land.tmx")),
        TilemapAnchor::BottomLeft,
    ))
    .observe(on_map_loaded);  // Attach observer for when map loads
}

// 2. Observer - convert when map loads
fn on_map_loaded(
    trigger: Trigger<TiledEvent<MapCreated>>,
    tiled_assets: Res<Assets<TiledMapAsset>>,
    mut world_map: ResMut<WorldMap>,
    mut map_loaded: ResMut<MapLoaded>,  // Custom resource flag
) {
    let Some(tiled_map_asset) = trigger.event().get_map(&tiled_assets) else {
        return;
    };

    // Convert to WorldMap
    *world_map = convert_tiled_to_world_map(tiled_map_asset);
    map_loaded.0 = true;

    println!("Map loaded: {}x{}", world_map.width, world_map.height);
}

// 3. Spawn civilizations only after map is loaded
fn spawn_civilizations_system(
    mut commands: Commands,
    mut world_map: ResMut<WorldMap>,
    map_loaded: Res<MapLoaded>,
    mut initialized: Local<bool>,
    // ... other resources
) {
    if *initialized || !map_loaded.0 {
        return;
    }

    spawn_initial_civilizations(&mut commands, &mut world_map, ...);
    *initialized = true;
}
```

## TMX File Structure to Support

The `grass-land.tmx` file should have:

- **Orthogonal orientation** (standard grid)
- **Tile layer(s)** containing terrain data
- **External tileset** (`.tsx`) or embedded tileset
- **Tile IDs** corresponding to terrain types

Example minimal TMX structure:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<map version="1.10" orientation="orthogonal" width="10" height="10" tilewidth="32" tileheight="32">
 <tileset firstgid="1" source="terrain.tsx"/>
 <layer name="Terrain" width="10" height="10">
  <data encoding="csv">
1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,
...
  </data>
 </layer>
</map>
```

## Debugging and Testing

1. **Enable debug plugins:**

   ```rust
   .add_plugins(TiledDebugPluginGroup)
   ```

2. **Log map loading events:**

   ```rust
   fn debug_map_events(
       mut events: EventReader<TiledEvent<MapCreated>>,
   ) {
       for event in events.read() {
           println!("Map loaded! Entity: {:?}", event.origin);
       }
   }
   ```

3. **Verify tile data:**
   ```rust
   fn debug_tiles(
       query: Query<(&TilePos, &TiledTile)>,
   ) {
       for (pos, tile) in query.iter().take(5) {
           println!("Tile at {:?}: {:?}", pos, tile);
       }
   }
   ```

## Migration Strategy

1. **Phase 1: Parallel Implementation**

   - Add TiledPlugin alongside existing code
   - Create conversion system but don't activate yet
   - Keep hardcoded map as fallback

2. **Phase 2: Switch to TMX Loading**

   - Replace hardcoded map with TMX loading
   - Test with grass-land.tmx (Debug map size)
   - Verify civilization spawning works

3. **Phase 3: Add More Maps**

   - Create TMX files for Small, Medium, Large, Huge sizes
   - Update GameSettings to select appropriate TMX file
   - Remove old world generation code

4. **Phase 4: Add Custom Properties (Optional)**
   - Enable user_properties feature
   - Define TerrainType, Resource properties
   - Export and use in Tiled for richer maps

## Key Takeaways

- **bevy_ecs_tiled** handles all TMX parsing and entity spawning
- **TiledMap** component triggers the loading process
- **TiledEvent<MapCreated>** signals when to convert data
- **WorldMap** remains our simulation data structure
- **Async loading** requires careful sequencing of game initialization
- **Tile ID mapping** is needed to convert TMX tiles to TerrainTypes

## References

- bevy_ecs_tiled docs: https://docs.rs/bevy_ecs_tiled/latest/
- Examples: `/docs/bevy_ecs_tiled_examples/docs/`
- Book: `/docs/bevy_ecs_tiled_examples/books/src/`
