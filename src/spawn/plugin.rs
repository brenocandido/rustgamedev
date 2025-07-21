use crate::prelude::*;
use crate::spawn::*;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEnemiesEvent>()
            .add_systems(Startup, load_core_assets)
            .add_systems(
                Startup,
                (spawn_text, spawn_player, spawn_map).after(load_core_assets),
            )
            .add_systems(
                Update,
                (spawn_enemy_on_key, spawn_enemies_event_handler).after(load_core_assets),
            );
    }
}