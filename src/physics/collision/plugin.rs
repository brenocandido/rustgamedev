use crate::physics::*;
use crate::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CollisionSet {
    Detect,
    EmitEvents,
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(FixedUpdate, CollisionSet::Detect.after(advance_physics))
            .configure_sets(
                FixedUpdate,
                CollisionSet::EmitEvents.after(CollisionSet::Detect),
            )
            // Resources and events
            .init_resource::<Contacts>()
            .add_event::<CollisionEvent>()
            // Systems
            .add_systems(
                FixedUpdate,
                (circle_wall_collision_system, circle_circle_collision_system)
                    .in_set(CollisionSet::Detect),
            )
            .add_systems(
                FixedUpdate,
                emit_collision_events.in_set(CollisionSet::EmitEvents),
            );
    }
}
