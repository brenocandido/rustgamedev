use crate::behavior::*;
use crate::prelude::*;

pub struct BehaviorPlugin;

impl Plugin for BehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            RunFixedMainLoop,
            enemy_steering_system.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
        );
    }
}
