use crate::components::*;
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct MovableBundle {
    pub movable: Movable,
    pub velocity: Velocity,
    pub transform: Transform,
    pub input: AccumulatedInput,
    pub acceleration: Acceleration,
    pub phy_translation: PhysicalTranslation,
    pub prev_phy_translation: PreviousPhysicalTranslation,
}

