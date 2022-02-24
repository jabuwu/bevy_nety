use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TestGameEvent {
    pub foo: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TestPlayerData {
    pub name: String,
}
