//! Here we define the syntax for loading [SpriteSheetAsset]s dynamically, using the [SpriteDynamicAssetCollection].
//!
use std::collections::BTreeMap;

use avian2d::prelude::{Friction, Restitution};
use bevy::{
    ecs::system::SystemState,
    prelude::*,
    render::texture::{ImageSampler, ImageSamplerDescriptor},
    utils::HashMap,
};
use bevy_asset_loader::prelude::*;
use serde::{Deserialize, Serialize};

use crate::assets::{
    optional, AmmonitionTextureCollection, AsteroidTextureCollection, SpriteSheetAsset,
};

#[derive(Asset, TypePath, Deserialize, Debug, Default, Clone)]
pub struct SpriteDynamicAssetCollection(HashMap<String, SpriteDynamicAsset>);

impl DynamicAssetCollection for SpriteDynamicAssetCollection {
    fn register(&self, dynamic_assets: &mut DynamicAssets) {
        for (key, asset) in self.0.iter() {
            dynamic_assets.register_asset(key, Box::new(asset.clone()));
        }
    }
}

/// This defines the syntax for your dynamic asset when loading it.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
enum SpriteDynamicAsset {
    /// A dynamic asset directly loaded from a single file
    File {
        /// Asset file path
        path: String,
    },
    SpriteSheet(SpriteSheet),
    AsteroidTextureCollection(BTreeMap<String, SpriteSheet>),
    AmmonitionTextureCollection(BTreeMap<String, SpriteSheet>),
}

#[derive(Deserialize, Serialize, Debug, Clone, Reflect, PartialEq)]
struct SpriteSheet {
    /// Optional name of the asset
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    name: Option<String>,

    /// Path to the image file
    texture: String,

    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    anchor: Option<Anchor>,

    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    size: Option<Vec2>,

    #[serde(
        skip_serializing_if = "SpriteSheet::is_false",
        default = "SpriteSheet::default_false"
    )]
    flip_x: bool,

    #[serde(
        skip_serializing_if = "SpriteSheet::is_false",
        default = "SpriteSheet::default_false"
    )]
    flip_y: bool,

    #[serde(
        skip_serializing_if = "SpriteSheet::is_white",
        default = "SpriteSheet::color_white"
    )]
    color: Color,

    /// Optional image sampler - Nearest or Linear
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    sampler: Option<ImageSamplerType>,

    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub position: Option<Vec3>,

    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub rotation: Option<f32>,

    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    atlas: Option<Atlas>,

    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    physics: Option<Vec<Physics>>,
}

impl SpriteSheet {
    fn is_false(value: &bool) -> bool {
        !*value
    }

    fn is_white(value: &Color) -> bool {
        *value == Color::WHITE
    }

    fn default_false() -> bool {
        false
    }
    fn color_white() -> Color {
        Color::WHITE
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Reflect)]
struct Atlas {
    /// layout of the atlas
    layout: AtlasGridLayout,

    /// default index to use when displaying an image with the atlas
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    index: Option<usize>,

    /// number of textures in the atlas
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    texture_count: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Reflect)]
pub struct Physics {
    pub collider: ColliderType,

    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub density: Option<f32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub margin: Option<f32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub friction: Option<Friction>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub restitution: Option<Restitution>,

    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub position: Option<Vec2>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub rotation: Option<f32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Reflect, Default)]
enum Anchor {
    #[default]
    Center,
    BottomLeft,
    BottomCenter,
    BottomRight,
    CenterLeft,
    CenterRight,
    TopLeft,
    TopCenter,
    TopRight,
    /// Custom anchor point. Top left is `(-0.5, 0.5)`, center is `(0.0, 0.0)`. The value will
    /// be scaled with the sprite size.
    Custom(Vec2),
}

/// Define the image sampler to configure for an image asset
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Reflect)]
pub enum ImageSamplerType {
    Nearest,
    Linear,
}

impl From<ImageSamplerType> for ImageSamplerDescriptor {
    fn from(value: ImageSamplerType) -> Self {
        match value {
            ImageSamplerType::Nearest => ImageSamplerDescriptor::nearest(),
            ImageSamplerType::Linear => ImageSamplerDescriptor::linear(),
        }
    }
}

