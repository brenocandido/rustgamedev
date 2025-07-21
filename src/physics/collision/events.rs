use crate::prelude::*;

#[derive(Event)]
pub enum CollisionEvent {
    Started(Entity, Entity),
    Stopped(Entity, Entity),
}