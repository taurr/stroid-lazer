use bevy::{
    input::{
        keyboard::{self, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};
use bevy_persistent::Persistent;
use strum::IntoEnumIterator;

use crate::{
    assets::{HighScoreBoard, HighScoreKey},
    player::{Player, Score},
    states::GameOverReason,
    ui::{
        constants::{BUTTON_PADDING, H1_FONT_SIZE, H3_FONT_SIZE},
        interaction::{InteractionHandlerExt, InteractionId, PressedEvent},
        UiSet,
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
    for reason in GameOverReason::iter() {
        let state = PlayState::GameOver(reason);
        app.add_systems(OnEnter(state), spawn_ui.in_set(UiSet))
            .add_interaction_handler_in_state::<GameOverButton>(state)
            .add_systems(
                Update,
                (handle_game_over_menu, blink_cursor, text_input_system)
                    .run_if(in_state(state))
                    .in_set(UiSet),
            );
    }
}

fn spawn_ui(
    state: Res<State<PlayState>>,
    score_query: Query<&Score, With<Player>>,
    highscore_key: Option<Res<HighScoreKey>>,
    mut asset_server: ResMut<AssetServer>,
    mut commands: Commands,
) {
    let gameover_reason = match *state.get() {
        PlayState::GameOver(reason) => reason,
        _ => return,
    };

    let _score = score_query.get_single().unwrap();
    match highscore_key {
        Some(highscore_key) => setup_highscore_menu(
            _score,
            &highscore_key,
            *state.get(),
            asset_server.as_mut(),
            &mut commands,
        ),
        None => setup_dead_menu(
            gameover_reason,
            *state.get(),
            asset_server.as_mut(),
            &mut commands,
        ),
    }
}

fn setup_dead_menu(
    gameover_reason: GameOverReason,
    state: PlayState,
    _asset_server: &mut AssetServer,
    commands: &mut Commands,
) {
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
            // title
            cmd.spawn(TextBundle::from_section(
                match gameover_reason {
                    GameOverReason::PlayerDead => "Game Over",
                    GameOverReason::GameWon => "Game Won",
                },
                TextStyle {
                    font_size: H1_FONT_SIZE,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ));
        })
        .id();
    let menu = spawn_menu!(
        commands,
        state,
        [
            ("Play Again", GameOverButton::PlayAgain),
            ("Main Menu", GameOverButton::MainMenu),
        ]
    );
    commands.entity(menu).insert_children(0, &[headline]);
}

fn setup_highscore_menu(
    _score: &Score,
    highscore: &HighScoreKey,
    state: PlayState,
    asset_server: &mut AssetServer,
    commands: &mut Commands,
) {
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
                "Congratulations",
                TextStyle {
                    font_size: H1_FONT_SIZE,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ));
            cmd.spawn(TextBundle::from_section(
                "You reached the scoreboard",
                TextStyle {
                    font_size: H3_FONT_SIZE,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ));
            cmd.spawn(TextBundle::from_section(
                match highscore.place() + 1 {
                    1 => "1st place".to_string(),
                    2 => "2nd place".to_string(),
                    3 => "3rd place".to_string(),
                    place => format!("{place}th place"),
                },
                TextStyle {
                    font_size: H1_FONT_SIZE,
                    font: asset_server.load("fonts/VictorMonoNerdFont-BoldItalic.ttf"),
                    color: Color::WHITE,
                },
            ));
            cmd.spawn(TextBundle {
                style: Style {
                    margin: UiRect::top(Val::Vh(5.0)),
                    ..default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value: "Enter your name: ".to_string(),
                        style: TextStyle {
                            font_size: H3_FONT_SIZE,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    }],
                    ..default()
                },
                ..default()
            });
            cmd.spawn((NodeBundle {
                style: Style {
                    width: Val::Px(600.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    padding: BUTTON_PADDING,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: crate::ui::constants::BUTTON_BORDER_SIZE,
                    ..Default::default()
                },
                border_color: crate::ui::constants::BUTTON_BORDER_COLOR.into(),
                border_radius: BorderRadius::percent(12.0, 12.0, 12.0, 12.0),
                ..Default::default()
            },))
                .with_children(|cmd| {
                    cmd.spawn((
                        HighScorerName,
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font_size: crate::ui::constants::BUTTON_FONT_SIZE,
                                color: crate::ui::constants::TEXT_COLOR,
                                ..Default::default()
                            },
                        ),
                    ));
                    cmd.spawn((
                        Cursor {
                            timer: Timer::from_seconds(0.333, TimerMode::Repeating),
                        },
                        TextBundle::from_section(
                            "|",
                            TextStyle {
                                font_size: crate::ui::constants::BUTTON_FONT_SIZE,
                                color: crate::ui::constants::TEXT_COLOR,
                                ..Default::default()
                            },
                        ),
                    ));
                });
            cmd.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(600.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::End,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|cmd| {
                button("Save", GameOverButton::MainMenu, cmd);
            });
        });
}

