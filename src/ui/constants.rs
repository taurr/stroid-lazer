#![allow(unused)]

use bevy::{color::palettes::css, prelude::*};

pub const TEXT_COLOR: Color = Color::Srgba(css::WHITE);
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub const BACKDROP_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.75);

pub const MENU_BUTTON_WIDTH: Val = Val::Px(300.0);
pub const BUTTON_BORDER_COLOR: Color = Color::Srgba(css::WHITE_SMOKE);
pub const BUTTON_BORDER_SIZE: UiRect = UiRect::all(Val::Px(2.0));
pub const BUTTON_BORDER_RADIUS: BorderRadius = BorderRadius::percent(14.0, 14.0, 12.0, 12.0);
pub const BUTTON_MARGIN: UiRect =
    UiRect::new(Val::Px(5.0), Val::Px(0.0), Val::Px(5.0), Val::Px(0.0));
pub const BUTTON_PADDING: UiRect =
    UiRect::new(Val::Px(15.0), Val::Px(15.0), Val::Px(5.0), Val::Px(5.0));

pub const TITLE_FONT_SIZE: f32 = 96.0;
pub const H1_FONT_SIZE: f32 = 72.0;
pub const H2_FONT_SIZE: f32 = 32.0;
pub const H3_FONT_SIZE: f32 = 24.0;
pub const H4_FONT_SIZE: f32 = 18.0;
pub const BUTTON_FONT_SIZE: f32 = H3_FONT_SIZE;

pub const MAX_NAME_LENGTH: usize = 20;
