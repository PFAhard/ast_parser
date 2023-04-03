mod break_statement;
mod continue_statement;
mod body;
mod do_while;
mod emit_statement;
mod expression_statement;
mod false_body;
mod for_statement;
mod if_statement;
mod placeholder_statement;
mod return_statement;
mod revert_statement;
mod try_statement;
mod unchecked_block;
mod variable_declaration_statement;
mod while_statement;
mod prelude;

use serde::Deserialize;

pub(crate) use prelude::*;

use super::common::Block;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub(crate) enum Statement {
    Block(Block),
    Break(Break),
    Continue(Continue),
    DoWhileStatement(DoWhileStatement),
    EmitStatement(EmitStatement),
    ExpressionStatement(ExpressionStatement),
    ForStatement(ForStatement),
    IfStatement(IfStatement),
    InlineAssembly(serde_json::Value),
    PlaceholderStatement(PlaceholderStatement),
    Return(Return),
    RevertStatement(RevertStatement),
    TryStatement(TryStatement),
    UncheckedBlock(UncheckedBlock),
    VariableDeclarationStatement(VariableDeclarationStatement),
    WhileStatement(WhileStatement),
}
