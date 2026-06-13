use crate::prelude::*;

#[derive(Message)]
pub enum CollisionEvent {
    Started {
        a: Entity,
        b: Entity,
        impulse: f32, /* impulse N·s */
        v_a_n: f32,
        v_b_n: f32,
    },

    Stopped {
        a: Entity,
        b: Entity,
    },
}
