mod assignment;
mod binary_operation;
mod conditional;
mod elementary_type_name_expression;
mod function_call;
mod function_call_options;
mod identifier;
mod index_access;
mod index_range_access;
mod literal;
mod member_access;
mod new_expression;
mod prelude;
mod tuple_expression;
mod unary_operation;

use serde::Deserialize;

pub use prelude::*;

// @note
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub enum Expression {
    Assignment(Assignment),
    BinaryOperation(BinaryOperation),
    Conditional(Conditional),
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
    FunctionCall(FunctionCall),
    FunctionCallOptions(FunctionCallOptions),
    Identifier(Identifier),
    IndexAccess(IndexAccess),
    IndexRangeAccess(IndexRangeAccess),
    Literal(Literal),
    MemberAccess(MemberAccess),
    NewExpression(NewExpression),
    TupleExpression(TupleExpression),
    UnaryOperation(UnaryOperation),
}

impl Expression {
    pub fn extract_name(&self) -> String {
        match self {
            Expression::Identifier(identifier) => identifier.name().to_owned(),
            Expression::NewExpression(new_expression) => new_expression.name(),
            Expression::MemberAccess(member_access) => member_access.name(),
            Expression::FunctionCall(function_call) => function_call.full_name(),
            Expression::ElementaryTypeNameExpression(elementary_type_name_expression) => {
                elementary_type_name_expression.name().to_owned()
            }
            Expression::BinaryOperation(binary_operation) => binary_operation.as_name(),
            Expression::IndexAccess(index_access) => index_access.as_name(),
            Expression::Literal(literal) => literal.as_name().to_owned(),
            Expression::FunctionCallOptions(function_call_options) => {
                function_call_options.full_name()
            }
            _ => unimplemented!("{:?}", self),
        }
    }

    pub fn extract_definition(&self) -> Option<isize> {
        match self {
            Expression::Identifier(identifier) => identifier.referenced_declaration(),
            Expression::NewExpression(_) => None,
            Expression::MemberAccess(member_access) => member_access.referenced_declaration(),
            Expression::ElementaryTypeNameExpression(_) => None,
            Expression::FunctionCallOptions(fco) => fco.expression().extract_definition(),
            _ => unimplemented!("{:?}", self),
        }
    }

    pub fn src(&self) -> &str {
        match self {
            Expression::Assignment(_) => todo!(),
            Expression::BinaryOperation(_) => todo!(),
            Expression::Conditional(_) => todo!(),
            Expression::ElementaryTypeNameExpression(_) => todo!(),
            Expression::FunctionCall(_) => todo!(),
            Expression::FunctionCallOptions(_) => todo!(),
            Expression::Identifier(ident) => ident.src(),
            Expression::IndexAccess(_) => todo!(),
            Expression::IndexRangeAccess(_) => todo!(),
            Expression::Literal(_) => todo!(),
            Expression::MemberAccess(_) => todo!(),
            Expression::NewExpression(_) => todo!(),
            Expression::TupleExpression(_) => todo!(),
            Expression::UnaryOperation(_) => todo!(),
        }
    }

    pub fn is_builtin(&self) -> bool {
        match self {
            Expression::Assignment(_) => todo!(),
            Expression::BinaryOperation(_) => todo!(),
            Expression::Conditional(_) => todo!(),
            Expression::ElementaryTypeNameExpression(etne) => false,
            Expression::FunctionCall(fc) => fc.is_builtin(),
            Expression::FunctionCallOptions(fco) => {
                fco.type_descriptions()
                    .type_identifier()
                    .map(|ti| ti.starts_with("t_function_barecall_payable"))
                    == Some(true)
            }
            Expression::Identifier(ident) => ident.is_builtin(),
            Expression::IndexAccess(_) => todo!(),
            Expression::IndexRangeAccess(_) => todo!(),
            Expression::Literal(_) => todo!(),
            Expression::MemberAccess(m_acc) => m_acc.is_builtin(),
            Expression::NewExpression(_) => todo!(),
            Expression::TupleExpression(_) => todo!(),
            Expression::UnaryOperation(_) => todo!(),
        }
    }

    pub fn id(&self) -> isize {
        match self {
            Expression::Assignment(i) => i.id(),
            Expression::BinaryOperation(bin_op) => bin_op.id(),
            Expression::Conditional(i) => i.id(),
            Expression::ElementaryTypeNameExpression(i) => i.id(),
            Expression::FunctionCall(i) => i.id(),
            Expression::FunctionCallOptions(i) => i.id(),
            Expression::Identifier(i) => i.id(),
            Expression::IndexAccess(i) => i.id(),
            Expression::IndexRangeAccess(i) => i.id(),
            Expression::Literal(i) => i.id(),
            Expression::MemberAccess(ma) => ma.id(),
            Expression::NewExpression(i) => i.id(),
            Expression::TupleExpression(i) => i.id(),
            Expression::UnaryOperation(i) => i.id(),
        }
    }
}
