pub struct Workflow {
    name: String,
}

impl Workflow {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
