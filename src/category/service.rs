#[derive(Debug, Clone)]
pub struct Category {
    pub name: String,
}
impl Category {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
