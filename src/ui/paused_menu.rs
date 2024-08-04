use bevy::prelude::*;
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

fn init_pause_countdown_timer(game_start: Res<GameStartSettings>, mut commands: Commands) {
    commands.insert_resource(CountdownTimer::with_duration(
        game_start.minimum_countdown_duration,
    ));
}

pub fn build_ui(app: &mut App) {
    let state = PlayState::Paused;

    app.add_systems(OnEnter(state), spawn_ui)
        .add_interaction_handler_in_state::<PausedButton>(state)
        .add_systems(
            Update,
            wait_for_esc_to_pause.run_if(in_state(PlayState::Running)),
        )
        .add_systems(
            OnTransition {
                exited: PlayState::Running,
                entered: PlayState::Paused,
            },
            init_pause_countdown_timer,
        )
        .add_systems(
            Update,
            wait_for_esc_to_pause.run_if(in_state(PlayState::CountdownBeforeRunning)),
        )
        .add_systems(
            Update,
            (
                highlight_interaction::<PausedButton>,
                handle_paused_menu,
                wait_for_esc_to_continue,
            )
                .run_if(in_state(state)),
        );
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

fn handle_paused_menu(
    mut event: EventReader<PressedEvent<PausedButton>>,
    mut play_state: ResMut<NextState<PlayState>>,
    timer: Res<CountdownTimer>,
    game_start: Res<GameStartSettings>,
    mut commands: Commands,
) {
    for PressedEvent { id, entity: _ } in event.read() {
        match id {
            PausedButton::Continue => {
                match timer.remaining() {
                    Some(remaining) if remaining < game_start.minimum_countdown_duration => {
                        commands.insert_resource(CountdownTimer::with_duration(
                            game_start.minimum_countdown_duration,
                        ));
                    }
                    _ => {}
                };
                play_state.set(PlayState::CountdownBeforeRunning);
            }
        }
    }
}

fn wait_for_esc_to_continue(
    keys: Res<ButtonInput<KeyCode>>,
    input_settings: Res<InputKeySettings>,
    mut next: ResMut<NextState<PlayState>>,
) {
    for key in keys.get_just_pressed() {
        debug!(?key, "was pressed");
        if *key == input_settings.pause {
            info!("Continuing");
            next.set(PlayState::CountdownBeforeRunning);
        }
    }
}

fn wait_for_esc_to_pause(
    keys: Res<ButtonInput<KeyCode>>,
    input_settings: Res<InputKeySettings>,
    mut next: ResMut<NextState<PlayState>>,
    timer: Res<CountdownTimer>,
    game_start: Res<GameStartSettings>,
    mut commands: Commands,
) {
    for key in keys.get_just_pressed() {
        debug!(?key, "was pressed");
        if *key == input_settings.pause {
            match timer.remaining() {
                Some(remaining) if remaining < game_start.minimum_countdown_duration => {
                    commands.insert_resource(CountdownTimer::with_duration(
                        game_start.minimum_countdown_duration,
                    ));
                }
                _ => {}
            };
            warn!("Pausing");
            next.set(PlayState::Paused);
        }
    }
}
