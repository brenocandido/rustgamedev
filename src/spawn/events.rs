use crate::prelude::*;

#[derive(Message, Default)]
pub struct SpawnEnemiesEvent {
    pub count: usize,
    pub pos: Vec2,
}
