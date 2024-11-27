use getters::Getters;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct Break {
    documentation: Option<String>,
    #[copy]
    id: isize,
    #[return_type = "&str"]
    src: String,
}
