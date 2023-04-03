use serde::Deserialize;


#[derive(Deserialize, Debug, Clone)]
pub struct PlaceholderStatement {
    documentation: Option<String>,
    id: isize,
    src: String,
}

impl PlaceholderStatement {
    pub fn id(&self) -> isize {
        self.id
    }
}