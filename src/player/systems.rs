use core::f32::consts::TAU;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_turborand::{DelegatedRng, RngComponent};
use bevy_tweening::{
    lens::{ColorMaterialColorLens, TransformPositionLens},
    *,
};
use itertools::Itertools;
use leafwing_input_manager::prelude::*;
use tracing::instrument;

use super::{input::PlayerAction, *};
use crate::{
    assets::{
        game_assets::PlayerSpriteSheet, EntityCommandsExt, GameAreaSettings, GameStartSettings,
        InputKeySettings, PlayerSettings,
    },
    asteroid::AsteroidSprite,
    constants::PLAYER_Z_POS,
    movement::{ClampMovementSpeed, MovementPaused, PausedLinearVelocity, Wrapping},
    projectile::SpawnProjectilesEvent,
    tween_events::TweenCompletedEvent,
    CollisionLayer, GameState, PlayState, PlayingField,
};

// region: general systems

#[instrument(skip_all)]
pub fn player_acceleration_and_turning(
    player_settings: Res<PlayerSettings>,
    mut q: ParamSet<(
        Query<
            (&mut LinearVelocity, &Rotation),
            (With<Player>, With<Accelerating>, Without<Jumping>),
        >,
        Query<(&mut Turning, &mut Rotation), (With<Player>, With<Turning>)>,
    )>,
    time: Res<Time>,
) {
    // Acceleration => Velocity
    for (mut velocity, rotation) in q.p0().iter_mut() {
        let a = Vec2::new(rotation.cos, rotation.sin)
            * player_settings.acceleration
            * time.delta_seconds();
        velocity.0 += a;
    }

    // Turning => Rotation
    for (mut turning, mut rotation) in q.p1().iter_mut() {
        let angle_speed = match *turning {
            Turning::Left(angle) => {
                let angle = (angle
                    + time.delta_seconds() * player_settings.rotation_speed_acceleration)
                    .clamp(0.0, player_settings.rotation_speed);
                *turning = Turning::Left(angle);
                angle
            }
            Turning::Right(angle) => {
                let angle = (angle
                    - time.delta_seconds() * player_settings.rotation_speed_acceleration)
                    .clamp(-player_settings.rotation_speed, 0.0);
                *turning = Turning::Right(angle);
                angle
            }
        };
        trace!(angle_speed, ?turning);
        *rotation *= Rotation::radians(angle_speed * time.delta_seconds());
    }
}

#[instrument(skip_all)]
pub fn detect_player_collisions(
    collision_query: Query<
        (&ColliderParent, &CollidingEntities),
        (With<PlayerSprite>, Changed<CollidingEntities>),
    >,
    player_query: Query<&Transform, (With<Player>, Without<Jumping>, Without<Dead>)>,
    mut commands: Commands,
) {
    // collect all player collisions by player (in case of multiple players)
    let collisions = collision_query
        .into_iter()
        .into_grouping_map_by(|(parent, _)| parent.get())
        .fold(vec![], |mut acc, _key, (_, collisions)| {
            acc.extend(collisions.iter().copied());
            acc
        });

    // filter out players that don't fullfill our criteria
    let collisions = collisions
        .into_iter()
        .filter_map(|(player, colliding_entities)| {
            // we're only interested in actual collisions
            if colliding_entities.is_empty() {
                return None;
            }
            // with players that fullfill our criteria
            player_query
                .get(player)
                .map(|transform| (player, transform, colliding_entities))
                .ok()
        });

    for (player, transform, colliding_entities) in collisions {
        debug!(
            ?player,
            ?colliding_entities,
            position=?transform.translation,
            "Player collisions"
        );
        commands.entity(player).insert(Dead);
        commands.trigger_targets(PlayerDeadEvent {}, player);
    }
}

// endregion

// region: state transitions

#[instrument(skip_all)]
pub fn resume_player_movement(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).remove::<MovementPaused>();
    }
}

#[instrument(skip_all)]
pub fn despawn_old_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for player in player_query.iter() {
        debug!(?player, "despawning old player");
        commands.entity(player).despawn_recursive();
    }
}

