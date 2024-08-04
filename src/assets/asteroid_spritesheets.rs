use bevy::prelude::*;
use derive_more::{Constructor, Deref, DerefMut};

use crate::assets::SpriteSheetAsset;

use super::game_assets::GameAssets;
//use super::optional;

/// Loaded as part of the [crate::assets::AsteroidAssets] collection, then inserted as a [Resource].
#[derive(Asset, Resource, Reflect, Debug, Clone, Deref, DerefMut, Constructor)]
pub struct AsteroidSpriteSheets(Vec<SpriteSheetAsset>);

impl FromWorld for AsteroidSpriteSheets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<GameAssets>();
        let asset = world
            .resource::<Assets<AsteroidSpriteSheets>>()
            .get(assets.asteroid_spritesheets.id())
            .unwrap()
            .clone();
        debug!(asteroid_sprites = asset.len());
        asset
    }
}
