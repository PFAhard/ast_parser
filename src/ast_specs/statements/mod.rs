mod body;
mod break_statement;
mod continue_statement;
mod do_while;
mod emit_statement;
mod expression_statement;
mod false_body;
mod for_statement;
mod if_statement;
mod placeholder_statement;
mod prelude;
mod return_statement;
mod revert_statement;
mod try_statement;
mod unchecked_block;
mod variable_declaration_statement;
mod while_statement;

use serde::Deserialize;

pub use prelude::*;

use super::{common::Block, inline_assembly::InlineAssembly};

crate::enum_refs! {
    #[derive_owned(Deserialize)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[serde(tag = "nodeType")]
    pub enum Statement {
        Block(Block),
        Break(Break),
        Continue(Continue),
        DoWhileStatement(DoWhileStatement),
        EmitStatement(EmitStatement),
        ExpressionStatement(ExpressionStatement),
        ForStatement(ForStatement),
        IfStatement(IfStatement),
        InlineAssembly(InlineAssembly),
        PlaceholderStatement(PlaceholderStatement),
        Return(Return),
        RevertStatement(RevertStatement),
        TryStatement(TryStatement),
        UncheckedBlock(UncheckedBlock),
        VariableDeclarationStatement(VariableDeclarationStatement),
        WhileStatement(WhileStatement),
    }
}

macro_rules! st_delegate_functions {
    (
        $($variant:ident),*
    ) => {
        impl Statement {
            pub fn id(&self) -> isize {
                match self {
                    $(
                        Statement::$variant(i) => i.id(),
                    )*
                }
            }
        }

        impl StatementRef<'_> {
            pub fn id(&self) -> isize {
                match self {
                    $(
                        StatementRef::$variant(i) => i.id(),
                    )*
                }
            }
        }
    };
}

macro_rules! external_to_st {
    (
        $($variant:ident),*
    ) => {
        impl From<Body> for Statement {
            fn from(value: Body) -> Self {
                match value {
                    $(
                        Body::$variant(value) => value.into(),
                    )*
                }
            }
        }

        impl<'a> From<&'a Body> for StatementRef<'a> {
            fn from(value: &'a Body) -> Self {
                match value {
                    $(
                        Body::$variant(value) => value.into(),
                    )*
                }
            }
        }
    };
}

macro_rules! combo_st {
    ($($variant:ident),*) => {
        st_delegate_functions!($($variant),*);
        external_to_st!($($variant),*);
    };
}

combo_st!(
    Block,
    Break,
    Continue,
    DoWhileStatement,
    EmitStatement,
    ExpressionStatement,
    ForStatement,
    IfStatement,
    InlineAssembly,
    PlaceholderStatement,
    Return,
    RevertStatement,
    TryStatement,
    UncheckedBlock,
    VariableDeclarationStatement,
    WhileStatement
);

impl From<InitializationExpression> for Statement {
    fn from(value: InitializationExpression) -> Self {
        match value {
            InitializationExpression::ExpressionStatement(expression_statement) => {
                expression_statement.into()
            }
            InitializationExpression::VariableDeclarationStatement(
                variable_declaration_statement,
            ) => variable_declaration_statement.into(),
        }
    }
}

impl<'a> From<&'a InitializationExpression> for StatementRef<'a> {
    fn from(value: &'a InitializationExpression) -> Self {
        match value {
            InitializationExpression::ExpressionStatement(expression_statement) => {
                expression_statement.into()
            }
            InitializationExpression::VariableDeclarationStatement(
                variable_declaration_statement,
            ) => variable_declaration_statement.into(),
        }
    }
}
