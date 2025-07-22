use crate::prelude::*;

#[derive(Event)]
pub enum CollisionEvent {
    Started(Entity, Entity, f32 /* impulse N·s */),
    Stopped(Entity, Entity),
}
