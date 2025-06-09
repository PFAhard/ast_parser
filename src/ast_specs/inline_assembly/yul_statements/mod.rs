use serde::{Deserialize, Serialize};

pub mod yul_assignment;
pub mod yul_block;
pub mod yul_break;
pub mod yul_continue;
pub mod yul_expression_statement;
pub mod yul_for_loop;
pub mod yul_function_definition;
pub mod yul_if;
pub mod yul_leave;
pub mod yul_switch;
pub mod yul_variable_declaration;

use self::{
    yul_assignment::YulAssignment, yul_block::YulBlock, yul_break::YulBreak,
    yul_continue::YulContinue, yul_expression_statement::YulExpressionStatement,
    yul_for_loop::YulForLoop, yul_function_definition::YulFunctionDefinition, yul_if::YulIf,
    yul_leave::YulLeave, yul_switch::YulSwitch, yul_variable_declaration::YulVariableDeclaration,
};

macro_rules! impl_from_for_yul_statement {
    ($variant:ident) => {
        impl From<$variant> for YulStatement {
            fn from(value: $variant) -> Self {
                YulStatement::$variant(value)
            }
        }

        impl From<&$variant> for YulStatement {
            fn from(value: &$variant) -> Self {
                YulStatement::from(value.to_owned())
            }
        }
    };
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "nodeType")]
pub enum YulStatement {
    YulAssignment(YulAssignment),
    YulBlock(YulBlock),
    YulBreak(YulBreak),
    YulContinue(YulContinue),
    YulExpressionStatement(YulExpressionStatement),
    YulLeave(YulLeave),
    YulForLoop(YulForLoop),
    YulFunctionDefinition(YulFunctionDefinition),
    YulIf(YulIf),
    YulSwitch(YulSwitch),
    YulVariableDeclaration(YulVariableDeclaration),
}

impl_from_for_yul_statement!(YulAssignment);
impl_from_for_yul_statement!(YulBlock);
impl_from_for_yul_statement!(YulBreak);
impl_from_for_yul_statement!(YulContinue);
impl_from_for_yul_statement!(YulExpressionStatement);
impl_from_for_yul_statement!(YulLeave);
impl_from_for_yul_statement!(YulForLoop);
impl_from_for_yul_statement!(YulFunctionDefinition);
impl_from_for_yul_statement!(YulIf);
impl_from_for_yul_statement!(YulSwitch);
impl_from_for_yul_statement!(YulVariableDeclaration);
