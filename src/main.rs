use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_turborand::prelude::RngPlugin;
use bevy_tweening::TweeningPlugin;
use derive_more::{Deref, DerefMut};

#[cfg(feature = "cmd_line")]
mod args;

mod assets;
mod asteroid;
mod constants;
mod movement;
mod player;
mod projectile;
mod states;
mod tween_events;
mod ui;
mod utils;

use self::{
    assets::GameAssetsPlugin,
    asteroid::AsteroidPlugin,
    movement::MovementPlugin,
    player::PlayerPlugin,
    projectile::ProjectilePlugin,
    states::{GameOverReason, GameState, GameStatesPlugin, PlayState},
    tween_events::TweenCompletedPlugin,
    ui::UiPlugin,
};

#[derive(Component, Debug)]
struct PlayingField;

#[derive(Resource, Debug, Default, Clone, Deref, DerefMut)]
pub struct GameLevel {
    #[deref]
    #[deref_mut]
    pub current: String,
}

#[derive(PhysicsLayer)]
pub enum CollisionLayer {
    Player,
    Laser,
    Asteroids,
}

fn print_window_stats(
    query: Query<&Window, With<PrimaryWindow>>,
    state: Res<State<PlayState>>,
    mut next_state: ResMut<NextState<PlayState>>,
) {
    if let Ok(window) = query.get_single() {
        match (state.get(), window.focused) {
            (PlayState::CountdownBeforeRunning, false) => {
                next_state.set(PlayState::Paused);
            }
            (PlayState::Running, false) => {
                next_state.set(PlayState::Paused);
            }
            (_, _) => {}
        }
    }
}

fn main() -> AppExit {
    let mut app = App::new();

    app.init_resource::<GameLevel>()
        .insert_resource(ClearColor(bevy::color::palettes::css::BLACK.into()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Stroid-Lazer".into(),
                resolution: Vec2::new(800., 800.).into(),
                present_mode: bevy::window::PresentMode::Fifo,
                resize_constraints: WindowResizeConstraints {
                    min_width: 550.0,
                    min_height: 400.0,
                    ..Default::default()
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugins((RngPlugin::default(), TweeningPlugin));

    app.add_plugins((
        GameStatesPlugin,
        GameAssetsPlugin,
        TweenCompletedPlugin,
        MovementPlugin,
        PlayerPlugin,
        ProjectilePlugin,
        AsteroidPlugin,
        UiPlugin,
    ));

    app.add_systems(
        Update,
        print_window_stats.run_if(in_state(GameState::Playing)),
    );

    add_features(&mut app);
    app.run()
}

#[allow(unused)]
fn add_features(app: &mut App) {
    #[cfg(feature = "cmd_line")]
    app.insert_resource(<args::Args as clap::Parser>::parse());

    app.add_systems(
        Last,
        (
            bevy::dev_tools::states::log_transitions::<GameState>,
            bevy::dev_tools::states::log_transitions::<PlayState>,
        ),
    );

    #[cfg(feature = "inspector")]
    app.add_plugins((
        bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
        //bevy_inspector_egui::quick::AssetInspectorPlugin::<assets::SpriteSheet>::default(),
        //ResourceInspectorPlugin::<AsteroidAssets>::default(),
    ));

    #[cfg(feature = "dbg_colliders")]
    app.add_plugins(PhysicsDebugPlugin::default());

    #[cfg(feature = "editor")]
    app.add_plugins(bevy_editor_pls::EditorPlugin::default());

    #[cfg(feature = "perf")]
    app.add_plugins((
        bevy::diagnostic::EntityCountDiagnosticsPlugin,
        bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        bevy::diagnostic::SystemInformationDiagnosticsPlugin,
        iyes_perf_ui::PerfUiPlugin,
    ));
}
