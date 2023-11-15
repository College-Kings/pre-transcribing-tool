pub struct SceneItem {
    pub id: String,
    pub description: String,
    pub occurrences: i32,
}

impl SceneItem {
    pub fn new(id: String, description: String) -> Self {
        Self {
            id,
            description,
            occurrences: 1,
        }
    }
}

impl PartialEq for SceneItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
