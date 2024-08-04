use bevy::{color::palettes::css, prelude::*};

pub const TEXT_COLOR: Color = Color::Srgba(css::WHITE);
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub const BACKDROP_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.75);

pub const BORDER_COLOR: Color = Color::Srgba(css::WHITE_SMOKE);
pub const BORDER_SIZE: UiRect = UiRect::all(Val::Px(3.0));
pub const BORDER_RADIUS: BorderRadius = BorderRadius::MAX;

pub const BUTTON_WIDTH: Val = Val::Px(300.0);
pub const BUTTON_HEIGHT: Val = Val::Px(50.0);
pub const FONT_SIZE: f32 = 20.0;

pub const BUTTON_GAP_HEIGHT: Val = Val::Px(10.0);
