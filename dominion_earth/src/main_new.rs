use crate::constants::{network, window};
use crate::plugins::{DominionEarthPlugins, resources::ResourceConfig};
use bevy::prelude::*;
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

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: window::TITLE.to_string(),
                resolution: (window::DEFAULT_WIDTH, window::DEFAULT_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(BrpExtrasPlugin::default().with_bevy_remote_plugin(
            bevy::remote::RemotePlugin {
                address: format!("127.0.0.1:{}", network::DEFAULT_REMOTE_PORT).parse().unwrap(),
                methods: vec![],
            },
        ))
        .insert_resource(ResourceConfig {
            auto_advance: false,
            ai_only: false,
            total_civs: 3,
            seed: args.seed,
            debug_logging: args.debug_logging,
        })
        .add_plugins(DominionEarthPlugins)
        .run();
}
