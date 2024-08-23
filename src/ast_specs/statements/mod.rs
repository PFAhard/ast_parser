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

#[derive(Deserialize, Debug, Clone)]
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

impl Statement {
    pub fn id(&self) -> isize {
        match self {
            Statement::Block(i) => i.id(),
            Statement::Break(i) => i.id(),
            Statement::Continue(i) => i.id(),
            Statement::DoWhileStatement(i) => i.id(),
            Statement::EmitStatement(i) => i.id(),
            Statement::ExpressionStatement(i) => i.id(),
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
