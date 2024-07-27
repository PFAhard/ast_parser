use serde::{Deserialize, Serialize};

use super::{
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
