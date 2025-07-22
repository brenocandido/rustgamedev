use crate::prelude::*;

#[derive(Bundle)]
pub struct CharacterBundle {
    shape: Shape2dBundle<ColorMaterial>,
    movable: MovableBundle,
    name: Name,
    health: Health,
    mass: Mass,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    character: CharacterBundle,
    tag: Enemy,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    character: CharacterBundle,
    tag: Player,
}

impl CharacterBundle {
    pub fn new(
        name: &str,
        meshes: &CoreMeshes,
        material: Handle<ColorMaterial>,
        pos: Vec2,
    ) -> Self {
        Self {
            name: Name::new(name.to_string()),
            shape: Shape2dBundle::circle(meshes.circle.clone(), material, 50.0, pos),
            movable: MovableBundle::default(),
            health: Health::new(100.0),
            mass: Mass::default(),
        }
    }
}

impl EnemyBundle {
    pub fn new(meshes: &CoreMeshes, materials: &CoreMaterials, pos: Vec2) -> Self {
        Self {
            character: CharacterBundle::new("Enemy", meshes, materials.enemy.clone(), pos),
            tag: Enemy,
        }
    }
}

impl PlayerBundle {
    pub fn new(meshes: &CoreMeshes, materials: &CoreMaterials, pos: Vec2) -> Self {
        Self {
            character: CharacterBundle::new("Player", meshes, materials.player.clone(), pos),
            tag: Player,
        }
    }
}
