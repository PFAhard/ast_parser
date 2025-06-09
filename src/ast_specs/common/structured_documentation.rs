use getters::Getters;
use serde::Deserialize;


#[derive(Deserialize, Debug, Clone, Getters, PartialEq, Eq)]
pub struct StructuredDocumentation {
    id: isize,
    src: String,
    #[return_type = "&str"]
    text: String,
}

