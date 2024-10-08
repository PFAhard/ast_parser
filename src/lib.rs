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
}

// fn test_cast() {
//     let x = 0;
//     let x = cast_node_type!(x; ContractDefinition; name, to_owned);
// }
