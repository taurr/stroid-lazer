use std::ops::Range;

use bevy::prelude::Vec3;

pub const PLAYINGFIELD_POS: Vec3 = Vec3::ZERO;
pub const PLAYINGFIELD_BORDER_RELATIVE_Z_POS: f32 = 1.0;
pub const PLAYINGFIELD_BACKGROUND_RELATIVE_Z_POS: f32 = -100.0;

pub const PLAYER_Z_POS: f32 = 0.0;
pub const ASTEROID_Z_RANGE: Range<f32> = 10.0..20.0;
