use crate::serializer::{deserialize, serialize};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::any::type_name;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkSerializedStruct {
    pub type_name: String,
    pub data: String,
}

impl NetworkSerializedStruct {
    pub fn from_struct<T>(s: &T) -> Self
    where
        T: Serialize,
    {
        Self {
            type_name: type_name::<T>().into(),
            data: serialize(s),
        }
    }

    pub fn to_struct<T>(&self) -> Option<T>
    where
        T: DeserializeOwned,
    {
        if self.type_name == type_name::<T>() {
            Some(deserialize(&self.data))
        } else {
            None
        }
    }
}
