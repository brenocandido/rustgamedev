use crate::prelude::*;

#[derive(Resource)]
pub struct CoreMeshes {
    pub rect: Handle<Mesh>,
    pub circle: Handle<Mesh>,
}
#[derive(Resource)]
pub struct CoreMaterials {
    pub bounds: Handle<ColorMaterial>,
    pub wall: Handle<ColorMaterial>,
    pub enemy: Handle<ColorMaterial>,
    pub player: Handle<ColorMaterial>,
}