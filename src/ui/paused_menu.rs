use bevy::{prelude::*, window::PrimaryWindow};
use tracing::instrument;

use crate::{
    assets::{GameStartSettings, InputKeySettings},
    ui::{
        common::highlight_interaction,
        interaction::{InteractionHandlerExt, InteractionId, PressedEvent},
    },
    PlayState,
};

use super::countdown_ui::CountdownTimer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PausedButton {
    Continue,
}
impl InteractionId for PausedButton {}

#[derive(Event)]
struct ResumeEvent;

#[derive(Event)]
struct PauseEvent;

pub fn build_ui(app: &mut App) {
    app.add_systems(OnEnter(PlayState::Paused), spawn_ui)
        .add_interaction_handler_in_state::<PausedButton>(PlayState::Paused)
        .add_systems(
            Update,
            (|mut commands: Commands| commands.trigger(PauseEvent)).run_if(
                (in_state(PlayState::CountdownBeforeRunning).or_else(in_state(PlayState::Running)))
                    .and_then(not(window_has_focus).or_else(pause_key_pressed)),
            ),
        )
        .add_systems(
            Update,
            (|mut commands: Commands| commands.trigger(ResumeEvent))
                .run_if(in_state(PlayState::Paused).and_then(pause_key_pressed)),
        )
        .add_systems(
            Update,
            (highlight_interaction::<PausedButton>, handle_paused_menu)
                .run_if(in_state(PlayState::Paused)),
        )
        .observe(on_resume_event)
        .observe(on_pause_event);
}

#[instrument(skip_all)]
fn spawn_ui(mut commands: Commands) {
    let menu = spawn_menu!(
        commands,
        PlayState::Paused,
        "Paused Menu",
        [("Continue", PausedButton::Continue),]
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
                "Game Paused",
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

fn handle_paused_menu(mut event: EventReader<PressedEvent<PausedButton>>, mut commands: Commands) {
    for PressedEvent { id, entity: _ } in event.read() {
        match id {
            PausedButton::Continue => {
                commands.trigger(ResumeEvent);
            }
        }
    }
}

fn on_resume_event(
    _trigger: Trigger<ResumeEvent>,
    mut play_state: ResMut<NextState<PlayState>>,
    timer: Res<CountdownTimer>,
    game_start: Res<GameStartSettings>,
    mut commands: Commands,
) {
    match timer.remaining() {
        None => {
            commands.insert_resource(CountdownTimer::with_duration(
                game_start.minimum_countdown_duration,
            ));
        }
        Some(remaining) if remaining < game_start.minimum_countdown_duration => {
            commands.insert_resource(CountdownTimer::with_duration(
                game_start.minimum_countdown_duration,
            ));
        }
        _ => {}
    };
    play_state.set(PlayState::CountdownBeforeRunning);
}

fn on_pause_event(_trigger: Trigger<PauseEvent>, mut next_state: ResMut<NextState<PlayState>>) {
    next_state.set(PlayState::Paused);
}

fn window_has_focus(query: Query<&Window, With<PrimaryWindow>>) -> bool {
    if let Ok(window) = query.get_single() {
        window.focused
    } else {
        false
    }
}

fn pause_key_pressed(
    keys: Res<ButtonInput<KeyCode>>,
    input_settings: Res<InputKeySettings>,
) -> bool {
    for key in keys.get_just_pressed() {
        if *key == input_settings.pause {
            debug!(?key, "pause key pressed");
            return true;
        }
    }
    false
}
