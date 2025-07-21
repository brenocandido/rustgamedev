use crate::prelude::*;
use crate::physics::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>().add_systems(
            FixedUpdate,
            (circle_wall_collision_system, circle_circle_collision_system)
                .after(advance_physics)
                .chain(),
        );
    }
}
