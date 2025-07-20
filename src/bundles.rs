use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use bevy::sprite::{Material2d, MeshMaterial2d};

#[derive(Bundle, Default)]
pub struct MovableBundle {
    pub movable: Movable,
    pub velocity: Velocity,
    pub input: AccumulatedInput,
    pub acceleration: Acceleration,
    pub phy_translation: PhysicalTranslation, // TODO: Ensure phy_translation is initialized to the same as transform
    pub prev_phy_translation: PreviousPhysicalTranslation,
}

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

#[derive(Bundle)]
pub struct EnemyBundle {
    shape: Shape2dBundle<ColorMaterial>,
    movable: MovableBundle,
    name: Name,
}

impl EnemyBundle {
    pub fn new(
        meshes: &CoreMeshes,
        materials: &CoreMaterials,
        pos: Vec2,
    ) -> Self {
        Self {
            name: Name::new("Enemy"),
            shape: Shape2dBundle::circle(
                meshes.circle.clone(),
                materials.enemy.clone(),   // Handle<ColorMaterial>
                50.0,
                pos,
            ),
            movable: MovableBundle::default(),
        }
    }
}