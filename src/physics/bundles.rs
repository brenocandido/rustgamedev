use crate::prelude::*;

#[derive(Bundle, Default)]
pub struct MovableBundle {
    pub movable: Movable,
    pub velocity: Velocity,
    pub input: AccumulatedInput,
    pub acceleration: Acceleration,
    pub phy_translation: PhysicalTranslation, // TODO: Ensure phy_translation is initialized to the same as transform
    pub prev_phy_translation: PreviousPhysicalTranslation,
}