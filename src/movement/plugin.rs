use avian2d::PhysicsPlugins;
use bevy::prelude::*;

use super::{
    systems::*, ClampMovementSpeed, GameArea, PausedAngularVelocity, PausedLinearVelocity,
    WrappingGameAreaOn,
};

/// Plugin for handling automatic movement.
pub struct MovementPlugin;

/// A [SystemSet] for all systems added by the [MovementPlugin].
#[derive(Debug, Clone, Default, SystemSet, PartialEq, Eq, Hash)]
pub struct MovementSet;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameArea>()
            .register_type::<ClampMovementSpeed>()
            .register_type::<PausedLinearVelocity>()
            .register_type::<PausedAngularVelocity>()
            .register_type::<WrappingGameAreaOn>()
            .add_plugins(PhysicsPlugins::default());

        app.add_systems(
            PreUpdate,
            (
                decay_linear_movement_velocity.pipe(clamp_linear_movement_velocity),
                on_wrapping_added,
                wrap_rigid_bodies,
                unpause_movement,
                pause_movement,
            )
                .in_set(MovementSet),
        );
    }
}
