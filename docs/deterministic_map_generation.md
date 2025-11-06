# Deterministic Map Generation with Seed-Based Regeneration

## Overview

The Dominion Earth game now implements **deterministic map generation using seed-based regeneration**. This means that when you load a saved game, the map is regenerated from the stored seed, guaranteeing that the exact same map terrain, mountains, forests, and resources are recreated every time.

## How It Works

### Architecture

1. **Seed Storage**: The `GameConfig` resource stores a `random_seed: u64` value that uniquely identifies the procedural generation parameters.

2. **Map Not Serialized**: Unlike previous versions, the `WorldMap` is **NOT serialized in save files**. Instead, only the seed is saved.

3. **Deterministic Regeneration**: When a save is loaded:
   - The `GameConfig` with the stored seed is deserialized first
   - A new system `regenerate_map_after_load` triggers
   - It reinitializes the `GameRng` with the stored seed
   - It calls `core_sim::world_gen::generate_island_map()` with the same seed
   - The identical map is reconstructed

### Benefits

- **Consistency**: Loading a save always produces the exact same map
- **Save Size**: Maps are not stored, reducing save file sizes significantly
- **Determinism**: Reproducible procedural generation ensures no surprises on load
- **Integration with bevy_rand**: Uses `bevy_rand` for robust RNG management

## Technical Details

### Random Number Generator

The game uses `rand_pcg::Pcg64` as its RNG, seeded with:

```rust
rng.0 = rand_pcg::Pcg64::seed_from_u64(game_config.random_seed);
```

### Map Generation Pipeline

The map generation follows these deterministic steps:

1. **Ocean Initialization** - All tiles start as ocean
2. **Landmass Generation** - Create islands using PCG64
3. **Mountain Placement** - Add linear mountain chains
4. **Forest Placement** - Place forests with clustering
5. **Resource Distribution** - Place resources by terrain type
6. **Coast Conversion** - Convert edges to coast tiles

Each step depends on the RNG state, so the same seed produces the same results.

### Save File Changes

**Before (Serialized Map)**:

```ron
"core_sim::resources::WorldMap": (
  width: 50,
  height: 25,
  tiles: [ /* ~1250 tile entries */ ]
)
```

**After (Seed Only)**:

- `WorldMap` is not included in the save file
- Only `GameConfig` with the seed is saved
- Map is regenerated on load

### Load Sequence

```
1. Load save file
   ├─ Deserialize GameConfig (with seed)
   ├─ Deserialize other resources
   └─ Deserialize entities (civilizations, cities, units)

2. Post-load systems execute
   ├─ regenerate_map_after_load
   │  ├─ Reinitialize RNG with stored seed
   │  └─ Call generate_island_map()
   ├─ restore_player_control_after_load
   ├─ refresh_fog_of_war_after_load
   ├─ respawn_ui_after_load
   └─ ... other restoration systems

3. Game state restored
```

## Code Implementation

### In `dominion_earth/src/plugins/save_load.rs`

**Removed from save**:

```rust
// REMOVED:
// .include_resource::<WorldMap>()
```

**Added regeneration system**:

```rust
fn regenerate_map_after_load(
    mut world_map: ResMut<WorldMap>,
    mut rng: ResMut<GameRng>,
    game_config: Res<GameConfig>,
    save_state: Res<SaveLoadState>,
) {
    if !save_state.is_loading_from_save || !game_config.is_changed() {
        return;
    }

    rng.0 = rand_pcg::Pcg64::seed_from_u64(game_config.random_seed);
    *world_map = core_sim::world_gen::generate_island_map(
        world_map.width,
        world_map.height,
        &mut rng.0
    );
}
```

## Verification

To verify the implementation:

1. **Generate a new game** with seed S

   - Note the map layout, resources, mountains, etc.

2. **Save the game** at turn 1 or 2

   - Check save file size (should be smaller)
   - Verify `GameConfig` contains the seed

3. **Load the saved game**

   - The map should be **identical** to the original

4. **Generate another new game** with the same seed S
   - The map should match exactly

## Future Considerations

- The regeneration happens every load. Consider caching in production for faster loads
- Fog of War is correctly recalculated after map regeneration
- Civilizations and their placements remain unchanged on load (entities are serialized)
- Resources are part of the map and are regenerated with it

## Potential Issues

If you encounter desyncs between saves and reloads:

1. Verify the seed is being saved correctly in `GameConfig`
2. Check that no systems are modifying the RNG during serialization
3. Ensure the PCG64 algorithm hasn't changed
4. Verify `world_gen` modules are using the RNG consistently

## Testing

A comprehensive test would:

1. Create a save with known seed
2. Load it and verify tile-by-tile map matches
3. Advance one turn and save again
4. Load and verify consistency

```rust
#[test]
fn test_deterministic_map_generation() {
    let seed = 12345u64;
    let mut rng1 = rand_pcg::Pcg64::seed_from_u64(seed);
    let mut rng2 = rand_pcg::Pcg64::seed_from_u64(seed);

    let map1 = generate_island_map(50, 25, &mut rng1);
    let map2 = generate_island_map(50, 25, &mut rng2);

    assert_eq!(map1.tiles, map2.tiles);
}
```
