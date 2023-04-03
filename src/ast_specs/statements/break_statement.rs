use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Break {
    documentation: Option<String>,
    id: isize,
    src: String,
}

impl Break {
    pub fn id(&self) -> isize {
        self.id
    }
}