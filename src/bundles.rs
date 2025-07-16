use crate::components::*;
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct MovableBundle {
    pub movable: Movable,
    pub velocity: Velocity,
    pub transform: Transform,
    pub input: AccumulatedInput,
    pub acceleration: Acceleration,
    pub phy_translation: PhysicalTranslation, // TODO: Ensure phy_translation is initialized to the same as transform
    pub prev_phy_translation: PreviousPhysicalTranslation,
}