impl From<ImageSamplerType> for ImageSampler {
    fn from(value: ImageSamplerType) -> Self {
        match value {
            ImageSamplerType::Nearest => ImageSampler::nearest(),
            ImageSamplerType::Linear => ImageSampler::linear(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Reflect, PartialEq)]
pub struct AtlasGridLayout {
    /// The image size in pixels
    pub size: UVec2,
    /// Columns on the sprite sheet
    pub columns: u32,
    /// Rows on the sprite sheet
    pub rows: u32,
    /// Padding between columns/rows in pixels
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub padding: Option<UVec2>,
    /// Number of pixels offset of the first tile
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub offset: Option<UVec2>,
}

impl From<AtlasGridLayout> for TextureAtlasLayout {
    fn from(layout: AtlasGridLayout) -> Self {
        TextureAtlasLayout::from_grid(
            layout.size,
            layout.columns,
            layout.rows,
            layout.padding,
            layout.offset,
        )
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Reflect, PartialEq)]
pub enum ColliderType {
    Circle(f32),
    Ellipse { x: f32, y: f32 },
    Rectangle { x: f32, y: f32 },
    RoundRectangle { x: f32, y: f32, radius: f32 },
    Capsule { radius: f32, length: f32 },
    CapsuleEndpoints { p1: Vec2, p2: Vec2, radius: f32 },
    Triangle { p1: Vec2, p2: Vec2, p3: Vec2 },
    Polygon { radius: f32, sides: u32 },
}

impl DynamicAsset for SpriteDynamicAsset {
    fn load(&self, asset_server: &AssetServer) -> Vec<UntypedHandle> {
        match self {
            SpriteDynamicAsset::File { path } => {
                vec![asset_server.load_untyped(path).untyped()]
            }

            SpriteDynamicAsset::SpriteSheet(SpriteSheet { texture, .. }) => {
                vec![asset_server.load_untyped(texture).untyped()]
            }

            SpriteDynamicAsset::AsteroidTextureCollection(sprite_sheets)
            | SpriteDynamicAsset::AmmonitionTextureCollection(sprite_sheets) => sprite_sheets
                .iter()
                .map(|(_, SpriteSheet { texture, .. })| {
                    asset_server.load_untyped(texture).untyped()
                })
                .collect(),
        }
    }

    fn build(&self, world: &mut World) -> Result<DynamicAssetType, anyhow::Error> {
        match self {
            SpriteDynamicAsset::File { path } => {
                let mut system_state = SystemState::<Res<AssetServer>>::new(world);
                let asset_server = system_state.get_mut(world);
                Ok(DynamicAssetType::Single(
                    asset_server.get_handle_untyped(path).unwrap(),
                ))
            }

            SpriteDynamicAsset::SpriteSheet(spritesheet) => {
                let typed_asset = self.build_spritesheet_asset(None, spritesheet, world);
                let untyped_asset = self.add_asset(world, typed_asset);
                Ok(DynamicAssetType::Single(untyped_asset))
            }

            SpriteDynamicAsset::AsteroidTextureCollection(spritesheets) => {
                let spritesheets = self.build_spritesheet_vec(spritesheets, world);
                Ok(DynamicAssetType::Single(self.add_asset(
                    world,
                    AsteroidTextureCollection::new(spritesheets),
                )))
            }

            SpriteDynamicAsset::AmmonitionTextureCollection(spritesheets) => {
                let spritesheets = self.build_spritesheet_vec(spritesheets, world);
                Ok(DynamicAssetType::Single(self.add_asset(
                    world,
                    AmmonitionTextureCollection::new(spritesheets),
                )))
            }
        }
    }
}

impl SpriteDynamicAsset {
    fn add_asset<T: Asset>(&self, world: &mut World, asset: T) -> UntypedHandle {
        let mut system_state = SystemState::<ResMut<Assets<T>>>::new(world);
        let mut asset_collection = system_state.get_mut(world);

        asset_collection.add(asset).untyped()
    }

    fn build_spritesheet_vec(
        &self,
        spritesheets: &BTreeMap<String, SpriteSheet>,
        world: &mut World,
    ) -> BTreeMap<String, SpriteSheetAsset> {
        spritesheets
            .iter()
            .map(|(key, spritesheet)| {
                (
                    key.clone(),
                    self.build_spritesheet_asset(Some(key), spritesheet, world),
                )
            })
            .collect()
    }

    fn build_spritesheet_asset(
        &self,
        key: Option<&str>,
        spritesheet: &SpriteSheet,
        world: &mut World,
    ) -> SpriteSheetAsset {
        let mut system_state = SystemState::<(
            ResMut<Assets<bevy::prelude::TextureAtlasLayout>>,
            ResMut<Assets<bevy::prelude::Image>>,
            Res<AssetServer>,
        )>::new(world);
        let (mut atlas_assets, mut image_assets, asset_server) = system_state.get_mut(world);

        let mut texture_handle = asset_server.load(&spritesheet.texture);
        if let Some(sampler) = spritesheet.sampler.as_ref() {
            Self::update_image_sampler(&mut texture_handle, &mut image_assets, sampler);
        }

        let mut transform = spritesheet
            .position
            .map(Transform::from_translation)
            .unwrap_or_default();
        if let Some(rotation) = spritesheet.rotation {
            transform.rotation = Quat::from_rotation_z(rotation);
        }

        let (atlas, texture_count, atlas_index) = match &spritesheet.atlas {
            Some(atlas) => {
                let layout = TextureAtlasLayout::from(atlas.layout.clone());
                let default_texture_count = layout.textures.len();
                let atlas_index = atlas.index.unwrap_or(0);
                let handle = atlas_assets.add(layout);
                (
                    Some(handle),
                    atlas.texture_count.unwrap_or(default_texture_count),
                    atlas_index,
                )
            }
            None => (None, 1, 0),
        };

        crate::assets::SpriteSheetAsset {
            name: spritesheet
                .name
                .clone()
                .or_else(|| key.map(|key| key.to_string())),
            texture: texture_handle,
            texture_count,
            anchor: match spritesheet.anchor {
                Some(Anchor::Center) => bevy::sprite::Anchor::Center,
                Some(Anchor::BottomLeft) => bevy::sprite::Anchor::BottomLeft,
                Some(Anchor::BottomCenter) => bevy::sprite::Anchor::BottomCenter,
                Some(Anchor::BottomRight) => bevy::sprite::Anchor::BottomRight,
                Some(Anchor::CenterLeft) => bevy::sprite::Anchor::CenterLeft,
                Some(Anchor::CenterRight) => bevy::sprite::Anchor::CenterRight,
                Some(Anchor::TopLeft) => bevy::sprite::Anchor::TopLeft,
                Some(Anchor::TopCenter) => bevy::sprite::Anchor::TopCenter,
                Some(Anchor::TopRight) => bevy::sprite::Anchor::TopRight,
                Some(Anchor::Custom(c)) => bevy::sprite::Anchor::Custom(c),
                None => bevy::sprite::Anchor::Center,
            },
            size: spritesheet.size,
            flip_x: spritesheet.flip_x,
            flip_y: spritesheet.flip_y,
            color: spritesheet.color,
            transform,
            atlas,
            atlas_index,
            physics: spritesheet.physics.clone(),
        }
    }

    fn update_image_sampler(
        handle: &mut Handle<Image>,
        images: &mut Assets<Image>,
        sampler_type: &ImageSamplerType,
    ) {
        let image = images.get_mut(&*handle).unwrap();
        let is_different_sampler = if let ImageSampler::Descriptor(descriptor) = &image.sampler {
            let configured_descriptor: ImageSamplerDescriptor = sampler_type.clone().into();
            !descriptor.as_wgpu().eq(&configured_descriptor.as_wgpu())
        } else {
            false
        };

        if is_different_sampler {
            let mut cloned_image = image.clone();
            cloned_image.sampler = sampler_type.clone().into();
            *handle = images.add(cloned_image);
        } else {
            image.sampler = sampler_type.clone().into();
        }
    }
}
