mod game;
mod headless;
mod input;
mod rendering;
mod ui;
mod unit_assets;

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
}

fn main() {
    let cli = Cli::parse();

    if cli.headless {
        // Run headless simulation for testing
        headless::run_headless_simulation();
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
        .add_plugins(bevy_egui::EguiPlugin::default());

        // Conditionally add remote protocol plugins
        if cli.enable_remote {
            println!("Enabling Bevy Remote Protocol on port {}", cli.remote_port);
            app.add_plugins(BrpExtrasPlugin::with_port(cli.remote_port));
        }

        app.init_resource::<ui::TerrainCounts>()
            .init_resource::<CurrentTurn>()
            .init_resource::<ActiveCivTurn>()
            .init_resource::<GameConfig>()
            .init_resource::<GameRng>()
            .init_resource::<WorldMap>()
            .insert_resource(game::GameState::with_auto_advance(cli.auto_advance))
            .init_resource::<core_sim::resources::TurnAdvanceRequest>()
            .init_resource::<InfluenceMap>()
            .add_systems(
                Startup,
                (
                    setup_camera,
                    rendering::setup_tile_assets,
                    unit_assets::setup_unit_assets,
                    game::setup_game,
                    rendering::spawn_world_tiles.after(game::setup_game),
                ),
            )
            .add_systems(
                Update,
                (
                    input::handle_input,
                    input::handle_mouse_input,
                    game::game_update_system,
                    core_sim::systems::turn_based_system,
                    rendering::spawn_unit_sprites,
                    rendering::update_unit_sprites,
                    rendering::spawn_capital_sprites,
                    ui::update_terrain_counts,
                    rendering::render_world_overlays,
                ),
            )
            .add_systems(bevy_egui::EguiPrimaryContextPass, ui::ui_system);

        app.run();
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
