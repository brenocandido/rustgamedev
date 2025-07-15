//! This example shows how to properly handle player input,
//! advance a physics simulation in a fixed timestep, and display the results.
//!
//! The classic source for how and why this is done is Glenn Fiedler's article
//! [Fix Your Timestep!](https://gafferongames.com/post/fix_your_timestep/).
//! For a more Bevy-centric source, see
//! [this cheatbook entry](https://bevy-cheatbook.github.io/fundamentals/fixed-timestep.html).
//!
//! ## Motivation
//!
//! The naive way of moving a player is to just update their position like so:
//! ```no_run
//! transform.translation += velocity;
//! ```
//! The issue here is that the player's movement speed will be tied to the frame rate.
//! Faster machines will move the player faster, and slower machines will move the player slower.
//! In fact, you can observe this today when running some old games that did it this way on modern hardware!
//! The player will move at a breakneck pace.
//!
//! The more sophisticated way is to update the player's position based on the time that has passed:
//! ```no_run
//! transform.translation += velocity * time.delta_secs();
//! ```
//! This way, velocity represents a speed in units per second, and the player will move at the same speed
//! regardless of the frame rate.
//!
//! However, this can still be problematic if the frame rate is very low or very high.
//! If the frame rate is very low, the player will move in large jumps. This may lead to
//! a player moving in such large jumps that they pass through walls or other obstacles.
//! In general, you cannot expect a physics simulation to behave nicely with *any* delta time.
//! Ideally, we want to have some stability in what kinds of delta times we feed into our physics simulation.
//!
//! The solution is using a fixed timestep. This means that we advance the physics simulation by a fixed amount
//! at a time. If the real time that passed between two frames is less than the fixed timestep, we simply
//! don't advance the physics simulation at all.
//! If it is more, we advance the physics simulation multiple times until we catch up.
//! You can read more about how Bevy implements this in the documentation for
//! [`bevy::time::Fixed`](https://docs.rs/bevy/latest/bevy/time/struct.Fixed.html).
//!
//! This leaves us with a last problem, however. If our physics simulation may advance zero or multiple times
//! per frame, there may be frames in which the player's position did not need to be updated at all,
//! and some where it is updated by a large amount that resulted from running the physics simulation multiple times.
//! This is physically correct, but visually jarring. Imagine a player moving in a straight line, but depending on the frame rate,
//! they may sometimes advance by a large amount and sometimes not at all. Visually, we want the player to move smoothly.
//! This is why we need to separate the player's position in the physics simulation from the player's position in the visual representation.
//! The visual representation can then be interpolated smoothly based on the previous and current actual player position in the physics simulation.
//!
//! This is a tradeoff: every visual frame is now slightly lagging behind the actual physical frame,
//! but in return, the player's movement will appear smooth.
//! There are other ways to compute the visual representation of the player, such as extrapolation.
//! See the [documentation of the lightyear crate](https://cbournhonesque.github.io/lightyear/book/concepts/advanced_replication/visual_interpolation.html)
//! for a nice overview of the different methods and their respective tradeoffs.
//!
//! ## Implementation
//!
//! - The player's inputs since the last physics update are stored in the `AccumulatedInput` component.
//! - The player's velocity is stored in a `Velocity` component. This is the speed in units per second.
//! - The player's current position in the physics simulation is stored in a `PhysicalTranslation` component.
//! - The player's previous position in the physics simulation is stored in a `PreviousPhysicalTranslation` component.
//! - The player's visual representation is stored in Bevy's regular `Transform` component.
//! - Every frame, we go through the following steps:
//!   - Accumulate the player's input and set the current speed in the `handle_input` system.
//!     This is run in the `RunFixedMainLoop` schedule, ordered in `RunFixedMainLoopSystem::BeforeFixedMainLoop`,
//!     which runs before the fixed timestep loop. This is run every frame.
//!   - Advance the physics simulation by one fixed timestep in the `advance_physics` system.
//!     Accumulated input is consumed here.
//!     This is run in the `FixedUpdate` schedule, which runs zero or multiple times per frame.
//!   - Update the player's visual representation in the `interpolate_rendered_transform` system.
//!     This interpolates between the player's previous and current position in the physics simulation.
//!     It is run in the `RunFixedMainLoop` schedule, ordered in `RunFixedMainLoopSystem::AfterFixedMainLoop`,
//!     which runs after the fixed timestep loop. This is run every frame.
//!
//!
//! ## Controls
//!
//! | Key Binding          | Action        |
//! |:---------------------|:--------------|
//! | `W`                  | Move up       |
//! | `S`                  | Move down     |
//! | `A`                  | Move left     |
//! | `D`                  | Move right    |

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Component, Default)]
pub struct Movable;

/// A vector representing the player's input, accumulated over all frames that ran
/// since the last time the physics simulation was advanced.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Reflect, InspectorOptions)]
#[reflect(Component)]
pub struct AccumulatedInput {
    pub vec: Vec2,
    pub cnt: i32,
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
pub struct PhysicalTranslation(Vec3);

/// The value [`PhysicalTranslation`] had in the last fixed timestep.
/// Used for interpolation in the `interpolate_rendered_transform` system.
#[derive(
    Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut, Reflect, InspectorOptions,
)]
#[reflect(Component)]
pub struct PreviousPhysicalTranslation(Vec3);

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
    const MAX_SPEED: f32 = 210.0;
    const DEFAULT_DECELERATION: f32 = 10.0;

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
