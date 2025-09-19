mod constants;
mod debug_utils;
mod game;
mod input;
mod plugins;
mod production_input;
mod rendering;
mod ui;
mod unit_assets;

use crate::constants::{network, window};
use crate::plugins::{resources::ResourceConfig, DominionEarthPlugins};
use bevy::prelude::*;
use bevy::window::MonitorSelection;
use bevy_brp_extras::BrpExtrasPlugin;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(long)]
    seed: Option<u64>,
    #[arg(long)]
    debug_logging: bool,
}

fn main() {
    let args = CliArgs::parse();

    let config = ResourceConfig {
        auto_advance: false,
        ai_only: false,
        total_civs: 3,
        seed: args.seed,
        debug_logging: args.debug_logging,
    };

    let window_mode = if args.debug_logging {
        bevy::window::WindowMode::Windowed
    } else {
        bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current)
    };

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: window::TITLE.to_string(),
                resolution: (window::DEFAULT_WIDTH, window::DEFAULT_HEIGHT).into(),
                mode: window_mode,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(BrpExtrasPlugin::default())
        .add_plugins(DominionEarthPlugins::with_config(config))
        .run();
}
