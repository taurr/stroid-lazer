use bevy::prelude::*;

use crate::tween_events::systems::handle_tween_completed;

pub struct TweenCompletedPlugin;

impl Plugin for TweenCompletedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_tween_completed);
    }
}
