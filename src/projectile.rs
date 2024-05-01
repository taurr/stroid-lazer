use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::prelude::*;

#[derive(Component)]
pub struct Projectile {
    timer: Timer,
}

#[derive(Debug, Clone, Default, SystemSet, PartialEq, Eq, Hash)]
struct ProjectileSet;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (load_assets)
                .in_set(GameSet::PreSpawn)
                .in_set(ProjectileSet),
        )
        .add_systems(
            Update,
            (
                spawn_projectiles
                    .run_if(on_event::<SpawnProjectile>())
                    .in_set(GameSet::Spawn),
                timeout_projectiles.in_set(GameSet::Cleanup),
            )
                .run_if(in_state(GameState::Playing))
                .in_set(ProjectileSet),
        )
        .add_systems(
            OnExit(GameState::Playing),
            despawn_with::<Projectile>
                .in_set(GameSet::Cleanup)
                .in_set(ProjectileSet),
        );
    }
}

#[derive(Resource)]
struct ProjectileAssets {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
}

#[derive(Bundle)]
struct ProjectileBundle {
    name: Name,
    projectile: Projectile,
    movement: LinearMovement,
    wrapping: Wrapping,
}

fn load_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(ProjectileAssets {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(10.0, 2.0))),
        material: materials.add(ColorMaterial::from(Color::RED)),
    })
}

fn spawn_projectiles(
    playing_field: Query<Entity, With<PlayingField>>,
    player: Query<&Transform, With<Player>>,
    projectile_assets: Res<ProjectileAssets>,
    mut ev_spawn: EventReader<SpawnProjectile>,
    mut commands: Commands,
    projectile_settings: Res<ProjectileSettings>,
) {
    let playing_field = playing_field.single();
    let player_transform = player.single();

    for _evt in ev_spawn.read() {
        commands.entity(playing_field).with_children(|commands| {
            let projectile_ship_translation = player_transform
                .rotation
                .mul_vec3(projectile_settings.ship_offset);
            let transform = Transform {
                translation: player_transform.translation + projectile_ship_translation,
                ..*player_transform
            };

            commands.spawn((
                ProjectileBundle {
                    name: Name::new("Projectile"),
                    projectile: Projectile {
                        timer: Timer::new(projectile_settings.timeout, TimerMode::Once),
                    },
                    movement: LinearMovement {
                        speed: projectile_settings.speed,
                        direction: player_transform.rotation,
                    },
                    wrapping: Wrapping,
                },
                MaterialMesh2dBundle {
                    mesh: projectile_assets.mesh.clone(),
                    material: projectile_assets.material.clone(),
                    transform,
                    ..default()
                },
            ));
        });
    }
}

fn timeout_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Projectile)>,
) {
    query
        .iter_mut()
        .filter_map(|(entity, mut projectile)| {
            projectile.timer.tick(time.delta());
            projectile.timer.just_finished().then_some(entity)
        })
        .for_each(|entity| {
            commands.entity(entity).despawn();
        });
}
