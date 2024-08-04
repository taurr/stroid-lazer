use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct Projectile {
    pub timer: Timer,
    pub shot_by_player: Entity,
}

#[derive(Component)]
pub struct ProjectileSprite;
