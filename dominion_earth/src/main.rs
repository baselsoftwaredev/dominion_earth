mod constants;
mod debug_utils;
mod game;
mod input;
mod rendering;
mod ui;
pub mod unit_assets;

use crate::constants::rendering::camera as camera_constants;
use crate::constants::{network, window};
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use bevy_brp_extras::BrpExtrasPlugin;
use bevy_framepace::FramepacePlugin;
use clap::Parser;
use core_sim::{
    influence_map::InfluenceMap,
    resources::{ActiveCivTurn, CurrentTurn, GameConfig, GameRng, WorldMap},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Enable auto-advance (AI turns run automatically)
    #[arg(long, default_value_t = false)]
    auto_advance: bool,

    /// Enable Bevy Remote Protocol for external tool access
    #[arg(long, default_value_t = false)]
    enable_remote: bool,

    /// Remote protocol port (default: 15702)
    #[arg(long, default_value_t = network::DEFAULT_REMOTE_PORT)]
    remote_port: u16,

    /// Random seed for world generation (default: current timestamp)
    #[arg(long)]
    seed: Option<u64>,

    /// Enable debug logging for coast generation and tile neighbors
    #[arg(long, default_value_t = false)]
    debug_logging: bool,
}

fn main() {
    let cli = Cli::parse();

    // Run full Bevy application
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: window::TITLE.to_string(),
            resolution: (window::DEFAULT_WIDTH, window::DEFAULT_HEIGHT).into(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(bevy_egui::EguiPlugin::default())
    .add_plugins(bevy_ecs_tilemap::TilemapPlugin)
    .add_plugins(FramepacePlugin);

    if cli.enable_remote {
        println!("Enabling Bevy Remote Protocol on port {}", cli.remote_port);
        app.add_plugins(BrpExtrasPlugin::with_port(cli.remote_port));
    }

    app.init_resource::<ui::TerrainCounts>()
        .init_resource::<ui::SelectedTile>()
        .init_resource::<CurrentTurn>()
        .init_resource::<ActiveCivTurn>()
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource({
            let mut config = GameConfig::default();
            if let Some(seed) = cli.seed {
                config.random_seed = seed;
                println!("Using custom random seed: {}", seed);
            }
            config.debug_logging = cli.debug_logging;
            config
        })
        .init_resource::<GameRng>()
        .init_resource::<WorldMap>()
        .insert_resource(game::GameState::with_auto_advance(cli.auto_advance))
        .insert_resource(debug_utils::DebugLogging(cli.debug_logging))
        .init_resource::<ui::SelectedTile>()
        .init_resource::<ui::LastLoggedTile>()
        .init_resource::<ui::TerrainCounts>()
        .init_resource::<core_sim::resources::TurnAdvanceRequest>()
        .init_resource::<InfluenceMap>()
        .add_systems(
            Startup,
            (
                setup_camera,
                core_sim::tile::tile_assets::setup_tile_assets,
                unit_assets::setup_unit_assets,
                game::setup_game,
                rendering::spawn_world_tiles
                    .after(core_sim::tile::tile_assets::setup_tile_assets)
                    .after(game::setup_game),
                rendering::spawn_unit_sprites.after(rendering::spawn_world_tiles),
                rendering::spawn_animated_capital_tiles.after(rendering::spawn_world_tiles),
            ),
        )
        .add_systems(
            Update,
            (
                input::handle_input,
                input::handle_mouse_input,
                input::select_tile_on_click,
                game::game_update_system,
                core_sim::systems::turn_based_system,
                core_sim::systems::capital_evolution_system,
                rendering::update_unit_sprites,
                rendering::update_capital_sprites,
                rendering::update_animated_capital_sprites,
                rendering::render_civilization_borders,
            ),
        )
        .add_systems(bevy_egui::EguiPrimaryContextPass, ui::ui_system);

    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d).insert(Transform::from_xyz(
        camera_constants::INITIAL_CAMERA_X,
        camera_constants::INITIAL_CAMERA_Y,
        camera_constants::INITIAL_CAMERA_Z,
    ));
}
