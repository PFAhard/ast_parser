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

#[derive(Debug, Serialize, Deserialize)]
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
