use core::time::Duration;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::assets::game_assets::GameAssets;
//use crate::assets::optional;

/// Loaded directly as part of the [crate::assets::GameAssets] collection, then inserted as a [Resource].
#[derive(Asset, Resource, Deserialize, Serialize, Reflect, Debug, Clone)]
pub struct InputKeySettings {
    pub auto_fire: Duration,
    pub rotate_left: KeyCode,
    pub rotate_right: KeyCode,
    pub accelerate: KeyCode,
    pub fire: KeyCode,
    pub jump: KeyCode,
    pub pause: KeyCode,
}

impl FromWorld for InputKeySettings {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<GameAssets>();
        world
            .resource::<Assets<InputKeySettings>>()
            .get(assets.input_keys.id())
            .unwrap()
            .clone()
    }
}
