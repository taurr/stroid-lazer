use bevy::prelude::*;
use tracing::instrument;

use crate::{
    ui::{
        common::highlight_interaction,
        interaction::{InteractionHandlerExt, InteractionId, PressedEvent},
    },
    GameState, PlayState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GameOverButton {
    PlayAgain,
    MainMenu,
}
impl InteractionId for GameOverButton {}

pub fn build_ui(app: &mut App) {
    let state = PlayState::GameOver(crate::GameOverReason::GameWon);

    app.add_systems(OnEnter(state), spawn_ui)
        .add_interaction_handler_in_state::<GameOverButton>(state)
        .add_systems(
            Update,
            (
                highlight_interaction::<GameOverButton>,
                handle_game_over_menu,
            )
                .run_if(in_state(state)),
        );
}

#[instrument(skip_all)]
fn spawn_ui(mut commands: Commands) {
    let menu = spawn_menu!(
        commands,
        PlayState::GameOver(crate::GameOverReason::GameWon),
        "Game Won Menu",
        [
            ("Play Again", GameOverButton::PlayAgain),
            ("Main Menu", GameOverButton::MainMenu),
        ]
    );

    let headline = commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::bottom(Val::Percent(5.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|cmd| {
            cmd.spawn(TextBundle::from_section(
                "Game Won",
                TextStyle {
                    font_size: 80.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ));
        })
        .id();
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
