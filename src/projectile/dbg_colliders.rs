use bevy::prelude::*;

use crate::assets::{AmmonitionDepot, WeaponCollection};

pub fn setup_dbg(app: &mut App) {
    use avian2d::prelude::*;
    use core::time::Duration;

    use crate::{projectile::SpawnSingleProjectileEvent, states::GameState};

    app.add_systems(
        OnEnter(GameState::DebugColliders),
        |mut commands: Commands, assets: Res<AmmonitionDepot>| {
            /*
            debug!("trigger spawning of debug projectile");
            let ev = SpawnSingleProjectileEvent {
                state: GameState::DebugColliders,
                position: Vec3::new(-300.0, 400.0, 0.0),
                direction: Rotation::IDENTITY,
                velocity: LinearVelocity::ZERO,
                timeout: Duration::ZERO,
                spritesheet: assets.ammonition_spritesheet.clone(),
            };
            commands.trigger(ev);
            */
        },
    );
}
