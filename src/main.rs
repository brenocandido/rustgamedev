use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod constants;
mod components;
mod bundles;

mod systems;
use systems::*;
use components::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .register_type::<AccumulatedInput>()
        .register_type::<Velocity>()
        .register_type::<PhysicalTranslation>()
        .register_type::<PreviousPhysicalTranslation>()
        .register_type::<Acceleration>()
        .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Startup, (spawn_text, spawn_player, spawn_map))
        // Advance the physics simulation using a fixed timestep.
        .add_systems(FixedUpdate, advance_physics)
        .add_systems(
            // The `RunFixedMainLoop` schedule allows us to schedule systems to run before and after the fixed timestep loop.
            RunFixedMainLoop,
            (
                circle_rect_collider.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                // The physics simulation needs to know the player's input, so we run this before the fixed timestep loop.
                // Note that if we ran it in `Update`, it would be too late, as the physics simulation would already have been advanced.
                // If we ran this in `FixedUpdate`, it would sometimes not register player input, as that schedule may run zero times per frame.
                handle_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                // The player's visual representation needs to be updated after the physics simulation has been advanced.
                // This could be run in `Update`, but if we run it here instead, the systems in `Update`
                // will be working with the `Transform` that will actually be shown on screen.
                interpolate_rendered_transform.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            ),
        )
        .run();
}
