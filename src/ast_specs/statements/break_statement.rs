use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Break {
    documentation: Option<String>,
    id: isize,
    src: String,
}

impl Break {
    pub(crate) fn id(&self) -> isize {
        self.id
    }
}