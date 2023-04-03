use serde::Deserialize;


#[derive(Deserialize, Debug, Clone)]
pub(crate) struct PlaceholderStatement {
    documentation: Option<String>,
    id: isize,
    src: String,
}

impl PlaceholderStatement {
    pub(crate) fn id(&self) -> isize {
        self.id
    }
}