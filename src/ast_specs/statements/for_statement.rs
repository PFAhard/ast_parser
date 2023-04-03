use serde::Deserialize;

use crate::ast_parser::ast_specs::Expression;

use super::{Body, ExpressionStatement, VariableDeclarationStatement};

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ForStatement {
    body: Box<Body>,
    condition: Option<Expression>,
    documentation: Option<String>,
    id: isize,
    #[serde(rename = "initializationExpression")]
    initialization_expression: Option<InitializationExpression>,
    #[serde(rename = "loopExpression")]
    loop_expression: Option<ExpressionStatement>,
    src: String,
}

impl ForStatement {
    pub(crate) fn body(&self) -> &Body {
        self.body.as_ref()
    }

    pub(crate) fn condition(&self) -> Option<&Expression> {
        self.condition.as_ref()
    }

    pub(crate) fn initialization_expression(&self) -> Option<&InitializationExpression> {
        self.initialization_expression.as_ref()
    }

    pub(crate) fn loop_expression(&self) -> Option<&ExpressionStatement> {
        self.loop_expression.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub(crate) enum InitializationExpression {
    ExpressionStatement(ExpressionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
}