#![allow(dead_code, unused_variables)]
#![allow(clippy::too_many_arguments)]
pub mod ast_framework;
pub mod ast_serialize;
pub mod ast_specs;
pub mod ast_visitor;
pub mod error;
pub mod utils;
#[cfg(feature = "zero-cost")]
pub mod zero_cost;
// pub mod ast_descriptor; TODO:

use std::fmt::Debug;

use ast_specs::SourceUnit;
pub use error::*;
use serde::Deserialize;

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
macro_rules! check_node_type {
    ($target: expr, $pat: path) => {{
        matches!($target, $pat(_))
    }};
}

#[macro_export]
macro_rules! cast_node_type {
    ($target:expr; !$pat:ident) => {{
        use ast_parser::ast_specs::NodeType;
        use ast_parser::ast_specs::NodeTypeInternal;
        use ast_parser::ast_visitor::AstVisitor;

        $target
            .filter_by_node_type(NodeType::$pat)
            .into_iter()
            .filter_map(|v| {
                if let NodeTypeInternal::$pat(a) = v {
                    Some(a)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>().pop().unwrap()
    }};
    ($target:expr; $pat:ident) => {{
        use ast_parser::ast_specs::NodeType;
        use ast_parser::ast_specs::NodeTypeInternal;
        use ast_parser::ast_visitor::AstVisitor;

        $target
            .filter_by_node_type(NodeType::$pat)
            .into_iter()
            .filter_map(|v| {
                if let NodeTypeInternal::$pat(a) = v {
                    Some(a)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }};
    ($target:expr; $pat:ident; $($func:ident),*) => {{
            use ast_parser::ast_specs::NodeType;
            use ast_parser::ast_specs::NodeTypeInternal;
            use ast_parser::ast_visitor::AstVisitor;

            $target
                .filter_by_node_type(NodeType::$pat)
                .into_iter()
                .filter_map(|v| {
                    if let NodeTypeInternal::$pat(a) = v {
                        Some(
                            a$(.$func())*
                        )
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
    }};
    ($target:expr; $pat:ident; $([$($func:ident),* ]),*) => {{
        use ast_parser::ast_specs::NodeType;
        use ast_parser::ast_specs::NodeTypeInternal;
        use ast_parser::ast_visitor::AstVisitor;

        $target
            .filter_by_node_type(NodeType::$pat)
            .into_iter()
            .filter_map(|v| {
                if let NodeTypeInternal::$pat(a) = v {
                    Some(
                        ($(
                            a$(.$func())*,
                        )*)
                    )
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }};
    ($target:expr; $pat:ident; $([$($func:ident),* ]),* unzip) => {{
        use ast_parser::ast_specs::NodeType;
        use ast_parser::ast_specs::NodeTypeInternal;
        use ast_parser::ast_visitor::AstVisitor;

        $target
            .filter_by_node_type(NodeType::$pat)
            .into_iter()
            .filter_map(|v| {
                if let NodeTypeInternal::$pat(a) = v {
                    Some(
                        ($(
                            a$(.$func())*,
                        )*)
                    )
                } else {
                    None
                }
            })
            .unzip()
    }};
    ($target:expr; $pat:ident; $filter:ident) => {{
        use ast_parser::ast_specs::NodeType;
        use ast_parser::ast_specs::NodeTypeInternal;
        use ast_parser::ast_visitor::AstVisitor;

        $target
            .filter_by_node_type(NodeType::$pat)
            .into_iter()
            .filter_map(|v| {
                if let NodeTypeInternal::$pat(a) = v {
                    if a.$filter() {
                        Some(a)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }};
    ($target:expr; $pat:ident; $filter:ident; $($func:ident),*) => {{
        use ast_parser::ast_specs::NodeType;
        use ast_parser::ast_specs::NodeTypeInternal;
        use ast_parser::ast_visitor::AstVisitor;

        $target
            .filter_by_node_type(NodeType::$pat)
            .into_iter()
            .filter_map(|v| {
                if let NodeTypeInternal::$pat(a) = v {
                    if a.$filter() {
                        Some(
                            a$(.$func())*
                        )
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }};
    ($target:expr; $pat:ident; $filter:ident; $([$($func:ident),* ]),*) => {{
        use ast_parser::ast_specs::NodeType;
        use ast_parser::ast_specs::NodeTypeInternal;
        use ast_parser::ast_visitor::AstVisitor;

        $target
            .filter_by_node_type(NodeType::$pat)
            .into_iter()
            .filter_map(|v| {
                if let NodeTypeInternal::$pat(a) = v {
                    if a.$filter() {
                        Some(
                            ($(
                                a$(.$func())*,
                            )*)
                        )
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }};
    ($target:expr; $pat:ident; $filter:ident; $([$($func:ident),* ]),* unzip) => {{
        use ast_parser::ast_specs::NodeType;
        use ast_parser::ast_specs::NodeTypeInternal;
        use ast_parser::ast_visitor::AstVisitor;

        $target
            .filter_by_node_type(NodeType::$pat)
            .into_iter()
            .filter_map(|v| {
                if let NodeTypeInternal::$pat(a) = v {
                    if a.$filter() {
                        Some(
                            ($(
                                a$(.$func())*,
                            )*)
                        )
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unzip()
    }};
}

// fn test_cast() {
//     let x = 0;
//     let x = cast_node_type!(x; ContractDefinition; name, to_owned);
// }

#[derive(Debug, Deserialize)]
struct FoundryWrapper {
    ast: SourceUnit,
}

pub fn cast_to_source_unit<R>(path: R) -> SourceUnit
where
    R: std::io::Read,
{
    serde_json::from_reader::<_, FoundryWrapper>(path)
        .unwrap()
        .ast
}
