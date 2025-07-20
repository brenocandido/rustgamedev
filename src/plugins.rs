use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::prelude::*;

pub struct GamePlugin;
pub struct SpawnerPlugin;
pub struct PhysicsPlugin;
pub struct InspectorPlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InspectorPlugin)
            .add_plugins(SpawnerPlugin)
            .add_plugins(PhysicsPlugin);
    }
}

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_core_assets)
            .add_systems(
                Startup,
                (spawn_text, spawn_player, spawn_map).after(load_core_assets),
            )
            .add_systems(Update, spawn_enemy_on_key.after(load_core_assets));
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Constants used for physics systems.
            .insert_resource(PhysicsConfig::default())
            .add_event::<CollisionEvent>()
            // Advance the physics simulation using a fixed timestep.
            .add_systems(
                FixedUpdate,
                (
                    advance_physics,
                    circle_wall_collision_system,
                    circle_circle_collision_system,
                )
                    .chain(),
            )
            .add_systems(
                // The `RunFixedMainLoop` schedule allows us to schedule systems to run before and after the fixed timestep loop.
                RunFixedMainLoop,
                (
                    // The physics simulation needs to know the player's input, so we run this before the fixed timestep loop.
                    // Note that if we ran it in `Update`, it would be too late, as the physics simulation would already have been advanced.
                    // If we ran this in `FixedUpdate`, it would sometimes not register player input, as that schedule may run zero times per frame.
                    handle_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                    // The player's visual representation needs to be updated after the physics simulation has been advanced.
                    // This could be run in `Update`, but if we run it here instead, the systems in `Update`
                    // will be working with the `Transform` that will actually be shown on screen.
                    interpolate_rendered_transform
                        .in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
                ),
            );
    }
}

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AccumulatedInput>()
            .register_type::<Velocity>()
            .register_type::<PhysicalTranslation>()
            .register_type::<PreviousPhysicalTranslation>()
            .register_type::<Acceleration>()
            .register_type::<PhysicsConfig>()
            .register_type::<Collider>()
            .add_plugins(EguiPlugin::default())
            .add_plugins(WorldInspectorPlugin::default());
    }
}
