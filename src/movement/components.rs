use core::ops::Range;

use avian2d::prelude::*;
use bevy::prelude::*;
use derive_more::{Constructor, Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

/// When put on an [Entity] it cause the entity to temporarily
/// loose it's [LinearVelocity] and [AngularVelocity], thus pausing movement.
#[derive(Component, Reflect, Debug, Clone)]
pub struct PauseMovement;

/// Component added to entities that we've pause automatically when leaving the [PlayState::Running] state.
#[derive(Component, Reflect, Debug, Clone)]
pub(super) struct AutoMovementPaused;

/// A component that clamps the movement speed of a [LinearVelocity] to a given range.
#[derive(Component, Reflect, Debug, Clone, Constructor, SmartDefault)]
pub struct ClampMovementSpeed {
    #[default(0.0..f32::MAX)]
    pub range: Range<f32>,
}

#[derive(Component, Reflect, Debug, Constructor, Deref, DerefMut, From, Into)]
pub struct PausedLinearVelocity(pub LinearVelocity);

#[derive(Component, Reflect, Debug, Constructor, Deref, DerefMut, From, Into)]
pub struct PausedAngularVelocity(pub AngularVelocity);

/// Describes a game area for constraining movement using the [Wrapping] component.
///
/// Any [Entity] that has a [Wrapping] component must also have a [GameArea]
/// component in the hierarchy.
#[derive(Component, Reflect, Debug, Clone, Deserialize, Serialize)]
pub struct GameArea {
    min: Vec3,
    max: Vec3,
}

impl GameArea {
    #[allow(unused)]
    pub fn new(min: Vec3, max: Vec3) -> Self {
        assert!(min.x <= max.x);
        assert!(min.y <= max.y);
        Self { min, max }
    }

    #[allow(unused)]
    pub fn min(&self) -> Vec3 {
        self.min
    }

    #[allow(unused)]
    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn horizontal_range(&self) -> Range<f32> {
        self.min.x..self.max.x
    }

    pub fn vertical_range(&self) -> Range<f32> {
        self.min.y..self.max.y
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    #[must_use]
    pub fn wrap(&self, position: &Position) -> Position {
        Position::new(Vec2::new(
            if position.x > self.max.x {
                (position.x - self.width()).clamp(self.min.x, self.max.x)
            } else if position.x < self.min.x {
                (position.x + self.width()).clamp(self.min.x, self.max.x)
            } else {
                position.x
            },
            if position.y > self.max.y {
                (position.y - self.height()).clamp(self.min.y, self.max.y)
            } else if position.y < self.min.x {
                (position.y + self.height()).clamp(self.min.y, self.max.y)
            } else {
                position.y
            },
        ))
    }
}

/// A component that makes sure an entity is within a [GameArea].
///
/// Either the entity itself, or one of its parents must have a [GameArea] component.
#[derive(Component, Debug, Clone, Constructor)]
pub struct Wrapping;

/// A component pointing to the entity has the [GameArea] component.
#[derive(Component, Reflect, Debug, Clone, Deref, Constructor)]
pub(super) struct WrappingGameAreaOn(Entity);
