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
    ($target: expr; $pat: ident) => {{
        paste! {
            $target
                .filter_by_node_type([<NodeType:: $pat>])
                .into_iter()
                .filter_map(|v| {
                    if let [<NodeTypeInternal:: $pat (a)>] = v {
                        Some(a)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        }
    }};
    ($target: expr; $pat: ident; $($func:expr),*) => {{
        paste! {
            $target
                .filter_by_node_type([<NodeType:: $pat>])
                .into_iter()
                .filter_map(|v| {
                    if let [<NodeTypeInternal:: $pat (a)>] = v {
                        Some(
                            [<a $(.$func() )* >]
                        )
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        }
    }};
}
