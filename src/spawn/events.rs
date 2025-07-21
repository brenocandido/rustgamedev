use crate::prelude::*;

#[derive(Event, Default)]
pub struct SpawnEnemiesEvent {
    pub count: usize,
    pub pos: Vec2,
}
