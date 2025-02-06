use bevy::prelude::*;

pub fn setup_dbg(app: &mut App) {
    use crate::{
        asteroid::{AsteroidSet, spawn_debug_asteroids},
        states::GameState,
    };

    app.add_systems(
        OnEnter(GameState::DebugColliders),
        spawn_debug_asteroids.in_set(AsteroidSet),
    );
}
