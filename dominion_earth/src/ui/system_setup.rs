use crate::debug_utils::DebugLogging;
use crate::entity_utils;
use crate::ui::traits::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

pub use crate::ui::capital_labels::{spawn_capital_labels, update_capital_labels};
pub use crate::ui::unit_labels::{spawn_unit_labels, update_unit_labels};

/// Native Bevy UI system implementation
pub struct BevyUiSystem;

impl BevyUiSystem {
    pub fn setup_plugins(app: &mut App) {
        app.add_systems(
            Startup,
            (
                crate::ui::top_panel::spawn_top_panel,
                crate::ui::right_panel::spawn_right_panel,
                crate::ui::left_panel::spawn_left_panel,
            ),
        )
        .add_systems(
            Update,
            (
                handle_ui_scroll.before(crate::input::handle_mouse_input),
                spawn_capital_labels,
                update_capital_labels,
                spawn_unit_labels,
                update_unit_labels,
                crate::ui::top_panel::update_player_resources,
                crate::ui::top_panel::update_turn_display,
                crate::ui::right_panel::update_statistics_panel,
                crate::ui::right_panel::update_hovered_tile_info,
                crate::ui::right_panel::update_civilizations_list,
                crate::ui::left_panel::update_next_turn_button_text,
                crate::ui::left_panel::handle_next_turn_button,
                crate::ui::left_panel::handle_infantry_button,
                crate::ui::left_panel::handle_archer_button,
                crate::ui::left_panel::handle_cavalry_button,
                crate::ui::left_panel::update_production_button_visuals,
                crate::ui::left_panel::update_production_menu,
                crate::ui::left_panel::update_unit_info,
            ),
        );
    }

    pub fn setup_plugins_for_screen<S: States>(app: &mut App, screen: S) {
        app.add_systems(
            OnEnter(screen.clone()),
            (
                crate::ui::top_panel::spawn_top_panel,
                crate::ui::right_panel::spawn_right_panel,
                crate::ui::left_panel::spawn_left_panel,
            ),
        )
        .add_systems(OnExit(screen.clone()), cleanup_ui)
        .add_systems(
            Update,
            handle_ui_scroll
                .before(crate::input::handle_mouse_input)
                .run_if(in_state(screen.clone())),
        )
        .add_systems(
            Update,
            (
                spawn_capital_labels,
                update_capital_labels,
                spawn_unit_labels,
                update_unit_labels,
                crate::ui::top_panel::update_player_resources,
                crate::ui::top_panel::update_turn_display,
                crate::ui::right_panel::update_statistics_panel,
                crate::ui::right_panel::update_hovered_tile_info,
                crate::ui::right_panel::update_civilizations_list,
                crate::ui::left_panel::update_next_turn_button_text,
                crate::ui::left_panel::handle_next_turn_button,
                crate::ui::left_panel::handle_infantry_button,
                crate::ui::left_panel::handle_archer_button,
                crate::ui::left_panel::handle_cavalry_button,
                crate::ui::left_panel::update_production_button_visuals,
                crate::ui::left_panel::update_production_menu,
                crate::ui::left_panel::update_unit_info,
            )
                .run_if(in_state(screen)),
        );
    }
}

fn cleanup_ui(
    mut commands: Commands,
    top_panel: Query<Entity, With<crate::ui::top_panel::TopPanel>>,
    right_panel: Query<Entity, With<crate::ui::right_panel::RightPanel>>,
    left_panel: Query<Entity, With<crate::ui::left_panel::LeftPanel>>,
    children_query: Query<&Children>,
) {
    let mut despawned = std::collections::HashSet::new();

    for entity in &top_panel {
        entity_utils::recursively_despawn_entity_with_children(
            &mut commands,
            entity,
            &children_query,
            &mut despawned,
        );
    }

    for entity in &right_panel {
        entity_utils::recursively_despawn_entity_with_children(
            &mut commands,
            entity,
            &children_query,
            &mut despawned,
        );
    }

    for entity in &left_panel {
        entity_utils::recursively_despawn_entity_with_children(
            &mut commands,
            entity,
            &children_query,
            &mut despawned,
        );
    }
}

impl UiSystem for BevyUiSystem {
    fn initialize(&self, app: &mut App) {
        Self::setup_plugins(app);
    }

    fn render_main_game_panel(&self, _data: &GamePanelData) {}

    fn render_production_menu(&self, _data: &ProductionMenuData) {}

    fn render_statistics_panel(&self, _data: &StatisticsPanelData) {}

    fn render_tile_info(&self, _data: &TileInfoData) {}

    fn render_minimap(&self, _data: &MinimapData) {}

    fn render_resources(&self, _data: &ResourcesData) {}
}

fn handle_ui_scroll(
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    mut scrollable_query: Query<(&mut ScrollPosition, &Node, &ComputedNode)>,
    window_query: Query<&Window>,
    debug_logging: Res<DebugLogging>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let left_panel_width = crate::ui::constants::display_layout::LEFT_SIDEBAR_WIDTH;
    let header_height = crate::ui::constants::display_layout::HEADER_HEIGHT;

    let is_cursor_over_left_panel =
        cursor_position.x <= left_panel_width && cursor_position.y >= header_height;

    if !is_cursor_over_left_panel {
        return;
    }

    for scroll_event in mouse_wheel_events.read() {
        crate::debug_println!(
            debug_logging,
            "Mouse wheel over left panel: y={}, unit={:?}",
            scroll_event.y,
            scroll_event.unit
        );

        let scroll_delta_in_pixels = match scroll_event.unit {
            MouseScrollUnit::Line => {
                scroll_event.y * crate::ui::constants::scroll_behavior::MOUSE_WHEEL_PIXELS_PER_LINE
            }
            MouseScrollUnit::Pixel => scroll_event.y,
        };

        for (mut scroll_position, node, computed_node) in &mut scrollable_query {
            if !matches!(node.overflow.y, OverflowAxis::Scroll) {
                continue;
            }

            let content_size = computed_node.content_size();
            let node_size = computed_node.size();
            let maximum_scroll_offset = (content_size.y - node_size.y).max(0.0);

            crate::debug_println!(
                debug_logging,
                "Scrolling left panel: content_size={:?}, node_size={:?}, max_offset={}, old_y={}, delta={}",
                content_size,
                node_size,
                maximum_scroll_offset,
                scroll_position.y,
                scroll_delta_in_pixels
            );

            scroll_position.y =
                (scroll_position.y - scroll_delta_in_pixels).clamp(0.0, maximum_scroll_offset);

            crate::debug_println!(debug_logging, "New scroll position: {}", scroll_position.y);

            break;
        }
    }
}
