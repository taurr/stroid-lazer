use bevy::prelude::*;

#[derive(Debug, Clone, Event)]
pub struct AddToScoreEvent {
    pub player: Entity,
    pub score: usize,
}

#[derive(Debug, Clone, Event)]
pub(crate) struct PlayerDeadEvent {}

#[derive(Debug, Clone, Event)]
pub(crate) struct PlayerFireEvent;

#[derive(Debug, Clone, Event)]
pub(crate) struct PlayerJumpingEvent;

#[derive(Debug, Clone, Event)]
pub(crate) struct PlayerJumpFinishedEvent;
