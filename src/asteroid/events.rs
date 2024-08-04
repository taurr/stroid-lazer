use avian2d::prelude::{AngularVelocity, LinearVelocity};
use bevy::prelude::*;

use crate::{
    assets::{AsteroidHitBehavior, SpriteSheetAsset},
    states::{GameState, PlayState},
};

#[derive(Event, Debug, Clone)]
pub struct AsteroidSpawnNewEvent {
    pub state: GameState,
    pub playstate: Option<PlayState>,

    pub position: Vec3,
    pub linear_velocity: LinearVelocity,
    pub angular_velocity: AngularVelocity,

    pub hit_behavior: AsteroidHitBehavior,

    pub spritesheet: SpriteSheetAsset,
    pub atlas_index: usize,
}

#[derive(Event, Debug, Clone)]
pub struct AsteroidRemoveEvent {
    pub asteroid: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct AsteroidHitEvent {
    pub position: Vec3,
    pub players: Vec<Entity>,
}
