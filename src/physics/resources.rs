use crate::prelude::*;
use crate::physics::*;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Reflect, InspectorOptions)]
#[reflect(Resource)]
pub struct PhysicsConfig {
    pub max_speed: f32,
    pub acceleration: f32,
    pub drag: f32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        PhysicsConfig {
            max_speed: DEFAULT_MAX_SPEED,
            acceleration: DEFAULT_ACCELERATION,
            drag: DEFAULT_DRAG,
        }
    }
}

impl PhysicsConfig {
    pub fn max_speed_sq(&self) -> f32 {
        self.max_speed * self.max_speed
    }
}