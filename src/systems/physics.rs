use crate::{components::*, prelude::PhysicsConfig};
use bevy::prelude::*;

/// Advance the physics simulation by one fixed timestep. This may run zero or multiple times per frame.
///
/// Note that since this runs in `FixedUpdate`, `Res<Time>` would be `Res<Time<Fixed>>` automatically.
/// We are being explicit here for clarity.
pub fn advance_physics(
    fixed_time: Res<Time<Fixed>>,
    cfg: Res<PhysicsConfig>,
    mut query: Query<
        (
            &mut PhysicalTranslation,
            &mut PreviousPhysicalTranslation,
            &mut AccumulatedInput,
            &mut Velocity,
            &mut Acceleration,
        ),
        With<Movable>,
    >,
) {
    let dt = fixed_time.delta_secs();
    let max_speed_sq = cfg.max_speed_sq();

    for (
        mut current_physical_translation,
        mut previous_physical_translation,
        mut input,
        mut velocity,
        mut acceleration,
    ) in query.iter_mut()
    {
        // Need to normalize and scale because otherwise
        // diagonal movement would be faster than horizontal or vertical movement.
        // This effectively averages the accumulated input.
        acceleration.0 = input.vec.extend(0.0).normalize_or_zero() * cfg.acceleration;

        if acceleration.0 == Vec3::ZERO {
            let drag_modulo = cfg.drag * input.cnt as f32 * dt;

            if (drag_modulo * drag_modulo) >= velocity.0.length_squared() {
                velocity.0 = Vec3::ZERO;
            } else {
                let drag_vec = velocity.0.normalize() * drag_modulo;
                velocity.0 -= drag_vec;
            }
        } else {
            velocity.0 += acceleration.0 * dt;
            if velocity.0.length_squared() > max_speed_sq {
                velocity.0 = velocity.normalize_or_zero() * cfg.max_speed;
            }
        }

        previous_physical_translation.0 = current_physical_translation.0;
        current_physical_translation.0 += velocity.0 * dt;

        // Reset the input accumulator, as we are currently consuming all input that happened since the last fixed timestep.
        *input = default();
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
