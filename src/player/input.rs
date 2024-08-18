use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    assets::InputKeySettings,
    player::{Accelerating, Jumping, Player, PlayerFireEvent, PlayerJumpingEvent, Turning},
};

/// Actions that can be performed by the player.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    Accelerate,
    TurnLeft,
    TurnRight,
    Fire,
    HyperJump,
}

pub fn accept_player_input(
    input_query: Query<
        (
            Entity,
            &ActionState<PlayerAction>,
            Option<&Jumping>,
            Option<&Accelerating>,
            Option<&Turning>,
        ),
        With<Player>,
    >,
    mut commands: Commands,
    time: Res<Time>,
    input_settings: Res<InputKeySettings>,
    mut fire_timer: Local<Option<Timer>>,
) {
    let Ok((player, action_state, jumping, acceleration, turning)) = input_query.get_single()
    else {
        return;
    };

    if action_state.pressed(&PlayerAction::HyperJump) && jumping.is_none() {
        commands.trigger_targets(PlayerJumpingEvent, player);
    }

    if action_state.pressed(&PlayerAction::Fire) && jumping.is_none() {
        if let Some(fire_timer) = fire_timer.as_mut() {
            fire_timer.tick(time.delta());
            if fire_timer.just_finished() {
                commands.trigger_targets(PlayerFireEvent, player);
            }
        } else {
            *fire_timer = Some(Timer::new(input_settings.auto_fire, TimerMode::Repeating));
            commands.trigger_targets(PlayerFireEvent, player);
        }
    } else {
        *fire_timer = None;
    }

    let acceleration_pressed = action_state.pressed(&PlayerAction::Accelerate);
    match (acceleration, acceleration_pressed, jumping) {
        (None, true, None) => {
            commands.entity(player).insert(Accelerating);
        }
        (Some(_), false, _) => {
            commands.entity(player).remove::<Accelerating>();
        }
        (_, _, _) => {}
    }

    let turn_left = action_state.pressed(&PlayerAction::TurnLeft);
    let turn_right = action_state.pressed(&PlayerAction::TurnRight);
    match (turning, turn_left, turn_right) {
        (None, true, false) | (Some(Turning::Right(..)), true, false) => {
            trace!(?turning, ?turn_left, ?turn_right, "turning left");
            commands.entity(player).insert(Turning::Left(0.0));
        }
        (None, false, true) | (Some(Turning::Left(..)), false, true) => {
            trace!(?turning, ?turn_left, ?turn_right, "turning right");
            commands.entity(player).insert(Turning::Right(0.0));
        }
        (Some(_), false, false) | (Some(_), true, true) => {
            trace!(?turning, ?turn_left, ?turn_right, "stopped turning");
            commands.entity(player).remove::<Turning>();
        }
        (_, _, _) => {}
    };
}
