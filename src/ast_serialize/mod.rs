use crate::ast_specs::SourceUnit;

pub trait AstSerializer {
    fn to_sol() -> Vec<u8>;
}

impl AstSerializer for SourceUnit {
    fn to_sol() -> Vec<u8> {
        
    }
}