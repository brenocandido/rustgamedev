use crate::prelude::*;

#[derive(Event)]
pub struct CollisionEvent(pub Entity, pub Entity);