use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;

/// Handle keyboard input and accumulate it in the `AccumulatedInput` component.
///
/// There are many strategies for how to handle all the input that happened since the last fixed timestep.
/// This is a very simple one: we just accumulate the input and average it out by normalizing it.
pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut AccumulatedInput, &mut Acceleration)>,
) {
    for (mut input, mut acceleration) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            input.vec.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            input.vec.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            input.vec.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            input.vec.x += 1.0;
        }

        input.cnt += 1;

        // Need to normalize and scale because otherwise
        // diagonal movement would be faster than horizontal or vertical movement.
        // This effectively averages the accumulated input.
        acceleration.0 = input.vec.extend(0.0).normalize_or_zero() * ACCELERATION;
    }
}