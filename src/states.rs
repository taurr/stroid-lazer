use avian2d::prelude::{AngularVelocity, LinearVelocity};
use bevy::{prelude::*, render::camera::ScalingMode, window::PrimaryWindow};
use bevy_asset_loader::prelude::*;
use bevy_turborand::{GlobalRng, RngComponent};
use tracing::instrument;

use crate::{
    assets::{
        DefaultLevelSettings, GameAreaSettings, GameLevelSettings, GameLevelSettingsCollection,
        GameStartSettings,
    },
    asteroid::AsteroidCount,
    constants::{
        PLAYINGFIELD_BACKGROUND_RELATIVE_Z_POS, PLAYINGFIELD_BORDER_RELATIVE_Z_POS,
        PLAYINGFIELD_POS,
    },
    movement::MovementPaused,
    player::Player,
    projectile::Projectile,
    GameLevel, PlayingField,
};

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum GameState {
    #[default]
    LoadingAssets,
    #[cfg(feature = "dbg_colliders")]
    DebugColliders,
    MainMenu,
    Playing,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
            .enable_state_scoped_entities::<PlayState>();

        // add the loading state for the game - done here as we need to configure where it goes
        // onece assets are loaded.
        app.add_loading_state(
            #[cfg(feature = "dbg_colliders")]
            LoadingState::new(GameState::LoadingAssets)
                .continue_to_state(GameState::DebugColliders),
            #[cfg(not(feature = "dbg_colliders"))]
            LoadingState::new(GameState::LoadingAssets).continue_to_state(GameState::MainMenu),
        );

        #[cfg(feature = "cmd_line")]
        app.add_systems(
            Update,
            (|args: Res<crate::args::Args>,
              mut next: ResMut<NextState<GameState>>,
              mut has_run: Local<bool>| {
                if !*has_run && args.play {
                    *has_run = true;
                    next.set(GameState::Playing);
                }
            })
            .run_if(in_state(GameState::MainMenu)),
        );

        app.add_systems(
            OnExit(GameState::LoadingAssets),
            setup_camera_and_playing_field.in_set(GameStatesSet),
        )
        .add_systems(
            OnEnter(PlayState::StartNewGame),
            (start_new_game, init_level_settings)
                .chain()
                .in_set(GameStatesSet),
        )
        .add_systems(
            OnEnter(PlayState::StartAfterDeath),
            start_after_death.in_set(GameStatesSet),
        )
        .add_systems(
            OnEnter(PlayState::StartNextLevel),
            (start_next_level, init_level_settings)
                .chain()
                .in_set(GameStatesSet),
        )
        .add_systems(
            OnEnter(PlayState::Running),
            resume_movement.in_set(GameStatesSet),
        )
        .add_systems(
            Update,
            pause_when_window_looses_focus.run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            PostUpdate,
            detect_level_cleared
                .run_if(in_state(PlayState::Running))
                .in_set(GameStatesSet),
        )
        .add_systems(
            OnExit(PlayState::Running),
            pause_movement.in_set(GameStatesSet),
        );
    }
}

#[derive(Component, Reflect, Debug, Clone)]
struct MovementAutoPaused;

fn pause_movement(
    query: Query<
        Entity,
        (
            Without<MovementPaused>,
            Or<(With<LinearVelocity>, With<AngularVelocity>)>,
        ),
    >,
    mut commands: Commands,
) {
    for entity in query.iter() {
        debug!(?entity, "pausing movement");
        commands
            .entity(entity)
            .insert(MovementPaused)
            .insert(MovementAutoPaused);
    }
}

fn resume_movement(query: Query<Entity, With<MovementAutoPaused>>, mut commands: Commands) {
    for entity in query.iter() {
        debug!(?entity, "resuming movement");
        commands
            .entity(entity)
            .remove::<MovementPaused>()
            .remove::<MovementAutoPaused>();
    }
}

#[instrument(skip_all)]
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
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(game_area.width(), game_area.height())),
                    color: Color::srgb(0.0, 0.0, 0.2),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    0.0,
                    0.0,
                    PLAYINGFIELD_BACKGROUND_RELATIVE_Z_POS,
                )),
                ..default()
            });
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

#[instrument(skip_all)]
pub fn init_level_settings(
    current_level: Res<GameLevel>,
    level_settings_collection: Res<GameLevelSettingsCollection>,
    default_level_settings: Res<DefaultLevelSettings>,
    mut commands: Commands,
) {
    let Some(level_settings) = level_settings_collection.get(&**current_level) else {
        error!(
            level = &**current_level,
            "Settings are flawed: Unrecognized level specified"
        );
        panic!();
    };

    let player_settings = default_level_settings
        .player
        .clone()
        .merge(level_settings.player.as_ref());

    debug!(?player_settings, "inserting PlayerSettings as resource");
    commands.insert_resource(player_settings);
    debug!(?level_settings, "inserting LevelSettings as resource");
    commands.insert_resource(level_settings.clone());
}

#[cfg(feature = "cmd_line")]
#[instrument(skip_all)]
fn start_new_game(
    mut current_level: ResMut<GameLevel>,
    args: Res<crate::args::Args>,
    game_start: Res<GameStartSettings>,
    mut next: ResMut<NextState<PlayState>>,
) {
    **current_level = args
        .level
        .as_ref()
        .cloned()
        .unwrap_or_else(|| game_start.level.clone());

    info!(level = **current_level, "Starting new game");
    next.set(PlayState::CountdownBeforeRunning);
}

#[cfg(not(feature = "cmd_line"))]
#[instrument(skip_all)]
fn start_new_game(
    mut next: ResMut<NextState<PlayState>>,
    mut level: ResMut<GameLevel>,
    game_start: Res<GameStartSettings>,
) {
    **level = game_start.level;
    info!(level = **level, "Starting new game");
    next.set(PlayState::CountdownBeforeRunning);
}

#[instrument(skip_all)]
fn start_after_death(mut next: ResMut<NextState<PlayState>>) {
    info!("restarting after death");
    next.set(PlayState::CountdownBeforeRunning);
}

#[instrument(skip_all)]
fn start_next_level(mut next: ResMut<NextState<PlayState>>) {
    info!("starting next level");
    next.set(PlayState::CountdownBeforeRunning);
}

#[instrument(skip_all)]
fn detect_level_cleared(
    asteroid_counter: Res<AsteroidCount>,
    player: Query<Entity, (With<Player>, Without<MovementPaused>)>,
    projectiles: Query<Entity, With<Projectile>>,
    level_settings: Res<GameLevelSettings>,
    mut current_level: ResMut<GameLevel>,
    mut commands: Commands,
    mut next: ResMut<NextState<PlayState>>,
) {
    if **asteroid_counter == 0 {
        if let Ok(player) = player.get_single() {
            commands.entity(player).insert(MovementPaused);
        }
        let projectiles = projectiles.iter().len();
        if projectiles == 0 {
            let Some(next_level) = &level_settings.next_level else {
                warn!("won the game!");
                next.set(PlayState::GameOver(GameOverReason::GameWon));
                return;
            };

            info!(next_level, "level cleared, starting next level");
            **current_level = next_level.clone();
            next.set(PlayState::StartNextLevel);
        }
    }
}

fn pause_when_window_looses_focus(
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
