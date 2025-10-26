use bevy::prelude::*;
use core_sim::resources::CurrentTurn;

use crate::ui::resources::TerrainCounts;

// ============================================================================
// Component Markers
// ============================================================================

#[derive(Component)]
pub struct StatisticsPanel;

#[derive(Component)]
pub struct StatisticsTurnText;

#[derive(Component)]
pub struct StatisticsLandText;

#[derive(Component)]
pub struct StatisticsWaterText;

#[derive(Component)]
pub struct StatisticsMountainText;

// ============================================================================
// Update Systems
// ============================================================================

// ============================================================================
// Update Systems
// ============================================================================

/// Update statistics panel with current turn and terrain counts
pub fn update_statistics_panel(
    current_turn: Res<CurrentTurn>,
    terrain_counts: Res<TerrainCounts>,
    mut turn_text: Query<
        &mut Text,
        (
            With<StatisticsTurnText>,
            Without<StatisticsLandText>,
            Without<StatisticsWaterText>,
            Without<StatisticsMountainText>,
        ),
    >,
    mut land_text: Query<
        &mut Text,
        (
            With<StatisticsLandText>,
            Without<StatisticsTurnText>,
            Without<StatisticsWaterText>,
            Without<StatisticsMountainText>,
        ),
    >,
    mut water_text: Query<
        &mut Text,
        (
            With<StatisticsWaterText>,
            Without<StatisticsTurnText>,
            Without<StatisticsLandText>,
            Without<StatisticsMountainText>,
        ),
    >,
    mut mountain_text: Query<
        &mut Text,
        (
            With<StatisticsMountainText>,
            Without<StatisticsTurnText>,
            Without<StatisticsLandText>,
            Without<StatisticsWaterText>,
        ),
    >,
) {
    // Update turn text when it changes
    if current_turn.is_changed() {
        if let Some(mut text) = turn_text.iter_mut().next() {
            **text = format!("Turn: {}", current_turn.0);
        }
    }

    // Update terrain counts when they change
    if terrain_counts.is_changed() {
        let land_count = terrain_counts.plains
            + terrain_counts.hills
            + terrain_counts.forest
            + terrain_counts.desert;

        let water_count = terrain_counts.ocean + terrain_counts.coast + terrain_counts.river;

        if let Some(mut text) = land_text.iter_mut().next() {
            **text = format!("Land: {}", land_count);
        }

        if let Some(mut text) = water_text.iter_mut().next() {
            **text = format!("Water: {}", water_count);
        }

        if let Some(mut text) = mountain_text.iter_mut().next() {
            **text = format!("Mountains: {}", terrain_counts.mountains);
        }
    }
}
