use core::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::prelude::*;

#[derive(Debug, Clone, Default, SystemSet, PartialEq, Eq, Hash)]
struct PlayerSet;

/// Component used to mark the player entity.
#[derive(Debug, Clone, Component, Default)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets.in_set(GameSet::PreSpawn))
            .add_systems(
                OnEnter(GameState::Playing),
                spawn_player.in_set(GameSet::Spawn).in_set(PlayerSet),
            )
            .add_systems(
                OnExit(GameState::Playing),
                despawn_with::<Player>
                    .in_set(GameSet::Cleanup)
                    .in_set(PlayerSet),
            )
            .add_systems(
                Update,
                (
                    start_random_jump
                        .run_if(on_event::<RandomJumpPlayer>())
                        .before(update_jumping),
                    update_jumping,
                    (
                        rotate_player.run_if(on_event::<RotatePlayer>()),
                        accelerate_player.run_if(on_event::<AcceleratePlayer>()),
                    )
                        .before(MovementSet),
                )
                    .run_if(in_state(GameState::Playing))
                    .after(ControlsSet)
                    .in_set(GameSet::Movement)
                    .in_set(PlayerSet),
            );
    }
}

/// Component put on the player while making a hyperjump.
///
/// `timer` defines how long the jump lasts.
/// `midpoint` is a ration between 0.0 and 1.0 that defines the exact point on the timer when the player switches position.
/// `destination` is the new position of the player after the jump.
#[derive(Debug, Clone, Component)]
#[component(storage = "SparseSet")]
struct Jumping {
    timer: Timer,
    midpoint: f32,
    destination: Option<Vec3>,
}

#[derive(Resource)]
struct PlayerAssets {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
}

#[derive(Bundle)]
struct PlayerBundle {
    name: Name,
    player: Player,
    movement: LinearMovement,
    clamp: ClampMovementSpeed,
    wrapping: Wrapping,
}

fn load_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    debug!("Loading player assets");

    commands.insert_resource(PlayerAssets {
        mesh: Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::new(-15.0, 10.0),
            Vec2::new(15.0, 0.0),
            Vec2::new(-15.0, -10.0),
        ))),
        material: materials.add(Color::WHITE),
    })
}

/// System for spawning the player.
///
/// The player is spawned at the defind spawn position (see [LevelSettings]), facing a random
/// direction and moving in another random direction.
fn spawn_player(
    playing_field: Query<Entity, With<PlayingField>>,
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    player_settings: Res<PlayerSettings>,
) {
    info!("Spawning player");

    let playing_field = playing_field.single();

    let facing_direction = rand::random::<f32>() * PI * 2.0;
    let movement_direction = rand::random::<f32>() * PI * 2.0;

    commands.entity(playing_field).with_children(|commands| {
        let mut player = commands.spawn((
            PlayerBundle {
                name: Name::new("Player"),
                player: Player,
                movement: LinearMovement {
                    speed: player_settings.minimum_speed,
                    direction: Quat::from_rotation_z(movement_direction),
                },
                clamp: ClampMovementSpeed {
                    range: player_settings.minimum_speed..player_settings.maximum_speed,
                },
                wrapping: Wrapping,
            },
            MaterialMesh2dBundle {
                transform: Transform {
                    translation: player_settings.spawn_position,
                    rotation: Quat::from_rotation_z(facing_direction),
                    ..default()
                },
                mesh: player_assets.mesh.clone(),
                material: player_assets.material.clone(),
                ..default()
            },
        ));

        if let Some(decay) = player_settings.acceleration_decay {
            player.insert(LinearMovementDecay::new(decay));
        }
    });
}

