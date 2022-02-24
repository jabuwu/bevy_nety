use bevy::ecs::system::Resource;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait NetworkPlayerDataTraits: Resource + Serialize + DeserializeOwned + Default {}
impl<T> NetworkPlayerDataTraits for T where T: Resource + Serialize + DeserializeOwned + Default {}
