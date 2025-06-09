use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::Expression;

use super::{Body, ExpressionStatement, VariableDeclarationStatement};

#[derive(Deserialize, Debug, Clone, Getters, PartialEq, Eq)]
pub struct ForStatement {
    body: Box<Body>,
    #[return_type = "Option<&Expression>"]
    #[use_as_ref]
    condition: Option<Expression>,
    documentation: Option<String>,
    #[copy]
    id: isize,
    #[serde(rename = "initializationExpression")]
    #[return_type = "Option<&InitializationExpression>"]
    #[use_as_ref]
    initialization_expression: Option<InitializationExpression>,
    #[serde(rename = "loopExpression")]
    #[return_type = "Option<&ExpressionStatement>"]
    #[use_as_ref]
    loop_expression: Option<ExpressionStatement>,
    src: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "nodeType")]
pub enum InitializationExpression {
    ExpressionStatement(ExpressionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
}
