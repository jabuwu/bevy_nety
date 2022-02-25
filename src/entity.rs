use std::collections::VecDeque;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{events::NetworkEventTraits, serialized_struct::NetworkSerializedStruct};

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

#[derive(Component, Default)]
pub struct NetworkEntityOwner {
    pub events: VecDeque<NetworkSerializedStruct>,
}

impl NetworkEntityOwner {
    pub fn send<T>(&mut self, event: &T)
    where
        T: NetworkEventTraits,
    {
        self.events
            .push_back(NetworkSerializedStruct::from_struct(event));
    }
}
