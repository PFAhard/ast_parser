use serde::Deserialize;


#[derive(Deserialize, Debug, Clone)]
pub struct StructuredDocumentation {
    id: isize,
    src: String,
    text: String,
}

