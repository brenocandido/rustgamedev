use crate::prelude::*;
use std::collections::HashSet;

/// Keeps track of which pairs touched in the previous step
#[derive(Resource, Default)]
pub struct Contacts {
    pub current: HashSet<(Entity, Entity)>,
    pub prev:    HashSet<(Entity, Entity)>,
}