use bevy::prelude::*;
use tracing::instrument;

#[derive(Debug, SystemSet, PartialEq, Eq, Hash, Clone)]
pub struct InteractionSet;

pub trait InteractionId: Send + Sync + core::fmt::Debug + Clone {}

#[derive(Debug, Component)]
pub(super) struct InteractionIdComponent<T>(pub T);

pub trait InteractionHandlerExt {
    #[allow(unused)]
    fn add_interaction_handler<T: InteractionId + 'static>(&mut self) -> &mut Self;

    #[allow(unused)]
    fn add_interaction_handler_in_state<T: InteractionId + 'static>(
        &mut self,
        state: impl States,
    ) -> &mut Self;
}

impl InteractionHandlerExt for App {
    fn add_interaction_handler<T: InteractionId + 'static>(&mut self) -> &mut Self {
        self.add_event::<PressedEvent<T>>()
            .add_event::<InteractionEvent<T>>()
            .add_systems(PreUpdate, interaction_handler::<T>.in_set(InteractionSet))
    }

    fn add_interaction_handler_in_state<T: InteractionId + 'static>(
        &mut self,
        state: impl States,
    ) -> &mut Self {
        self.add_event::<PressedEvent<T>>()
            .add_event::<InteractionEvent<T>>()
            .add_systems(
                PreUpdate,
                interaction_handler::<T>
                    .run_if(in_state(state))
                    .in_set(InteractionSet),
            )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Event)]
pub enum InteractionEvent<T> {
    Press { id: T, entity: Entity },
    Normal { id: T, entity: Entity },
    Hover { id: T, entity: Entity },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Event)]
pub struct PressedEvent<T> {
    pub id: T,
    pub entity: Entity,
}

#[instrument(skip_all)]
fn interaction_handler<T: InteractionId + 'static>(
    query: Query<(Entity, &Interaction, &InteractionIdComponent<T>), Changed<Interaction>>,
    mut interaction_event: EventWriter<InteractionEvent<T>>,
    mut press_event: EventWriter<PressedEvent<T>>,
) {
    for (entity, interaction, button_id) in &query {
        let id = button_id.0.clone();
        let event = match *interaction {
            Interaction::Pressed => {
                press_event.send(PressedEvent {
                    id: id.clone(),
                    entity,
                });
                let event = InteractionEvent::Press { id, entity };
                info!(?event);
                event
            }
            Interaction::Hovered => {
                let event = InteractionEvent::Hover { id, entity };
                trace!(?event);
                event
            }
            Interaction::None => {
                let event = InteractionEvent::Normal { id, entity };
                trace!(?event);
                event
            }
        };
        interaction_event.send(event);
    }
}
