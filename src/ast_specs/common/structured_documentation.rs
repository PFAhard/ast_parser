use serde::Deserialize;


#[derive(Deserialize, Debug, Clone)]
pub(crate) struct StructuredDocumentation {
    id: isize,
    src: String,
    text: String,
}

