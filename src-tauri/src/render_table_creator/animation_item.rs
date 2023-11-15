pub struct AnimationItem {
    pub id: String,
    pub description: String,
    pub clothing: String,
    pub angle: String,
    pub speed: String,
    pub occurrences: i32,
}

impl AnimationItem {
    pub fn new(
        id: String,
        description: String,
        clothing: String,
        angle: String,
        speed: String,
    ) -> Self {
        Self {
            id,
            description,
            clothing,
            angle,
            speed,
            occurrences: 1,
        }
    }
}

impl PartialEq for AnimationItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
