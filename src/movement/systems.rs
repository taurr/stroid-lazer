use avian2d::prelude::*;
use bevy::prelude::*;
use tracing::instrument;

use crate::projectile::Projectile;

use super::*;

pub fn wrap_rigid_bodies(
    mut query: Query<(&mut Position, &WrappingGameAreaOn)>,
    game_area_query: Query<&GameArea>,
) {
    for (mut position, wrapping_entity) in query.iter_mut() {
        let Ok(game_area) = game_area_query.get(**wrapping_entity) else {
            continue;
        };
        *position = game_area.wrap(&position);
    }
}

pub fn clamp_linear_movement_velocity(
    mut query: Query<(&mut LinearVelocity, &ClampMovementSpeed)>,
) {
    for (mut velocity, clamp) in query.iter_mut() {
        velocity.0 = velocity.0.clamp_length(clamp.range.start, clamp.range.end);
    }
}

pub fn decay_linear_movement_velocity(
    time: Res<Time>,
    mut query: Query<(&mut LinearVelocity, &RigidBody, &LinearDamping)>,
) {
    for (mut velocity, body, decay) in query.iter_mut() {
        if body.is_kinematic() {
            velocity.0 *= 1.0 - decay.0 * time.delta_seconds();
        }
    }
}

pub fn unpause_movement(
    mut commands: Commands,
    mut linear_query: Query<(Entity, &PausedLinearVelocity), Without<PauseMovement>>,
    mut angular_query: Query<(Entity, &PausedAngularVelocity), Without<PauseMovement>>,
) {
    for (entity, velocity) in linear_query.iter_mut() {
        let velocity = velocity.0;
        commands
            .entity(entity)
            .remove::<PausedLinearVelocity>()
            .insert(velocity);
    }

    for (entity, velocity) in angular_query.iter_mut() {
        let velocity = velocity.0;
        commands
            .entity(entity)
            .remove::<PausedAngularVelocity>()
            .insert(velocity);
    }
}

pub fn pause_movement(
    mut commands: Commands,
    mut linear_query: Query<(Entity, &LinearVelocity), With<PauseMovement>>,
    mut angular_query: Query<(Entity, &AngularVelocity), With<PauseMovement>>,
) {
    for (entity, velocity) in linear_query.iter_mut() {
        commands
            .entity(entity)
            .remove::<LinearVelocity>()
            .insert(PausedLinearVelocity(*velocity));
    }

    for (entity, velocity) in angular_query.iter_mut() {
        commands
            .entity(entity)
            .remove::<AngularVelocity>()
            .insert(PausedAngularVelocity(*velocity));
    }
}

#[instrument(skip_all)]
pub fn on_wrapping_added(
    mut commands: Commands,
    wrapped_query: Query<Entity, (With<Wrapping>, Without<WrappingGameAreaOn>)>,
    game_area_query: Query<(Option<&GameArea>, Option<&Parent>)>,
) {
    for wrapping_entity in wrapped_query.iter() {
        let mut possible_area_entity = Some(wrapping_entity);

        while let Some(area_entity) = possible_area_entity {
            let Ok((game_area, parent)) = game_area_query.get(area_entity) else {
                break;
            };

            if game_area.is_none() {
                possible_area_entity = parent.map(|p| p.get());
                continue;
            }

            trace!(entity=?wrapping_entity, wrapping_on=?area_entity, "GameArea found");
            commands
                .entity(wrapping_entity)
                .insert(WrappingGameAreaOn::new(area_entity));

            break;
        }
    }
}

/// Pauses all movement and rotation by temporarily inserting a [MovementPaused] component,
/// and adding the [MovementAutoPaused] component.
pub fn auto_pause_movement_when_not_playing(
    query: Query<
        Entity,
        (
            Without<Projectile>,
            Without<PauseMovement>,
            Or<(With<LinearVelocity>, With<AngularVelocity>)>,
        ),
    >,
    mut commands: Commands,
) {
    for entity in query.iter() {
        debug!(?entity, "pausing movement");
        commands
            .entity(entity)
            .insert(PauseMovement)
            .insert(AutoMovementPaused);
    }
}

/// Resumes movement and rotation from all [Entity]s that have the [MovementAutoPaused] component by
/// removing the [MovementPaused] and [MovementAutoPaused] components.
pub fn auto_resume_movement_when_playing(
    query: Query<Entity, With<AutoMovementPaused>>,
    mut commands: Commands,
) {
    for entity in query.iter() {
        debug!(?entity, "resuming movement");
        commands
            .entity(entity)
            .remove::<PauseMovement>()
            .remove::<AutoMovementPaused>();
    }
}
