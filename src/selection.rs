#[derive(Clone, Debug, Default)]
pub struct Selection {
    pub content: String,
}

impl Selection {
    pub fn new(content: &String) -> Self {
        Selection {
            content: content.clone(),
        }
    }
}
