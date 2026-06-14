use crate::behavior::*;
use crate::prelude::*;

pub struct BehaviorPlugin;

impl Plugin for BehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            RunFixedMainLoop,
            // enemy_steering_system.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
            (
                // Behavior systems
                seek_player_system.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
                flee_from_player_system.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
                wander_system.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
                maintain_range_from_player_system
                    .in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
                avoid_allies_system.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
            ),
        );
    }
}
