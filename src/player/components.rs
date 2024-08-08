use bevy::prelude::*;
use derive_more::{Debug, Deref, DerefMut, Display, Constructor};

#[derive(Component, Debug, Clone, Copy)]
pub struct Player {
    pub lives: usize,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct PlayerSprite;

#[derive(Component, Reflect, Debug, Display, Deref, DerefMut, Constructor, Clone, Copy)]
pub struct Score(usize);

#[derive(Component, Reflect, Debug, Display, Deref, DerefMut, Constructor, Clone)]
pub struct EquippedWeapon(String);

#[derive(Component, Debug, Clone, Copy)]
pub struct Dead;

#[derive(Component, Debug, Clone, Copy)]
pub struct Jumping;

#[derive(Component, Debug, Clone, Copy)]
pub struct Accelerating;

#[derive(Component, Debug, Clone, PartialEq)]
pub enum Turning {
    Left(f32),
    Right(f32),
}
