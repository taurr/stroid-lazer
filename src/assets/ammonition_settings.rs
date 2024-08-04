use core::time::Duration;

use bevy::prelude::*;
use derive_more::{Deref, DerefMut};
use serde::Deserialize;

use super::game_assets::GameAssets;
//use super::optional;

/// Loaded as part of the [GameAssets] collection, then inserted as a [Resource].
#[derive(Asset, Resource, Reflect, Deserialize, Debug, Clone, Deref, DerefMut)]
pub struct AmmonitionDepot(Vec<AmmonitionInfo>);

impl FromWorld for AmmonitionDepot {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<GameAssets>();
        world
            .resource::<Assets<AmmonitionDepot>>()
            .get(assets.ammonition_settings.id())
            .unwrap()
            .clone()
    }
}

#[derive(Reflect, Deserialize, Debug, Clone)]
pub struct AmmonitionInfo {
    pub spritesheet_idx: usize,
    pub speed: f32,
    pub timeout: Duration,
}