#[derive(Debug, Clone, Component)]
struct Cursor {
    timer: Timer,
}

#[derive(Debug, Clone, Component)]
struct HighScorerName;

fn blink_cursor(time: Res<Time>, mut query: Query<(&mut Cursor, &mut Visibility)>) {
    for (mut cursor, mut visibility) in query.iter_mut() {
        if cursor.timer.tick(time.delta()).just_finished() {
            *visibility = match *visibility {
                Visibility::Inherited => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
                Visibility::Visible => Visibility::Hidden,
            };
        }
    }
}

fn text_input_system(
    mut query: Query<&mut Text, With<HighScorerName>>,
    mut key_input: EventReader<KeyboardInput>,
    mut event: EventWriter<PressedEvent<GameOverButton>>,
) {
    let Ok(mut text) = query.get_single_mut() else {
        return;
    };
    let Some(text) = text.sections.get_mut(0) else {
        return;
    };

    for key in key_input.read() {
        if key.state == ButtonState::Released {
            continue;
        }
        match &key.logical_key {
            keyboard::Key::Space => {
                if text.value.is_empty() || text.value.len() > 32 {
                    continue;
                }
                text.value.push(' ');
            }
            keyboard::Key::Character(input) => {
                if text.value.len() > 32 {
                    continue;
                }
                if input.chars().any(|c| c.is_control()) {
                    continue;
                }
                text.value.push_str(input);
            }
            keyboard::Key::Enter => {
                if !text.value.is_empty() {
                    event.send(PressedEvent {
                        id: GameOverButton::MainMenu,
                        entity: Entity::PLACEHOLDER,
                    });
                }
            }
            keyboard::Key::Backspace => {
                text.value.pop();
            }
            _ => {}
        }
    }
}

fn button<E: InteractionId + 'static>(
    text: &str,
    button_event: E,
    cmd: &mut ChildBuilder,
) -> Entity {
    cmd.spawn((
        Name::new(format!("{} Button", text)),
        crate::ui::interaction::InteractionIdComponent(button_event),
        ButtonBundle {
            style: Style {
                margin: UiRect::top(Val::Px(5.0)),
                padding: UiRect::new(Val::Px(15.0), Val::Px(15.0), Val::Px(5.0), Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: crate::ui::constants::BUTTON_BORDER_SIZE,
                ..Default::default()
            },
            background_color: crate::ui::constants::NORMAL_BUTTON.into(),
            border_color: crate::ui::constants::BUTTON_BORDER_COLOR.into(),
            border_radius: crate::ui::constants::BUTTON_BORDER_RADIUS,
            ..Default::default()
        },
    ))
    .with_children(|cmd| {
        cmd.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: crate::ui::constants::BUTTON_FONT_SIZE,
                color: crate::ui::constants::TEXT_COLOR,
                ..Default::default()
            },
        ));
    })
    .id()
}

fn handle_game_over_menu(
    mut event: EventReader<PressedEvent<GameOverButton>>,
    mut play_state: ResMut<NextState<PlayState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut highscore_board: ResMut<Persistent<HighScoreBoard>>,
    mut highscore_key: Option<ResMut<HighScoreKey>>,
    text_query: Query<&Text, With<HighScorerName>>,
    mut commands: Commands,
) {
    for PressedEvent { id, entity: _ } in event.read() {
        if let (Ok(name), Some(highscore_key)) =
            (text_query.get_single(), highscore_key.as_deref_mut())
        {
            commands.remove_resource::<HighScoreKey>();
            let name = name.sections.first().unwrap().value.trim();
            highscore_board.assign_name(name, highscore_key);
            highscore_board
                .persist()
                .expect("failed to persist high-scores");
        };

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
