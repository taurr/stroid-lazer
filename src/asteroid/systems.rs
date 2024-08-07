use std::f32::consts::TAU;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_turborand::{DelegatedRng, RngComponent};
use tracing::instrument;

use crate::{
    assets::{
        AsteroidDisplacement, AsteroidHitBehavior, AsteroidPool, AsteroidPoolCollection,
        AsteroidSpeedRange, AsteroidSplitCount, AsteroidSplitSelectionExt,
        AsteroidTextureCollection, AsteroidTextureSelection, EntityCommandsExt, GameAreaSettings,
        GameLevelSettings, PlayerSettings,
    },
    constants::ASTEROID_Z_RANGE,
    movement::{GameArea, MovementPaused, Wrapping},
    player::AddToScoreEvent,
    projectile::ProjectileCollisionEvent,
    states::PlayState,
    utils::RngComponentExt,
    CollisionLayer, GameState, PlayingField,
};

use super::*;

#[instrument(skip_all)]
pub fn resume_asteroid_movement(mut commands: Commands, query: Query<Entity, With<Asteroid>>) {
    for entity in query.iter() {
        commands.entity(entity).remove::<MovementPaused>();
    }
}

#[instrument(skip_all)]
pub fn despawn_all_asteroids(mut commands: Commands, query: Query<Entity, With<Asteroid>>) {
    debug!("despawning old asteroids");
    for asteroid in query.iter() {
        commands.entity(asteroid).despawn_recursive();
    }
}

/// System responsible for spawning the asteroids at the beginning of a new level.
#[allow(clippy::too_many_arguments)]
#[instrument(skip_all)]
pub fn spawn_level_asteroids(
    mut playing_field: Query<(Entity, &mut RngComponent), With<PlayingField>>,
    mut commands: Commands,
    player_settings: Res<PlayerSettings>,
    level_settings: Res<GameLevelSettings>,
    game_area_settings: Res<GameAreaSettings>,
    asteroid_pool_collection: Res<AsteroidPoolCollection>,
    asteroid_spritesheets: Res<AsteroidTextureCollection>,
    playstate: Res<State<PlayState>>,
) {
    let (playing_field, mut rand) = playing_field.single_mut();
    let asteroid_startup_settings = &level_settings.startup.asteroids;

    let asteroid_count = match asteroid_startup_settings.count {
        AsteroidSplitCount::Exact(count) => count,
        AsteroidSplitCount::Range { start, end } => rand.usize(start..end),
    };

    debug!(asteroid_count, "Spawning level asteroids");
    for _ in 0..asteroid_count {
        // calculate asteroid spawn position
        let position = {
            let game_area = &game_area_settings.game_area;
            let player_pos = player_settings.spawn_position;
            let min_distance = player_settings.safe_radius;

            loop {
                let angle = rand.f32_range(0.0..TAU);
                let intersection = calc_intersection(player_pos, angle, game_area);
                let max_distance = player_pos.distance(intersection);
                if max_distance > min_distance {
                    let distance = rand.f32_range(min_distance..max_distance);
                    break (Vec2::from_angle(angle) * distance)
                        .extend(-rand.f32_range(ASTEROID_Z_RANGE));
                }
            }
        };

        let Some(pool) = asteroid_startup_settings
            .select_from
            .pick_random_pool(&mut rand, &asteroid_pool_collection)
        else {
            warn!("did not find an asteroid pool");
            continue;
        };

        spawn_asteroid_from_pool(
            GameState::Playing,
            Some(**playstate),
            playing_field,
            position,
            pool,
            None,
            &asteroid_spritesheets,
            &mut rand,
            &mut commands,
        );
    }
}

