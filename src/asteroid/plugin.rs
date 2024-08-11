use bevy::prelude::*;
use smart_default::SmartDefault;

use crate::{levels::GameLevelsSet, PlayState};

use super::*;

#[derive(Debug, SystemSet, PartialEq, Eq, Hash, Clone)]
pub struct AsteroidSet;

#[derive(SmartDefault, Debug)]
pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "dbg_colliders")]
        super::dbg_colliders::setup_dbg(app);

        app.init_resource::<AsteroidCount>()
            .add_event::<AsteroidRemoveEvent>();

        app.add_systems(
            OnEnter(PlayState::StartNewGame),
            (
                despawn_all_asteroids,
                init_asteroid_counter,
                spawn_level_asteroids,
            )
                .chain()
                .in_set(AsteroidSet)
                .after(GameLevelsSet),
        )
        .add_systems(
            OnExit(PlayState::StartNextLevel),
            (init_asteroid_counter, spawn_level_asteroids)
                .chain()
                .in_set(AsteroidSet)
                .after(GameLevelsSet),
        )
        .add_systems(
            OnEnter(PlayState::Running),
            resume_asteroid_movement.in_set(AsteroidSet),
        )
        .add_systems(
            PostUpdate,
            on_remove_asteroid
                .run_if(in_state(PlayState::Running))
                .in_set(AsteroidSet),
        )
        .add_systems(
            Update,
            (detect_asteroid_hits)
                .run_if(in_state(PlayState::Running))
                .in_set(AsteroidSet),
        );

        app.observe(on_asteroid_spawn_new)
            .observe(on_asteroid_hit)
            .observe(on_asteroid_added)
            .observe(on_asteroid_removed);
    }
}
