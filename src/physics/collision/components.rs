use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Reflect, InspectorOptions)]
pub enum ColliderShape {
    Circle { radius: f32 },
    Rect { half_extents: Vec2 }, // half-size in x and y (or x and z) directions
    None,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Deref, DerefMut, Reflect, InspectorOptions)]
#[reflect(Component)]
pub struct Collider(pub ColliderShape);
