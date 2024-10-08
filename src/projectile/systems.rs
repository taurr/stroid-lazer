use avian2d::prelude::*;
use bevy::{audio::Volume, prelude::*};
use bevy_turborand::{DelegatedRng, RngComponent};

use crate::{
    assets::{
        AmmonitionDepot, AmmonitionSelection, AmmonitionTextureCollection,
        EntitySpriteSheetCommands, WeaponCollection,
    },
    movement::Wrapping,
    player::Player,
    projectile::{
        Projectile, ProjectileCollisionEvent, ProjectileSprite, SpawnProjectilesEvent,
        SpawnSingleProjectileEvent,
    },
    utils::RngComponentExt,
    CollisionLayer, GameState, PlayingField,
};

pub fn despawn_all_projectiles(
    projectile_query: Query<Entity, With<Projectile>>,
    mut commands: Commands,
) {
    for projectile in projectile_query.iter() {
        trace!(?projectile, "Despawning projectile");
        commands.entity(projectile).despawn_recursive();
    }
}

pub fn spawn_projectiles(
    mut ev_spawn: EventReader<SpawnProjectilesEvent>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &Rotation, &mut RngComponent), With<Player>>,
    weapon_collection: Res<WeaponCollection>,
    ammonition_depot: Res<AmmonitionDepot>,
) {
    for SpawnProjectilesEvent { player, weapon } in ev_spawn.read() {
        let weapon_info = &weapon_collection[weapon];

        let (player_transform, player_rotation, mut rand) = player_query.get_mut(*player).unwrap();
        for weapon_port in weapon_info.weapon_ports.iter() {
            let direction = Rotation::degrees(weapon_port.rotation) * *player_rotation;
            debug!(?direction, "projectile direction");
            let position = {
                let port_rotation = Rotation::degrees(-90.0) * *player_rotation;
                (player_transform.translation.truncate() + port_rotation * weapon_port.position)
                    .extend(player_transform.translation.z)
            };

            let ammonition = pick_random_ammonition_index(
                &mut rand,
                weapon_port
                    .ammonition
                    .as_ref()
                    .unwrap_or(&weapon_info.default_ammonition),
                &ammonition_depot,
            );
            commands.trigger_targets(
                SpawnSingleProjectileEvent {
                    state: GameState::Playing,
                    position,
                    direction,
                    ammonition,
                    audio: weapon_info.audio.clone(),
                },
                *player,
            );
        }
    }
}

fn pick_random_ammonition_index(
    rand: &mut RngComponent,
    ammonition_selection: impl AsRef<[AmmonitionSelection]>,
    ammonition_depot: &AmmonitionDepot,
) -> String {
    let slice = ammonition_selection.as_ref();
    let weight_sum = slice
        .iter()
        .map(|s| match s {
            AmmonitionSelection::Exact { weight, .. } => weight,
            AmmonitionSelection::IndexRange { weight, .. } => weight,
        })
        .sum();
    let random = rand.f32_range(0.0..weight_sum);

    let selection = {
        let mut w = 0.0;
        slice
            .iter()
            .find(|s| {
                let weight = match s {
                    AmmonitionSelection::Exact { weight, .. } => weight,
                    AmmonitionSelection::IndexRange { weight, .. } => weight,
                };
                w += weight;
                random < w
            })
            .unwrap()
    };

    match selection {
        AmmonitionSelection::Exact { name, .. } => name.clone(),
        AmmonitionSelection::IndexRange {
            start_index,
            end_index,
            ..
        } => ammonition_depot
            .iter()
            .nth(rand.usize(*start_index..*end_index))
            .map(|(name, _)| name.clone())
            .unwrap(),
    }
}

pub fn on_projectile_spawn(
    trigger: Trigger<SpawnSingleProjectileEvent>,
    playing_field: Query<Entity, With<PlayingField>>,
    ammonition_depot: Res<AmmonitionDepot>,
    ammonition_spritesheets: Res<AmmonitionTextureCollection>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let event = trigger.event();
    let playing_field = playing_field.single();

    let ammonition_info = &ammonition_depot[&event.ammonition];
    let ammonitio_gfx = &ammonition_spritesheets[ammonition_info.texture_key.as_str()];

    let direction = event.direction;
    let velocity = LinearVelocity(direction * Vec2::X * ammonition_info.speed);
    let timeout = ammonition_info.timeout;

    commands.entity(playing_field).with_children(|children| {
        let mut projectile = children.spawn((
            Name::new("Projectile"),
            StateScoped(event.state),
            Projectile {
                timer: Timer::new(timeout, TimerMode::Once),
                shot_by_player: trigger.entity(),
            },
            Wrapping,
            RigidBody::Kinematic,
            SpatialBundle {
                transform: Transform::from_translation(event.position),
                ..default()
            },
            direction,
            velocity,
        ));

        projectile.with_children(|projectile_children| {
            projectile_children
                .spawn(ProjectileSprite)
                .insert_spritesheet(ammonitio_gfx, None, || {
                    (
                        ProjectileSprite,
                        CollisionLayers::new([CollisionLayer::Laser], [CollisionLayer::Asteroids]),
                    )
                });
        });
        let projectile = projectile.id();

        if let Some(audio) = &event.audio {
            // spawn the audio as a parallel entity, as we want the audio effect to play in its entirety
            // even if the projectile is despawned
            children.spawn((
                Name::new("Projectile Audio"),
                StateScoped(event.state),
                Projectile {
                    timer: Timer::new(timeout, TimerMode::Once),
                    shot_by_player: trigger.entity(),
                },
                Wrapping,
                RigidBody::Kinematic,
                SpatialBundle {
                    transform: Transform::from_translation(event.position),
                    ..default()
                },
                direction,
                velocity,
                AudioBundle {
                    source: asset_server.load(audio),
                    settings: PlaybackSettings::DESPAWN
                        .with_spatial(true)
                        .with_volume(Volume::new(0.6)),
                },
            ));
        }

        debug!(?projectile, "Spawned projectile");
    });
}

/// System for detecting player collision with any collider as setup when the player is spawned.
pub fn detect_projetile_collision(
    mut commands: Commands,
    collision_query: Query<
        (&ColliderParent, &CollidingEntities),
        (With<ProjectileSprite>, Changed<CollidingEntities>),
    >,
    projectile_query: Query<&Projectile>,
    mut events: EventWriter<ProjectileCollisionEvent>,
) {
    for (projectile_entity, colliding_entities) in collision_query
        .iter()
        .filter(|(_, colliding_entities)| !colliding_entities.is_empty())
    {
        debug!(
            ?projectile_entity,
            ?colliding_entities,
            "Projectile collisions"
        );
        let projectile = projectile_query.get(projectile_entity.get()).unwrap();

        // we're only interested in the first collision
        for entity in colliding_entities.iter().take(1) {
            events.send(ProjectileCollisionEvent {
                entity_hit: *entity,
                shot_by_player: projectile.shot_by_player,
            });
        }

        // get rid of the spent round!
        commands.entity(projectile_entity.get()).despawn_recursive();
    }
}

pub fn timeout_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Projectile)>,
) {
    for projectile_entity in query.iter_mut().filter_map(|(entity, mut projectile)| {
        projectile.timer.tick(time.delta());
        projectile.timer.just_finished().then_some(entity)
    }) {
        commands.entity(projectile_entity).despawn_recursive();
    }
}
