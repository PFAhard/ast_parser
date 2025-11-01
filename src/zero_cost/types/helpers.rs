use crate::zero_cost::types::abstraction::{
    ZcBody, ZcFalseBody, ZcInitializationExpression, ZcStatement,
};

impl<'a> From<ZcBody<'a>> for ZcStatement<'a> {
    fn from(value: ZcBody<'a>) -> Self {
        match value {
            ZcBody::ZcBlock(zc_block) => ZcStatement::ZcBlock(zc_block),
            ZcBody::ZcBreak(zc_break) => ZcStatement::ZcBreak(zc_break),
            ZcBody::ZcContinue(zc_continue) => ZcStatement::ZcContinue(zc_continue),
            ZcBody::ZcDoWhileStatement(zc_do_while_statement) => {
                ZcStatement::ZcDoWhileStatement(zc_do_while_statement)
            }
            ZcBody::ZcEmitStatement(zc_emit_statement) => {
                ZcStatement::ZcEmitStatement(zc_emit_statement)
            }
            ZcBody::ZcExpressionStatement(zc_expression_statement) => {
                ZcStatement::ZcExpressionStatement(zc_expression_statement)
            }
            ZcBody::ZcForStatement(zc_for_statement) => {
                ZcStatement::ZcForStatement(zc_for_statement)
            }
            ZcBody::ZcIfStatement(zc_if_statement) => ZcStatement::ZcIfStatement(zc_if_statement),
            ZcBody::ZcInlineAssembly(zc_inline_assembly) => {
                ZcStatement::ZcInlineAssembly(zc_inline_assembly)
            }
            ZcBody::ZcPlaceholderStatement(zc_placeholder_statement) => {
                ZcStatement::ZcPlaceholderStatement(zc_placeholder_statement)
            }
            ZcBody::ZcReturn(zc_return) => ZcStatement::ZcReturn(zc_return),
            ZcBody::ZcRevertStatement(zc_revert_statement) => {
                ZcStatement::ZcRevertStatement(zc_revert_statement)
            }
            ZcBody::ZcTryStatement(zc_try_statement) => {
                ZcStatement::ZcTryStatement(zc_try_statement)
            }
            ZcBody::ZcUncheckedBlock(zc_unchecked_block) => {
                ZcStatement::ZcUncheckedBlock(zc_unchecked_block)
            }
            ZcBody::ZcVariableDeclarationStatement(zc_variable_declaration_statement) => {
                ZcStatement::ZcVariableDeclarationStatement(zc_variable_declaration_statement)
            }
            ZcBody::ZcWhileStatement(zc_while_statement) => {
                ZcStatement::ZcWhileStatement(zc_while_statement)
            }
        }
    }
}

impl<'a> From<ZcFalseBody<'a>> for ZcStatement<'a> {
    fn from(value: ZcFalseBody<'a>) -> Self {
        match value {
            ZcFalseBody::ZcBlock(zc_block) => ZcStatement::ZcBlock(zc_block),
            ZcFalseBody::ZcBreak(zc_break) => ZcStatement::ZcBreak(zc_break),
            ZcFalseBody::ZcContinue(zc_continue) => ZcStatement::ZcContinue(zc_continue),
            ZcFalseBody::ZcDoWhileStatement(zc_do_while_statement) => {
                ZcStatement::ZcDoWhileStatement(zc_do_while_statement)
            }
            ZcFalseBody::ZcEmitStatement(zc_emit_statement) => {
                ZcStatement::ZcEmitStatement(zc_emit_statement)
            }
            ZcFalseBody::ZcExpressionStatement(zc_expression_statement) => {
                ZcStatement::ZcExpressionStatement(zc_expression_statement)
            }
            ZcFalseBody::ZcForStatement(zc_for_statement) => {
                ZcStatement::ZcForStatement(zc_for_statement)
            }
            ZcFalseBody::ZcIfStatement(zc_if_statement) => {
                ZcStatement::ZcIfStatement(zc_if_statement)
            }
            ZcFalseBody::ZcPlaceholderStatement(zc_placeholder_statement) => {
                ZcStatement::ZcPlaceholderStatement(zc_placeholder_statement)
            }
            ZcFalseBody::ZcReturn(zc_return) => ZcStatement::ZcReturn(zc_return),
            ZcFalseBody::ZcRevertStatement(zc_revert_statement) => {
                ZcStatement::ZcRevertStatement(zc_revert_statement)
            }
            ZcFalseBody::ZcTryStatement(zc_try_statement) => {
                ZcStatement::ZcTryStatement(zc_try_statement)
            }
            ZcFalseBody::ZcUncheckedBlock(zc_unchecked_block) => {
                ZcStatement::ZcUncheckedBlock(zc_unchecked_block)
            }
            ZcFalseBody::ZcVariableDeclarationStatement(zc_variable_declaration_statement) => {
                ZcStatement::ZcVariableDeclarationStatement(zc_variable_declaration_statement)
            }
            ZcFalseBody::ZcWhileStatement(zc_while_statement) => {
                ZcStatement::ZcWhileStatement(zc_while_statement)
            }
        }
    }
}

impl<'a> From<ZcInitializationExpression<'a>> for ZcStatement<'a> {
    fn from(value: ZcInitializationExpression<'a>) -> Self {
        match value {
            ZcInitializationExpression::ZcExpressionStatement(zc_expression_statement) => {
                ZcStatement::ZcExpressionStatement(zc_expression_statement)
            }
            ZcInitializationExpression::ZcVariableDeclarationStatement(
                zc_variable_declaration_statement,
            ) => ZcStatement::ZcVariableDeclarationStatement(zc_variable_declaration_statement),
        }
    }
}
