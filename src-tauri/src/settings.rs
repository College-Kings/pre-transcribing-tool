use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Settings {
    Episode,
    SceneNumber,
    SceneName,
}