#[instrument(skip_all)]
pub fn spawn_new_player(
    playing_field_query: Query<Entity, With<PlayingField>>,
    mut rand_query: Query<&mut RngComponent, With<PlayingField>>,
    game_start_settings: Res<GameStartSettings>,
    player_settings: Res<PlayerSettings>,
    spritesheet_asset: PlayerSpriteSheet,
    input_assets: Res<InputKeySettings>,
    mut commands: Commands,
    #[cfg(feature = "dbg_colliders")] gamestate: Res<State<GameState>>,
) {
    let mut rand = RngComponent::from(&mut rand_query.single_mut());
    let position = Position::new(player_settings.spawn_position);
    let facing_direction = Rotation::radians(rand.f32() * TAU);
    let velocity = LinearVelocity(
        player_settings.speed_range.start * Vec2::from_angle(rand.f32_normalized() * TAU),
    );
    let input_manager_bundle = {
        InputManagerBundle::with_map(InputMap::new([
            (PlayerAction::Accelerate, input_assets.accelerate),
            (PlayerAction::HyperJump, input_assets.jump),
            (PlayerAction::TurnLeft, input_assets.rotate_left),
            (PlayerAction::TurnRight, input_assets.rotate_right),
            (PlayerAction::Fire, input_assets.fire),
        ]))
    };
    let spritesheet = spritesheet_asset.spritesheet();
    let clamp_speed = ClampMovementSpeed::new(player_settings.speed_range.clone());
    let equipped_weapon = EquippedWeapon::new(game_start_settings.weapon_key.clone());
    let score = Score::new(0);

    commands
        .entity(playing_field_query.single())
        .with_children(|commands| {
            let mut player = commands.spawn((
                Name::new("Player"),
                StateScoped(GameState::Playing),
                Player {
                    lives: game_start_settings.lives,
                },
                score,
                equipped_weapon,
                input_manager_bundle,
                rand,
                (
                    // Rendering...
                    SpatialBundle {
                        transform: Transform::from_translation(Vec3::new(
                            position.x,
                            position.y,
                            PLAYER_Z_POS,
                        )),
                        ..default()
                    },
                ),
                (
                    // Physics...
                    RigidBody::Kinematic,
                    position,
                    facing_direction,
                    velocity,
                    MovementPaused,
                ),
                (
                    // Movement restrictions...
                    Wrapping,
                    clamp_speed,
                ),
            ));

            #[cfg(feature = "dbg_colliders")]
            if *gamestate == GameState::DebugColliders {
                player.insert(Accelerating);
            }

            if player_settings.speed_decay > 0.0 {
                player.insert(LinearDamping(player_settings.speed_decay));
            }

            player.with_children(|commands| {
                commands
                    .spawn(PlayerSprite)
                    .insert_spritesheet(spritesheet, None, || {
                        (
                            PlayerSprite,
                            CollisionLayers::new(
                                [CollisionLayer::Player],
                                [CollisionLayer::Asteroids],
                            ),
                        )
                    });
            });

            debug!(player=?player.id(), "Spawned new player");
        });
}

#[instrument(skip_all)]
pub fn reset_player_movement_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut RngComponent), With<Player>>,
    player_settings: Res<PlayerSettings>,
) {
    let (player_entity, mut rand) = player_query.single_mut();

    let position = Position::new(player_settings.spawn_position);
    let facing_direction = Rotation::radians(rand.f32() * TAU);
    let velocity = PausedLinearVelocity(LinearVelocity(
        player_settings.speed_range.start * Vec2::from_angle(rand.f32_normalized() * TAU),
    ));
    let clamp_movement = ClampMovementSpeed::new(player_settings.speed_range.clone());

    debug!(player=?player_entity, "reset position,rotation and velocity for next level");
    commands
        .entity(player_entity)
        .insert((position, facing_direction, velocity, clamp_movement));
}

