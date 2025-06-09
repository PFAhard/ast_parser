use serde::Deserialize;

use crate::ast_specs::common::Block;

use super::{prelude::*, Statement};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "nodeType")]
pub enum FalseBody {
    Block(Block),
    Break(Break),
    Continue(Continue),
    DoWhileStatement(DoWhileStatement),
    EmitStatement(EmitStatement),
    ExpressionStatement(ExpressionStatement),
    ForStatement(ForStatement),
    IfStatement(IfStatement),
    // InlineAssembly(InlineAssembly),
    PlaceholderStatement(PlaceholderStatement),
    Return(Return),
    RevertStatement(RevertStatement),
    TryStatement(TryStatement),
    UncheckedBlock(UncheckedBlock),
    VariableDeclarationStatement(VariableDeclarationStatement),
    WhileStatement(WhileStatement),
}

impl FalseBody {
    pub fn is_block(&self) -> bool {
        matches!(self, FalseBody::Block(_)) || matches!(self, FalseBody::UncheckedBlock(_))
    }
}

impl From<FalseBody> for Statement {
    fn from(value: FalseBody) -> Self {
        match value {
            FalseBody::Block(block) => Statement::Block(block),
            FalseBody::Break(break_stmt) => Statement::Break(break_stmt),
            FalseBody::Continue(continue_stmt) => Statement::Continue(continue_stmt),
            FalseBody::DoWhileStatement(do_while_statement) => {
                Statement::DoWhileStatement(do_while_statement)
            }
            FalseBody::EmitStatement(emit_statement) => Statement::EmitStatement(emit_statement),
            FalseBody::ExpressionStatement(expression_statement) => {
                Statement::ExpressionStatement(expression_statement)
            }
            FalseBody::ForStatement(for_statement) => Statement::ForStatement(for_statement),
            FalseBody::IfStatement(if_statement) => Statement::IfStatement(if_statement),
            FalseBody::PlaceholderStatement(placeholder_statement) => {
                Statement::PlaceholderStatement(placeholder_statement)
            }
            FalseBody::Return(return_stmt) => Statement::Return(return_stmt),
            FalseBody::RevertStatement(revert_statement) => {
                Statement::RevertStatement(revert_statement)
            }
            FalseBody::TryStatement(try_statement) => Statement::TryStatement(try_statement),
            FalseBody::UncheckedBlock(unchecked_block) => {
                Statement::UncheckedBlock(unchecked_block)
            }
            FalseBody::VariableDeclarationStatement(variable_declaration_statement) => {
                Statement::VariableDeclarationStatement(variable_declaration_statement)
            }
            FalseBody::WhileStatement(while_statement) => {
                Statement::WhileStatement(while_statement)
            }
        }
    }
}
