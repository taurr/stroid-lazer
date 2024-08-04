use bevy::prelude::*;
use tracing::instrument;

use crate::{
    ui::{
        common::highlight_interaction,
        interaction::{InteractionHandlerExt, InteractionId, PressedEvent},
    },
    GameState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MainMenuButton {
    Play,
    HighScore,
    Quit,
}
impl InteractionId for MainMenuButton {}

pub fn build_ui(app: &mut App) {
    let state = GameState::MainMenu;

    app.add_systems(OnEnter(state), spawn_ui)
        .add_interaction_handler_in_state::<MainMenuButton>(state)
        .add_systems(
            Update,
            (highlight_interaction::<MainMenuButton>, handle_main_menu).run_if(in_state(state)),
        );
}

#[instrument(skip_all)]
fn spawn_ui(mut commands: Commands) {
    let menu = spawn_menu!(
        commands,
        GameState::MainMenu,
        "Game Menu",
        [
            ("Play", MainMenuButton::Play),
            ("Highscores", MainMenuButton::HighScore),
            ("Quit", MainMenuButton::Quit),
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
                env!("CARGO_PKG_NAME"),
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

fn handle_main_menu(
    mut event: EventReader<PressedEvent<MainMenuButton>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for PressedEvent { id, entity: _ } in event.read() {
        match id {
            MainMenuButton::Play => {
                next_state.set(GameState::Playing);
            }
            MainMenuButton::HighScore => {}
            MainMenuButton::Quit => {
                exit.send(AppExit::Success);
            }
        }
    }
}
