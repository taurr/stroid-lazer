use bevy::prelude::*;
use derive_more::{Constructor, Deref, DerefMut};

use crate::assets::SpriteSheetAsset;

use super::game_assets::GameAssets;

/// Loaded as part of the [crate::assets::AsteroidAssets] collection, then inserted as a [Resource].
#[derive(Asset, Resource, Reflect, Debug, Clone, Deref, DerefMut, Constructor)]
pub struct AmmonitionSpriteSheets(Vec<SpriteSheetAsset>);

impl FromWorld for AmmonitionSpriteSheets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<GameAssets>();
        let asset = world
            .resource::<Assets<AmmonitionSpriteSheets>>()
            .get(assets.ammonition_spritesheet.id())
            .unwrap()
            .clone();
        debug!(ammonition_sprites = asset.len());
        asset
    }
}
