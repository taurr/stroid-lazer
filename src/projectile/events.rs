use avian2d::prelude::*;
use bevy::prelude::*;

use crate::states::GameState;

#[derive(Event)]
pub struct SpawnProjectilesEvent {
    pub player: Entity,
    pub weapon_idx: usize,
}

#[derive(Event)]
pub struct SpawnSingleProjectileEvent {
    pub state: GameState,
    pub position: Vec3,
    pub direction: Rotation,
    pub ammonition_idx: usize,
}

#[allow(unused)]
#[derive(Event)]
pub struct ProjectileCollisionEvent {
    /// The entity that was hit by a projectile
    pub entity_hit: Entity,
    /// Player entities that shot the one of the projectiles that hit `entity_hit`
    pub shot_by_player: Entity,
}
