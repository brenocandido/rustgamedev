use crate::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InspectorPlugin)
            .add_plugins(SpawnerPlugin)
            .add_plugins(PhysicsPlugin)
            .add_plugins(CombatPlugin)
            .add_plugins(InputPlugin);
    }
}
