use crate::prelude::*;

#[derive(Component, Default)]
pub struct Movable;

#[derive(Component)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Mass(1.0)
    }
}

/// A vector representing the player's velocity in the physics simulation.
#[derive(
    Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut, Reflect, InspectorOptions,
)]
#[reflect(Component)]
pub struct Velocity(pub Vec3);

/// A vector representing the player's velocity in the physics simulation.
#[derive(
    Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut, Reflect, InspectorOptions,
)]
#[reflect(Component)]
pub struct Acceleration(pub Vec3);

/// The actual position of the player in the physics simulation.
/// This is separate from the `Transform`, which is merely a visual representation.
///
/// If you want to make sure that this component is always initialized
/// with the same value as the `Transform`'s translation, you can
/// use a [component lifecycle hook](https://docs.rs/bevy/0.14.0/bevy/ecs/component/struct.ComponentHooks.html)
#[derive(
    Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut, Reflect, InspectorOptions,
)]
#[reflect(Component)]
pub struct PhysicalTranslation(pub Vec3);

/// The value [`PhysicalTranslation`] had in the last fixed timestep.
/// Used for interpolation in the `interpolate_rendered_transform` system.
#[derive(
    Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut, Reflect, InspectorOptions,
)]
#[reflect(Component)]
pub struct PreviousPhysicalTranslation(pub Vec3);
