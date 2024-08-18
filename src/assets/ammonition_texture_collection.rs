use std::collections::BTreeMap;

use bevy::prelude::*;
use derive_more::{Constructor, Deref, DerefMut};

use crate::assets::{game_assets::GameAssets, SpriteSheetAsset};

/// Loaded as part of the [crate::assets::AsteroidAssets] collection, then inserted as a [Resource].
#[derive(Asset, Resource, Reflect, Debug, Clone, Deref, DerefMut, Constructor)]
pub struct AmmonitionTextureCollection(BTreeMap<String, SpriteSheetAsset>);

impl FromWorld for AmmonitionTextureCollection {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<GameAssets>();
        let asset = world
            .resource::<Assets<AmmonitionTextureCollection>>()
            .get(assets.ammonition_texture_collection_handle.id())
            .unwrap()
            .clone();
        debug!(ammonition_sprites = asset.len());
        asset
    }
}
