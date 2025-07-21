use crate::physics::*;
use crate::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CollisionPlugin)
            // Constants used for physics systems.
            .insert_resource(PhysicsConfig::default())
            // Advance the physics simulation using a fixed timestep.
            .add_systems(FixedUpdate, advance_physics)
            .add_systems(
                // The `RunFixedMainLoop` schedule allows us to schedule systems to run before and after the fixed timestep loop.
                RunFixedMainLoop,
                (
                    // The player's visual representation needs to be updated after the physics simulation has been advanced.
                    // This could be run in `Update`, but if we run it here instead, the systems in `Update`
                    // will be working with the `Transform` that will actually be shown on screen.
                    interpolate_rendered_transform
                        .in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
                ),
            );
    }
}
