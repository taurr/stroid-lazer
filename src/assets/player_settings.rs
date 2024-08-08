use core::time::Duration;
use std::ops::Range;

use bevy::prelude::*;
use serde::Deserialize;

use super::optional;

#[derive(Reflect, Deserialize, Debug, Clone)]
pub struct PlayerSettingOptions {
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub spawn_position: Option<Vec2>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub safe_radius: Option<f32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub speed_range: Option<Range<f32>>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub acceleration: Option<f32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub speed_decay: Option<f32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub rotation_speed: Option<f32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub rotation_speed_acceleration: Option<f32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub jump_animation_duration: Option<Duration>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub minimum_jump_distance: Option<f32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub flames_audio: Option<String>,
}

/// Resource is initialized during [crate::states::init_level_settings].
#[derive(Resource, Reflect, Deserialize, Debug, Clone)]
pub struct PlayerSettings {
    pub spawn_position: Vec2,
    pub safe_radius: f32,
    pub speed_range: Range<f32>,
    pub acceleration: f32,
    pub speed_decay: f32,
    pub rotation_speed: f32,
    pub rotation_speed_acceleration: f32,
    pub jump_animation_duration: Duration,
    pub minimum_jump_distance: f32,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none", default)]
    pub flames_audio: Option<String>,
}

impl PlayerSettings {
    pub fn merge(mut self, options: Option<&PlayerSettingOptions>) -> Self {
        if let Some(options) = options {
            if let Some(value) = &options.spawn_position {
                self.spawn_position = *value;
            }
            if let Some(value) = &options.safe_radius {
                self.safe_radius = *value;
            }
            if let Some(value) = &options.speed_range {
                self.speed_range = value.clone();
            }
            if let Some(value) = &options.acceleration {
                self.acceleration = *value;
            }
            if let Some(value) = &options.speed_decay {
                self.speed_decay = *value;
            }
            if let Some(value) = &options.rotation_speed {
                self.rotation_speed = *value;
            }
            if let Some(value) = &options.rotation_speed_acceleration {
                self.rotation_speed_acceleration = *value;
            }
            if let Some(value) = &options.jump_animation_duration {
                self.jump_animation_duration = *value;
            }
            if let Some(value) = &options.minimum_jump_distance {
                self.minimum_jump_distance = *value;
            }
            if options.flames_audio.is_some() {
                self.flames_audio = options.flames_audio.clone();
            }
        }
        self
    }
}