#[cfg(feature = "dbg_colliders")]
#[instrument(skip_all)]
pub fn spawn_debug_asteroids(
    mut playing_field: Query<(Entity, &mut RngComponent), With<PlayingField>>,
    mut commands: Commands,
    game_area_settings: Res<GameAreaSettings>,
    asteroid_pool_collection: Res<AsteroidPoolCollection>,
    asteroid_spritesheets: Res<AsteroidTextureCollection>,
) {
    let (playing_field, mut rand) = playing_field.single_mut();

    let min_pos = game_area_settings.game_area.min;
    let max_pos = game_area_settings.game_area.max;
    let x_pos_delta = (max_pos.x - min_pos.x) / (asteroid_pool_collection.len() + 2) as f32;

    for pool_index in 0..asteroid_pool_collection.len() {
        let pool = &(*asteroid_pool_collection)[pool_index];
        let y_pos_delta = (max_pos.y - min_pos.x) / (pool.spritesheets.len() + 2) as f32;
        for pool_sheet_index in 0..pool.spritesheets.len() {
            let position = Vec3::new(
                x_pos_delta * (pool_index + 1) as f32,
                y_pos_delta * (pool_sheet_index + 1) as f32,
                rand.f32_range(ASTEROID_Z_RANGE),
            ) + min_pos;

            spawn_asteroid_from_pool(
                GameState::DebugColliders,
                None,
                playing_field,
                position,
                pool,
                Some(pool_sheet_index),
                &asteroid_spritesheets,
                &mut rand,
                &mut commands,
            );
        }
    }
}

#[instrument(skip_all)]
pub fn on_asteroid_spawn_new(
    trigger: Trigger<AsteroidSpawnNewEvent>,
    mut rand: Query<&mut RngComponent, With<PlayingField>>,
    mut commands: Commands,
) {
    let event = trigger.event();
    let playing_field = trigger.entity();
    let rand = RngComponent::from(&mut rand.single_mut());

    trace!(?event, "spawning new asteroid");

    commands.entity(playing_field).with_children(|commands| {
        let mut asteroid = commands.spawn((
            StateScoped(event.state),
            SpatialBundle {
                transform: Transform::from_translation(event.position),
                ..Default::default()
            },
            Name::new("Asteroid"),
            Asteroid,
            RigidBody::Kinematic,
            Position::new(event.position.truncate()),
            event.linear_velocity,
            event.angular_velocity,
            event.hit_behavior.clone(),
            Wrapping,
            rand,
        ));
        if event.playstate != Some(PlayState::Running) {
            asteroid.insert(MovementPaused);
        }
        asteroid.insert_spritesheet(&event.spritesheet, Some(event.atlas_index), || {
            (
                AsteroidSprite,
                CollisionLayers::new(
                    [CollisionLayer::Asteroids],
                    [CollisionLayer::Player, CollisionLayer::Laser],
                ),
            )
        });
    });
}

#[instrument(skip_all)]
pub fn on_remove_asteroid(
    mut remove_events: EventReader<AsteroidRemoveEvent>,
    mut commands: Commands,
) {
    for event in remove_events.read() {
        let asteroid = event.asteroid;
        trace!(?asteroid, "Despawning asteroid");
        commands.entity(event.asteroid).despawn_recursive();
    }
}

#[instrument(skip_all)]
pub fn init_asteroid_counter(mut counter: ResMut<AsteroidCount>) {
    **counter = 0;
    trace!(asteroid_count = **counter);
}

#[instrument(skip_all)]
pub fn on_asteroid_added(_trigger: Trigger<OnAdd, Asteroid>, mut counter: ResMut<AsteroidCount>) {
    **counter += 1;
    debug!(asteroid_count = **counter);
}

#[instrument(skip_all)]
pub fn on_asteroid_removed(
    _trigger: Trigger<OnRemove, Asteroid>,
    mut counter: ResMut<AsteroidCount>,
) {
    if **counter > 0 {
        **counter -= 1;
        debug!(asteroid_count = **counter);
    }
}

