use core::{ops::Range, time::Duration};

use crate::prelude::*;
use bevy::{prelude::*, utils::tracing::instrument};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use smart_default::SmartDefault;

#[derive(Debug, SystemSet, PartialEq, Eq, Hash, Clone)]
pub struct LevelSet;

#[derive(SmartDefault, Debug)]
pub struct LevelPlugin;

#[derive(Resource, Debug, Clone)]
pub struct GameSettings {
    pub current_level: usize,
    pub level_settings: SettingsVec<LevelSettings>,
    pub player_settings: SettingsVec<PlayerSettings>,
    pub projectile_settings: SettingsVec<ProjectileSettings>,
    pub game_area: GameArea,
}

#[derive(Debug, Clone, Deref, serde::Deserialize, Asset, TypePath)]
pub struct SettingsVec<T>(Vec<T>)
where
    T: TypePath + Send + Sync;

#[derive(Resource, Debug, Clone, serde::Deserialize, TypePath)]
pub struct LevelSettings {
    pub asteroid_count: u32,
    pub asteroid_size: usize,
    pub asteroid_speed: Range<f32>,
    pub player_settings_index: usize,
    pub projectile_settings_index: usize,
}

#[derive(Resource, Debug, Clone, serde::Deserialize, TypePath)]
pub struct PlayerSettings {
    pub spawn_position: Vec3,
    pub rotation_speed: f32,
    pub acceleration: f32,
    pub acceleration_decay: Option<f32>,
    pub minimum_speed: f32,
    pub maximum_speed: f32,
    pub jump_animation_duration: Duration,
    pub jump_time_fraction: f32,
}

#[derive(Resource, Debug, Clone, serde::Deserialize, TypePath)]
pub struct ProjectileSettings {
    pub speed: f32,
    pub timeout: Duration,
    pub ship_offset: Vec3,
}

#[derive(AssetCollection, Resource)]
struct SettingsAsset {
    #[asset(path = "settings.levels.ron")]
    pub level_settings: Handle<SettingsVec<LevelSettings>>,

    #[asset(path = "settings.player.ron")]
    pub player_settings: Handle<SettingsVec<PlayerSettings>>,

    #[asset(path = "settings.projectile.ron")]
    pub projectile_settings: Handle<SettingsVec<ProjectileSettings>>,
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RonAssetPlugin::<SettingsVec<LevelSettings>>::new(&["levels.ron"]),
            RonAssetPlugin::<SettingsVec<PlayerSettings>>::new(&["player.ron"]),
            RonAssetPlugin::<SettingsVec<ProjectileSettings>>::new(&["projectile.ron"]),
        ))
        .configure_loading_state(
            LoadingStateConfig::new(GameState::AssetLoading)
                .load_collection::<SettingsAsset>()
                .init_resource::<GameSettings>(),
        );
        app.add_systems(
            OnEnter(GameState::Playing),
            set_level_settings
                .in_set(GameSet::PreSpawn)
                .before(GameSet::Spawn)
                .in_set(LevelSet),
        );
    }
}

impl FromWorld for GameSettings {
    #[instrument]
    fn from_world(world: &mut World) -> Self {
        let settings_asset = world
            .remove_resource::<SettingsAsset>()
            .expect("Failed to get LevelsAsset");

        let world = world.cell();

        let mut level_settings_assets = world
            .get_resource_mut::<Assets<SettingsVec<LevelSettings>>>()
            .expect("Failed to get Assets<SettingsVec<LevelSettings>>");

        let mut player_settings_assets = world
            .get_resource_mut::<Assets<SettingsVec<PlayerSettings>>>()
            .expect("Failed to get Assets<SettingsVec<PlayerSettings>>>");

        let mut projectile_settings_assets = world
            .get_resource_mut::<Assets<SettingsVec<ProjectileSettings>>>()
            .expect("Failed to get Assets<SettingsVec<ProjectileSettings>>");

        let level_settings = level_settings_assets
            .remove(&settings_asset.level_settings)
            .unwrap();
        let player_settings = player_settings_assets
            .remove(&settings_asset.player_settings)
            .unwrap();
        let projectile_settings = projectile_settings_assets
            .remove(&settings_asset.projectile_settings)
            .unwrap();

        let settings = GameSettings {
            current_level: 0,
            level_settings,
            player_settings,
            projectile_settings,
            game_area: GameArea::new(Vec3::new(-300.0, -300.0, 0.0), Vec3::new(300.0, 300.0, 0.0)),
        };
        debug!(game_settings=?settings, "created GameSettings");
        settings
    }
}

fn set_level_settings(game_settings: Res<GameSettings>, mut commands: Commands) {
    let current_level = game_settings.current_level;
    let level_settings = game_settings.level_settings[current_level].clone();
    let player_settings =
        game_settings.player_settings[level_settings.player_settings_index].clone();
    let projectile_settings =
        game_settings.projectile_settings[level_settings.projectile_settings_index].clone();

    commands.insert_resource(level_settings.clone());
    commands.insert_resource(player_settings.clone());
    commands.insert_resource(projectile_settings.clone());
}
