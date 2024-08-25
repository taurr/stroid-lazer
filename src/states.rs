use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_asset_loader::prelude::*;
use bevy_turborand::{GlobalRng, RngComponent};
use strum_macros::EnumIter;

use crate::{
    assets::GameAreaSettings,
    background::Background,
    constants::{
        PLAYINGFIELD_BACKGROUND_RELATIVE_Z_POS, PLAYINGFIELD_BORDER_RELATIVE_Z_POS,
        PLAYINGFIELD_POS,
    },
    PlayingField,
};

#[cfg_attr(doc, aquamarine::aquamarine)]
/// General states of the game.
///
/// include_mmd!("docs/game-state.mmd")
///
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum GameState {
    #[default]
    LoadingAssets,
    #[cfg(feature = "dbg_colliders")]
    DebugColliders,
    MainMenu,
    HighscoreMenu,
    Playing,
}

#[cfg_attr(doc, aquamarine::aquamarine)]
/// States during actual gameplay ([GameState::Playing]).
///
/// include_mmd!("docs/play-state.mmd")
///
#[derive(SubStates, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[source(GameState = GameState::Playing)]
pub enum PlayState {
    #[default]
    StartNewGame,
    StartAfterDeath,
    StartNextLevel,
    CountdownBeforeRunning,
    Running,
    Paused,
    GameOver(GameOverReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum GameOverReason {
    PlayerDead,
    GameWon,
}

pub struct GameStatesPlugin;

#[derive(Debug, Clone, Default, SystemSet, PartialEq, Eq, Hash)]
pub struct GameStatesSet;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_sub_state::<PlayState>()
            .enable_state_scoped_entities::<GameState>()
            .enable_state_scoped_entities::<PlayState>()
            .add_loading_state(
                #[cfg(feature = "dbg_colliders")]
                LoadingState::new(GameState::LoadingAssets)
                    .continue_to_state(GameState::DebugColliders),
                #[cfg(not(feature = "dbg_colliders"))]
                LoadingState::new(GameState::LoadingAssets).continue_to_state(GameState::MainMenu),
            );

        app.add_systems(
            OnExit(GameState::LoadingAssets),
            setup_camera_and_playing_field.in_set(GameStatesSet),
        );
    }
}

fn setup_camera_and_playing_field(
    settings: Res<GameAreaSettings>,
    asset_server: Res<AssetServer>,
    mut global_rng: ResMut<GlobalRng>,
    mut commands: Commands,
) {
    let random_generator = RngComponent::from(&mut global_rng);
    let game_area = settings.game_area.clone();
    let display_area = Vec2::new(game_area.width(), game_area.height())
        + settings.border_area.min
        + settings.border_area.max;
    debug!(
        ?display_area,
        ?game_area,
        "setting up camera and playing field"
    );

    // Spawn a spatial listener
    commands.spawn((
        SpatialBundle::default(),
        SpatialListener::new(game_area.width() / 2.0),
    ));

    // The PlayingField is placed at (0,0) - world center.
    // All asteroids, ships, lazers etc. will be added as children of the PlayingField, thus
    // their positions are relative to the PlayingField, making it easy to move everything
    // spatially.
    let playing_field = commands
        .spawn((
            Name::new("PlayingField"),
            PlayingField,
            SpatialBundle {
                transform: Transform::from_translation(PLAYINGFIELD_POS),
                ..default()
            },
            game_area.clone(),
            random_generator,
        ))
        .with_children(|commands| {
            // Place a background behind the PlayingField.
            commands.spawn((
                Background,
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(game_area.width(), game_area.height())),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        0.0,
                        0.0,
                        PLAYINGFIELD_BACKGROUND_RELATIVE_Z_POS,
                    )),
                    ..default()
                },
            ));
            // Add an entity for displaying a border around the PlayingField.
            // The border helps us hide game elements as they move out or into the game-area,
            // especially in a situation where the window is scaled such that we have an empty
            // swatch beside the game-area!
            commands.spawn((
                Name::new("PlayingField Border"),
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(display_area.x, display_area.y)),
                        ..default()
                    },
                    // TODO: Add the border texture to settings!
                    texture: asset_server.load("images/border.png"),
                    transform: Transform::from_translation(Vec3::new(
                        0.0,
                        0.0,
                        PLAYINGFIELD_BORDER_RELATIVE_Z_POS,
                    )),
                    ..Default::default()
                },
                ImageScaleMode::Sliced(TextureSlicer {
                    border: BorderRect {
                        left: settings.border_area.min.x,
                        top: settings.border_area.min.y,
                        right: settings.border_area.max.x,
                        bottom: settings.border_area.max.y,
                    },
                    center_scale_mode: SliceScaleMode::Tile {
                        stretch_value: 1000.0,
                    },
                    sides_scale_mode: SliceScaleMode::Stretch,
                    ..Default::default()
                }),
            ));
        })
        .id();
    debug!(?playing_field, "spawned playing field");

    // Place the camera and setup the projection so we can see the entire display-area.
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: -1.0,
            far: 1000.0,
            scaling_mode: ScalingMode::AutoMin {
                min_width: display_area.x,
                min_height: display_area.y,
            },
            ..default()
        },
        ..default()
    });

    #[cfg(feature = "perf")]
    commands.spawn(iyes_perf_ui::prelude::PerfUiCompleteBundle::default());
}
