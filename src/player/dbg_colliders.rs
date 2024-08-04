use bevy::prelude::*;

pub fn setup_dbg(app: &mut App) {
    use crate::{states::GameState, PlayingField};
    use avian2d::prelude::*;
    use bevy_turborand::RngComponent;

    /*
    app.add_systems(
        OnEnter(GameState::DebugColliders),
        |mut commands: Commands, mut rand_query: Query<&mut RngComponent, With<PlayingField>>| {
            debug!("trigger spawning of debug player");
            commands.trigger_targets(
                PlayerSpawnEvent {
                    state: GameState::DebugColliders,
                    rand: RngComponent::from(&mut rand_query.single_mut()),
                    position: Position(Vec2::new(-400.0, 400.0)),
                    rotation: Rotation::IDENTITY,
                    velocity: LinearVelocity(Vec2::ZERO),
                },
                Entity::PLACEHOLDER,
            );
        },
    );*/
}
