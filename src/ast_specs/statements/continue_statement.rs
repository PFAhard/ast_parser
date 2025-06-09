use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Continue {
    documentation: Option<String>,
    id: isize,
    src: String,
}

impl Continue {
    pub fn id(&self) -> isize {
        self.id
    }
}