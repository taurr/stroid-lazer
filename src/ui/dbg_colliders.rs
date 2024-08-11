use bevy::{
    color::palettes::css,
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use tracing::instrument;

use super::UiSet;
use crate::states::GameState;

pub fn build_ui(app: &mut App) {
    let state = GameState::DebugColliders;

    app.add_systems(OnEnter(state), spawn_ui).add_systems(
        Update,
        wait_for_esc
            .run_if(in_state(GameState::DebugColliders))
            .in_set(UiSet),
    );
}

#[instrument(skip_all)]
fn spawn_ui(mut commands: Commands) {
    debug!("spawning collider debug ui");
    commands
        .spawn((
            StateScoped(GameState::DebugColliders),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|commands| {
            commands.spawn(TextBundle::from_section(
                "Press ESC to continue",
                TextStyle {
                    font_size: BUTTON_FONT_SIZE,
                    color: css::WHITE.into(),
                    ..Default::default()
                },
            ));
        });
}

pub fn wait_for_esc(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut next: ResMut<NextState<GameState>>,
) {
    for event in keyboard_input_events.read() {
        if event.key_code == KeyCode::Escape && event.state == ButtonState::Pressed {
            next.set(GameState::MainMenu);
        }
    }
}
