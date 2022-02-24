use crate::network_type_name::NetworkTypeName;
use crate::serializer::{deserialize, serialize};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkSerializedStruct {
    pub type_name: NetworkTypeName,
    pub data: String,
}

impl NetworkSerializedStruct {
    pub fn from_struct<T>(s: &T) -> Self
    where
        T: Serialize,
    {
        Self {
            type_name: NetworkTypeName::of::<T>(),
            data: serialize(s),
        }
    }

    pub fn to_struct<T>(&self) -> Option<T>
    where
        T: DeserializeOwned,
    {
        if self.type_name == NetworkTypeName::of::<T>() {
            Some(deserialize(&self.data))
        } else {
            None
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkSerializedStructMap {
    data: HashMap<NetworkTypeName, NetworkSerializedStruct>,
}

impl NetworkSerializedStructMap {
    pub fn set<T>(&mut self, data: T)
    where
        T: Serialize,
    {
        self.data.insert(
            NetworkTypeName::of::<T>().into(),
            NetworkSerializedStruct::from_struct(&data),
        );
    }

    pub fn get<T>(&self) -> Option<T>
    where
        T: DeserializeOwned,
    {
        if let Some(s) = self.data.get(&NetworkTypeName::of::<T>()) {
            s.to_struct::<T>()
        } else {
            None
        }
    }
}
