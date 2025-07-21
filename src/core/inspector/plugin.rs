use crate::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct InspectorPlugin;

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
