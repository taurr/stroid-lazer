use avian2d::prelude::*;
use bevy::{
    audio::{AudioPlugin, SpatialScale},
    prelude::*,
};
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
    constants::AUDIO_SCALE,
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
        .insert_resource(ClearColor(bevy::color::palettes::css::BLACK.into()))
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
        GameAssetsPlugin,
        TweenCompletedPlugin,
        MovementPlugin,
        PlayerPlugin,
        ProjectilePlugin,
        AsteroidPlugin,
        UiPlugin,
    ));

    add_features(&mut app);
    app.run()
}

#[allow(unused)]
fn add_features(app: &mut App) {
    #[cfg(feature = "cmd_line")]
    app.insert_resource(<args::Args as clap::Parser>::parse());

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
