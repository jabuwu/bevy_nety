use super::serializer::{deserialize, serialize};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NetworkMessage {
    Event,
}

impl NetworkMessage {
    pub fn deserialize(string: &str) -> Self {
        deserialize::<Self>(string)
    }
    pub fn serialize(&self) -> String {
        serialize(&self)
    }
}
