use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

/// Advance the physics simulation by one fixed timestep. This may run zero or multiple times per frame.
///
/// Note that since this runs in `FixedUpdate`, `Res<Time>` would be `Res<Time<Fixed>>` automatically.
/// We are being explicit here for clarity.
pub fn advance_physics(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<
        (
            &mut PhysicalTranslation,
            &mut PreviousPhysicalTranslation,
            &mut AccumulatedInput,
            &mut Velocity,
            &Acceleration,
        ),
        With<Movable>,
    >,
) {


    for (
        mut current_physical_translation,
        mut previous_physical_translation,
        mut input,
        mut velocity,
        acceleration,
    ) in query.iter_mut()
    {
        if acceleration.0 == Vec3::ZERO {
            if DEFAULT_DECELERATION >= velocity.0.length() {
                velocity.0 = Vec3::ZERO;
            } else {
                let deceleration_vec = velocity.0.normalize() * DEFAULT_DECELERATION;
                velocity.0 -= deceleration_vec;
            }
        } else {
            velocity.0 += acceleration.0;
            if velocity.0.length() > MAX_SPEED {
                velocity.0 = velocity.normalize_or_zero() * MAX_SPEED;
            }
        }

        previous_physical_translation.0 = current_physical_translation.0;
        current_physical_translation.0 += velocity.0 * fixed_time.delta_secs();

        // Reset the input accumulator, as we are currently consuming all input that happened since the last fixed timestep.
        input.vec = Vec2::ZERO;
    }
}

pub fn interpolate_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &PhysicalTranslation,
        &PreviousPhysicalTranslation,
    )>,
) {
    for (mut transform, current_physical_translation, previous_physical_translation) in
        query.iter_mut()
    {
        let previous = previous_physical_translation.0;
        let current = current_physical_translation.0;
        // The overstep fraction is a value between 0 and 1 that tells us how far we are between two fixed timesteps.
        let alpha = fixed_time.overstep_fraction();

        let rendered_translation = previous.lerp(current, alpha);
        transform.translation = rendered_translation;
    }
}
