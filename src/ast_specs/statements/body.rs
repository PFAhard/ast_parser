use serde::Deserialize;

use crate::ast_specs::common::Block;

use super::{
    Break, Continue, DoWhileStatement, EmitStatement, ExpressionStatement, ForStatement,
    IfStatement, PlaceholderStatement, Return, RevertStatement, TryStatement, UncheckedBlock,
    VariableDeclarationStatement, WhileStatement,
};

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub enum Body {
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
