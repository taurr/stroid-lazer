
use bevy::prelude::*;
use serde::Deserialize;

use super::{
    AsteroidSplitCount, AsteroidSplitSelection,
};
#[derive(Resource, Deserialize, Reflect, Debug, Clone)]
pub struct LevelStartupSettings {
    pub asteroids: LevelAsteroidStartupSettings,
}

#[derive(Resource, Deserialize, Reflect, Debug, Clone)]
pub struct LevelAsteroidStartupSettings {
    pub count: AsteroidSplitCount,
    pub select_from: Vec<AsteroidSplitSelection>,
}
