mod game;
mod headless;
mod input;
mod rendering;
mod ui;
pub mod unit_assets;

use bevy::prelude::*;
use bevy_brp_extras::BrpExtrasPlugin;
use clap::Parser;
use core_sim::{
    influence_map::InfluenceMap,
    resources::{ActiveCivTurn, CurrentTurn, GameConfig, GameRng, WorldMap},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Run in headless mode (no graphics)
    #[arg(long)]
    headless: bool,

    /// Enable auto-advance (AI turns run automatically)
    #[arg(long, default_value_t = false)]
    auto_advance: bool,

    /// Enable Bevy Remote Protocol for external tool access
    #[arg(long, default_value_t = false)]
    enable_remote: bool,

    /// Remote protocol port (default: 15702)
    #[arg(long, default_value_t = 15702)]
    remote_port: u16,

    /// Random seed for world generation (default: current timestamp)
    #[arg(long)]
    seed: Option<u64>,
}

fn main() {
    let cli = Cli::parse();

    if cli.headless {
        // Run headless simulation for testing
        headless::run_headless_simulation(cli.seed);
    } else {
        // Run full Bevy application
        let mut app = App::new();

        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dominion Earth".to_string(),
                resolution: (1200.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(bevy_egui::EguiPlugin::default())
        .add_plugins(bevy_ecs_tilemap::TilemapPlugin);

        // Conditionally add remote protocol plugins
        if cli.enable_remote {
            println!("Enabling Bevy Remote Protocol on port {}", cli.remote_port);
            app.add_plugins(BrpExtrasPlugin::with_port(cli.remote_port));
        }

        app.init_resource::<ui::TerrainCounts>()
            .init_resource::<ui::SelectedTile>()
            .init_resource::<CurrentTurn>()
            .init_resource::<ActiveCivTurn>()
            .insert_resource({
                let mut config = GameConfig::default();
                if let Some(seed) = cli.seed {
                    config.random_seed = seed;
                    println!("Using custom random seed: {}", seed);
                }
                config
            })
            .init_resource::<GameRng>()
            .init_resource::<WorldMap>()
            .insert_resource(game::GameState::with_auto_advance(cli.auto_advance))
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
                    rendering::spawn_capital_sprites.after(rendering::spawn_world_tiles),
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
                    rendering::update_unit_sprites,
                    // Removed: ui::update_terrain_counts,
                    // Removed: rendering::render_world_overlays,
                ),
            )
            .add_systems(bevy_egui::EguiPrimaryContextPass, ui::ui_system);

        app.run();
    }
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera2d)
        .insert(Transform::from_xyz(1600.0, 800.0, 0.0));
}
