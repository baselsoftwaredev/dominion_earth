use bevy::prelude::*;
use core_sim::components::TerrainType;

use crate::ui::resources::HoveredTile;

// ============================================================================
// Component Markers
// ============================================================================

#[derive(Component)]
pub struct HoveredTileInfoPanel;

#[derive(Component)]
pub struct HoveredPositionText;

#[derive(Component)]
pub struct HoveredTerrainText;

// ============================================================================
// Update Systems
// ============================================================================

/// Update hovered tile information
pub fn update_hovered_tile_info(
    hovered_tile: Res<HoveredTile>,
    mut position_text: Query<&mut Text, (With<HoveredPositionText>, Without<HoveredTerrainText>)>,
    mut terrain_text: Query<&mut Text, (With<HoveredTerrainText>, Without<HoveredPositionText>)>,
) {
    if hovered_tile.is_changed() {
        match hovered_tile.position {
            Some(position) => {
                // Update position
                if let Some(mut text) = position_text.iter_mut().next() {
                    **text = format!("Position: ({}, {})", position.x, position.y);
                }

                // Update terrain
                if let Some(mut text) = terrain_text.iter_mut().next() {
                    let terrain_name = match &hovered_tile.terrain_type {
                        Some(terrain) => format_terrain_type(terrain),
                        None => "Unknown".to_string(),
                    };
                    **text = format!("Terrain: {}", terrain_name);
                }
            }
            None => {
                // No tile hovered
                if let Some(mut text) = position_text.iter_mut().next() {
                    **text = "Position: None".to_string();
                }

                if let Some(mut text) = terrain_text.iter_mut().next() {
                    **text = "Terrain: None".to_string();
                }
            }
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Format terrain type for display
fn format_terrain_type(terrain: &TerrainType) -> String {
    match terrain {
        TerrainType::Plains => "Plains".to_string(),
        TerrainType::Hills => "Hills".to_string(),
        TerrainType::Mountains => "Mountains".to_string(),
        TerrainType::Forest => "Forest".to_string(),
        TerrainType::Desert => "Desert".to_string(),
        TerrainType::Coast => "Coast".to_string(),
        TerrainType::ShallowCoast => "Shallow Coast".to_string(),
        TerrainType::Ocean => "Ocean".to_string(),
        TerrainType::River => "River".to_string(),
    }
}
