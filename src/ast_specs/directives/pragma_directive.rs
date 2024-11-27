use getters::Getters;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct PragmaDirective {
    #[copy]
    id: isize,
    #[return_type = "&[String]"]
    literals: Vec<String>,
    #[return_type = "&str"]
    src: String,
}
