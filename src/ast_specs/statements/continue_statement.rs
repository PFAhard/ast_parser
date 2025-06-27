use getters::Getters;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Getters, Eq)]
pub struct Continue {
    documentation: Option<String>,
    #[copy]
    id: isize,
    src: String,
}
