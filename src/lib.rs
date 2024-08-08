#![allow(dead_code, unused_variables)]
pub mod ast_specs;
pub mod ast_visitor;
pub mod ast_framework;
pub mod error;
// pub mod ast_descriptor; TODO:

pub use error::*;


#[macro_export]
macro_rules! unwrap_node_type {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = $target {
            // #1
            Ok(a)
        } else {
            AstParserError::result_node_type_internal_cast()
            // panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
        }
    }};
}

#[macro_export]
macro_rules! cast_node_type {
    ($target: expr, $pat: path) => {{
        $target.filter_by_node_type($pat)
            .into_iter()
            .map(|v| $crate::unwrap_node_type!(v, $pat))
            .collect()
    }};
}