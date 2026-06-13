use crate::prelude::*;

#[derive(Message)]
pub struct DamageEvent {
    pub victim: Entity,
    pub amount: f32,
}