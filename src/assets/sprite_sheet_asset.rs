use avian2d::prelude::{Collider, ColliderDensity, CollisionMargin};
use bevy::{ecs::system::EntityCommands, prelude::*, sprite::Anchor};

use crate::assets::{ColliderType, Physics};

/// This asset can be loaded from a file using the [SpriteDynamicAssetCollection].
/// It is also part of the [AsteroidSpriteSheets] and [AmmonitionSpriteSheets] collections.
///
/// The purpose of this [Asset] is to define a complete [SpriteBundle] and easily insert it for an entity.
#[derive(Asset, Reflect, Debug, Clone)]
pub struct SpriteSheetAsset {
    /// Optional name, used for for a name component on the Entity with a SpriteBundle
    pub name: Option<String>,

    pub texture: Handle<Image>,
    pub texture_count: usize,
    pub anchor: Anchor,
    pub size: Option<Vec2>,
    pub flip_x: bool,
    pub flip_y: bool,
    pub color: Color,
    pub transform: Transform,

    pub atlas: Option<Handle<bevy::prelude::TextureAtlasLayout>>,
    pub atlas_index: usize,

    pub physics: Option<Vec<Physics>>,
}

#[derive(Component, Reflect, Debug, Clone, Copy)]
pub struct TextureCount(usize);

pub trait EntitySpriteSheetCommands {
    fn insert_spritesheet<F: Fn() -> T, T: Bundle>(
        &mut self,
        spritesheet: &SpriteSheetAsset,
        atlas_index: Option<usize>,
        physics_bundle: F,
    ) -> &mut Self;
}

// TODO: is this a custom [Command]?
impl<'a> EntitySpriteSheetCommands for EntityCommands<'a> {
    fn insert_spritesheet<F: Fn() -> T, T: Bundle>(
        &mut self,
        spritesheet: &SpriteSheetAsset,
        atlas_index: Option<usize>,
        physics_bundle: F,
    ) -> &mut Self {
        if let Some(name) = &spritesheet.name {
            self.insert(Name::new(name.clone()));
        }

        self.insert((
            SpriteBundle {
                sprite: Sprite {
                    anchor: spritesheet.anchor,
                    custom_size: spritesheet.size,
                    flip_x: spritesheet.flip_x,
                    flip_y: spritesheet.flip_y,
                    color: spritesheet.color,
                    ..Default::default()
                },
                texture: spritesheet.texture.clone(),
                transform: spritesheet.transform,
                ..Default::default()
            },
            TextureCount(spritesheet.texture_count),
        ));

        if let Some(atlas_layout) = &spritesheet.atlas {
            self.insert(TextureAtlas {
                index: atlas_index.unwrap_or(spritesheet.atlas_index),
                ..TextureAtlas::from(atlas_layout.clone())
            });
        }

        if let Some(physics_collection) = &spritesheet.physics {
            for physics in physics_collection {
                self.with_children(|child| {
                    let mut transform = physics
                        .position
                        .map(|p| Transform::from_translation(p.extend(0.0)))
                        .unwrap_or_default();
                    if let Some(rotation) = physics.rotation {
                        transform.rotation = Quat::from_rotation_z(rotation);
                    }

                    let mut child = child.spawn((
                        SpatialBundle {
                            transform,
                            ..Default::default()
                        },
                        Name::new("Physics collider"),
                        physics_bundle(),
                    ));

                    let collider = match physics.collider {
                        ColliderType::Circle(r) => Collider::circle(r),
                        ColliderType::Ellipse { x, y } => Collider::ellipse(x, y),
                        ColliderType::Rectangle { x, y } => Collider::rectangle(x, y),
                        ColliderType::RoundRectangle {
                            x,
                            y,
                            radius: border_radius,
                        } => Collider::round_rectangle(x, y, border_radius),
                        ColliderType::Capsule { radius, length } => {
                            Collider::capsule(radius, length)
                        }
                        ColliderType::CapsuleEndpoints { p1, p2, radius } => {
                            Collider::capsule_endpoints(radius, p1, p2)
                        }
                        ColliderType::Triangle { p1, p2, p3 } => Collider::triangle(p1, p2, p3),
                        ColliderType::Polygon { radius, sides } => {
                            Collider::regular_polygon(radius, sides as usize)
                        }
                    };
                    child.insert(collider);

                    if let Some(density) = physics.density {
                        child.insert(ColliderDensity(density));
                    }

                    if let Some(margin) = physics.margin {
                        child.insert(CollisionMargin(margin));
                    }

                    if let Some(friction) = physics.friction {
                        child.insert(friction);
                    }

                    if let Some(restitution) = physics.restitution {
                        child.insert(restitution);
                    }
                });
            }
        }

        self
    }
}
