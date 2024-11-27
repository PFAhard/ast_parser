use crate::ast_specs::{
    ArrayTypeName, Assignment, BaseName, BinaryOperation, Conditional, ContractDefinition,
    ContractKind, Directive, ElementaryTypeName, ElementaryTypeNameExpression, EventDefinition,
    Expression, FunctionCall, FunctionCallOptions, FunctionTypeName, Identifier, IdentifierPath,
    IndexAccess, IndexRangeAccess, InheritanceSpecifier, Literal, Mapping, MemberAccess,
    NewExpression, ParameterList, SourceUnit, StateMutability, StructuredDocumentation,
    TupleExpression, TypeName, UnaryOperation, UserDefinedTypeName, VariableDeclaration,
    Visibility,
};

pub const LICENSE: &str = "// SPDX-License-Identifier: <LICENSE>";
pub const LICENSE_KEY: &str = "<LICENSE>";

pub const EVENT: &str = "<DOCUMENTATION>event <EVENT_NAME>(<EVENT_ARGS>);";
pub const EVENT_DOCUMENTATION_KEY: &str = "<DOCUMENTATION>";
pub const EVENT_NAME_KEY: &str = "<EVENT_NAME>";
pub const EVENT_ARGS_KEY: &str = "<EVENT_ARGS>";

pub const VARIABLE: &str = "<TYPE> <INDEXED> <NAME><TERMINATOR>";
pub const VARIABLE_TYPE_KEY: &str = "<TYPE>";
pub const VARIABLE_INDEXED_KEY: &str = "<INDEXED>";
pub const VARIABLE_INDEXED_KEYWORD: &str = "indexed";
pub const VARIABLE_NAME_KEY: &str = "<NAME>";
pub const VARIABLE_TERMINATOR_KEY: &str = "<TERMINATOR>";

pub const ARRAY_TYPE_NAME: &str = "<BASE_TYPE>[<SIZE>]";
pub const ARRAY_TYPE_NAME_BASE_TYPE_KEY: &str = "<BASE_TYPE>";
pub const ARRAY_TYPE_NAME_SIZE_KEY: &str = "<SIZE>";

pub const ASSIGNMENT: &str = "<LEFT_HAND_SIDE> <OPERATOR> <RIGHT_HAND_SIDE>;";
pub const ASSIGNMENT_LEFT_HAND_SIDE_KEY: &str = "<LEFT_HAND_SIDE>";
pub const ASSIGNMENT_OPERATOR_KEY: &str = "<OPERATOR>";
pub const ASSIGNMENT_RIGHT_HAND_SIDE_KEY: &str = "<RIGHT_HAND_SIDE>";

pub const BINARY_OPERATION: &str = "<LEFT_EXPRESSION> <OPERATOR> <RIGHT_EXPRESSION>;";
pub const BINARY_OPERATION_LEFT_EXPRESSION_KEY: &str = "<LEFT_EXPRESSION>";
pub const BINARY_OPERATION_OPERATOR_KEY: &str = "<OPERATOR>";
pub const BINARY_OPERATION_RIGHT_EXPRESSION_KEY: &str = "<RIGHT_EXPRESSION>";

pub const CONDITIONAL: &str = "<CONDITION> ? <TRUE_EXPRESSION> : <FALSE_EXPRESSION>";
pub const CONDITIONAL_CONDITION_KEY: &str = "<CONDITION>";
pub const CONDITIONAL_TRUE_EXPRESSION_KEY: &str = "<TRUE_EXPRESSION>";
pub const CONDITIONAL_FALSE_EXPRESSION_KEY: &str = "<FALSE_EXPRESSION>";

pub const FUNCTION_CALL: &str = "<FUNCTION_NAME>(<FUNCTION_ARGS>)";
pub const FUNCTION_CALL_NAME_KEY: &str = "<FUNCTION_NAME>";
pub const FUNCTION_CALL_ARGS_KEY: &str = "<FUNCTION_ARGS>";

