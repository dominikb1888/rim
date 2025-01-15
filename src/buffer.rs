pub struct Buffer {
    pub content: Vec<String>,
}

impl Buffer {
    pub fn default() -> Self {
        Self { content: vec!["Hello, World".to_string()] }
    }

    pub fn load () -> Self {
        todo!();
    }
}
