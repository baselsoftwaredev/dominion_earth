use crate::debug_println;
use crate::debug_utils::DebugLogging;
use crate::game::GameState;
use crate::production_input::SelectedCapital;
use crate::ui::resources::*;
use crate::ui::traits::*;
use bevy::prelude::*;
use bevy_hui::prelude::*;
use core_sim::{
    components::{Capital, MilitaryUnit},
    resources::{CurrentTurn, WorldMap},
    Civilization, Position, ProductionQueue,
};

/// Bevy HUI implementation of the UI system
pub struct BevyHuiSystem;

impl BevyHuiSystem {
    pub fn setup_plugins(app: &mut App) {
        app.add_plugins((HuiPlugin, HuiAutoLoadPlugin::new(&["ui"])))
            .add_systems(Startup, setup_main_ui)
            .add_systems(Update, update_ui_properties);
    }
}

impl UiSystem for BevyHuiSystem {
    fn initialize(&self, app: &mut App) {
        Self::setup_plugins(app);
    }

    fn render_main_game_panel(&self, _data: &GamePanelData) {
        // For bevy_hui, rendering is handled by the component system
    }

    fn render_production_menu(&self, _data: &ProductionMenuData) {
        // For bevy_hui, rendering is handled by the component system
    }

    fn render_statistics_panel(&self, _data: &StatisticsPanelData) {
        // For bevy_hui, rendering is handled by the component system
    }

    fn render_tile_info(&self, _data: &TileInfoData) {
        // For bevy_hui, rendering is handled by the component system
    }

    fn render_minimap(&self, _data: &MinimapData) {
        // For bevy_hui, rendering is handled by the component system
    }

    fn render_resources(&self, _data: &ResourcesData) {
        // For bevy_hui, rendering is handled by the component system
    }
}

/// Component that holds a template handle for bevy_hui elements
#[derive(Component)]
pub struct HuiComponent {
    pub template: Handle<HtmlTemplate>,
}

impl UiComponent for HuiComponent {
    type Id = Entity;

    fn get_id(&self) -> Self::Id {
        Entity::from_raw(0) // This would be set when spawned
    }

    fn update_data(&mut self, _data: Box<dyn std::any::Any + Send + Sync>) {
        // Update template properties based on data
    }
}

/// Setup main UI with proper component registration
fn setup_main_ui(
    mut cmd: Commands,
    server: Res<AssetServer>,
    mut html_comps: HtmlComponents,
    mut html_funcs: HtmlFunctions,
) {
    // Register function bindings for dynamic updates (following bevy_hui example pattern)
    html_funcs.register(
        "update_player_gold",
        |In(entity): In<Entity>,
         mut cmd: Commands,
         mut template_props: Query<&mut TemplateProperties>,
         scopes: Query<&TemplateScope>,
         player_civs: Query<&Civilization, With<core_sim::PlayerControlled>>| {
            let Ok(scope) = scopes.get(entity) else {
                return;
            };
            let Ok(mut props) = template_props.get_mut(**scope) else {
                return;
            };

            // Get latest player gold
            let player_gold = player_civs
                .iter()
                .next()
                .map(|civ| civ.economy.gold as i32)
                .unwrap_or(0);

            props.insert("player_gold".to_string(), player_gold.to_string());
            cmd.trigger_targets(CompileContextEvent, **scope);
        },
    );

    html_funcs.register(
        "update_turn",
        |In(entity): In<Entity>,
         mut cmd: Commands,
         mut template_props: Query<&mut TemplateProperties>,
         scopes: Query<&TemplateScope>,
         current_turn: Res<CurrentTurn>| {
            let Ok(scope) = scopes.get(entity) else {
                return;
            };
            let Ok(mut props) = template_props.get_mut(**scope) else {
                return;
            };

            props.insert("current_turn".to_string(), current_turn.0.to_string());
            cmd.trigger_targets(CompileContextEvent, **scope);
        },
    );

    // Spawn main UI layout
    cmd.spawn((
        HtmlNode(server.load("ui/main_layout.html")),
        Name::new("MainUI"),
        TemplateProperties::default()
            .with("game_title", "Dominion Earth")
            .with("current_turn", "1")
            .with("player_gold", "0")
            .with("player_production", "0")
            .with("player_cities", "0")
            .with("capital_names", "No capital")
            .with("terrain_land_count", "0")
            .with("terrain_water_count", "0")
            .with("terrain_mountain_count", "0")
            .with("selected_position", "None")
            .with("selected_terrain", "None")
            .with("civilizations_list", "Loading...")
            .with("show_production_menu", "none")
            .with("capital_name", "Unknown Capital")
            .with("civilization_name", "Unknown Civilization")
            .with("civilization_gold", "0")
            .with("civilization_production", "0")
            .with("current_production_name", "None")
            .with("current_production_progress", "0")
            .with("production_queue_length", "0"),
    ));

    // Register custom components first
    html_comps.register("header", server.load("ui/header.html"));
    html_comps.register("game_panel", server.load("ui/game_panel.html"));
    html_comps.register("statistics_panel", server.load("ui/statistics_panel.html"));
    html_comps.register("production_menu", server.load("ui/production_menu.html"));
    html_comps.register("tile_info", server.load("ui/tile_info.html"));
    html_comps.register(
        "civilizations_list",
        server.load("ui/civilizations_list.html"),
    );
    html_comps.register("minimap", server.load("ui/minimap.html"));
    html_comps.register("resources_panel", server.load("ui/resources_panel.html"));
}

