use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NetworkPlayer(Uuid);

impl NetworkPlayer {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