pub const FUNCTION_CALL_OPTIONS: &str = "<FUNCTION_NAME>{<OPTIONS>}(<FUNCTION_ARGS>)";
pub const FUNCTION_CALL_OPTIONS_NAME_KEY: &str = "<FUNCTION_NAME>";
// pub const FUNCTION_CALL_OPTIONS_ARGS_KEY: &str = "<FUNCTION_ARGS>";
pub const FUNCTION_CALL_OPTIONS_OPTIONS_KEY: &str = "<OPTIONS>";

pub const INDEX_ACCESS: &str = "<BASE>[<INDEX>]";
pub const INDEX_ACCESS_BASE_KEY: &str = "<BASE>";
pub const INDEX_ACCESS_INDEX_KEY: &str = "<INDEX>";

pub const INDEX_RANGE_ACCESS: &str = "<BASE>[<START>:<END>]";
pub const INDEX_RANGE_ACCESS_BASE_KEY: &str = "<BASE>";
pub const INDEX_RANGE_ACCESS_START_KEY: &str = "<START>";
pub const INDEX_RANGE_ACCESS_END_KEY: &str = "<END>";

pub const MEMBER_ACCESS: &str = "<BASE>.<MEMBER>";
pub const MEMBER_ACCESS_BASE_KEY: &str = "<BASE>";
pub const MEMBER_ACCESS_MEMBER_KEY: &str = "<MEMBER>";

pub const NEW_EXPRESSION: &str = "new <EXPRESSION>";
pub const NEW_EXPRESSION_EXPRESSION_KEY: &str = "<EXPRESSION>";

pub const TUPLE_EXPRESSION: &str = "(<COMPONENTS>)";
pub const TUPLE_EXPRESSION_COMPONENTS_KEY: &str = "<COMPONENTS>";

pub const UNARY_OPERATION: &str = "<PREFIX><EXPRESSION><SUFFIX>";
pub const UNARY_OPERATION_PREFIX_KEY: &str = "<PREFIX>";
pub const UNARY_OPERATION_EXPRESSION_KEY: &str = "<EXPRESSION>";
pub const UNARY_OPERATION_SUFFIX_KEY: &str = "<SUFFIX>";

pub const FUNCTION_TYPE_NAME: &str = "function(<PARAMETERS>) <VISIBILITY> <MUTABILITY>";
pub const FUNCTION_TYPE_NAME_PARAMETERS_KEY: &str = "<PARAMETERS>";
pub const FUNCTION_TYPE_NAME_VISIBILITY_KEY: &str = "<VISIBILITY>";
pub const FUNCTION_TYPE_NAME_MUTABILITY_KEY: &str = "<MUTABILITY>";

pub const MAPPING: &str = "mapping(<TYPE_LEFT> <NAME_LEFT> => <TYPE_RIGHT> <NAME_RIGHT>)";
pub const MAPPING_TYPE_LEFT_KEY: &str = "<TYPE_LEFT>";
pub const MAPPING_NAME_LEFT_KEY: &str = "<NAME_LEFT>";
pub const MAPPING_TYPE_RIGHT_KEY: &str = "<TYPE_RIGHT>";
pub const MAPPING_NAME_RIGHT_KEY: &str = "<NAME_RIGHT>";

pub const CONTRACT: &str = "<ABSTRACT> <CONTRACT_KIND> <CONTRACT_NAME> <IS> <INHERITANCE> {<BODY>}";
pub const CONTRACT_ABSTRACT_KEY: &str = "<ABSTRACT>";
pub const CONTRACT_CONTRACT_KIND_KEY: &str = "<CONTRACT_KIND>";
pub const CONTRACT_CONTRACT_NAME_KEY: &str = "<CONTRACT_NAME>";
pub const CONTRACT_IS_KEY: &str = "<IS>";
pub const CONTRACT_INHERITANCE_KEY: &str = "<INHERITANCE>";
pub const CONTRACT_BODY_KEY: &str = "<BODY>";

pub trait AstSerializer {
    fn to_sol_vec(&self) -> Vec<u8>;

    fn to_sol_string(&self) -> String {
        to_string(self.to_sol_vec())
    }
}

