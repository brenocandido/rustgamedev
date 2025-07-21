use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
    pub fn damage(&mut self, amt: f32) {
        self.current = (self.current - amt).max(0.0);
    }
    pub fn heal(&mut self, amt: f32) {
        self.current = (self.current + amt).min(self.max);
    }
}
