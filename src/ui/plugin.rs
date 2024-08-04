use bevy::prelude::*;

#[derive(Debug)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        super::main_menu::build_ui(app);
        super::game_ui::build_ui(app);
        super::countdown_ui::build_ui(app);
        super::paused_menu::build_ui(app);
        super::game_over_menu::build_ui(app);

        #[cfg(feature = "dbg_colliders")]
        super::dbg_colliders::build_ui(app);
    }
}
