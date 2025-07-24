use crate::behavior::*;
use crate::prelude::*;

#[derive(Component)]
pub struct Seek {
    // Detection radius
    pub distance: f32,
}

#[derive(Component)]
pub struct Flee {
    // Detection radius
    pub distance: f32,
}

#[derive(Component)]
pub struct Wander {
    pub direction: Vec2,
    pub current_variation: f32, // Angle that will be incremented wi //TODO
    pub base_variation: f32,    // In radians
}

#[derive(Component)]
pub struct AvoidAllies {
    // avoid entities with Ally tag within this distance
    pub radius: f32,
}

#[derive(Component)]
pub struct MaintainRange {
    // desired range from target
    pub distance: f32,
}

#[derive(Component)]
pub struct Ally; // marker for ally units (to avoid)

impl Default for Wander {
    fn default() -> Self {
        let angle = fastrand::f32() * std::f32::consts::TAU;

        Self {
            direction: Vec2::from_angle(angle),
            base_variation: WANDER_DEFAULT_BASE_VARIATION,
        }
    }
}
