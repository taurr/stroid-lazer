use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    projectile::{
        despawn_all_projectiles, detect_projetile_collision, on_projectile_spawn,
        spawn_projectiles, timeout_projectiles, ProjectileCollisionEvent, SpawnProjectilesEvent,
    },
    states::GameState,
    PlayState,
};

#[derive(Debug, Clone, Default, SystemSet, PartialEq, Eq, Hash)]
pub struct ProjectileSet;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "dbg_colliders")]
        dbg_colliders::setup_dbg(app);

        app.add_event::<SpawnProjectilesEvent>()
            .add_event::<ProjectileCollisionEvent>();

        app.add_systems(
            Update,
            (
                spawn_projectiles.run_if(on_event::<SpawnProjectilesEvent>()),
                detect_projetile_collision.run_if(on_event::<CollisionStarted>()),
            )
                .run_if(in_state(PlayState::Running))
                .in_set(ProjectileSet),
        )
        .add_systems(
            Update,
            (timeout_projectiles,)
                .run_if(in_state(GameState::Playing))
                .in_set(ProjectileSet),
        )
        .add_systems(
            OnExit(PlayState::StartNextLevel),
            despawn_all_projectiles.in_set(ProjectileSet),
        )
        .add_systems(
            OnExit(PlayState::StartNewGame),
            despawn_all_projectiles.in_set(ProjectileSet),
        )
        .observe(on_projectile_spawn);
    }
}
