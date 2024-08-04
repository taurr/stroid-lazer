use bevy::prelude::*;

pub fn setup_dbg(app: &mut App) {
    use crate::{
        asteroid::{spawn_debug_asteroids, AsteroidSet},
        states::GameState,
    };

    app.add_systems(
        OnEnter(GameState::DebugColliders),
        spawn_debug_asteroids.in_set(AsteroidSet),
    );
}
