use core::time::Duration;
use std::collections::BTreeMap;

use bevy::prelude::*;
use derive_more::{Deref, DerefMut};
use serde::Deserialize;

use crate::assets::game_assets::GameAssets;

/// Loaded as part of the [GameAssets] collection, then inserted as a [Resource].
#[derive(Asset, Resource, Reflect, Deserialize, Debug, Clone, Deref, DerefMut)]
pub struct AmmonitionDepot(BTreeMap<String, AmmonitionInfo>);

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
    pub texture_key: String,
    pub speed: f32,
    pub timeout: Duration,
}
