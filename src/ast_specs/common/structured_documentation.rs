use getters::Getters;
use serde::Deserialize;


#[derive(Deserialize, Debug, Clone, Getters)]
pub struct StructuredDocumentation {
    id: isize,
    src: String,
    #[return_type = "&str"]
    text: String,
}

