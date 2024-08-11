use bevy::prelude::*;
use strum::IntoEnumIterator;
use tracing::instrument;

use crate::{
    assets::HighScoreKey,
    player::{Player, Score},
    states::GameOverReason,
    ui::interaction::{InteractionHandlerExt, InteractionId, PressedEvent},
    GameState, PlayState,
};

use super::{
    constants::{H1_FONT_SIZE, H2_FONT_SIZE, TITLE_FONT_SIZE},
    UiSet,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GameOverButton {
    PlayAgain,
    MainMenu,
}
impl InteractionId for GameOverButton {}

pub fn build_ui(app: &mut App) {
    for reason in GameOverReason::iter() {
        let state = PlayState::GameOver(reason);
        app.add_systems(OnEnter(state), spawn_ui.in_set(UiSet))
            .add_interaction_handler_in_state::<GameOverButton>(state)
            .add_systems(
                Update,
                handle_game_over_menu.run_if(in_state(state)).in_set(UiSet),
            );
    }
}

#[instrument(skip_all)]
fn spawn_ui(
    state: Res<State<PlayState>>,
    score_query: Query<(&Score, Option<&HighScoreKey>), With<Player>>,
    asset_server: ResMut<AssetServer>,
    mut commands: Commands,
) {
    let gameover_reason = match *state.get() {
        PlayState::GameOver(reason) => reason,
        _ => return,
    };

    let headline = commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::bottom(Val::Percent(5.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|cmd| {
            let (_score, highscore_key) = score_query.get_single().unwrap();
            let title = match (highscore_key.is_some(), gameover_reason) {
                (true, _) => "Congratulations",
                (false, GameOverReason::PlayerDead) => "Game Over",
                (false, GameOverReason::GameWon) => "Game Won",
            };
            cmd.spawn(TextBundle::from_section(
                title,
                TextStyle {
                    font_size: H1_FONT_SIZE,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ));
            if let Some(highscore) = highscore_key {
                cmd.spawn(TextBundle::from_section(
                    "You reached the scoreboard",
                    TextStyle {
                        font_size: H2_FONT_SIZE,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                ));
                let place = match highscore.place() {
                    0 => "1st place".to_string(),
                    1 => "2nd place".to_string(),
                    2 => "3rd place".to_string(),
                    place => format!("{}th place", place + 1),
                };
                cmd.spawn(TextBundle::from_section(
                    place,
                    TextStyle {
                        font_size: TITLE_FONT_SIZE,
                        font: asset_server.load("fonts/VictorMonoNerdFont-BoldItalic.ttf"),
                        color: Color::WHITE,
                    },
                ));
            }
        })
        .id();

    let menu = spawn_menu!(
        commands,
        *state.get(),
        [
            ("Play Again", GameOverButton::PlayAgain),
            ("Main Menu", GameOverButton::MainMenu),
        ]
    );

    commands.entity(menu).insert_children(0, &[headline]);
}

fn handle_game_over_menu(
    mut event: EventReader<PressedEvent<GameOverButton>>,
    mut play_state: ResMut<NextState<PlayState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for PressedEvent { id, entity: _ } in event.read() {
        match id {
            GameOverButton::PlayAgain => {
                play_state.set(PlayState::StartNewGame);
            }
            GameOverButton::MainMenu => {
                game_state.set(GameState::MainMenu);
            }
        }
    }
}
