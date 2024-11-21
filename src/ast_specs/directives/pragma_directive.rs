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

    pub fn src(&self) -> &str {
        &self.src
    }

    pub fn literals(&self) -> &[String] {
        &self.literals
    }
}
