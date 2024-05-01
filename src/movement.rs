use core::ops::Range;

use bevy::prelude::*;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

use crate::prelude::*;

/// A [SystemSet] for all systems added by the [MovementPlugin].
#[derive(Debug, Clone, Default, SystemSet, PartialEq, Eq, Hash)]
pub struct MovementSet;

/// A component that represents a linear movement with a speed and a direction from the X axis.
#[derive(Component, Default, Debug, Clone, Constructor)]
pub struct LinearMovement {
    pub speed: f32,
    pub direction: Quat,
}

/// A component that clamps the movement speed of a [LinearMovement] to a given range.
#[derive(Component, SmartDefault, Debug, Clone, Constructor)]
pub struct ClampMovementSpeed {
    #[default(0.0..f32::MAX)]
    pub range: Range<f32>,
}

/// A component that decays the movement speed of a [LinearMovement].
///
/// `decay` is the rate of decay per second, where 1.0 is 100% decay per second.
#[derive(Component, Deref, DerefMut, Debug, Constructor)]
pub struct LinearMovementDecay {
    pub decay: f32,
}

#[derive(Debug, Clone, Component, Constructor)]
pub struct RotatingMovement {
    direction: Quat,
}

/// A component that makes sure an entity is within a [GameArea].
///
/// Either the entity itself, or one of its parents must have a [GameArea] component.
#[derive(Component, Debug, Clone, Constructor)]
pub struct Wrapping;

/// Describes a game area for constraining movement using the [Wrapping] component.
#[derive(Debug, Clone, Deserialize, Serialize, Component)]
pub struct GameArea {
    min: Vec3,
    max: Vec3,
}

impl GameArea {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        assert!(min.x <= max.x);
        assert!(min.y <= max.y);
        Self { min, max }
    }

    pub fn horizontal_range(&self) -> Range<f32> {
        self.min.x..self.max.x
    }

    pub fn vertical_range(&self) -> Range<f32> {
        self.min.y..self.max.y
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    #[must_use]
    pub fn wrap(&self, position: &Vec3) -> Vec3 {
        Vec3::new(
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
            position.z,
        )
    }
}

/// Plugin for handling automatic movement.
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (decay_and_clamp_linear_movement, initialize_wrapping),
                (
                    wrapping_non_linear_movement,
                    rotating_movement,
                    linear_movement_with_optional_wrapping,
                ),
            )
                .chain()
                .in_set(GameSet::Movement)
                .in_set(MovementSet),
        );
    }
}

fn rotating_movement(time: Res<Time>, mut query: Query<(&mut Transform, &RotatingMovement)>) {
    query.par_iter_mut().for_each(|(mut transform, movement)| {
        let (rotation_axis, angle) = movement.direction.to_axis_angle();
        let angle = angle * time.delta_seconds();
        transform.rotate(Quat::from_axis_angle(rotation_axis, angle));
    });
}

#[derive(Component, Debug, Clone, Deref, Constructor)]
struct WrappingGameAreaOn(Entity);

fn initialize_wrapping(
    mut commands: Commands,
    new_wrappings_query: Query<Entity, Added<Wrapping>>,
    game_area_query: Query<&GameArea>,
    parent_query: Query<&Parent>,
) {
    'wrap_loop: for wrapping_entity in new_wrappings_query.iter() {
        let mut game_area_entity = wrapping_entity;
        loop {
            match game_area_query.get(game_area_entity) {
                Ok(_) => {
                    commands
                        .entity(wrapping_entity)
                        .insert(WrappingGameAreaOn(game_area_entity));
                    continue 'wrap_loop;
                }
                Err(_) => {
                    let Ok(parent_entity) = parent_query.get(game_area_entity) else {
                        warn!(?wrapping_entity, "No game area found for wrapping entity");
                        continue 'wrap_loop;
                    };
                    game_area_entity = parent_entity.get();
                }
            };
        }
    }
}

fn wrapping_non_linear_movement(
    mut query: Query<(&mut Transform, &WrappingGameAreaOn), Without<LinearMovement>>,
    game_area_query: Query<&GameArea>,
) {
    query
        .par_iter_mut()
        .for_each(|(mut transform, wrapping_on)| {
            let Ok(game_area) = game_area_query.get(wrapping_on.0) else {
                warn!(entity=?wrapping_on.0, "No game area found for wrapping entity");
                return;
            };
            transform.translation = game_area.wrap(&transform.translation);
        });
}

fn linear_movement_with_optional_wrapping(
    mut query: Query<(&mut Transform, &LinearMovement, Option<&WrappingGameAreaOn>)>,
    game_area_query: Query<&GameArea>,
    time: Res<Time>,
) {
    query
        .par_iter_mut()
        .for_each(|(mut transform, movement, wrapping_on)| {
            transform.translation += movement
                .direction
                .mul_vec3(Vec3::X * movement.speed * time.delta_seconds());
            if let Some(wrapping_on) = wrapping_on {
                let Ok(game_area) = game_area_query.get(wrapping_on.0) else {
                    warn!(entity=?wrapping_on.0, "No game area found for wrapping entity");
                    return;
                };
                transform.translation = game_area.wrap(&transform.translation);
            }
        });
}

fn decay_and_clamp_linear_movement(
    mut query: Query<(
        &mut LinearMovement,
        &LinearMovementDecay,
        Option<&ClampMovementSpeed>,
    )>,
    time: Res<Time>,
) {
    query
        .par_iter_mut()
        .for_each(|(mut movement, decay, clamp)| {
            // decay the movement speed
            movement.speed *= (1.0 - decay.decay * time.delta_seconds()).clamp(0.0, 1.0);

            // clamp the movement speed
            if let Some(clamp) = clamp {
                movement.speed = movement.speed.clamp(clamp.range.start, clamp.range.end);
            }
        });
}

#[cfg(test)]
mod tests {
    use core::time::Duration;

    use super::*;

    trait TimeAdvance {
        fn advance_time_by(&mut self, duration: Duration);
    }

    impl TimeAdvance for App {
        fn advance_time_by(&mut self, duration: Duration) {
            self.world
                .get_resource_mut::<Time>()
                .unwrap()
                .advance_by(duration);
        }
    }

    #[test]
    fn linear_movement() {
        let mut app = App::new();
        app.add_systems(Update, linear_movement_with_optional_wrapping);
        app.init_resource::<Time>();
        app.update();

        let entity_id = app
            .world
            .spawn((
                SpatialBundle::default(),
                LinearMovement {
                    speed: 1.0,
                    direction: Quat::from_rotation_z(0.0),
                },
            ))
            .id();

        app.advance_time_by(Duration::from_secs(1));
        app.update();
        assert_eq!(
            app.world.get::<Transform>(entity_id).unwrap().translation,
            (1.0, 0.0, 0.0).into()
        );
    }
}
