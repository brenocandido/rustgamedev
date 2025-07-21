use crate::prelude::*;

/// A vector representing the player's input, accumulated over all frames that ran
/// since the last time the physics simulation was advanced.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Reflect, InspectorOptions)]
#[reflect(Component)]
pub struct AccumulatedInput(pub Vec2);