/// System for rotating the player when receiving the [RotatePlayer] event.
///
/// The rotation amount is defined as a delta time fraction of the player's rotation speed as
/// defined in [LevelSettings].
fn rotate_player(
    time: Res<Time>,
    mut events: EventReader<RotatePlayer>,
    mut query: Query<&mut Transform, With<Player>>,
    player_settings: Res<PlayerSettings>,
) {
    let mut transform = query.single_mut();
    for evt in events.read() {
        let rotation = match evt {
            RotatePlayer::Clockwise => -player_settings.rotation_speed,
            RotatePlayer::AntiClockwise => player_settings.rotation_speed,
        };
        transform.rotate(Quat::from_rotation_z(rotation * time.delta_seconds()));
    }
}

/// System for accelerating the [LinearMovement] speed of the player when receiving the
/// [AcceleratePlayer] event.
///
/// The acceleration amount is defined as a delta time fraction of
/// the player's acceleration speed as defined in [LevelSettings].
fn accelerate_player(
    time: Res<Time>,
    mut events: EventReader<AcceleratePlayer>,
    mut query: Query<(&Transform, &mut LinearMovement), With<Player>>,
    player_settings: Res<PlayerSettings>,
) {
    let (transform, mut current_movement) = query.single_mut();
    events.clear();

    // we don't actually move the player here, we just need to calculate a new LinearMovement
    // component that will place the user where we want them to end up!

    // Current LinearMovement translation, for a full second!
    let current_movement_translation = current_movement
        .direction
        .mul_vec3(Vec3::X * current_movement.speed);

    // Movement in the player facing direction, scaled in regards to time!
    let acceleration_translation = transform
        .rotation
        .mul_vec3(Vec3::X * player_settings.acceleration * time.delta_seconds());

    let movement_translation_after_acceleration =
        acceleration_translation + current_movement_translation;

    *current_movement = LinearMovement {
        direction: Quat::from_rotation_arc(
            Vec3::X,
            movement_translation_after_acceleration
                .try_normalize()
                .unwrap_or(Vec3::X),
        ),
        speed: movement_translation_after_acceleration.length(),
    };
}

/// System for initiating a random jump when receiving the [RandomJumpPlayer] event.
///
/// Jump time and exact point is defined in [LevelSettings].
fn start_random_jump(
    game_settings: Res<GameSettings>,
    player_settings: Res<PlayerSettings>,
    mut commands: Commands,
    mut events: EventReader<RandomJumpPlayer>,
    query: Query<Entity, (With<Player>, Without<Jumping>)>,
) {
    let game_area = &game_settings.game_area;

    events.clear();
    let Ok(player) = query.get_single() else {
        return;
    };

    let min_x = game_area.min().x;
    let max_x = game_area.max().x;
    let x = min_x.lerp(max_x, rand::random());

    let min_y = game_area.min().y;
    let max_y = game_area.max().y;
    let y = min_y.lerp(max_y, rand::random());

    commands.entity(player).insert(Jumping {
        timer: Timer::new(player_settings.jump_animation_duration, TimerMode::Once),
        destination: Some(Vec3::new(x, y, player_settings.spawn_position.z)),
        midpoint: player_settings.jump_time_fraction,
    });
}

/// System for updating the jump animation and moving the player to the new position.
fn update_jumping(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &Handle<ColorMaterial>, &mut Transform, &mut Jumping)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok((entity, material_handle, mut transform, mut jumping)) = query.get_single_mut() else {
        return;
    };

    jumping.timer.tick(time.delta());

    let fraction = {
        let fraction = jumping.timer.fraction();
        if fraction < jumping.midpoint {
            1.0 - fraction.remap(0.0, jumping.midpoint, 0.0, 1.0)
        } else {
            fraction.remap(jumping.midpoint, 1.0, 0.0, 1.0)
        }
    };

    materials
        .get_mut(material_handle)
        .unwrap()
        .color
        .set_a(fraction);

    if jumping.timer.fraction() >= jumping.midpoint {
        if let Some(destination) = jumping.destination.take() {
            transform.translation = destination;
        }
    }

    if jumping.timer.just_finished() {
        commands.entity(entity).remove::<Jumping>();
    }
}
