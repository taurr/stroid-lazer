use std::collections::BTreeMap;

use bevy::prelude::*;
use derive_more::{Deref, DerefMut};
use serde::Deserialize;

use super::optional;
use super::{game_assets::GameAssets, AsteroidSelection};

/// Loaded as part of the [crate::assets::AsteroidAssets] collection, then inserted as a [Resource].
#[derive(Asset, Resource, Reflect, Deserialize, Debug, Clone, Deref, DerefMut)]
pub struct AsteroidPoolCollection(BTreeMap<String, AsteroidPool>);

impl FromWorld for AsteroidPoolCollection {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<GameAssets>();
        world
            .resource::<Assets<AsteroidPoolCollection>>()
            .get(assets.asteroid_pool_settings.id())
            .unwrap()
            .clone()
    }
}

#[derive(Reflect, Deserialize, Debug, Clone)]
pub struct AsteroidPool {
    /// spritesheets to choose from when spawning from this pool
    pub textures: Vec<AsteroidTextureSelection>,
    /// default range to find the asteroid speed within
    pub speed: AsteroidSpeedRange,
    /// how far to displace the asteroid position when spawning as a result of being shot!
    pub displacement: AsteroidDisplacement,
    /// default rotation of the asteroid
    pub rotation: AsteroidRotationSpeed,
    /// how does asteroid behave when hit
    pub hit_behavior: AsteroidHitBehavior,
}

#[derive(Deserialize, Debug, Reflect, Clone)]
pub enum AsteroidTextureSelection {
    AtlasIndex {
        /// sprite sheet key in the [AsteroidTextureCollection](super::asteroid_texture_collection::AsteroidTextureCollection)
        key: String,
        /// index of sprite if in an atlas
        #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
        atlas_idx: Option<usize>,
        /// range to find the asteroid speed within (overrides setting of the pool)
        #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
        speed: Option<AsteroidSpeedRange>,
        /// how to rotate the asteroid (overrides setting of the pool)
        #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
        rotation: Option<AsteroidRotationSpeed>,
    },
}

#[derive(Component, Deserialize, Debug, Reflect, Clone)]
pub enum AsteroidHitBehavior {
    None,
    Points(usize),
    PointsAndSplit {
        points: usize,
        count: AsteroidSplitCount,
        select_from: Vec<AsteroidSelection>,
    },
}

#[derive(Deserialize, Debug, Reflect, Clone, Copy)]
pub enum AsteroidSplitCount {
    Exact(usize),
    Range { start: usize, end: usize },
}

#[derive(Deserialize, Debug, Reflect, Clone, Copy)]
pub enum AsteroidSpeedRange {
    None,
    Exact(usize),
    Range { start: f32, end: f32 },
}

#[derive(Deserialize, Debug, Reflect, Clone, Copy)]
pub enum AsteroidDisplacement {
    None,
    Exact(f32),
    Range { start: f32, end: f32 },
}

#[derive(Deserialize, Debug, Reflect, Clone, Copy)]
pub enum AsteroidRotationSpeed {
    None,
    Exact(f32),
    Range { start: f32, end: f32 },
}
