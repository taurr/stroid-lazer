use bevy::prelude::*;
use derive_more::{Constructor, Debug, Deref, DerefMut, Display};
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Copy)]
pub struct Player {
    pub lives: usize,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct PlayerSprite;

#[derive(
    Component,
    Reflect,
    Debug,
    Display,
    Deref,
    DerefMut,
    Constructor,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub struct Score {
    #[deref]
    #[deref_mut]
    score: usize,
}

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
