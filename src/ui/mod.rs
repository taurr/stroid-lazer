#[macro_use]
mod menu;

mod constants;
mod countdown_ui;
mod game_over_menu;
mod game_ui;
mod highscore_ui;
mod interaction;
mod main_menu;
mod paused_menu;
mod plugin;

#[cfg(feature = "dbg_colliders")]
mod dbg_colliders;

pub use self::plugin::*;