impl AstSerializer for SourceUnit {
    fn to_sol_vec(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let license = LICENSE.replace(
            LICENSE_KEY,
            self.license().as_deref().unwrap_or("UNLICENSED"),
        );

        out.extend(self.nodes().to_sol_vec());
        out.extend_from_slice(license.as_bytes());
        out
    }
}

impl AstSerializer for Directive {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            Directive::EventDefinition(event_definition) => event_definition.to_sol_vec(),
            Directive::ContractDefinition(contract_definition) => contract_definition.to_sol_vec(),
            Directive::EnumDefinition(enum_definition) => enum_definition.to_sol_vec(),
            Directive::ErrorDefinition(error_definition) => error_definition.to_sol_vec(),
            Directive::FunctionDefinition(function_definition) => function_definition.to_sol_vec(),
            Directive::ImportDirective(import_directive) => import_directive.to_sol_vec(),
            Directive::PragmaDirective(pragma_directive) => pragma_directive.to_sol_vec(),
            Directive::StructDefinition(struct_definition) => struct_definition.to_sol_vec(),
            Directive::UserDefinedValueTypeDefinition(user_defined_value_type_definition) => {
                user_defined_value_type_definition.to_sol_vec()
            }
            Directive::UsingForDirective(using_for_directive) => using_for_directive.to_sol_vec(),
            Directive::VariableDeclaration(variable_declaration) => {
                variable_declaration.to_sol_vec()
            }
        }
    }
}

impl AstSerializer for EventDefinition {
    fn to_sol_vec(&self) -> Vec<u8> {
        let mut event = EVENT.replace(
            EVENT_DOCUMENTATION_KEY,
            &to_string(self.documentation().to_sol_vec()),
        );
        event = event.replace(EVENT_NAME_KEY, self.name());
        event = event.replace(EVENT_ARGS_KEY, &to_string(self.parameters().to_sol_vec()));

        event.as_bytes().to_vec()
    }
}

impl AstSerializer for StructuredDocumentation {
    fn to_sol_vec(&self) -> Vec<u8> {
        self.text().as_bytes().to_owned()
    }
}

impl AstSerializer for ParameterList {
    fn to_sol_vec(&self) -> Vec<u8> {
        self.parameters().to_sol_vec()
    }
}

impl AstSerializer for VariableDeclaration {
    fn to_sol_vec(&self) -> Vec<u8> {
        let mut var =
            VARIABLE.replace(VARIABLE_TYPE_KEY, &to_string(self.type_name().to_sol_vec()));
        var = var.replace(
            VARIABLE_INDEXED_KEY,
            match self.indexed() {
                Some(true) => VARIABLE_INDEXED_KEYWORD,
                _ => "",
            },
        );
        var = var.replace(VARIABLE_NAME_KEY, &self.name());
        var = var.replace(
            VARIABLE_TERMINATOR_KEY,
            if self.state_variable() { ";" } else { "," },
        );

        var.as_bytes().to_vec()
    }
}

impl AstSerializer for TypeName {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            TypeName::ArrayTypeName(array_type_name) => array_type_name.to_sol_vec(),
            TypeName::ElementaryTypeName(elementary_type_name) => elementary_type_name.to_sol_vec(),
            TypeName::FunctionTypeName(function_type_name) => function_type_name.to_sol_vec(),
            TypeName::Mapping(mapping) => mapping.to_sol_vec(),
            TypeName::UserDefinedTypeName(user_defined_type_name) => {
                user_defined_type_name.to_sol_vec()
            }
        }
    }
}

impl AstSerializer for ArrayTypeName {
    fn to_sol_vec(&self) -> Vec<u8> {
        let mut array = ARRAY_TYPE_NAME.replace(
            ARRAY_TYPE_NAME_BASE_TYPE_KEY,
            &to_string(self.base_type().to_sol_vec()),
        );
        if let Some(l_exp) = self.length() {
            array = array.replace(ARRAY_TYPE_NAME_SIZE_KEY, &to_string(l_exp.to_sol_vec()));
        } else {
            array = array.replace(ARRAY_TYPE_NAME_SIZE_KEY, "");
        }

        array.as_bytes().to_vec()
    }
}

