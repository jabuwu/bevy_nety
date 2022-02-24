use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NetworkEntity(pub Uuid);

impl NetworkEntity {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for NetworkEntity {
    fn default() -> Self {
        Self::new()
    }
}
