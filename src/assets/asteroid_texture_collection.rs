use std::collections::BTreeMap;

use bevy::prelude::*;
use derive_more::{Constructor, Deref, DerefMut};

use crate::assets::{game_assets::GameAssets, SpriteSheetAsset};

/// Loaded as part of the [crate::assets::AsteroidAssets] collection, then inserted as a [Resource].
#[derive(Asset, Resource, Reflect, Debug, Clone, Deref, DerefMut, Constructor)]
pub struct AsteroidTextureCollection(BTreeMap<String, SpriteSheetAsset>);

impl FromWorld for AsteroidTextureCollection {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<GameAssets>();
        let asset = world
            .resource::<Assets<AsteroidTextureCollection>>()
            .get(assets.asteroid_texture_collection_handle.id())
            .unwrap()
            .clone();
        debug!(asteroid_sprites = asset.len());
        asset
    }
}
