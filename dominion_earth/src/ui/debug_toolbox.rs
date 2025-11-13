/// Debug toolbox UI - only enabled in debug builds
/// Provides development utilities as an egui window
///
/// Based on bevy-inspector-egui patterns for proper egui context handling
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPrimaryContextPass, PrimaryEguiContext};
use bevy_inspector_egui::bevy_inspector;
use egui::Color32;

#[derive(Resource, Default)]
pub struct DebugToolboxState {
    pub is_visible: bool,
    pub sprite_search_query: String,
    pub selected_entity: Option<Entity>,
}

pub struct DebugToolboxPlugin;

impl Plugin for DebugToolboxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugToolboxState>()
            .add_systems(Startup, spawn_debug_toolbox)
            .add_systems(
                Update,
                (toggle_debug_toolbox, auto_name_capitals_and_cities),
            )
            .add_systems(EguiPrimaryContextPass, update_debug_toolbox);
    }
}

pub fn spawn_debug_toolbox(mut debug_state: ResMut<DebugToolboxState>) {
    debug_state.is_visible = true;
}

pub fn update_debug_toolbox(world: &mut World) {
    let debug_state = world.resource::<DebugToolboxState>();
    if !debug_state.is_visible {
        return;
    }

    let egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world);

    let Ok(egui_context) = egui_context else {
        return;
    };

    let mut egui_context = egui_context.clone();

    egui::Window::new("Debug Toolbox")
        .default_open(true)
        .resizable(true)
        .collapsible(true)
        .show(egui_context.get_mut(), |ui| {
            // Title
            ui.heading("Development Tools");
            ui.separator();

            // Entity/Sprite Browser Section
            ui.heading("Entity Browser");

            // Get search query and selected entity
            let search_query = {
                let debug_state = world.resource::<DebugToolboxState>();
                debug_state.sprite_search_query.clone()
            };

            ui.horizontal(|ui| {
                ui.label("Search by name:");
                let mut debug_state = world.resource_mut::<DebugToolboxState>();
                ui.text_edit_singleline(&mut debug_state.sprite_search_query);
            });

            // Collect ALL named entities (not just sprites)
            let mut found_entities: Vec<(Entity, String)> = Vec::new();
            {
                let mut query = world.query_filtered::<(Entity, &Name), ()>();
                for (entity, name) in query.iter(world) {
                    let name_str = name.as_str();
                    if search_query.is_empty()
                        || name_str
                            .to_lowercase()
                            .contains(&search_query.to_lowercase())
                    {
                        found_entities.push((entity, name_str.to_string()));
                    }
                }
            }

            // Display found entities
            ui.label(format!("Found: {} entities", found_entities.len()));
            egui::ScrollArea::vertical()
                .max_height(200.0)
                .show(ui, |ui| {
                    if found_entities.is_empty() {
                        ui.label("No entities found. Search for 'capital' or type a name");
                    } else {
                        let mut debug_state = world.resource_mut::<DebugToolboxState>();
                        for (entity, name) in &found_entities {
                            let is_selected = debug_state.selected_entity == Some(*entity);
                            ui.push_id(entity, |ui| {
                                if ui.selectable_label(is_selected, name).clicked() {
                                    debug_state.selected_entity = Some(*entity);
                                }
                            });
                        }
                    }
                });

            ui.separator();

            // Selected Entity Inspector
            let selected_entity = world.resource::<DebugToolboxState>().selected_entity;
            if let Some(selected_entity) = selected_entity {
                ui.heading("Entity Inspector");
                ui.push_id(selected_entity, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(300.0)
                        .show(ui, |ui| {
                            bevy_inspector::ui_for_entity(world, selected_entity, ui);
                        });
                });
            }

            ui.separator();
            if ui.button("Close Toolbox").clicked() {
                let mut debug_state = world.resource_mut::<DebugToolboxState>();
                debug_state.is_visible = false;
            }
        });
}

pub fn toggle_debug_toolbox(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_state: ResMut<DebugToolboxState>,
) {
    // Toggle toolbox with Cmd+D on macOS or Ctrl+D on other platforms
    #[cfg(target_os = "macos")]
    let is_modifier_pressed =
        keyboard.pressed(KeyCode::SuperLeft) || keyboard.pressed(KeyCode::SuperRight);

    #[cfg(not(target_os = "macos"))]
    let is_modifier_pressed =
        keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);

    if is_modifier_pressed && keyboard.just_pressed(KeyCode::KeyD) {
        debug_state.is_visible = !debug_state.is_visible;
    }
}

/// System to auto-name entities with Capital, City, or Unit components for easier inspection
pub fn auto_name_capitals_and_cities(
    mut commands: Commands,
    capitals_without_name: Query<
        (
            Entity,
            &core_sim::components::city::Capital,
            &core_sim::Position,
        ),
        Without<Name>,
    >,
    cities_without_name: Query<
        (
            Entity,
            &core_sim::components::city::City,
            &core_sim::Position,
        ),
        (Without<Name>, Without<core_sim::components::city::Capital>),
    >,
    units_without_name: Query<
        (
            Entity,
            &core_sim::components::military::MilitaryUnit,
            &core_sim::Position,
        ),
        (
            Without<Name>,
            Without<core_sim::components::city::Capital>,
            Without<core_sim::components::city::City>,
        ),
    >,
) {
    for (entity, capital, pos) in capitals_without_name.iter() {
        let name = format!("Capital_{}_({},{})", capital.owner.0, pos.x, pos.y);
        commands.entity(entity).insert(Name::new(name));
    }

    for (entity, city, pos) in cities_without_name.iter() {
        let name = format!("{}({},{})", city.name, pos.x, pos.y);
        commands.entity(entity).insert(Name::new(name));
    }

    for (entity, unit, pos) in units_without_name.iter() {
        let name = format!(
            "{:?}_{}_({},{})",
            unit.unit_type, unit.owner.0, pos.x, pos.y
        );
        commands.entity(entity).insert(Name::new(name));
    }
}