impl AstSerializer for Expression {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            Expression::Assignment(assignment) => assignment.to_sol_vec(),
            Expression::BinaryOperation(binary_operation) => binary_operation.to_sol_vec(),
            Expression::Conditional(conditional) => conditional.to_sol_vec(),
            Expression::ElementaryTypeNameExpression(elementary_type_name_expression) => {
                elementary_type_name_expression.to_sol_vec()
            }
            Expression::FunctionCall(function_call) => function_call.to_sol_vec(),
            Expression::FunctionCallOptions(function_call_options) => {
                function_call_options.to_sol_vec()
            }
            Expression::Identifier(identifier) => identifier.to_sol_vec(),
            Expression::IndexAccess(index_access) => index_access.to_sol_vec(),
            Expression::IndexRangeAccess(index_range_access) => index_range_access.to_sol_vec(),
            Expression::Literal(literal) => literal.to_sol_vec(),
            Expression::MemberAccess(member_access) => member_access.to_sol_vec(),
            Expression::NewExpression(new_expression) => new_expression.to_sol_vec(),
            Expression::TupleExpression(tuple_expression) => tuple_expression.to_sol_vec(),
            Expression::UnaryOperation(unary_operation) => unary_operation.to_sol_vec(),
        }
    }
}

impl AstSerializer for Assignment {
    fn to_sol_vec(&self) -> Vec<u8> {
        let mut ass = ASSIGNMENT.replace(
            ASSIGNMENT_LEFT_HAND_SIDE_KEY,
            &self.left_hand_side().to_sol_string(),
        );
        ass = ass.replace(ASSIGNMENT_OPERATOR_KEY, self.operator());
        ass = ass.replace(
            ASSIGNMENT_RIGHT_HAND_SIDE_KEY,
            &self.right_hand_side().to_sol_string(),
        );

        ass.as_bytes().to_vec()
    }
}

impl AstSerializer for BinaryOperation {
    fn to_sol_vec(&self) -> Vec<u8> {
        let mut bin_op = BINARY_OPERATION.replace(
            BINARY_OPERATION_LEFT_EXPRESSION_KEY,
            &self.left_expression().to_sol_string(),
        );
        bin_op = bin_op.replace(BINARY_OPERATION_OPERATOR_KEY, self.operator());
        bin_op = bin_op.replace(
            BINARY_OPERATION_RIGHT_EXPRESSION_KEY,
            &self.right_expression().to_sol_string(),
        );

        bin_op.as_bytes().to_vec()
    }
}

impl AstSerializer for Conditional {
    fn to_sol_vec(&self) -> Vec<u8> {
        let mut c =
            CONDITIONAL.replace(CONDITIONAL_CONDITION_KEY, &self.condition().to_sol_string());
        c = c.replace(
            CONDITIONAL_TRUE_EXPRESSION_KEY,
            &self.true_expression().to_sol_string(),
        );
        c = c.replace(
            CONDITIONAL_FALSE_EXPRESSION_KEY,
            &self.false_expression().to_sol_string(),
        );

        c.as_bytes().to_vec()
    }
}

impl AstSerializer for ElementaryTypeNameExpression {
    fn to_sol_vec(&self) -> Vec<u8> {
        self.type_name().to_sol_vec()
    }
}

impl AstSerializer for ElementaryTypeName {
    fn to_sol_vec(&self) -> Vec<u8> {
        self.name().as_bytes().to_vec()
    }
}

impl AstSerializer for FunctionCall {
    fn to_sol_vec(&self) -> Vec<u8> {
        let mut fc =
            FUNCTION_CALL.replace(FUNCTION_CALL_NAME_KEY, &self.expression().to_sol_string());
        fc = fc.replace(FUNCTION_CALL_ARGS_KEY, &self.arguments().to_sol_string());

        fc.as_bytes().to_vec()
    }
}

