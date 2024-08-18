use bevy::prelude::*;
use smart_default::SmartDefault;
#[allow(unused)]
use tracing::*;

use crate::{
    assets::{
        DefaultLevelSettings, GameLevelSettings, GameLevelSettingsCollection, GameStartSettings,
    },
    asteroid::AsteroidCount,
    projectile::Projectile,
    states::{GameOverReason, PlayState},
    GameLevel,
};

#[derive(SmartDefault, Debug)]
pub struct GameLevelsPlugin;

/// All systems added by the [GameLevelPlugin] plugin belongs to this set.
#[derive(Debug, Clone, Default, SystemSet, PartialEq, Eq, Hash)]
pub struct GameLevelsSet;

impl Plugin for GameLevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(PlayState::StartNewGame),
            (start_new_game, init_level_settings)
                .chain()
                .in_set(GameLevelsSet),
        )
        .add_systems(
            Update,
            start_after_death
                .run_if(in_state(PlayState::StartAfterDeath))
                .in_set(GameLevelsSet),
        )
        .add_systems(
            Update,
            start_next_level
                .run_if(in_state(PlayState::StartNextLevel))
                .in_set(GameLevelsSet),
        )
        .add_systems(
            OnExit(PlayState::StartNextLevel),
            init_level_settings.in_set(GameLevelsSet),
        )
        .add_systems(
            PostUpdate,
            detect_level_cleared
                .run_if(in_state(PlayState::Running))
                .in_set(GameLevelsSet),
        );
    }
}

/// Reads the current level from the [GameLevel] resource and looks up the [GameLevelSettings] to
/// insert it, and a [PlayerSettings] as resources.
///
/// This wastes a little bit of memory, but saves us from looking up and merging settings every
/// time we need them during the gameplay.
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

    debug!(
        ?player_settings,
        ?level_settings,
        "inserting PlayerSettings & LevelSettings as resources"
    );
    commands.insert_resource(player_settings);
    commands.insert_resource(level_settings.clone());
}

/// Initialize [GameLevel] to the correct starting level.
fn start_new_game(
    mut next: ResMut<NextState<PlayState>>,
    mut level: ResMut<GameLevel>,
    game_start: Res<GameStartSettings>,
) {
    **level = game_start.level.clone();
    info!(level = **level, "Starting new game");
    next.set(PlayState::CountdownBeforeRunning);
}

/// Currently, just go directly to the countdown state.
fn start_after_death(
    projectiles: Query<Entity, With<Projectile>>,
    mut next: ResMut<NextState<PlayState>>,
) {
    if projectiles.is_empty() {
        info!("restarting after death");
        next.set(PlayState::CountdownBeforeRunning);
    }
}

/// Currently, just go directly to the countdown state.
fn start_next_level(
    projectiles: Query<Entity, With<Projectile>>,
    mut next: ResMut<NextState<PlayState>>,
) {
    if projectiles.iter().count() == 0 {
        info!("no more projectiles, starting next level");
        next.set(PlayState::CountdownBeforeRunning);
    }
}

/// Detects when all asteroids have been destroyed, and all projectiles are gone, then transitions
/// to either [PlayState::GameOver] or [PlayState::StartNextLevel] depending on the current level.
fn detect_level_cleared(
    asteroid_counter: Res<AsteroidCount>,
    level_settings: Res<GameLevelSettings>,
    mut current_level: ResMut<GameLevel>,
    mut next: ResMut<NextState<PlayState>>,
) {
    if **asteroid_counter == 0 {
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
