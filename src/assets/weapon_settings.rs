use bevy::prelude::*;
use derive_more::{Deref, DerefMut};
use serde::Deserialize;

use super::game_assets::GameAssets;
//use super::optional;

/// Loaded as part of the [crate::assets::GameAssets] collection, then inserted as a resource.
#[derive(Asset, Resource, Reflect, Deserialize, Debug, Clone, Deref, DerefMut)]
pub struct WeaponCollection(Vec<WeaponInfo>);

impl FromWorld for WeaponCollection {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<GameAssets>();
        world
            .resource::<Assets<WeaponCollection>>()
            .get(assets.weapon_settings.id())
            .unwrap()
            .clone()
    }
}

#[derive(Reflect, Deserialize, Debug, Clone)]
pub struct WeaponInfo {
    pub ammonition_idx: Vec<AmmonitionIndex>,
    pub weapon_ports: Vec<Vec2>,
}

#[derive(Deserialize, Debug, Reflect, Clone, Copy)]
pub enum AmmonitionIndex {
    Exact {
        index: usize,
        #[serde(default = "AmmonitionIndex::default_weight")]
        weight: f32,
    },
    Range {
        start: usize,
        end: usize,
        #[serde(default = "AmmonitionIndex::default_weight")]
        weight: f32,
    },
}

impl AmmonitionIndex {
    fn default_weight() -> f32 {
        1.0
    }
}
