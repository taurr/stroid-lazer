use self::prelude::*;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_asset_loader::prelude::*;

mod asteroid;
mod controls;
mod movement;
mod player;
mod prelude;
mod projectile;
mod settings;

#[derive(Debug, Clone, SystemSet, PartialEq, Eq, Hash)]
enum GameSet {
    PreSpawn,
    Spawn,
    Input,
    Movement,
    Cleanup,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    AssetLoading,
    //Menu,
    Playing,
    //GameOver,
}

#[derive(Debug, Component)]
pub struct PlayingField;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.1)))
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Stroid-Lazer".into(),
                resolution: Vec2::new(800., 800.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Playing),
        )
        .add_plugins((
            LevelPlugin,
            MovementPlugin,
            ControlsPlugin,
            PlayerPlugin,
            ProjectilePlugin,
            AsteroidPlugin,
        ))
        .add_systems(OnExit(GameState::AssetLoading), setup_playing_field)
        .add_systems(Startup, setup_camera);

    #[cfg(feature = "editor")]
    app.add_plugins(bevy_editor_pls::EditorPlugin::default());

    #[cfg(feature = "perf")]
    app.add_plugins((
        bevy::diagnostic::EntityCountDiagnosticsPlugin,
        bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        bevy::diagnostic::SystemInformationDiagnosticsPlugin,
        iyes_perf_ui::PerfUiPlugin,
    ));

    app.configure_sets(StateTransition, GameSet::PreSpawn.before(GameSet::Spawn));
    app.configure_sets(Update, GameSet::PreSpawn.before(GameSet::Spawn));
    app.configure_sets(Update, GameSet::Input.before(GameSet::Movement));
    app.configure_sets(Update, GameSet::Cleanup.after(GameSet::Movement));

    app.run();
}

fn setup_camera(mut commands: Commands) {
    debug!("setting up camera");

    #[cfg(feature = "perf")]
    commands.spawn(iyes_perf_ui::PerfUiCompleteBundle::default());

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: -100.0,
            far: 100.0,
            ..default()
        },
        ..default()
    });
}

fn setup_playing_field(
    settings: Res<GameSettings>,
    mut projection_query: Query<&mut OrthographicProjection, With<Camera2d>>,
    mut commands: Commands,
) {
    debug!("setting up playing field");
    commands.spawn((
        Name::new("PlayingField"),
        PlayingField,
        settings.game_area.clone(),
        SpatialBundle::default(),
    ));

    let mut projection = projection_query.single_mut();
    projection.scaling_mode = ScalingMode::AutoMin {
        min_width: settings.game_area.width(),
        min_height: settings.game_area.height(),
    };
}

pub fn despawn_with<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
