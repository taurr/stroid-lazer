use core::time::Duration;

use bevy::{color::palettes::css, prelude::*};
use tracing::instrument;

use crate::{
    assets::GameStartSettings, states::GameStatesSet, ui::constants::BACKDROP_COLOR, GameLevel,
    PlayState,
};

#[derive(Component, Debug, Clone)]
struct CountdownText;

/// Resource for keeping track of the countdown timer.
///
/// After the countdown, the inner timer will be set to None.
/// Can be initialised before entering the PlayState::CountdownBeforeRunning state.
#[derive(Resource, Debug, Clone, Deref, DerefMut, Default)]
pub struct CountdownTimer(Option<Timer>);

impl CountdownTimer {
    pub fn with_duration(duration: Duration) -> Self {
        Self(Some(Timer::new(duration, TimerMode::Once)))
    }
    pub fn remaining(&self) -> Option<Duration> {
        self.0.as_ref().map(|timer| timer.remaining())
    }

    pub fn tick(&mut self, time: &Time) -> Option<Duration> {
        match &mut self.0 {
            Some(timer) => {
                if timer.tick(time.delta()).just_finished() {
                    self.0 = None;
                    None
                } else {
                    Some(timer.remaining())
                }
            }
            None => None,
        }
    }
}

pub fn build_ui(app: &mut App) {
    let state = PlayState::CountdownBeforeRunning;

    app.init_resource::<CountdownTimer>()
        .add_systems(
            OnEnter(state),
            setup_coundown_time.pipe(spawn_ui).after(GameStatesSet),
        )
        .add_systems(
            Update,
            update_countdown_text.run_if(in_state(PlayState::CountdownBeforeRunning)),
        );
}

#[instrument(skip_all)]
fn setup_coundown_time(
    timer_res: Res<CountdownTimer>,
    game_start: Res<GameStartSettings>,
    mut commands: Commands,
) {
    match timer_res.remaining() {
        None => {
            commands.insert_resource(CountdownTimer::with_duration(game_start.countdown_duration));
        }
        Some(remaining) if remaining < game_start.minimum_countdown_duration => {
            commands.insert_resource(CountdownTimer::with_duration(
                game_start.minimum_countdown_duration,
            ));
        }
        _ => {}
    }
}

#[instrument(skip_all)]
fn spawn_ui(
    mut commands: Commands,
    current_level: Res<GameLevel>,
    settings: Res<GameStartSettings>,
    mut timer: ResMut<CountdownTimer>,
) {
    let timer =
        timer.get_or_insert_with(|| Timer::new(settings.countdown_duration, TimerMode::Once));
    debug!(remaining=?timer.remaining(), "spawning countdown ui");

    let menu = commands
        .spawn((
            StateScoped(PlayState::CountdownBeforeRunning),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BACKDROP_COLOR.into(),
                ..default()
            },
        ))
        .with_children(|commands| {
            commands
                .spawn(NodeBundle::default())
                .with_children(|commands| {
                    commands.spawn((
                        Name::new("Countdown Display"),
                        CountdownText,
                        TextBundle::from_sections([TextSection::new(
                            "",
                            TextStyle {
                                //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 144.0,
                                color: css::WHITE.into(),
                                ..Default::default()
                            },
                        )]),
                    ));
                });
        })
        .id();

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
                (**current_level).clone(),
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

fn update_countdown_text(
    time: Res<Time>,
    mut query: Query<&mut Text, With<CountdownText>>,
    mut timer_res: ResMut<CountdownTimer>,
    mut next: ResMut<NextState<PlayState>>,
) {
    if let Some(remaining) = timer_res.tick(&time) {
        let mut text = query.single_mut();
        text.sections[0].value = format!("{:1.1}", remaining.as_secs_f32());
    } else {
        info!("Countdown finished");
        next.set(PlayState::Running);
    }
}
