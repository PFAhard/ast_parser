use crate::zero_cost::types::{
    abstraction::{ZcExpression, ZcStatement},
    wrappers::ZcType,
};

impl ZcStatement<'_> {
    pub fn id(&self) -> isize {
        let id = match self {
            Self::ZcBlock(zc_block) => zc_block.id(),
            Self::ZcBreak(zc_break) => zc_break.id(),
            Self::ZcContinue(zc_continue) => zc_continue.id(),
            Self::ZcDoWhileStatement(zc_do_while_statement) => zc_do_while_statement.id(),
            Self::ZcEmitStatement(zc_emit_statement) => zc_emit_statement.id(),
            Self::ZcExpressionStatement(zc_expression_statement) => zc_expression_statement.id(),
            Self::ZcForStatement(zc_for_statement) => zc_for_statement.id(),
            Self::ZcIfStatement(zc_if_statement) => zc_if_statement.id(),
            Self::ZcInlineAssembly(zc_inline_assembly) => zc_inline_assembly.id(),
            Self::ZcPlaceholderStatement(zc_placeholder_statement) => zc_placeholder_statement.id(),
            Self::ZcReturn(zc_return) => zc_return.id(),
            Self::ZcRevertStatement(zc_revert_statement) => zc_revert_statement.id(),
            Self::ZcTryStatement(zc_try_statement) => zc_try_statement.id(),
            Self::ZcUncheckedBlock(zc_unchecked_block) => zc_unchecked_block.id(),
            Self::ZcVariableDeclarationStatement(zc_variable_declaration_statement) => {
                zc_variable_declaration_statement.id()
            }
            Self::ZcWhileStatement(zc_while_statement) => zc_while_statement.id(),
        };
        id.std_type()
    }

    pub fn node_type(&self) -> &'static str {
        let nt = match self {
            Self::ZcBlock(zc_block) => "Block",
            Self::ZcBreak(zc_break) => "Break",
            Self::ZcContinue(zc_continue) => "Continue",
            Self::ZcDoWhileStatement(zc_do_while_statement) => "DoWhileStatement",
            Self::ZcEmitStatement(zc_emit_statement) => "EmitStatement",
            Self::ZcExpressionStatement(zc_expression_statement) => "ExpressionStatement",
            Self::ZcForStatement(zc_for_statement) => "ForStatement",
            Self::ZcIfStatement(zc_if_statement) => "IfStatement",
            Self::ZcInlineAssembly(zc_inline_assembly) => "InlineAssembly",
            Self::ZcPlaceholderStatement(zc_placeholder_statement) => "PlaceholderStatement",
            Self::ZcReturn(zc_return) => "Return",
            Self::ZcRevertStatement(zc_revert_statement) => "RevertStatement",
            Self::ZcTryStatement(zc_try_statement) => "TryStatement",
            Self::ZcUncheckedBlock(zc_unchecked_block) => "UncheckedBlock",
            Self::ZcVariableDeclarationStatement(zc_variable_declaration_statement) => {
                "VariableDeclarationStatement"
            }
            Self::ZcWhileStatement(zc_while_statement) => "WhileStatement",
        };
        nt
    }
}

impl ZcExpression<'_> {
    pub fn id(&self) -> isize {
        let id = match self {
            Self::ZcAssignment(zc_assignment) => zc_assignment.id(),
            Self::ZcBinaryOperation(zc_binary_operation) => zc_binary_operation.id(),
            Self::ZcConditional(zc_conditional) => zc_conditional.id(),
            Self::ZcElementaryTypeNameExpression(zc_elementary_type_name_expression) => {
                zc_elementary_type_name_expression.id()
            }
            Self::ZcFunctionCall(zc_function_call) => zc_function_call.id(),
            Self::ZcFunctionCallOptions(zc_function_call_options) => zc_function_call_options.id(),
            Self::ZcIdentifier(zc_identifier) => zc_identifier.id(),
            Self::ZcIndexAccess(zc_index_access) => zc_index_access.id(),
            Self::ZcIndexRangeAccess(zc_index_range_access) => zc_index_range_access.id(),
            Self::ZcLiteral(zc_literal) => zc_literal.id(),
            Self::ZcMemberAccess(zc_member_access) => zc_member_access.id(),
            Self::ZcNewExpression(zc_new_expression) => zc_new_expression.id(),
            Self::ZcTupleExpression(zc_tuple_expression) => zc_tuple_expression.id(),
            Self::ZcUnaryOperation(zc_unary_operation) => zc_unary_operation.id(),
        };
        id.std_type()
    }

    pub fn node_type(&self) -> &'static str {
        let nt = match self {
            Self::ZcAssignment(zc_assignment) => "ZcAssignment",
            Self::ZcBinaryOperation(zc_binary_operation) => "ZcBinaryOperation",
            Self::ZcConditional(zc_conditional) => "ZcConditional",
            Self::ZcElementaryTypeNameExpression(zc_elementary_type_name_expression) => {
                "ZcElementaryTypeNameExpression"
            }
            Self::ZcFunctionCall(zc_function_call) => "ZcFunctionCall",
            Self::ZcFunctionCallOptions(zc_function_call_options) => "ZcFunctionCallOptions",
            Self::ZcIdentifier(zc_identifier) => "ZcIdentifier",
            Self::ZcIndexAccess(zc_index_access) => "ZcIndexAccess",
            Self::ZcIndexRangeAccess(zc_index_range_access) => "ZcIndexRangeAccess",
            Self::ZcLiteral(zc_literal) => "ZcLiteral",
            Self::ZcMemberAccess(zc_member_access) => "ZcMemberAccess",
            Self::ZcNewExpression(zc_new_expression) => "ZcNewExpression",
            Self::ZcTupleExpression(zc_tuple_expression) => "ZcTupleExpression",
            Self::ZcUnaryOperation(zc_unary_operation) => "ZcUnaryOperation",
        };
        nt
    }
}
