use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct PragmaDirective {
    id: isize,
    literals: Vec<String>,
    src: String,
}

impl PragmaDirective {
    pub(crate) fn id(&self) -> isize {
        self.id
    }
}