#[instrument(skip_all)]
pub fn detect_asteroid_hits(
    mut projectile_hit_events: EventReader<ProjectileCollisionEvent>,
    collider_query: Query<&ColliderParent, With<AsteroidSprite>>,
    transform_query: Query<&Transform, With<Asteroid>>,
    mut commands: Commands,
) {
    use itertools::Itertools;

    for (asteroid, position, players) in projectile_hit_events
        .read()
        .into_group_map_by(|event| event.entity_hit)
        .into_iter()
        .filter_map(|(asteroid_sprite, hit_events_for_asteroid)| {
            collider_query.get(asteroid_sprite).ok().map(|asteroid| {
                let position = transform_query.get(asteroid.get()).unwrap().translation;
                (
                    asteroid.get(),
                    position,
                    hit_events_for_asteroid
                        .iter()
                        .map(|collision_evt| collision_evt.shot_by_player)
                        .unique()
                        .collect(),
                )
            })
        })
    {
        debug!(
            ?asteroid,
            ?position,
            ?players,
            "Asteroid hit by a projectile"
        );

        commands.trigger_targets(AsteroidHitEvent { position, players }, asteroid);
    }
}

#[allow(clippy::too_many_arguments)]
#[instrument(skip_all)]
pub fn on_asteroid_hit(
    trigger: Trigger<AsteroidHitEvent>,
    mut playing_field: Query<(Entity, &mut RngComponent), With<PlayingField>>,
    mut hit_behavior_query: Query<&AsteroidHitBehavior>,
    asteroid_pool_collection: Res<AsteroidPoolCollection>,
    asteroid_spritesheets: Res<AsteroidTextureCollection>,
    mut score_events: EventWriter<AddToScoreEvent>,
    mut remove_events: EventWriter<AsteroidRemoveEvent>,
    mut commands: Commands,
    playstate: Res<State<PlayState>>,
) {
    let hit_evt = trigger.event();
    let asteroid = trigger.entity();
    let (playing_field, mut rand) = playing_field.single_mut();
    let hit_behavior = hit_behavior_query.get_mut(asteroid).unwrap();

    match hit_behavior {
        AsteroidHitBehavior::None => {}
        AsteroidHitBehavior::Points(points) => {
            add_to_score(*points, &hit_evt.players, &mut score_events);
        }
        AsteroidHitBehavior::PointsAndSplit {
            points,
            count,
            select_from,
        } => {
            for _ in match count {
                AsteroidSplitCount::Exact(max) => 0..*max,
                AsteroidSplitCount::Range { start, end } => 0..rand.usize(start..end),
            } {
                let Some(pool) = select_from.pick_random_pool(&mut rand, &asteroid_pool_collection)
                else {
                    warn!("did not find an asteroid pool");
                    continue;
                };

                let position = match &pool.displacement {
                    AsteroidDisplacement::None => hit_evt.position,
                    AsteroidDisplacement::Exact(distance) => {
                        let angle = rand.f32_range(0.0..TAU);
                        let displacement = Vec2::from_angle(angle) * *distance;
                        hit_evt.position + displacement.extend(0.0)
                    }
                    AsteroidDisplacement::Range { start, end } => {
                        let angle = rand.f32_range(0.0..TAU);
                        let distance = rand.f32_range(start..end);
                        let displacement = Vec2::from_angle(angle) * distance;
                        hit_evt.position + displacement.extend(0.0)
                    }
                };

                spawn_asteroid_from_pool(
                    GameState::Playing,
                    Some(**playstate),
                    playing_field,
                    position,
                    pool,
                    None,
                    &asteroid_spritesheets,
                    &mut rand,
                    &mut commands,
                );
            }
            add_to_score(*points, &hit_evt.players, &mut score_events);
        }
    }

    // Remove the original asteroid
    remove_events.send(AsteroidRemoveEvent { asteroid });
}

// region: general functions

