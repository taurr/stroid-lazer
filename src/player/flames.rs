use bevy::{audio::Volume, prelude::*};
use tracing::instrument;

use super::Accelerating;
use crate::assets::{game_assets::FlameSpriteSheet, EntityCommandsExt, PlayerSettings};

pub fn init_rocket_flames(app: &mut App) {
    app.observe(on_add_accelerating)
        .observe(on_remove_accelerating);
}

#[derive(Component, Debug)]
struct RocketFlames;

#[instrument(skip_all)]
fn on_add_accelerating(
    trigger: Trigger<OnAdd, Accelerating>,
    mut commands: Commands,
    spritesheet_asset: FlameSpriteSheet,
    asset_server: Res<AssetServer>,
    player_settings: Res<PlayerSettings>,
) {
    commands.entity(trigger.entity()).with_children(|children| {
        let mut flame = children.spawn(RocketFlames);
        if let Some(audio) = &player_settings.flames_audio {
            flame.insert(AudioBundle {
                source: asset_server.load(audio),
                settings: PlaybackSettings {
                    volume: Volume::new(1.0),
                    spatial: true,
                    ..PlaybackSettings::LOOP
                }
            });
        }
        flame.insert_spritesheet(spritesheet_asset.spritesheet(), None, || ());
    });
    trace!("Added rocket flame");
}

#[instrument(skip_all)]
fn on_remove_accelerating(
    trigger: Trigger<OnRemove, Accelerating>,
    mut commands: Commands,
    flame_query: Query<(Entity, &Parent), With<RocketFlames>>,
) {
    for (flame, player) in flame_query.iter() {
        if player.get() == trigger.entity() {
            trace!("Removed rocket flame");
            commands.entity(flame).despawn_recursive();
        }
    }
}
