use crate::prelude::*;
use std::collections::HashMap;

/// Keeps track of which pairs touched in the previous step
#[derive(Resource, Default)]
pub struct Contacts {
    pub current: HashMap<(Entity, Entity), ContactData>,
    pub prev:    HashMap<(Entity, Entity), ContactData>,
}

#[derive(Clone, Copy, Default)]
pub struct ContactData {
    pub impulse: f32,          // N·s magnitude of this frame’s hit
}