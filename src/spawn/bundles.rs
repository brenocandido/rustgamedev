use crate::prelude::*;

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
                materials.enemy.clone(),
                50.0,
                pos,
            ),
            movable: MovableBundle::default(),
        }
    }
}