use avian2d::prelude::*;
use bevy::{
    audio::{AudioPlugin, SpatialScale},
    prelude::*,
};
use bevy_turborand::prelude::RngPlugin;
use bevy_tweening::TweeningPlugin;
use derive_more::{Deref, DerefMut};

mod assets;
mod asteroid;
mod background;
mod constants;
mod levels;
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
    background::BackgroundPlugin,
    constants::AUDIO_SCALE,
    levels::GameLevelsPlugin,
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
pub struct GameLevel(String);

#[derive(PhysicsLayer)]
pub enum CollisionLayer {
    Player,
    Laser,
    Asteroids,
}

fn main() -> AppExit {
    let mut app = App::new();

    app.init_resource::<GameLevel>()
        .add_plugins(
            DefaultPlugins
                .set(AudioPlugin {
                    default_spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
                    ..default()
                })
                .set(WindowPlugin {
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
                }),
        )
        .add_plugins((RngPlugin::default(), TweeningPlugin));

    app.add_plugins((
        GameStatesPlugin,
        GameLevelsPlugin,
        GameAssetsPlugin,
        TweenCompletedPlugin,
        MovementPlugin,
        PlayerPlugin,
        ProjectilePlugin,
        AsteroidPlugin,
        UiPlugin,
        BackgroundPlugin,
    ));

    add_features(&mut app);
    app.run()
}

#[cfg(feature = "cmd_line")]
mod cmd_line;

#[allow(unused)]
fn add_features(app: &mut App) {
    #[cfg(feature = "cmd_line")]
    app.add_plugins(self::cmd_line::CmdLinePlugin);

    #[cfg(feature = "dbg_colliders")]
    app.add_plugins(PhysicsDebugPlugin::default());

    #[cfg(feature = "inspector")]
    app.add_plugins((
        bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
        //bevy_inspector_egui::quick::AssetInspectorPlugin::<assets::SpriteSheet>::default(),
        //ResourceInspectorPlugin::<AsteroidAssets>::default(),
    ));

    #[cfg(feature = "editor")]
    app.add_plugins(bevy_editor_pls::EditorPlugin::default());

    #[cfg(feature = "perf")]
    app.add_plugins((
        bevy::diagnostic::EntityCountDiagnosticsPlugin,
        bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        bevy::diagnostic::SystemInformationDiagnosticsPlugin,
        iyes_perf_ui::PerfUiPlugin,
    ));

    app.add_systems(
        Last,
        (
            bevy::dev_tools::states::log_transitions::<GameState>,
            bevy::dev_tools::states::log_transitions::<PlayState>,
        ),
    );
}
