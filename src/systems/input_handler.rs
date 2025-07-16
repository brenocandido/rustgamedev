use crate::components::*;
use bevy::prelude::*;

/// Handle keyboard input and accumulate it in the `AccumulatedInput` component.
///
/// There are many strategies for how to handle all the input that happened since the last fixed timestep.
/// This is a very simple one: we just accumulate the input and average it out by normalizing it.
pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut AccumulatedInput>,
) {
    for mut input in query.iter_mut() {
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
    }
}
