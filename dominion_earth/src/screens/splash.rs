//! A splash screen that plays briefly at startup.

use bevy::prelude::*;

use crate::screens::Screen;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Splash), spawn_splash_screen);

    // Add splash timer
    app.add_systems(OnEnter(Screen::Splash), insert_splash_timer);
    app.add_systems(OnExit(Screen::Splash), remove_splash_timer);
    app.add_systems(
        Update,
        (tick_splash_timer, check_splash_timer).run_if(in_state(Screen::Splash)),
    );
}

const SPLASH_DURATION_SECS: f32 = 2.0;

fn spawn_splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("Splash Screen"),
            StateScoped(Screen::Splash),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            GlobalZIndex(100),
        ))
        .with_child((
            Name::new("Splash Text"),
            Text::new("Dominion Earth"),
            TextFont {
                font_size: 80.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
}

#[derive(Resource, Debug, Clone, PartialEq, Reflect)]
#[reflect(Resource)]
struct SplashTimer(Timer);

impl Default for SplashTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(SPLASH_DURATION_SECS, TimerMode::Once))
    }
}

fn insert_splash_timer(mut commands: Commands) {
    commands.insert_resource(SplashTimer::default());
}

fn remove_splash_timer(mut commands: Commands) {
    commands.remove_resource::<SplashTimer>();
}

fn tick_splash_timer(time: Res<Time>, mut timer: ResMut<SplashTimer>) {
    timer.0.tick(time.delta());
}

fn check_splash_timer(timer: Res<SplashTimer>, mut next_screen: ResMut<NextState<Screen>>) {
    if timer.0.finished() {
        next_screen.set(Screen::MainMenu);
    }
}
