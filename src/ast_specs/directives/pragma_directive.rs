use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PragmaDirective {
    id: isize,
    literals: Vec<String>,
    src: String,
}

impl PragmaDirective {
    pub fn id(&self) -> isize {
        self.id
    }
}
