use core::time::Duration;

use bevy::prelude::*;
use smart_default::SmartDefault;

use crate::prelude::*;

#[derive(Event)]
pub struct SpawnProjectile;

#[derive(Event)]
pub struct AcceleratePlayer;

#[derive(Event)]
pub enum RotatePlayer {
    Clockwise,
    AntiClockwise,
}

#[derive(Event)]
pub struct RandomJumpPlayer;

#[derive(Resource, SmartDefault)]
pub struct ControlSettings {
    #[default(Duration::from_millis(333))]
    pub auto_fire: Duration,

    pub keys: InputKeys,
}

#[derive(SmartDefault)]
pub struct InputKeys {
    #[default(KeyCode::ArrowLeft)]
    pub rotate_left: KeyCode,

    #[default(KeyCode::ArrowRight)]
    pub rotate_right: KeyCode,

    #[default(KeyCode::ArrowUp)]
    pub accelerate: KeyCode,

    #[default(KeyCode::Space)]
    pub fire: KeyCode,

    #[default(KeyCode::Enter)]
    pub jump: KeyCode,
}

#[derive(Debug, Clone, Default, SystemSet, PartialEq, Eq, Hash)]
pub struct ControlsSet;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ControlSettings>()
            .add_event::<SpawnProjectile>()
            .add_event::<AcceleratePlayer>()
            .add_event::<RotatePlayer>()
            .add_event::<RandomJumpPlayer>()
            .add_systems(
                Update,
                (check_playing_input.run_if(in_state(GameState::Playing)))
                    .in_set(GameSet::Input)
                    .in_set(ControlsSet),
            );
    }
}

#[allow(clippy::too_many_arguments)]
fn check_playing_input(
    time: Res<Time>,
    settings: Res<ControlSettings>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut projectile_events: EventWriter<SpawnProjectile>,
    mut accelerate_events: EventWriter<AcceleratePlayer>,
    mut rotate_events: EventWriter<RotatePlayer>,
    mut jump_events: EventWriter<RandomJumpPlayer>,
    mut fire_timer: Local<Option<Timer>>,
) {
    if keyboard_input.just_pressed(settings.keys.fire) {
        projectile_events.send(SpawnProjectile);
        *fire_timer = Some(Timer::new(settings.auto_fire, TimerMode::Repeating));
    } else if keyboard_input.pressed(settings.keys.fire) {
        let fire_timer = fire_timer.as_mut().expect("Fire timer should be set");
        fire_timer.tick(time.delta());
        if fire_timer.just_finished() {
            projectile_events.send(SpawnProjectile);
        }
    }

    if keyboard_input.pressed(settings.keys.rotate_left) {
        rotate_events.send(RotatePlayer::AntiClockwise);
    }
    if keyboard_input.pressed(settings.keys.rotate_right) {
        rotate_events.send(RotatePlayer::Clockwise);
    }
    if keyboard_input.pressed(settings.keys.accelerate) {
        accelerate_events.send(AcceleratePlayer);
    }
    if keyboard_input.just_pressed(settings.keys.jump) {
        jump_events.send(RandomJumpPlayer);
    }
}
