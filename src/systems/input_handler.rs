use crate::components::*;
use bevy::prelude::*;

/// Handle keyboard input and accumulate it in the `AccumulatedInput` component.
///
/// There are many strategies for how to handle all the input that happened since the last fixed timestep.
/// This is a very simple one: we just accumulate the input and average it out by normalizing it.
pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut AccumulatedInput, With<Player>>,
) {
    for mut input in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            input.0.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            input.0.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            input.0.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            input.0.x += 1.0;
        }
    }
}
