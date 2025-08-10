mod game;
mod ui;
mod rendering;
mod input;
mod headless;

use bevy::prelude::*;
use core_sim::{influence_map::InfluenceMap, resources::{CurrentTurn, GameConfig, GameRng, WorldMap}};
use ai_planner::AICoordinator;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 && args[1] == "--headless" {
        // Run headless simulation for testing
        headless::run_headless_simulation();
    } else {
        // Run full Bevy application
        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Dominion Earth".to_string(),
                    resolution: (1200.0, 800.0).into(),
                    ..default()
                }),
                ..default()
            }))
            .add_plugins(bevy_egui::EguiPlugin)
            .init_resource::<CurrentTurn>()
            .init_resource::<GameConfig>()
            .init_resource::<GameRng>()
            .init_resource::<WorldMap>()
            .init_resource::<game::GameState>()  // Use our local GameState wrapper
            .init_resource::<InfluenceMap>()
            .add_systems(Startup, (
                setup_camera,
                game::setup_game,
            ))
            .add_systems(Update, (
                input::handle_input,
                input::handle_mouse_input,
                game::game_update_system,
                ui::ui_system,
                rendering::render_world,
            ))
            .run();
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
