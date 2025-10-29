mod ai_decision_systems;
mod audio;
mod civilization_spawning;
mod constants;
mod debug_utils;
mod entity_utils;
mod game;
mod input;
mod menus;
mod plugins;
mod production_input;
mod rendering;
mod screens;
mod settings;
mod theme;
mod ui;
mod unit_assets;

use crate::constants::{network, window};
use crate::plugins::{resources::ResourceConfig, DominionEarthPlugins};
use bevy::prelude::*;
use bevy::window::MonitorSelection;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(long)]
    seed: Option<u64>,
    #[arg(long)]
    debug_logging: bool,
    #[arg(
        long,
        help = "Start game with AI-only civilizations - no player controlled civilization"
    )]
    ai_only: bool,
}

fn main() {
    let args = CliArgs::parse();

    let config = ResourceConfig {
        auto_advance: false,
        ai_only: args.ai_only,
        total_civs: 3,
        seed: args.seed,
        debug_logging: args.debug_logging,
    };

    let window_mode = if args.debug_logging {
        bevy::window::WindowMode::Windowed
    } else {
        bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current)
    };

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: window::TITLE.to_string(),
            resolution: (window::DEFAULT_WIDTH as u32, window::DEFAULT_HEIGHT as u32).into(),
            mode: window_mode,
            ..default()
        }),
        ..default()
    }));

    let plugins = DominionEarthPlugins::with_config(config);

    app.add_plugins(plugins).run();
}
