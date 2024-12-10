use getters::Getters;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Getters, Default)]
pub struct PragmaDirective {
    #[copy]
    id: isize,
    #[return_type = "&[String]"]
    literals: Vec<String>,
    #[return_type = "&str"]
    src: String,
}

impl PragmaDirective {
    pub fn artificial_new(literals: Vec<String>) -> Self {
        Self {
            literals,
            ..Default::default()
        }
    }
}
