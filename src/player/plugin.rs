use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{
    input::{accept_player_input, PlayerAction},
    *,
};
use crate::{levels::GameLevelsSet, PlayState};

pub struct PlayerPlugin;

#[derive(Debug, Clone, Default, SystemSet, PartialEq, Eq, Hash)]
pub struct PlayerSet;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "dbg_colliders")]
        dbg_colliders::setup_dbg(app);

        app.register_type::<Score>()
            .add_event::<AddToScoreEvent>()
            .add_plugins(InputManagerPlugin::<PlayerAction>::default());

        // transitions
        app.add_systems(
            OnEnter(PlayState::StartNewGame),
            (
                despawn_old_player.run_if(any_with_component::<Player>),
                spawn_new_player,
            )
                .chain()
                .in_set(PlayerSet)
                .after(GameLevelsSet),
        )
        .add_systems(
            OnExit(PlayState::StartNextLevel),
            reset_player_movement_system
                .in_set(PlayerSet)
                .after(GameLevelsSet),
        )
        .add_systems(
            OnExit(PlayState::StartAfterDeath),
            clear_safe_radius.in_set(PlayerSet).after(GameLevelsSet),
        )
        .add_systems(
            OnEnter(PlayState::Running),
            resume_player_movement.in_set(PlayerSet),
        )
        .add_systems(
            OnExit(PlayState::Running),
            dont_accelerate.in_set(PlayerSet),
        );

        app.add_systems(
            PreUpdate,
            (accept_player_input)
                .run_if(in_state(PlayState::Running))
                .in_set(PlayerSet),
        );
        app.add_systems(
            Update,
            (player_acceleration_and_turning,)
                .run_if(in_state(PlayState::Running))
                .in_set(PlayerSet),
        )
        .add_systems(
            PostUpdate,
            (detect_player_collisions, update_player_score)
                .run_if(in_state(PlayState::Running))
                .in_set(PlayerSet),
        )
        .observe(on_player_death)
        .observe(on_player_firing)
        .observe(on_player_jumping)
        .observe(on_player_jump_finished);

        self::flames::init_rocket_flames(app);
    }
}
