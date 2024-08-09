use bevy::prelude::*;
use derive_more::derive::{Deref, From};

use crate::assets::AsteroidHitBehavior;

#[derive(Component, Debug, Clone, Copy)]
pub struct Asteroid;

#[derive(Component, Debug, Clone, Copy)]
pub struct AsteroidSprite;

#[derive(Component, Debug, Clone, Deref, From)]
pub struct HitBehavior(Vec<AsteroidHitBehavior>);
