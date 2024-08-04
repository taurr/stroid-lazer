use bevy::prelude::*;

use super::{
    constants::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
    interaction::*,
};

pub fn highlight_interaction<T: InteractionId + 'static>(
    mut button_event: EventReader<InteractionEvent<T>>,
    mut query: Query<&mut BackgroundColor>,
) {
    for evt in button_event.read() {
        match evt {
            InteractionEvent::Press { id: _, entity } => {
                let mut color = query.get_mut(*entity).unwrap();
                *color = PRESSED_BUTTON.into();
            }
            InteractionEvent::Normal { id: _, entity } => {
                let mut color = query.get_mut(*entity).unwrap();
                *color = NORMAL_BUTTON.into();
            }
            InteractionEvent::Hover { id: _, entity } => {
                let mut color = query.get_mut(*entity).unwrap();
                *color = HOVERED_BUTTON.into();
            }
        }
    }
}
