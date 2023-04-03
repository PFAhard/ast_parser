pub type AstParserResult<T> = std::result::Result<T, AstParserError>;

#[derive(Debug, Clone)]
pub struct AstParserError(ErrorKind);

#[derive(Debug, Clone)]
pub enum ErrorKind {
    JsonParsingError {
        struct_name: Option<String>,
        inner: Option<String>,
    },
    NodeTypeInternalCast, /* {
                              expr: String,
                              pattern: String, // TODO: More information about
                          }, */
}

impl AstParserError {
    pub fn result_node_type_internal_cast<T>() -> AstParserResult<T> {
        Err(AstParserError(ErrorKind::NodeTypeInternalCast))
    }
}
