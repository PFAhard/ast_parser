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

macro_rules! impl_statement_from {
    ($variant:ident) => {
        impl From<$variant> for Statement {
            fn from(value: $variant) -> Self {
                Statement::$variant(value)
            }
        }

        impl TryFrom<Statement> for $variant {
            type Error = Statement;

            fn try_from(value: Statement) -> Result<Self, Self::Error> {
                match value {
                    Statement::$variant(v) => Ok(v),
                    other => Err(other),
                }
            }
        }
    };
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
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

impl_statement_from!(Block);
impl_statement_from!(Break);
impl_statement_from!(Continue);
impl_statement_from!(DoWhileStatement);
impl_statement_from!(EmitStatement);
impl_statement_from!(ExpressionStatement);
impl_statement_from!(ForStatement);
impl_statement_from!(IfStatement);
impl_statement_from!(InlineAssembly);
impl_statement_from!(PlaceholderStatement);
impl_statement_from!(Return);
impl_statement_from!(RevertStatement);
impl_statement_from!(TryStatement);
impl_statement_from!(UncheckedBlock);
impl_statement_from!(VariableDeclarationStatement);
impl_statement_from!(WhileStatement);

impl Statement {
    pub fn id(&self) -> isize {
        match self {
            Statement::Block(i) => i.id(),
            Statement::Break(i) => i.id(),
            Statement::Continue(i) => i.id(),
            Statement::DoWhileStatement(i) => i.id(),
            Statement::EmitStatement(i) => i.id(),
            Statement::ExpressionStatement(i) => i.expression().as_ref().unwrap().id(),
            Statement::ForStatement(i) => i.id(),
            Statement::IfStatement(i) => i.id(),
            Statement::InlineAssembly(i) => i.id(),
            Statement::PlaceholderStatement(i) => i.id(),
            Statement::Return(i) => i.id(),
            Statement::RevertStatement(i) => i.id(),
            Statement::TryStatement(i) => i.id(),
            Statement::UncheckedBlock(i) => i.id(),
            Statement::VariableDeclarationStatement(i) => i.id(),
            Statement::WhileStatement(i) => i.id(),
        }
    }
}

impl From<Body> for Statement {
    fn from(value: Body) -> Self {
        match value {
            Body::Block(block) => block.into(),
            Body::Break(break_statement) => break_statement.into(),
            Body::Continue(continue_statement) => continue_statement.into(),
            Body::DoWhileStatement(do_while_statement) => do_while_statement.into(),
            Body::EmitStatement(emit_statement) => emit_statement.into(),
            Body::ExpressionStatement(expression_statement) => expression_statement.into(),
            Body::ForStatement(for_statement) => for_statement.into(),
            Body::IfStatement(if_statement) => if_statement.into(),
            Body::InlineAssembly(inline_assembly) => inline_assembly.into(),
            Body::PlaceholderStatement(placeholder_statement) => placeholder_statement.into(),
            Body::Return(ret) => ret.into(),
            Body::RevertStatement(revert_statement) => revert_statement.into(),
            Body::TryStatement(try_statement) => try_statement.into(),
            Body::UncheckedBlock(unchecked_block) => unchecked_block.into(),
            Body::VariableDeclarationStatement(variable_declaration_statement) => {
                variable_declaration_statement.into()
            }
            Body::WhileStatement(while_statement) => while_statement.into(),
        }
    }
}

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
