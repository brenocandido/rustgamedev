use crate::prelude::*;
use bevy::sprite::{Material2d, MeshMaterial2d};

#[derive(Bundle)]
pub struct Shape2dBundle<M: Material2d + Clone> {
    mesh:      Mesh2d,
    material:  MeshMaterial2d<M>,
    transform: Transform,
    collider:  Collider,
}

impl<M: Material2d + Clone> Shape2dBundle<M> {
    pub fn rect(
        mesh: Handle<Mesh>,
        material: Handle<M>,
        size: Vec2,
        pos: Vec2,
    ) -> Self {
        Self {
            mesh:     Mesh2d(mesh),
            material: MeshMaterial2d(material),
            transform: Transform::from_translation(pos.extend(0.0))
                .with_scale(Vec3::new(size.x, size.y, 1.0)),
            collider: Collider(ColliderShape::Rect { half_extents: size * 0.5 }),
        }
    }

    pub fn circle(
        mesh: Handle<Mesh>,
        material: Handle<M>,
        radius: f32,
        pos: Vec2,
    ) -> Self {
        Self {
            mesh:     Mesh2d(mesh),
            material: MeshMaterial2d(material),
            transform: Transform::from_translation(pos.extend(0.0))
                .with_scale(Vec3::splat(radius)),
            collider: Collider(ColliderShape::Circle { radius }),
        }
    }
}

