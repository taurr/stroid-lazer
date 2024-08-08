use std::collections::BTreeMap;

use bevy::prelude::*;
use derive_more::{Deref, DerefMut};
use serde::Deserialize;

use super::game_assets::GameAssets;
use super::optional;

/// Loaded as part of the [crate::assets::GameAssets] collection, then inserted as a resource.
#[derive(Asset, Resource, Reflect, Deserialize, Debug, Clone, Deref, DerefMut)]
pub struct WeaponCollection(BTreeMap<String, WeaponInfo>);

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
    pub weapon_ports: Vec<WeaponsPort>,
    pub default_ammonition: Vec<AmmonitionSelection>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub audio: Option<String>,
}

#[derive(Reflect, Deserialize, Debug, Clone)]
pub struct WeaponsPort {
    pub position: Vec2,
    #[serde(default = "WeaponsPort::default_rotation")]
    pub rotation: f32,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub ammonition: Option<Vec<AmmonitionSelection>>,
}

impl WeaponsPort {
    fn default_rotation() -> f32 {
        0.0
    }
}

#[derive(Deserialize, Debug, Reflect, Clone)]
pub enum AmmonitionSelection {
    Exact {
        name: String,
        #[serde(default = "AmmonitionSelection::default_weight")]
        weight: f32,
    },
    IndexRange {
        start_index: usize,
        end_index: usize,
        #[serde(default = "AmmonitionSelection::default_weight")]
        weight: f32,
    },
}

impl AmmonitionSelection {
    fn default_weight() -> f32 {
        1.0
    }
}
