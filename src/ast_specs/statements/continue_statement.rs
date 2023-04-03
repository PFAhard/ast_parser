use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Continue {
    documentation: Option<String>,
    id: isize,
    src: String,
}

impl Continue {
    pub(crate) fn id(&self) -> isize {
        self.id
    }
}