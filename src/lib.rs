#![allow(dead_code, unused_variables)]
pub mod ast_framework;
pub mod ast_specs;
pub mod ast_visitor;
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
    ($target: expr; $pat: path; $int: path) => {{
        $target
            .filter_by_node_type($pat)
            .into_iter()
            .filter_map(|v| {
                if let $int(a) = $target {
                    // #1
                    Some(a)
                } else {
                    $crate::AstParserError::result_node_type_internal_cast().ok()
                    // panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
                }
            })
            .collect()
    }};
}
