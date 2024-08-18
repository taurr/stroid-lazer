use bevy::prelude::*;
use bevy_tweening::TweenCompleted;
use enum_ordinalize::Ordinalize;

use crate::{player::PlayerJumpFinishedEvent, tween_events::TweenCompletedEvent};

pub fn handle_tween_completed(
    mut tween_events: EventReader<TweenCompleted>,
    mut commands: Commands,
) {
    // TODO: add feature to bevy_tweening to support observers instead of this 'hack'
    for TweenCompleted { entity, user_data } in tween_events.read() {
        match TweenCompletedEvent::from_ordinal(*user_data) {
            Some(TweenCompletedEvent::JumpFinished) => {
                commands.trigger_targets(PlayerJumpFinishedEvent, *entity);
            }
            None => {
                warn!("Unknown user data: {}", user_data);
            }
        };
    }
}
