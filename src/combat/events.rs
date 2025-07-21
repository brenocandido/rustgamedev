use crate::prelude::*;

#[derive(Event)]
pub struct DamageEvent {
    pub victim: Entity,
    pub amount: f32,
}