#[instrument(skip_all)]
fn add_to_score(score: usize, players: &[Entity], score_events: &mut EventWriter<AddToScoreEvent>) {
    let score = (score as f32 / players.len() as f32).ceil() as usize;
    for player in players.iter() {
        score_events.send(AddToScoreEvent {
            player: *player,
            score,
        });
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_asteroid_from_pool(
    state: GameState,
    playstate: Option<PlayState>,
    parent_entity: Entity,
    position: Vec3,
    pool: &AsteroidPool,
    // Inside the pool, which spritesheet index should be used
    pool_sheet_index: Option<usize>,
    asteroid_spritesheets: &AsteroidTextureCollection,
    rand: &mut RngComponent,
    commands: &mut Commands,
) {
    let (spritesheet, atlas_index, speed_range, rotation_range) = {
        let pool_sheet_index =
            pool_sheet_index.unwrap_or_else(|| rand.usize(0..pool.textures.len()));
        let Some(texture_selection) = pool.textures.get(pool_sheet_index) else {
            warn!(pool_sheet_index, "Could not find sprite sheet");
            return;
        };
        match texture_selection {
            AsteroidTextureSelection::AtlasIndex {
                key: spritesheet_key,
                atlas_idx: index,
                speed,
                rotation,
            } => {
                let Some(spritesheet) = asteroid_spritesheets.get(spritesheet_key) else {
                    warn!(spritesheet_key, "Could not find sprite sheet");
                    return;
                };
                let index = index.unwrap_or(spritesheet.atlas_index);
                let atlas_index = if spritesheet.texture_count <= index {
                    warn!(
                        texture_count = spritesheet.texture_count,
                        specified_index = index,
                        using_index = spritesheet.atlas_index,
                        "Index out of bounds, using default index"
                    );
                    spritesheet.atlas_index
                } else {
                    index
                };

                let speed = speed.unwrap_or(pool.speed);
                let rotation = rotation.unwrap_or(pool.rotation);
                (spritesheet.clone(), atlas_index, speed, rotation)
            }
        }
    };

    let linear_velocity = LinearVelocity(
        (Quat::from_rotation_z(rand.f32() * TAU)
            * Vec3::X
            * match speed_range {
                AsteroidSpeedRange::None => 0.0,
                AsteroidSpeedRange::Exact(units_per_second) => units_per_second as f32,
                AsteroidSpeedRange::Range { start, end } => rand.f32_range(start..end),
            })
        .truncate(),
    );
    let angular_velocity = match rotation_range {
        crate::assets::AsteroidRotationSpeed::None => AngularVelocity::ZERO,
        crate::assets::AsteroidRotationSpeed::Exact(radians) => AngularVelocity(radians),
        crate::assets::AsteroidRotationSpeed::Range { start, end } => {
            AngularVelocity(rand.f32_range(start..end))
        }
    };

    let hit_behavior = pool.hit_behavior.clone();

    commands.trigger_targets(
        AsteroidSpawnNewEvent {
            state,
            playstate,
            position,
            linear_velocity,
            angular_velocity,
            hit_behavior,
            spritesheet,
            atlas_index,
        },
        parent_entity,
    );
}

/// Given an area, a position and a direction, calculate the intersection point with the edge of the area.
fn calc_intersection(position: Vec2, direction: f32, area: &GameArea) -> Vec2 {
    let dir = Vec2::from_angle(direction);
    let mut intersection = position + dir;

    if dir.x != 0.0 {
        let t = ((area.horizontal_range().start - position.x) / dir.x).abs();
        let possible_intersection = position + t * dir;
        intersection.x = possible_intersection.x;
    }

    if dir.y != 0.0 {
        let t = ((area.vertical_range().start - position.y) / dir.y).abs();
        let possible_intersection = position + t * dir;
        intersection.y = possible_intersection.y;
    }

    intersection = intersection.clamp(area.min.truncate(), area.max.truncate());
    intersection
}

// endregion
