use core::iter::repeat;

use bevy::{
    input::{
        keyboard::{self, KeyboardInput},
        ButtonState,
    },
    prelude::*,
    text::BreakLineOn,
};
use bevy_persistent::Persistent;
use chrono::{DateTime, Utc};

use crate::{
    assets::HighScoreBoard,
    ui::{
        constants::{H1_FONT_SIZE, H2_FONT_SIZE, H3_FONT_SIZE},
        interaction::{InteractionHandlerExt, InteractionId, PressedEvent},
        menu::ButtonBuilderExt,
        UiSet,
    },
    GameState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HighScoreMenuButton {
    Back,
}
impl InteractionId for HighScoreMenuButton {}

pub fn build_ui(app: &mut App) {
    let state = GameState::HighscoreMenu;

    app.add_systems(OnEnter(state), spawn_ui.in_set(UiSet))
        .add_interaction_handler_in_state::<HighScoreMenuButton>(state)
        .add_systems(
            Update,
            (handle_highscore_menu, text_input_system)
                .run_if(in_state(state))
                .in_set(UiSet),
        );
}

const MENU_WIDTH: Val = Val::Px(600.0);
const MENU_HEIGHT: Val = Val::Px(400.0);

fn spawn_ui(
    state: Res<State<GameState>>,
    highscore_board: Res<Persistent<HighScoreBoard>>,
    mut commands: Commands,
) {
    let state = *state.get();

    commands
        .spawn((
            StateScoped(state),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: crate::ui::constants::BACKDROP_COLOR.into(),
                ..Default::default()
            },
        ))
        .with_children(|cmd| {
            // title
            cmd.spawn(TextBundle::from_section(
                "Highscores",
                TextStyle {
                    font_size: H1_FONT_SIZE,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ));
            cmd.spawn(NodeBundle {
                border_color: crate::ui::constants::BUTTON_BORDER_COLOR.into(),
                border_radius: crate::ui::constants::BUTTON_BORDER_RADIUS,
                style: Style {
                    margin: UiRect::top(Val::Px(5.0)),
                    width: MENU_WIDTH,
                    height: MENU_HEIGHT,
                    display: Display::Grid,
                    justify_content: JustifyContent::Stretch,
                    justify_items: JustifyItems::Center,
                    align_content: AlignContent::Stretch,
                    align_items: AlignItems::Center,
                    //padding: UiRect::all(Val::Px(5.0)),
                    border: crate::ui::constants::BUTTON_BORDER_SIZE,
                    grid_template_columns: vec![
                        RepeatedGridTrack::px(1, 36.0),
                        RepeatedGridTrack::px(1, 290.0),
                        RepeatedGridTrack::px(1, 130.0),
                        RepeatedGridTrack::px(1, 140.0),
                    ],
                    grid_template_rows: vec![RepeatedGridTrack::percent(11, 9.0)],
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|cmd| {
                for column in 1..=4 {
                    cmd.spawn(NodeBundle {
                        border_color: crate::ui::constants::BUTTON_BORDER_COLOR.into(),
                        style: Style {
                            border: UiRect::new(
                                Val::Px(0.0),
                                Val::Px(0.0),
                                Val::Px(0.0),
                                Val::Px(3.0),
                            ),
                            grid_row: GridPlacement::start(1),
                            grid_column: GridPlacement::start(column),
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            overflow: Overflow::clip(),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|cmd| {
                        cmd.spawn(TextBundle {
                            text: Text {
                                linebreak_behavior: BreakLineOn::NoWrap,
                                sections: vec![TextSection {
                                    style: TextStyle {
                                        font_size: H2_FONT_SIZE,
                                        color: Color::WHITE,
                                        ..Default::default()
                                    },
                                    value: match column {
                                        2 => "Name",
                                        3 => "Date",
                                        4 => "Score",
                                        _ => "",
                                    }
                                    .to_string(),
                                }],
                                ..default()
                            },
                            ..default()
                        });
                    });
                }
                for column in 1..=4 {
                    for (row, highscore) in highscore_board
                        .iter()
                        .map(Some)
                        .chain(repeat(None))
                        .take(10)
                        .enumerate()
                    {
                        cmd.spawn(NodeBundle {
                            style: Style {
                                grid_row: GridPlacement::start((row + 2) as i16),
                                grid_column: GridPlacement::start(column),
                                width: Val::Percent(98.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                overflow: Overflow::clip(),
                                ..Default::default()
                            },
                            //background_color: match (row % 2, column) {
                            //    (0, 1) => Color::srgb(0.2, 0.1, 0.1).into(),
                            //    (0, 2) => Color::srgb(0.1, 0.2, 0.1).into(),
                            //    (0, 3) => Color::srgb(0.1, 0.1, 0.2).into(),
                            //    (1, 1) => Color::srgb(0.25, 0.15, 0.15).into(),
                            //    (1, 2) => Color::srgb(0.15, 0.25, 0.15).into(),
                            //    (1, 3) => Color::srgb(0.15, 0.15, 0.25).into(),
                            //    _ => Color::srgb(0.1, 0.1, 0.1).into(),
                            //},
                            ..Default::default()
                        })
                        .with_children(|cmd| {
                            cmd.spawn(TextBundle {
                                text: Text {
                                    linebreak_behavior: BreakLineOn::NoWrap,
                                    sections: vec![TextSection {
                                        style: TextStyle {
                                            font_size: H3_FONT_SIZE,
                                            color: match column {
                                                1 => bevy::color::palettes::basic::GRAY.into(),
                                                _ => Color::WHITE,
                                            },
                                            ..Default::default()
                                        },
                                        value: match column {
                                            1 => (row + 1).to_string(),
                                            2 => highscore
                                                .map(|h| h.name())
                                                .unwrap_or("")
                                                .to_string(),
                                            3 => highscore
                                                .map(|h| {
                                                    format!(
                                                        "{}",
                                                        DateTime::<Utc>::from(h.datetime())
                                                            .format("%Y-%m-%d")
                                                    )
                                                })
                                                .unwrap_or("".to_string()),
                                            4 => highscore
                                                .map(|h| h.score().to_string())
                                                .unwrap_or("".to_string()),
                                            _ => "?".to_string(),
                                        },
                                    }],
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    }
                }
            });
            cmd.spawn(NodeBundle {
                style: Style {
                    width: MENU_WIDTH,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::End,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|cmd| {
                cmd.spawn_button("Back", HighScoreMenuButton::Back);
            });
        });
}

fn text_input_system(
    mut key_input: EventReader<KeyboardInput>,
    mut event: EventWriter<PressedEvent<HighScoreMenuButton>>,
) {
    for key in key_input.read() {
        if key.state == ButtonState::Released {
            continue;
        }
        if key.logical_key == keyboard::Key::Escape || key.logical_key == keyboard::Key::Enter {
            event.send(PressedEvent {
                id: HighScoreMenuButton::Back,
                entity: Entity::PLACEHOLDER,
            });
        }
    }
}

fn handle_highscore_menu(
    mut event: EventReader<PressedEvent<HighScoreMenuButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for PressedEvent { id, entity: _ } in event.read() {
        match id {
            HighScoreMenuButton::Back => {
                next_state.set(GameState::MainMenu);
            }
        }
    }
}
