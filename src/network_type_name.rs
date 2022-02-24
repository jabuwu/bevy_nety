use serde::{Deserialize, Serialize};
use std::any::type_name;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NetworkTypeName(String);

impl NetworkTypeName {
    pub fn of<T>() -> Self {
        Self(type_name::<T>().into())
    }
}
