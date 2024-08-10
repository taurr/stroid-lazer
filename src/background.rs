//! Module for setting the background of the different "screens" in the game.

use bevy::prelude::*;
use smart_default::SmartDefault;
#[allow(unused)]
use tracing::*;

use crate::{
    assets::{GameLevelSettings, StateBackgrounds},
    levels::GameLevelsSet,
    states::{GameOverReason, GameState, PlayState},
};

/// Marks an entity as the background.
/// There should only be one!
#[derive(Debug, Clone, Component)]
pub struct Background;

/// Plugin for setting the background of the different "screens" in the game.
#[derive(SmartDefault, Debug)]
pub struct BackgroundPlugin;

/// All systems added by the [BackgroundPlugin] plugin belongs to this set.
#[derive(Debug, Clone, Default, SystemSet, PartialEq, Eq, Hash)]
pub struct BackgroundSet;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            set_state_background
                .run_if(
                    not(in_state(GameState::LoadingAssets))
                        .and_then(state_changed::<GameState>.or_else(state_changed::<PlayState>)),
                )
                .in_set(BackgroundSet),
        )
        .add_systems(
            OnEnter(PlayState::StartNewGame),
            set_level_background
                .after(GameLevelsSet)
                .in_set(BackgroundSet),
        )
        .add_systems(
            OnExit(PlayState::StartNextLevel),
            set_level_background
                .after(GameLevelsSet)
                .in_set(BackgroundSet),
        );
    }
}

fn set_state_background(
    game_state: Res<State<GameState>>,
    play_state: Option<Res<State<PlayState>>>,
    background_query: Query<Entity, With<Background>>,
    backgrounds: Res<StateBackgrounds>,
    mut commands: Commands,
) {
    let background = match **game_state {
        GameState::LoadingAssets => None,
        GameState::MainMenu => Some(backgrounds.main_menu.clone()),
        GameState::Playing => match **play_state.unwrap() {
            PlayState::StartNewGame => None,
            PlayState::StartAfterDeath => None,
            PlayState::StartNextLevel => None,
            PlayState::CountdownBeforeRunning => None,
            PlayState::Running => None,
            PlayState::Paused => None,
            PlayState::GameOver(reason) => match reason {
                GameOverReason::PlayerDead => Some(backgrounds.game_over.clone()),
                GameOverReason::GameWon => Some(backgrounds.game_won.clone()),
            },
        },
    };

    if let Some(background) = background {
        commands
            .entity(background_query.single())
            .insert(background);
    }
}

fn set_level_background(
    level_settings: Res<GameLevelSettings>,
    background_query: Query<Entity, With<Background>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let background: Handle<Image> = asset_server.load(&level_settings.background);
    commands
        .entity(background_query.single())
        .insert(background);
}
