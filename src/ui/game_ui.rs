use bevy::{color::palettes::css, prelude::*};
use tracing::instrument;

use crate::{
    asteroid::AsteroidCount,
    player::{Player, Score},
    states::PlayState,
    GameLevel, GameState,
};

pub fn build_ui(app: &mut App) {
    let state = GameState::Playing;

    app.add_systems(OnEnter(state), spawn_ui)
        .add_systems(
            Update,
            (update_level_text, update_lives_text, update_asteroid_count)
                .run_if(in_state(PlayState::CountdownBeforeRunning)),
        )
        .add_systems(
            Update,
            (
                update_level_text,
                update_score_text,
                update_lives_text,
                update_asteroid_count,
            )
                .run_if(in_state(PlayState::Running)),
        );
}

#[derive(Component, Debug, Clone)]
struct LevelText;

#[derive(Component, Debug, Clone)]
struct ScoreText;

#[derive(Component, Debug, Clone)]
struct LivesText;

#[derive(Component, Debug, Clone)]
struct AsteroidText;

#[instrument(skip_all)]
fn spawn_ui(mut commands: Commands) {
    debug!("spawning game ui");
    commands
        .spawn((
            StateScoped(GameState::Playing),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|commands| {
            // Top row
            commands
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|commands| {
                    commands
                        .spawn(NodeBundle::default())
                        .with_children(|commands| {
                            commands.spawn((
                                Name::new("Level Display"),
                                TextBundle::from_sections([
                                    TextSection::new(
                                        "Level: ",
                                        TextStyle {
                                            //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 20.0,
                                            color: css::WHITE.into(),
                                            ..Default::default()
                                        },
                                    ),
                                    TextSection::new(
                                        "0",
                                        TextStyle {
                                            //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 20.0,
                                            color: css::GREEN.into(),
                                            ..Default::default()
                                        },
                                    ),
                                ]),
                                LevelText,
                            ));
                        });
                    commands.spawn(NodeBundle {
                        style: Style {
                            width: Val::VMin(10.0),
                            ..default()
                        },
                        ..default()
                    });
                    commands
                        .spawn(NodeBundle::default())
                        .with_children(|commands| {
                            commands.spawn((
                                Name::new("Score Display"),
                                TextBundle::from_sections([
                                    TextSection::new(
                                        "Score: ",
                                        TextStyle {
                                            //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 20.0,
                                            color: css::WHITE.into(),
                                            ..Default::default()
                                        },
                                    ),
                                    TextSection::new(
                                        "0",
                                        TextStyle {
                                            //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 20.0,
                                            color: css::RED.into(),
                                            ..Default::default()
                                        },
                                    ),
                                ]),
                                ScoreText,
                            ));
                        });
                    commands.spawn(NodeBundle {
                        style: Style {
                            width: Val::VMin(10.0),
                            ..default()
                        },
                        ..default()
                    });
                    commands
                        .spawn(NodeBundle::default())
                        .with_children(|commands| {
                            commands.spawn((
                                Name::new("Lives Display"),
                                TextBundle::from_sections([
                                    TextSection::new(
                                        "Lives: ",
                                        TextStyle {
                                            //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 20.0,
                                            color: css::WHITE.into(),
                                            ..Default::default()
                                        },
                                    ),
                                    TextSection::new(
                                        "0",
                                        TextStyle {
                                            //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 20.0,
                                            color: css::GOLD.into(),
                                            ..Default::default()
                                        },
                                    ),
                                ]),
                                LivesText,
                            ));
                        });
                });

            // Bottom row
            commands
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_self: AlignSelf::FlexEnd,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|commands| {
                    commands
                        .spawn(NodeBundle::default())
                        .with_children(|commands| {
                            commands.spawn((
                                Name::new("Asteroids Counter"),
                                TextBundle::from_sections([
                                    TextSection::new(
                                        "Asteroids: ",
                                        TextStyle {
                                            //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 20.0,
                                            color: css::WHITE.into(),
                                            ..Default::default()
                                        },
                                    ),
                                    TextSection::new(
                                        "0",
                                        TextStyle {
                                            //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 20.0,
                                            color: css::LIGHT_BLUE.into(),
                                            ..Default::default()
                                        },
                                    ),
                                ]),
                                AsteroidText,
                            ));
                        });
                });
        });
}

fn update_level_text(mut query: Query<&mut Text, With<LevelText>>, level: Res<GameLevel>) {
    for mut text in query.iter_mut() {
        text.sections[1].value = (**level).to_string();
    }
}

fn update_score_text(
    mut query: Query<&mut Text, With<ScoreText>>,
    score: Query<&Score, Changed<Score>>,
) {
    if let Ok(score) = score.get_single() {
        for mut text in query.iter_mut() {
            text.sections[1].value = format!("{}", **score);
        }
    }
}

fn update_lives_text(
    mut query: Query<&mut Text, With<LivesText>>,
    score: Query<&Player, Changed<Player>>,
) {
    if let Ok(player) = score.get_single() {
        for mut text in query.iter_mut() {
            text.sections[1].value = format!("{}", player.lives);
        }
    }
}

fn update_asteroid_count(
    mut query: Query<&mut Text, With<AsteroidText>>,
    counter: Res<AsteroidCount>,
) {
    for mut text in query.iter_mut() {
        text.sections[1].value = format!("{}", **counter);
    }
}