impl AstSerializer for FunctionCallOptions {
    fn to_sol_vec(&self) -> Vec<u8> {
        let mut fc =
            FUNCTION_CALL.replace(FUNCTION_CALL_NAME_KEY, &self.expression().to_sol_string());

        fc = fc.replace(
            FUNCTION_CALL_OPTIONS_OPTIONS_KEY,
            &self.options().to_sol_string(),
        );

        fc.as_bytes().to_vec()
    }
}

impl AstSerializer for Identifier {
    fn to_sol_vec(&self) -> Vec<u8> {
        self.name().as_bytes().to_vec()
    }
}

impl AstSerializer for IndexAccess {
    fn to_sol_vec(&self) -> Vec<u8> {
        INDEX_ACCESS
            .replace(
                INDEX_ACCESS_BASE_KEY,
                &self.base_expression().to_sol_string(),
            )
            .replace(
                INDEX_ACCESS_INDEX_KEY,
                &self.index_expression().to_sol_string(),
            )
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for IndexRangeAccess {
    fn to_sol_vec(&self) -> Vec<u8> {
        INDEX_RANGE_ACCESS
            .replace(
                INDEX_RANGE_ACCESS_BASE_KEY,
                &self.base_expression().to_sol_string(),
            )
            .replace(
                INDEX_RANGE_ACCESS_START_KEY,
                &self.start_expression().to_sol_string(),
            )
            .replace(
                INDEX_RANGE_ACCESS_END_KEY,
                &self.end_expression().to_sol_string(),
            )
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for Literal {
    fn to_sol_vec(&self) -> Vec<u8> {
        self.value()
            .expect("Expect Value in literal AST node, but....")
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for MemberAccess {
    fn to_sol_vec(&self) -> Vec<u8> {
        MEMBER_ACCESS
            .replace(MEMBER_ACCESS_BASE_KEY, &self.expression().to_sol_string())
            .replace(MEMBER_ACCESS_MEMBER_KEY, &self.expression().to_sol_string())
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for NewExpression {
    fn to_sol_vec(&self) -> Vec<u8> {
        NEW_EXPRESSION
            .replace(
                NEW_EXPRESSION_EXPRESSION_KEY,
                &self.type_name().to_sol_string(),
            )
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for TupleExpression {
    fn to_sol_vec(&self) -> Vec<u8> {
        TUPLE_EXPRESSION
            .replace(
                TUPLE_EXPRESSION_COMPONENTS_KEY,
                &self.components().to_sol_string(),
            )
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for UnaryOperation {
    fn to_sol_vec(&self) -> Vec<u8> {
        if self.prefix() {
            UNARY_OPERATION
                .replace(UNARY_OPERATION_PREFIX_KEY, self.operator())
                .replace(
                    UNARY_OPERATION_EXPRESSION_KEY,
                    &self.sub_expression().to_sol_string(),
                )
                .replace(UNARY_OPERATION_SUFFIX_KEY, "")
        } else {
            UNARY_OPERATION
                .replace(UNARY_OPERATION_PREFIX_KEY, "")
                .replace(
                    UNARY_OPERATION_EXPRESSION_KEY,
                    &self.sub_expression().to_sol_string(),
                )
                .replace(UNARY_OPERATION_SUFFIX_KEY, self.operator())
        }
        .as_bytes()
        .to_vec()
    }
}

impl AstSerializer for FunctionTypeName {
    fn to_sol_vec(&self) -> Vec<u8> {
        FUNCTION_TYPE_NAME
            .replace(
                FUNCTION_TYPE_NAME_PARAMETERS_KEY,
                &self.parameter_types().to_sol_string(),
            )
            .replace(
                FUNCTION_TYPE_NAME_VISIBILITY_KEY,
                &self.visibility().to_sol_string(),
            )
            .replace(
                FUNCTION_TYPE_NAME_MUTABILITY_KEY,
                &self.state_mutability().to_sol_string(),
            )
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for Visibility {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            Visibility::External => b"external".to_vec(),
            Visibility::Public => b"public".to_vec(),
            Visibility::Internal => b"internal".to_vec(),
            Visibility::Private => b"private".to_vec(),
        }
    }
}

impl AstSerializer for StateMutability {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            StateMutability::Payable => b"payable".to_vec(),
            StateMutability::Pure => b"pure".to_vec(),
            StateMutability::Nonpayable => b"nonpayable".to_vec(),
            StateMutability::View => b"view".to_vec(),
        }
    }
}

impl AstSerializer for Mapping {
    fn to_sol_vec(&self) -> Vec<u8> {
        MAPPING
            .replace(MAPPING_TYPE_LEFT_KEY, &self.key_type().to_sol_string())
            .replace(MAPPING_NAME_LEFT_KEY, self.key_name().unwrap_or_default())
            .replace(MAPPING_TYPE_RIGHT_KEY, &self.value_type().to_sol_string())
            .replace(
                MAPPING_NAME_RIGHT_KEY,
                self.value_name().unwrap_or_default(),
            )
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for UserDefinedTypeName {
    fn to_sol_vec(&self) -> Vec<u8> {
        self.path_node()
            .expect("Expected Path Node in UserDefinedTypeName, but...")
            .name()
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for ContractDefinition {
    fn to_sol_vec(&self) -> Vec<u8> {
        CONTRACT
            .replace(
                CONTRACT_ABSTRACT_KEY,
                if self._abstract() { "abstract" } else { "" },
            )
            .replace(
                CONTRACT_CONTRACT_KIND_KEY,
                &self.contract_kind().to_sol_string(),
            )
            .replace(CONTRACT_CONTRACT_NAME_KEY, self.name())
            .replace(
                CONTRACT_IS_KEY,
                if self.base_contracts().is_empty() {
                    ""
                } else {
                    "is"
                },
            )
            .replace(
                CONTRACT_INHERITANCE_KEY,
                if self.base_contracts().is_empty() {
                    ""
                } else {
                    &self.base_contracts().to_sol_string()
                },
            )
            .replace(CONTRACT_BODY_KEY, &self.nodes().to_sol_string())
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for ContractKind {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            ContractKind::Contract => b"contract".to_vec(),
            ContractKind::Interface => b"interface".to_vec(),
            ContractKind::Library => b"kibrary".to_vec(),
        }
    }
}

impl AstSerializer for InheritanceSpecifier {
    fn to_sol_vec(&self) -> Vec<u8> {
        self.base_name().to_sol_vec()
    }
}

impl AstSerializer for BaseName {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            BaseName::UserDefinedTypeName(user_defined_type_name) => {
                user_defined_type_name.to_sol_vec()
            }
            BaseName::IdentifierPath(identifier_path) => identifier_path.to_sol_vec(),
        }
    }
}

impl AstSerializer for IdentifierPath {
    fn to_sol_vec(&self) -> Vec<u8> {
        self.name().as_bytes().to_vec()
    }
}

//
// COMMON
//
impl<T: AstSerializer> AstSerializer for Vec<T> {
    fn to_sol_vec(&self) -> Vec<u8> {
        self.iter().fold(Vec::new(), |mut acc, t| {
            acc.extend(t.to_sol_vec());
            acc
        })
    }
}

impl<T: AstSerializer> AstSerializer for &[T] {
    fn to_sol_vec(&self) -> Vec<u8> {
        self.iter().fold(Vec::new(), |mut acc, t| {
            acc.extend(t.to_sol_vec());
            acc
        })
    }
}

impl<T: AstSerializer> AstSerializer for Option<T> {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            Some(t) => t.to_sol_vec(),
            None => vec![],
        }
    }
}

impl<T: AstSerializer> AstSerializer for &T {
    fn to_sol_vec(&self) -> Vec<u8> {
        AstSerializer::to_sol_vec(*self)
    }
}

fn to_string(v: Vec<u8>) -> String {
    String::from_utf8(v).unwrap()
}

#[test]
fn test_counter() {
    pub const AST: &str = "debug\\out\\Counter.sol\\Counter.json";
}
