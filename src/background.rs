//! Module for setting the background of the different "screens" in the game.

use bevy::prelude::*;
use smart_default::SmartDefault;
#[allow(unused)]
use tracing::*;

use crate::{
    assets::{GameLevelSettings, StateBackgrounds},
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
        );
    }
}

fn set_state_background(
    game_state: Res<State<GameState>>,
    background_query: Query<Entity, With<Background>>,
    backgrounds: Res<StateBackgrounds>,
    play_state: Option<Res<State<PlayState>>>,
    level_settings: Option<Res<GameLevelSettings>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let background = match **game_state {
        GameState::LoadingAssets => None,
        GameState::MainMenu => Some(backgrounds.main_menu.clone()),
        GameState::HighscoreMenu => Some(backgrounds.highscores_menu.clone()),
        GameState::Playing => match **play_state.unwrap() {
            PlayState::CountdownBeforeRunning => level_settings
                .map(|s| s.background.clone())
                .map(|h| asset_server.load(h)),
            PlayState::GameOver(reason) => match reason {
                GameOverReason::PlayerDead => Some(backgrounds.game_over.clone()),
                GameOverReason::GameWon => Some(backgrounds.game_won.clone()),
            },
            _ => {
                // Do NOT change background for any other state.
                None
            }
        },
    };

    if let (Some(background), Ok(backgound_entity)) = (background, background_query.get_single()) {
        commands.entity(backgound_entity).insert(background);
    }
}
