use bevy::prelude::*;
use derive_more::{Deref, DerefMut, *};

#[derive(Resource, Debug, Default, Clone, Copy, Constructor, Deref, DerefMut)]
pub struct AsteroidCount(usize);