/// Update UI properties with current game data
fn update_ui_properties(
    current_turn: Res<CurrentTurn>,
    terrain_counts: Res<TerrainCounts>,
    selected_tile: Res<SelectedTile>,
    selected_capital: Res<SelectedCapital>,
    debug_logging: Res<DebugLogging>,
    civs: Query<&Civilization>,
    player_civs: Query<&Civilization, With<core_sim::PlayerControlled>>,
    capitals: Query<(&Capital, &Position)>,
    cities: Query<(&core_sim::City, &Position)>,
    production_queues: Query<&ProductionQueue>,
    mut ui_nodes: Query<(Entity, &mut TemplateProperties), With<HtmlNode>>,
    mut cmd: Commands,
) {
    // Only update when resources change or when production menu state changes
    if current_turn.is_changed()
        || terrain_counts.is_changed()
        || selected_tile.is_changed()
        || selected_capital.is_changed()
    {
        // Collect game data
        let all_civs: Vec<&Civilization> = civs.iter().collect();
        let player_civilization_list: Vec<&Civilization> = player_civs.iter().collect();
        let capital_list: Vec<(&Capital, &Position)> = capitals.iter().collect();
        let city_list: Vec<(&core_sim::City, &Position)> = cities.iter().collect();

        debug_println!(
            debug_logging,
            "UI UPDATE: Found {} civilizations, {} player civs, {} capitals, {} cities",
            all_civs.len(),
            player_civilization_list.len(),
            capital_list.len(),
            city_list.len()
        );

        // Calculate player stats
        let player_gold = player_civilization_list
            .first()
            .map(|civ| civ.economy.gold as i32)
            .unwrap_or(0);

        let total_production: f32 = production_queues
            .iter()
            .map(|queue| queue.accumulated_production)
            .sum();

        // Production menu logic
        let (
            show_production_menu,
            capital_name,
            civilization_name,
            civilization_gold,
            civilization_production,
            current_production_name,
            current_production_progress,
            production_queue_length,
        ) = if selected_capital.show_production_menu {
            if let (Some(capital_entity), Some(civ_entity)) =
                (selected_capital.capital_entity, selected_capital.civ_entity)
            {
                // Get capital information
                let capital_name = format!("Capital"); // TODO: Get actual capital name

                // Get civilization information
                let (civ_name, civ_gold, civ_production) = if let Ok(civ) = civs.get(civ_entity) {
                    (
                        civ.name.clone(),
                        civ.economy.gold as i32,
                        civ.economy.production as i32,
                    )
                } else {
                    ("Unknown".to_string(), 0, 0)
                };

                // Get production queue information
                let (current_production, progress, queue_length) =
                    if let Ok(production_queue) = production_queues.get(capital_entity) {
                        let current =
                            if let Some(ref current_item) = production_queue.current_production {
                                current_item.name().to_string()
                            } else {
                                "None".to_string()
                            };
                        let progress = (production_queue.get_progress_percentage() * 100.0) as i32;
                        let queue_len = production_queue.queue_length();
                        (current, progress, queue_len)
                    } else {
                        ("None".to_string(), 0, 0)
                    };

                (
                    "flex".to_string(),
                    capital_name,
                    civ_name,
                    civ_gold,
                    civ_production,
                    current_production,
                    progress,
                    queue_length,
                )
            } else {
                (
                    "none".to_string(),
                    "Unknown Capital".to_string(),
                    "Unknown Civilization".to_string(),
                    0,
                    0,
                    "None".to_string(),
                    0,
                    0,
                )
            }
        } else {
            (
                "none".to_string(),
                "Unknown Capital".to_string(),
                "Unknown Civilization".to_string(),
                0,
                0,
                "None".to_string(),
                0,
                0,
            )
        };

        // Format capital and city names
        let capital_names = if capital_list.is_empty() && city_list.is_empty() {
            "No capitals founded".to_string()
        } else {
            let mut names = Vec::new();

            // Add capital information
            for (capital, pos) in &capital_list {
                let civ_name = all_civs
                    .iter()
                    .find(|civ| civ.id == capital.owner)
                    .map(|civ| civ.name.as_str())
                    .unwrap_or("Unknown");
                names.push(format!("{} Capital at ({}, {})", civ_name, pos.x, pos.y));
            }

            // Add city information
            for (city, pos) in &city_list {
                names.push(format!("{} at ({}, {})", city.name, pos.x, pos.y));
            }

            if names.is_empty() {
                "No cities founded".to_string()
            } else {
                names.join(", ")
            }
        };

        // Format civilization details
        let civ_details = if all_civs.is_empty() {
            "No civilizations".to_string()
        } else {
            all_civs
                .iter()
                .enumerate()
                .map(|(i, civ)| {
                    let civ_type = if player_civs.iter().any(|pc| pc.id == civ.id) {
                        "Player"
                    } else {
                        "AI"
                    };
                    format!(
                        "{} - {} (Gold: {})",
                        civ.name, civ_type, civ.economy.gold as i32
                    )
                })
                .collect::<Vec<_>>()
                .join(", ")
        };

        for (entity, mut properties) in ui_nodes.iter_mut() {
            // Update all existing properties
            properties.insert("current_turn".to_string(), current_turn.0.to_string());
            properties.insert("player_gold".to_string(), player_gold.to_string());
            properties.insert(
                "player_production".to_string(),
                (total_production as i32).to_string(),
            );
            properties.insert(
                "total_production".to_string(),
                (total_production as i32).to_string(),
            );
            properties.insert(
                "player_cities".to_string(),
                (capital_list.len() + city_list.len()).to_string(),
            );
            properties.insert(
                "capital_count".to_string(),
                (capital_list.len() + city_list.len()).to_string(),
            );
            properties.insert("capital_names".to_string(), capital_names.clone());

            // World statistics
            properties.insert(
                "terrain_land_count".to_string(),
                (terrain_counts.plains
                    + terrain_counts.hills
                    + terrain_counts.forest
                    + terrain_counts.desert)
                    .to_string(),
            );
            properties.insert(
                "terrain_water_count".to_string(),
                (terrain_counts.ocean + terrain_counts.coast + terrain_counts.river).to_string(),
            );
            properties.insert(
                "terrain_mountain_count".to_string(),
                terrain_counts.mountains.to_string(),
            );

            // Civilizations list
            properties.insert("civilizations_list".to_string(), civ_details.clone());

            // Update selected tile info
            if let Some(pos) = selected_tile.position {
                properties.insert(
                    "selected_position".to_string(),
                    format!("({}, {})", pos.x, pos.y),
                );
                properties.insert("selected_terrain".to_string(), "Unknown".to_string());
            } else {
                properties.insert("selected_position".to_string(), "None".to_string());
                properties.insert("selected_terrain".to_string(), "None".to_string());
            }

            // Update production menu properties
            properties.insert(
                "show_production_menu".to_string(),
                show_production_menu.clone(),
            );
            properties.insert("capital_name".to_string(), capital_name.clone());
            properties.insert("civilization_name".to_string(), civilization_name.clone());
            properties.insert(
                "civilization_gold".to_string(),
                civilization_gold.to_string(),
            );
            properties.insert(
                "civilization_production".to_string(),
                civilization_production.to_string(),
            );
            properties.insert(
                "current_production_name".to_string(),
                current_production_name.clone(),
            );
            properties.insert(
                "current_production_progress".to_string(),
                current_production_progress.to_string(),
            );
            properties.insert(
                "production_queue_length".to_string(),
                production_queue_length.to_string(),
            );

            // Trigger recompilation - this is the key part from the bevy_hui example!
            cmd.trigger_targets(CompileContextEvent, entity);
        }
    }
}
