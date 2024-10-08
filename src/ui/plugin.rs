use bevy::prelude::*;

#[derive(Debug)]
pub struct UiPlugin;

#[derive(Debug, Clone, Default, SystemSet, PartialEq, Eq, Hash)]
pub struct UiSet;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        super::main_menu::build_ui(app);
        super::game_ui::build_ui(app);
        super::countdown_ui::build_ui(app);
        super::paused_menu::build_ui(app);
        super::game_over_menu::build_ui(app);
        super::highscore_ui::build_ui(app);

        #[cfg(feature = "dbg_colliders")]
        super::dbg_colliders::build_ui(app);
    }
}
