use bevy::prelude::*;
use derive_more::{Constructor, Debug, Deref, DerefMut};

#[derive(Resource, Debug, Default, Clone, Copy, Constructor, Deref, DerefMut)]
pub struct AsteroidCount(usize);