#[instrument(skip_all)]
pub fn clear_safe_radius(
    mut commands: Commands,
    player_query: Query<(Entity, &Position, &Rotation), With<Player>>,
    asteroid_query: Query<(&ColliderParent, &Collider, &Position, &Rotation), With<AsteroidSprite>>,
    player_settings: Res<PlayerSettings>,
) {
    let (player_entity, &position, &rotation) = player_query.single();
    let collider = Collider::circle(player_settings.safe_radius);

    debug!(safe_radius = player_settings.safe_radius);
    asteroid_query
        .iter()
        .filter(|(_, a_colider, &a_position, &a_rotation)| {
            contact_query::intersection_test(
                &collider, position, rotation, a_colider, a_position, a_rotation,
            )
            .unwrap_or(false)
        })
        .map(|(asteroid, _, _, _)| asteroid)
        .for_each(|asteroid| {
            debug!(?asteroid, "clearing asteroid within death radius");
            commands.entity(asteroid.get()).despawn_recursive();
        });
    commands.entity(player_entity).remove::<Dead>();
}

// endregion

// region: event handling

#[instrument(skip_all)]
pub fn update_player_score(
    mut add_score_event: EventReader<AddToScoreEvent>,
    mut score_query: Query<&mut Score>,
) {
    for hit_evt in add_score_event.read() {
        let mut score = score_query.get_mut(hit_evt.player).unwrap();
        **score += hit_evt.score;
    }
}

// endregion

// region: observed events

#[instrument(skip_all)]
pub fn on_player_death(
    trigger: Trigger<PlayerDeadEvent>,
    mut player: Query<&mut Player>,
    mut next: ResMut<NextState<PlayState>>,
) {
    let mut player = player.get_mut(trigger.entity()).unwrap();
    player.lives -= 1;
    if player.lives == 0 {
        warn!("Player dead - Game Over");
        next.set(PlayState::GameOver(crate::GameOverReason::PlayerDead));
    } else {
        warn!("Player dead - Restart");
        next.set(PlayState::StartAfterDeath);
    }
}

#[instrument(skip_all)]
pub fn on_player_firing(
    trigger: Trigger<PlayerFireEvent>,
    weapon_query: Query<&EquippedWeapon>,
    mut projectile_events: EventWriter<SpawnProjectilesEvent>,
) {
    let player = trigger.entity();
    let weapon = (**weapon_query.get(player).unwrap()).clone();
    trace!(?player, "Firing");
    projectile_events.send(SpawnProjectilesEvent { player, weapon });
}

#[instrument(skip_all)]
pub fn on_player_jumping(
    trigger: Trigger<PlayerJumpingEvent>,
    mut commands: Commands,
    mut query: Query<(&Transform, &mut RngComponent), With<Player>>,
    game_area_settings: Res<GameAreaSettings>,
    player_settings: Res<PlayerSettings>,
) {
    let player = trigger.entity();
    let (transform, mut rand) = query.get_mut(player).unwrap();

    let destination = {
        // TODO: create setting for minimum jump distance!
        let game_area = &game_area_settings.game_area;

        let min_x = game_area.min.x;
        let max_x = game_area.max.x;
        let x = min_x.lerp(max_x, rand.f32());

        let min_y = game_area.min.y;
        let max_y = game_area.max.y;
        let y = min_y.lerp(max_y, rand.f32());

        Vec3::new(x, y, PLAYER_Z_POS)
    };

    // setup color overlay animation
    let color_tween = Tween::new(
        EaseFunction::CircularInOut,
        player_settings.jump_animation_duration / 2,
        ColorMaterialColorLens {
            start: Color::WHITE,
            end: Color::BLACK,
        },
    )
    .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
    .with_repeat_count(RepeatCount::Finite(2));

    // setup position animation
    let pos_tween = Tween::new(
        EaseFunction::QuinticInOut,
        player_settings.jump_animation_duration,
        TransformPositionLens {
            start: transform.translation,
            end: destination,
        },
    )
    .with_completed_event(TweenCompletedEvent::JumpFinished.ordinal());

    debug!(?player, ?destination, "Player jumping");
    commands
        .entity(player)
        .insert(Jumping)
        .insert(MovementPaused)
        .insert(AssetAnimator::new(color_tween))
        .insert(Animator::new(pos_tween));
}

#[instrument(skip_all)]
pub fn on_player_jump_finished(trigger: Trigger<PlayerJumpFinishedEvent>, mut commands: Commands) {
    let player = trigger.entity();
    commands
        .entity(player)
        .remove::<MovementPaused>()
        .remove::<Jumping>();
    debug!("jump finished");
}

// endregion
