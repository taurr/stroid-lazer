use core::f32::consts::TAU;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_persistent::Persistent;
use bevy_turborand::{DelegatedRng, RngComponent};
use bevy_tweening::{
    lens::{ColorMaterialColorLens, TransformPositionLens},
    *,
};
use itertools::Itertools;
use leafwing_input_manager::prelude::*;

use crate::{
    assets::{
        game_assets::PlayerSpriteSheet, EntitySpriteSheetCommands, GameAreaSettings,
        GameStartSettings, HighScoreBoard, HighScoreKey, InputKeySettings, PlayerSettings,
    },
    asteroid::AsteroidSprite,
    constants::PLAYER_Z_POS,
    movement::{ClampMovementSpeed, PauseMovement, PausedLinearVelocity, Wrapping},
    player::{
        input::PlayerAction, Accelerating, AddToScoreEvent, Dead, EquippedWeapon, Jumping, NewLife,
        Player, PlayerDeadEvent, PlayerFireEvent, PlayerJumpFinishedEvent, PlayerJumpingEvent,
        PlayerSprite, Score, Turning,
    },
    projectile::SpawnProjectilesEvent,
    tween_events::TweenCompletedEvent,
    CollisionLayer, GameState, PlayState, PlayingField,
};

// region: general systems

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

pub fn detect_player_collisions(
    player_collision_query: Query<(&ColliderParent, &CollidingEntities), With<PlayerSprite>>,
    player_query: Query<&Transform, (With<Player>, Without<Jumping>, Without<Dead>)>,
    asteroid_query: Query<&Parent>,
    mut commands: Commands,
) {
    // collect all player collisions by player (in case of multiple players)
    let player_collisions = player_collision_query
        .into_iter()
        .into_grouping_map_by(|(player_parent, _)| player_parent.get())
        .fold(vec![], |mut acc, _key, (_, collisions)| {
            let asteroids = collisions
                .iter()
                .filter_map(|entity| asteroid_query.get(*entity).ok().map(|p| p.get()));
            acc.extend(asteroids);
            acc
        });

    // filter out players that don't fullfill our criteria
    let player_collisions = player_collisions
        .into_iter()
        .filter_map(|(player, asteroids)| {
            // we're only interested in actual collisions
            if asteroids.is_empty() {
                return None;
            }
            // with players that fullfill our criteria
            player_query
                .get(player)
                .map(|transform| (player, transform, asteroids))
                .ok()
        })
        .collect_vec();
    if player_collisions.is_empty() {
        return;
    }

    for (player, transform, asteroids) in player_collisions {
        debug!(
            ?player,
            ?asteroids,
            position=?transform.translation,
            "Player collisions"
        );
        asteroids.iter().for_each(|asteroid| {
            debug!(?asteroid, "clearing colliding asteroid");
            commands.entity(*asteroid).despawn_recursive();
        });
        commands.entity(player).insert(Dead);
        commands.trigger_targets(PlayerDeadEvent {}, player);
    }
}

// endregion

// region: state transitions

pub fn stop_accelerating(players: Query<Entity, With<Player>>, mut commands: Commands) {
    for player in players.iter() {
        commands.entity(player).remove::<Accelerating>();
    }
}

pub fn resume_player_movement(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).remove::<PauseMovement>();
    }
}

pub fn despawn_old_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for player in player_query.iter() {
        debug!(?player, "despawning old player");
        commands.entity(player).despawn_recursive();
    }
}

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
        (player_settings.speed_range.start
            + (player_settings.speed_range.end - player_settings.speed_range.start) / 16.0)
            * Vec2::from_angle(rand.f32_normalized() * TAU),
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
                    PauseMovement,
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

pub fn update_player_score(
    mut add_score_event: EventReader<AddToScoreEvent>,
    mut highscores: ResMut<Persistent<HighScoreBoard>>,
    highscore_key: Option<Res<HighScoreKey>>,
    mut score_query: Query<&mut Score>,
    game_start_settings: Res<GameStartSettings>,
    mut commands: Commands,
) {
    for hit_evt in add_score_event.read() {
        let player = hit_evt.player;
        let mut score = score_query.get_mut(player).unwrap();
        let new_score = **score + hit_evt.score;

        // add new life
        if (**score / game_start_settings.new_life_every)
            != (new_score / game_start_settings.new_life_every)
        {
            commands.trigger_targets(NewLife, player);
        }

        // update score
        **score = new_score;

        // maybe update highscore key
        if let Some(new_highscore_key) = highscores.add_score(*score, highscore_key.as_deref()) {
            debug!(?score, "Highscore reached");
            commands.insert_resource(new_highscore_key);
        }
    }
}

// endregion

// region: observed events

pub fn on_new_life(trigger: Trigger<NewLife>, mut player_query: Query<&mut Player>) {
    let mut player = player_query.get_mut(trigger.entity()).unwrap();
    player.lives += 1;
}

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
        loop {
            let game_area = &game_area_settings.game_area;

            let min_x = game_area.min().x;
            let max_x = game_area.max().x;
            let x = min_x.lerp(max_x, rand.f32());

            let min_y = game_area.min().y;
            let max_y = game_area.max().y;
            let y = min_y.lerp(max_y, rand.f32());

            let destination = Vec3::new(x, y, PLAYER_Z_POS);
            if destination.distance(transform.translation) > player_settings.minimum_jump_distance {
                break destination;
            }
        }
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
        .insert(PauseMovement)
        .insert(AssetAnimator::new(color_tween))
        .insert(Animator::new(pos_tween));
}

pub fn on_player_jump_finished(trigger: Trigger<PlayerJumpFinishedEvent>, mut commands: Commands) {
    let player = trigger.entity();
    commands
        .entity(player)
        .remove::<PauseMovement>()
        .remove::<Jumping>();
    debug!("jump finished");
}

// endregion
