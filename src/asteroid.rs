use core::{f32::consts::PI, ops::Range};

use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use rand::prelude::Rng;
use smart_default::SmartDefault;

const ASTEROID_Z_RANGE: Range<f32> = 10.0..20.0;
const ROTATION_SPEED_RANGE: Range<f32> = -1.0..1.0;

#[derive(Debug, SystemSet, PartialEq, Eq, Hash, Clone)]
pub struct AsteroidSet;

#[derive(Debug, Clone, Component)]
pub struct Asteroid;

#[derive(Debug, Clone, Copy, Component)]
pub struct AsteroidSize(usize);

#[derive(Debug, Clone, Event)]
pub struct SpawnAsteroidEvent {
    size: AsteroidSize,
    speed: Range<f32>,
    position: Vec3,
}

#[derive(SmartDefault, Debug)]
pub struct AsteroidPlugin;

#[derive(AssetCollection, Resource)]
struct AsteroidAssets {
    #[asset(key = "layouts", collection(typed))]
    layouts: Vec<Handle<TextureAtlasLayout>>,

    #[asset(key = "textures", collection(typed))]
    textures: Vec<Handle<Image>>,
}

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(GameState::AssetLoading)
                .load_collection::<AsteroidAssets>()
                .with_dynamic_assets_file::<StandardDynamicAssetArrayCollection>(
                    "asteroids.assets.ron",
                ),
        )
        .add_event::<SpawnAsteroidEvent>()
        .add_systems(
            OnEnter(GameState::Playing),
            spawn_level_asteroids
                .in_set(GameSet::Spawn)
                .in_set(AsteroidSet)
                .after(LevelSet),
        )
        .add_systems(
            PreUpdate,
            spawn_asteroid
                .run_if(in_state(GameState::Playing))
                .run_if(on_event::<SpawnAsteroidEvent>())
                .in_set(GameSet::Spawn)
                .in_set(AsteroidSet),
        );
    }
}

/// System responsible for spawning the asteroids at the beginning of a new level.
fn spawn_level_asteroids(
    settings: Res<GameSettings>,
    level_settings: Res<LevelSettings>,
    asteroid_assets: Res<AsteroidAssets>,
    query: Query<Entity, With<Asteroid>>,
    mut commands: Commands,
    mut events: EventWriter<SpawnAsteroidEvent>,
) {
    // make sure we don't have any old asteroid hanging around!
    for asteroid in query.iter() {
        commands.entity(asteroid).despawn_recursive();
    }

    let mut ran = rand::thread_rng();

    let size = AsteroidSize(usize::min(
        level_settings.asteroid_size,
        usize::min(
            asteroid_assets.layouts.len(),
            asteroid_assets.textures.len(),
        ) - 1,
    ));
    let game_area = &settings.game_area;
    for _ in 0..level_settings.asteroid_count {
        let position = {
            let x = ran.gen_range(game_area.horizontal_range());
            let y = ran.gen_range(game_area.vertical_range());
            let z = -ran.gen_range(ASTEROID_Z_RANGE);
            Vec3::new(x, y, z)
        };
        let speed = level_settings.asteroid_speed.clone();

        events.send(SpawnAsteroidEvent {
            size,
            position,
            speed,
        });
    }
}

/// System for spawning an asteroid upon a [SpawnAsteroidEvent].
fn spawn_asteroid(
    mut events: EventReader<SpawnAsteroidEvent>,
    playing_field: Query<Entity, With<PlayingField>>,
    asteroid_assets: Res<AsteroidAssets>,
    layout_assets: Res<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let playing_field = playing_field.single();
    commands.entity(playing_field).with_children(|commands| {
        for SpawnAsteroidEvent {
            size,
            speed,
            position,
        } in events.read()
        {
            let size = size.0;

            let texture_count = layout_assets
                .get(asteroid_assets.layouts[size].id())
                .unwrap()
                .len();

            let mut ran = rand::thread_rng();
            let movement_direction = rand::random::<f32>() * PI * 2.0;
            let speed = ran.gen_range(speed.clone());

            debug!(
                ?size,
                ?speed,
                ?position,
                ?movement_direction,
                ?speed,
                "Spawning asteroid"
            );

            commands.spawn((
                Name::new("Asteroid"),
                Asteroid,
                AsteroidSize(size),
                LinearMovement {
                    speed,
                    direction: Quat::from_rotation_z(movement_direction),
                },
                Wrapping,
                SpriteBundle {
                    texture: asteroid_assets.textures[size].clone(),
                    transform: Transform {
                        translation: *position,
                        ..default()
                    },
                    ..Default::default()
                },
                TextureAtlas {
                    index: ran.gen_range(0..texture_count),
                    ..TextureAtlas::from(asteroid_assets.layouts[size].clone())
                },
                RotatingMovement::new(Quat::from_rotation_z(ran.gen_range(ROTATION_SPEED_RANGE))),
            ));
        }
    });
}
