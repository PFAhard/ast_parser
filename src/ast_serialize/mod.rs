use crate::ast_specs::{
    inline_assembly::InlineAssembly, ArrayTypeName, Assignment, BaseName, BaseNode,
    BinaryOperation, Block, Body, Break, Conditional, Continue, ContractDefinition, ContractKind,
    Directive, DoWhileStatement, ElementaryTypeName, ElementaryTypeNameExpression, EmitStatement,
    EnumDefinition, EnumValue, ErrorDefinition, EventDefinition, Expression, ExpressionStatement,
    FalseBody, ForStatement, FunctionCall, FunctionCallOptions, FunctionDefinition, FunctionKind,
    FunctionTypeName, Identifier, IdentifierPath, IfStatement, ImportDirective, IndexAccess,
    IndexRangeAccess, InheritanceSpecifier, InitializationExpression, LibraryName, Literal,
    Mapping, MemberAccess, ModifierDefinition, ModifierInvocation, ModifierName, Mutability,
    NewExpression, OverrideSpecifier, Overrides, ParameterList, PlaceholderStatement,
    PragmaDirective, Return, RevertStatement, SourceUnit, StateMutability, Statement,
    StorageLocation, StructDefinition, StructuredDocumentation, SymbolAliases, TryCatchClause,
    TryStatement, TupleExpression, TypeName, UnaryOperation, UncheckedBlock, UserDefinedTypeName,
    UserDefinedValueTypeDefinition, UsingForDirective, VariableDeclaration,
    VariableDeclarationStatement, Visibility, WhileStatement,
};

pub const LICENSE: &str = "// SPDX-License-Identifier: <LICENSE>\n";
pub const LICENSE_KEY: &str = "<LICENSE>";

pub const EVENT: &str = "<DOCUMENTATION>event <EVENT_NAME>(<EVENT_ARGS>);";
pub const EVENT_DOCUMENTATION_KEY: &str = "<DOCUMENTATION>";
pub const EVENT_NAME_KEY: &str = "<EVENT_NAME>";
pub const EVENT_ARGS_KEY: &str = "<EVENT_ARGS>";

pub const VARIABLE: &str =
    "<TYPE><INDEXED><VISIBILITY><MUTABILITY><STORAGE_LOCATION><NAME><ASSIGNMENT><INITIAL_VALUE><TERMINATOR>";
pub const VARIABLE_TYPE_KEY: &str = "<TYPE>";
pub const VARIABLE_INDEXED_KEY: &str = "<INDEXED>";
pub const VARIABLE_INDEXED_KEYWORD: &str = "indexed";
pub const VARIABLE_VISIBILITY_KEY: &str = "<VISIBILITY>";
pub const VARIABLE_MUTABILITY_KEY: &str = "<MUTABILITY>";
pub const VARIABLE_STORAGE_LOCATION_KEY: &str = "<STORAGE_LOCATION>";
pub const VARIABLE_NAME_KEY: &str = "<NAME>";
pub const VARIABLE_ASSIGNMENT_KEY: &str = "<ASSIGNMENT>";
pub const VARIABLE_INITIAL_VALUE_KEY: &str = "<INITIAL_VALUE>";
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

pub const CONTRACT: &str =
    "<DOCUMENTATION><ABSTRACT> <CONTRACT_KIND> <CONTRACT_NAME> <IS> <INHERITANCE> {<BODY>}";
pub const CONTRACT_DOCUMENTATION_KEY: &str = "<DOCUMENTATION>";
pub const CONTRACT_ABSTRACT_KEY: &str = "<ABSTRACT>";
pub const CONTRACT_CONTRACT_KIND_KEY: &str = "<CONTRACT_KIND>";
pub const CONTRACT_CONTRACT_NAME_KEY: &str = "<CONTRACT_NAME>";
pub const CONTRACT_IS_KEY: &str = "<IS>";
pub const CONTRACT_INHERITANCE_KEY: &str = "<INHERITANCE>";
pub const CONTRACT_BODY_KEY: &str = "<BODY>";

pub const ENUM: &str = "enum <ENUM_NAME> {<ENUM_VALUES>}";
pub const ENUM_NAME_KEY: &str = "<ENUM_NAME>";
pub const ENUM_ENUM_VALUES_KEY: &str = "<ENUM_VALUES>";

pub const ERROR: &str = "error <NAME>(<PARAMETERS>)";
pub const ERROR_NAME_KEY: &str = "<NAME>";
pub const ERROR_PARAMETERS_KEY: &str = "<PARAMETERS>";

pub const FUNCTION: &str = "<KIND> <NAME>(<PARAMETERS>) <STATE_MUTABILITY> <VISIBILITY> <OVERRIDE> <MODIFIERS> <RETURNS> <RETURN_PARAMETERS> {<BODY>}";
pub const FUNCTION_KIND_KEY: &str = "<KIND>";
pub const FUNCTION_NAME_KEY: &str = "<NAME>";
pub const FUNCTION_PARAMETERS_KEY: &str = "<PARAMETERS>";
pub const FUNCTION_STATE_MUTABILITY_KEY: &str = "<STATE_MUTABILITY>";
pub const FUNCTION_VISIBILITY_KEY: &str = "<VISIBILITY>";
pub const FUNCTION_OVERRIDE_KEY: &str = "<OVERRIDE>";
pub const FUNCTION_MODIFIERS_KEY: &str = "<MODIFIERS>";
pub const FUNCTION_RETURNS_KEY: &str = "<RETURNS>";
pub const FUNCTION_RETURN_PARAMETERS_KEY: &str = "<RETURN_PARAMETERS>";
pub const FUNCTION_BODY_KEY: &str = "<BODY>";

pub const MODIFIER_INVOCATION: &str = "<NAME> <ARGUMENTS>";
pub const MODIFIER_INVOCATION_NAME_KEY: &str = "<NAME>";
pub const MODIFIER_INVOCATION_ARGUMENTS_KEY: &str = "<ARGUMENTS>";

pub const DO_WHILE_STATEMENT: &str = "do {<BODY>} while (<CONDITION>)";
pub const DO_WHILE_STATEMENT_BODY_KEY: &str = "<BODY>";
pub const DO_WHILE_STATEMENT_CONDITION_KEY: &str = "<CONDITION>";

pub const EMIT_STATEMENT: &str = "emit <FUNCTION>";
pub const EMIT_EVENT_CALL_KEY: &str = "<FUNCTION>";

pub const FOR_STATEMENT: &str = "for (<INITIALIZATION>;<CONDITION>;<SUB_EXPRESSION>) {<BODY>}";
pub const FOR_STATEMENT_INITIALIZATION_KEY: &str = "<INITIALIZATION>";
pub const FOR_STATEMENT_CONDITION_KEY: &str = "<CONDITION>";
pub const FOR_STATEMENT_SUB_EXPRESSION_KEY: &str = "<SUB_EXPRESSION>";
pub const FOR_STATEMENT_BODY_KEY: &str = "<BODY>";

pub const VARIABLE_DECLARATION_STATEMENT: &str = "<DECLARATIONS> = <INITIALIZATION>";
pub const VARIABLE_DECLARATION_STATEMENT_DECLARATION_KEY: &str = "<DECLARATIONS>";
pub const VARIABLE_DECLARATION_STATEMENT_INITIALIZATION_KEY: &str = "<INITIALIZATION>";

pub const IF_STATEMENT: &str = "if <CONDITION> {<TRUE_BODY>} else {<FALSE_BODY>}";
pub const IF_STATEMENT_CONDITION_KEY: &str = "<CONDITION>";
pub const IF_STATEMENT_TRUE_BODY_KEY: &str = "<TRUE_BODY>";
pub const IF_STATEMENT_FALSE_BODY_KEY: &str = "<FALSE_BODY>";

pub const REVERT_STATEMENT: &str = "revert <FUNCTION_CALL>";
pub const REVERT_STATEMENT_FUNCTION_CALL: &str = "<FUNCTION_CALL>";

pub const TRY_CATCH_STATEMENT: &str = "try <EXPRESSION> {} catch {<CLAUSES>}";
pub const TRY_CATCH_STATEMENT_EXPRESSION_KEY: &str = "<EXPRESSION>";
pub const TRY_CATCH_STATEMENT_CLAUSES_KEY: &str = "<CLAUSES>";

pub const CATCH_CLAUSE: &str = "catch <ERROR> <PARAMS> {<BODY>}";
pub const CATCH_CLAUSE_ERROR_KEY: &str = "<ERROR>";
pub const CATCH_CLAUSE_PARAMS_KEY: &str = "<PARAMS>";
pub const CATCH_CLAUSE_BODY_KEY: &str = "<BODY>";

pub const UNCHECKED_BLOCK: &str = "unchecked {<BLOCK>}";
pub const UNCHECKED_BLOCK_BLOCK_KEY: &str = "<BLOCK>";

pub const WHILE_STATEMENT: &str = "while <CONDITION> {<BODY>}";
pub const WHILE_STATEMENT_CONDITION_KEY: &str = "<CONDITION>";
pub const WHILE_STATEMENT_BODY_KEY: &str = "<BODY>";

pub const STRUCT_STATEMENT: &str = "struct <NAME> {<MEMBERS>}";
pub const STRUCT_STATEMENT_NAME_KEY: &str = "<NAME>";
pub const STRUCT_STATEMENT_MEMBERS_KEY: &str = "<MEMBERS>";

pub const USER_DEFINED_TYPE_DEFINITION: &str = "type <NAME> is <TYPE>";
pub const USER_DEFINED_TYPE_DEFINITION_NAME_KEY: &str = "<NAME>";
pub const USER_DEFINED_TYPE_DEFINITION_TYPE_KEY: &str = "<TYPE>";

pub const USING_FOR_DIRECTIVE: &str = "using <LIBRARY> for <TYPE>;";
pub const USING_FOR_DIRECTIVE_TYPE_KEY: &str = "<TYPE>";
pub const USING_FOR_DIRECTIVE_LIBRARY_KEY: &str = "<LIBRARY>";

pub const MODIFIER: &str = "<NAME>(<PARAMETERS>) <VISIBILITY> <OVERRIDE> {<BODY>}";
pub const MODIFIER_NAME_KEY: &str = "<NAME>";
pub const MODIFIER_PARAMETERS_KEY: &str = "<PARAMETERS>";
pub const MODIFIER_VISIBILITY_KEY: &str = "<VISIBILITY>";
pub const MODIFIER_OVERRIDE_KEY: &str = "<OVERRIDE>";
pub const MODIFIER_BODY_KEY: &str = "<BODY>";

pub const IMPORT_DIRECTIVE: &str = "import {<ALIASES>} from \"<PATH>\";";
pub const IMPORT_DIRECTIVE_ALIASES_KEY: &str = "<ALIASES>";
pub const IMPORT_DIRECTIVE_PATH_KEY: &str = "<PATH>";

pub const SYMBOL_ALIASES: &str = "<FOREIGN><AS><LOCAL>";
pub const SYMBOL_ALIASES_AS_KEY: &str = "<AS>";
pub const SYMBOL_ALIASES_FOREIGN_KEY: &str = "<FOREIGN>";
pub const SYMBOL_ALIASES_LOCAL_KEY: &str = "<LOCAL>";

pub const PRAGMA: &str = "pragma <LITERALS>;\n";
pub const PRAGMA_LITERALS_KEY: &str = "<LITERALS>";

pub trait AstSerializer {
    fn to_sol_vec(&self) -> Vec<u8>;

    fn to_sol_string(&self) -> String {
        to_string(self.to_sol_vec())
    }
}

pub trait AstSerializerDelimited<'a, D: Into<&'a [u8]> + Copy> {
    fn to_sol_vec_with_delimiter(&self, d: D) -> Vec<u8>;

    fn to_sol_string_with_delimiter(&self, d: D) -> String {
        to_string(self.to_sol_vec_with_delimiter(d))
    }
}

pub trait AstSerializerContexted {
    fn to_sol_vec_contexted(&self, context: Context) -> Vec<u8>;

    fn to_sol_string_contexted(&self, context: Context) -> String {
        to_string(self.to_sol_vec_contexted(context))
    }
}

pub trait AstSerializerContextedAndDelimited<'a, D: Into<&'a [u8]> + Copy> {
    fn to_sol_vec_contexted_and_delimited(&self, context: Context, d: D) -> Vec<u8>;

    fn to_sol_string_contexted_and_delimited(&self, context: Context, d: D) -> String {
        to_string(self.to_sol_vec_contexted_and_delimited(context, d))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Context {
    ContractScope,
    ParameterList,
}

impl AstSerializer for SourceUnit {
    fn to_sol_vec(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let license = LICENSE.replace(
            LICENSE_KEY,
            self.license().as_deref().unwrap_or("UNLICENSED"),
        );

        out.extend_from_slice(license.as_bytes());
        out.extend(self.nodes().to_sol_vec_with_delimiter(Delimiter::NewLine));

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
                variable_declaration.to_sol_vec_contexted(Context::ContractScope)
            }
        }
    }
}

impl AstSerializer for EventDefinition {
    fn to_sol_vec(&self) -> Vec<u8> {
        EVENT
            .replace(
                EVENT_DOCUMENTATION_KEY,
                &to_string(self.documentation().to_sol_vec()),
            )
            .replace(EVENT_NAME_KEY, self.name())
            .replace(EVENT_ARGS_KEY, &to_string(self.parameters().to_sol_vec()))
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for StructuredDocumentation {
    fn to_sol_vec(&self) -> Vec<u8> {
        self.text()
            .lines()
            .map(|line| {
                [
                    if line.starts_with(" ") { "///" } else { "/// " },
                    line,
                    "\n",
                ]
                .concat()
            })
            .collect::<Vec<String>>()
            .join("")
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for ParameterList {
    fn to_sol_vec(&self) -> Vec<u8> {
        self.parameters()
            .to_sol_vec_contexted_and_delimited(Context::ParameterList, Delimiter::Comma)
    }
}

impl AstSerializerContexted for VariableDeclaration {
    fn to_sol_vec_contexted(&self, context: Context) -> Vec<u8> {
        match context {
            Context::ContractScope => VARIABLE
                .replace(VARIABLE_TYPE_KEY, &to_string(self.type_name().to_sol_vec()))
                .replace(
                    VARIABLE_INDEXED_KEY,
                    &match self.indexed() {
                        Some(true) => [" ", VARIABLE_INDEXED_KEYWORD].concat(),
                        _ => String::default(),
                    },
                )
                .replace(
                    VARIABLE_VISIBILITY_KEY,
                    &self.visibility().to_sol_string().pad_front(),
                )
                .replace(
                    VARIABLE_MUTABILITY_KEY,
                    &self.mutability().to_sol_string().pad_front(),
                )
                .replace(VARIABLE_STORAGE_LOCATION_KEY, "")
                .replace(VARIABLE_NAME_KEY, &self.name().pad_front())
                .replace(
                    VARIABLE_ASSIGNMENT_KEY,
                    &if self.value().is_none() {
                        String::default()
                    } else {
                        "=".pad_front()
                    },
                )
                .replace(
                    VARIABLE_INITIAL_VALUE_KEY,
                    &if self.value().is_none() {
                        String::default()
                    } else {
                        self.value().to_sol_string().pad_front()
                    },
                )
                .replace(VARIABLE_TERMINATOR_KEY, ";")
                .as_bytes()
                .to_vec(),
            Context::ParameterList => VARIABLE
                .replace(VARIABLE_TYPE_KEY, &to_string(self.type_name().to_sol_vec()))
                .replace(
                    VARIABLE_INDEXED_KEY,
                    &match self.indexed() {
                        Some(true) => VARIABLE_INDEXED_KEYWORD.pad_front(),
                        _ => String::default(),
                    },
                )
                .replace(VARIABLE_VISIBILITY_KEY, "")
                .replace(VARIABLE_MUTABILITY_KEY, "")
                .replace(
                    VARIABLE_STORAGE_LOCATION_KEY,
                    &self.storage_location().to_sol_string().pad_front(),
                )
                .replace(VARIABLE_NAME_KEY, &self.name().pad_front())
                .replace(VARIABLE_ASSIGNMENT_KEY, "")
                .replace(VARIABLE_INITIAL_VALUE_KEY, "")
                .replace(VARIABLE_TERMINATOR_KEY, "")
                .as_bytes()
                .to_vec(),
        }
    }
}

impl AstSerializer for StorageLocation {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            StorageLocation::Calldata => "calldata",
            StorageLocation::Default => "",
            StorageLocation::Memory => "memory",
            StorageLocation::Storage => "storage",
        }
        .as_bytes()
        .to_vec()
    }
}

impl AstSerializer for Mutability {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            Mutability::Mutable => b"".to_vec(),
            Mutability::Immutable => b"immutable".to_vec(),
            Mutability::Constant => b"constant".to_vec(),
        }
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
                CONTRACT_DOCUMENTATION_KEY,
                &self.documentation().to_sol_string(),
            )
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
                &if self.base_contracts().is_empty() {
                    String::default()
                } else {
                    self.base_contracts()
                        .to_sol_string_with_delimiter(Delimiter::Comma)
                },
            )
            .replace(
                CONTRACT_BODY_KEY,
                &self
                    .nodes()
                    .to_sol_string_with_delimiter(Delimiter::NewLine),
            )
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

impl AstSerializer for BaseNode {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            BaseNode::EnumDefinition(enum_definition) => enum_definition.to_sol_vec(),
            BaseNode::ErrorDefinition(error_definition) => error_definition.to_sol_vec(),
            BaseNode::FunctionDefinition(function_definition) => function_definition.to_sol_vec(),
            BaseNode::StructDefinition(struct_definition) => struct_definition.to_sol_vec(),
            BaseNode::UserDefinedValueTypeDefinition(user_defined_value_type_definition) => {
                user_defined_value_type_definition.to_sol_vec()
            }
            BaseNode::UsingForDirective(using_for_directive) => using_for_directive.to_sol_vec(),
            BaseNode::VariableDeclaration(variable_declaration) => {
                variable_declaration.to_sol_vec_contexted(Context::ContractScope)
            }
            BaseNode::EventDefinition(event_definition) => event_definition.to_sol_vec(),
            BaseNode::ModifierDefinition(modifier_definition) => modifier_definition.to_sol_vec(),
        }
    }
}

impl AstSerializer for EnumDefinition {
    fn to_sol_vec(&self) -> Vec<u8> {
        ENUM.replace(ENUM_NAME_KEY, self.name())
            .replace(ENUM_ENUM_VALUES_KEY, &self.members().to_sol_string())
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for EnumValue {
    fn to_sol_vec(&self) -> Vec<u8> {
        self.name().as_bytes().to_vec()
    }
}

impl AstSerializer for ErrorDefinition {
    fn to_sol_vec(&self) -> Vec<u8> {
        ERROR
            .replace(ERROR_NAME_KEY, self.name())
            .replace(ERROR_PARAMETERS_KEY, &self.parameters().to_sol_string())
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for FunctionDefinition {
    fn to_sol_vec(&self) -> Vec<u8> {
        FUNCTION
            .replace(FUNCTION_KIND_KEY, &self.kind().to_sol_string())
            .replace(FUNCTION_NAME_KEY, self.name())
            .replace(FUNCTION_PARAMETERS_KEY, &self.parameters().to_sol_string())
            .replace(
                FUNCTION_STATE_MUTABILITY_KEY,
                &self.state_mutability().to_sol_string(),
            )
            .replace(FUNCTION_VISIBILITY_KEY, &self.visibility().to_sol_string())
            .replace(FUNCTION_OVERRIDE_KEY, &self.overrides().to_sol_string())
            .replace(FUNCTION_MODIFIERS_KEY, &self.modifiers().to_sol_string())
            .replace(
                FUNCTION_RETURNS_KEY,
                if self.return_parameter_list().is_none()
                    || self.return_parameter_list().unwrap().is_empty()
                {
                    ""
                } else {
                    "returns"
                },
            )
            .replace(
                FUNCTION_RETURN_PARAMETERS_KEY,
                &if self.return_parameter_list().is_none()
                    || self.return_parameter_list().unwrap().is_empty()
                {
                    String::default()
                } else {
                    self.return_parameters().to_sol_string()
                },
            )
            .replace(FUNCTION_BODY_KEY, &self.body().to_sol_string())
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for OverrideSpecifier {
    fn to_sol_vec(&self) -> Vec<u8> {
        self.overrides().to_sol_vec()
    }
}

impl AstSerializer for Overrides {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            Overrides::UserDefinedTypeName(user_defined_type_name) => {
                user_defined_type_name.to_sol_vec()
            }
            Overrides::IdentifierPath(identifier_path) => identifier_path.to_sol_vec(),
        }
    }
}

impl AstSerializer for ModifierInvocation {
    fn to_sol_vec(&self) -> Vec<u8> {
        MODIFIER_INVOCATION
            .replace(
                MODIFIER_INVOCATION_NAME_KEY,
                &self.modifier_name().to_sol_string(),
            )
            .replace(
                MODIFIER_INVOCATION_ARGUMENTS_KEY,
                &self.arguments().to_sol_string(),
            )
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for ModifierName {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            ModifierName::Identifier(identifier) => identifier.to_sol_vec(),
            ModifierName::IdentifierPath(identifier_path) => identifier_path.to_sol_vec(),
        }
    }
}

impl AstSerializer for FunctionKind {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            FunctionKind::Function => b"function".to_vec(),
            FunctionKind::Receive => b"receive".to_vec(),
            FunctionKind::Constructor => b"constructor".to_vec(),
            FunctionKind::Fallback => b"fallback".to_vec(),
            FunctionKind::FreeFunction => b"freeFunction".to_vec(),
        }
    }
}

impl AstSerializer for Block {
    fn to_sol_vec(&self) -> Vec<u8> {
        self.statements().to_sol_vec()
    }
}

impl AstSerializer for Statement {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            Statement::Block(block) => block.to_sol_vec(),
            Statement::Break(_break) => _break.to_sol_vec(),
            Statement::Continue(_continue) => _continue.to_sol_vec(),
            Statement::DoWhileStatement(do_while_statement) => do_while_statement.to_sol_vec(),
            Statement::EmitStatement(emit_statement) => emit_statement.to_sol_vec(),
            Statement::ExpressionStatement(expression_statement) => {
                expression_statement.to_sol_vec()
            }
            Statement::ForStatement(for_statement) => for_statement.to_sol_vec(),
            Statement::IfStatement(if_statement) => if_statement.to_sol_vec(),
            Statement::InlineAssembly(inline_assembly) => inline_assembly.to_sol_vec(),
            Statement::PlaceholderStatement(placeholder_statement) => {
                placeholder_statement.to_sol_vec()
            }
            Statement::Return(_return) => _return.to_sol_vec(),
            Statement::RevertStatement(revert_statement) => revert_statement.to_sol_vec(),
            Statement::TryStatement(try_statement) => try_statement.to_sol_vec(),
            Statement::UncheckedBlock(unchecked_block) => unchecked_block.to_sol_vec(),
            Statement::VariableDeclarationStatement(variable_declaration_statement) => {
                variable_declaration_statement.to_sol_vec()
            }
            Statement::WhileStatement(while_statement) => while_statement.to_sol_vec(),
        }
    }
}

impl AstSerializer for Break {
    fn to_sol_vec(&self) -> Vec<u8> {
        b"break".to_vec()
    }
}

impl AstSerializer for Continue {
    fn to_sol_vec(&self) -> Vec<u8> {
        b"continue".to_vec()
    }
}

impl AstSerializer for Return {
    fn to_sol_vec(&self) -> Vec<u8> {
        b"return".to_vec()
    }
}

impl AstSerializer for DoWhileStatement {
    fn to_sol_vec(&self) -> Vec<u8> {
        DO_WHILE_STATEMENT
            .replace(DO_WHILE_STATEMENT_BODY_KEY, &self.body().to_sol_string())
            .replace(
                DO_WHILE_STATEMENT_CONDITION_KEY,
                &self.condition().to_sol_string(),
            )
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for Body {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            Body::Block(block) => block.to_sol_vec(),
            Body::Break(_break) => _break.to_sol_vec(),
            Body::Continue(_continue) => _continue.to_sol_vec(),
            Body::DoWhileStatement(do_while_statement) => do_while_statement.to_sol_vec(),
            Body::EmitStatement(emit_statement) => emit_statement.to_sol_vec(),
            Body::ExpressionStatement(expression_statement) => expression_statement.to_sol_vec(),
            Body::ForStatement(for_statement) => for_statement.to_sol_vec(),
            Body::IfStatement(if_statement) => if_statement.to_sol_vec(),
            Body::InlineAssembly(value) => value.to_sol_vec(),
            Body::PlaceholderStatement(placeholder_statement) => placeholder_statement.to_sol_vec(),
            Body::Return(_return) => _return.to_sol_vec(),
            Body::RevertStatement(revert_statement) => revert_statement.to_sol_vec(),
            Body::TryStatement(try_statement) => try_statement.to_sol_vec(),
            Body::UncheckedBlock(unchecked_block) => unchecked_block.to_sol_vec(),
            Body::VariableDeclarationStatement(variable_declaration_statement) => {
                variable_declaration_statement.to_sol_vec()
            }
            Body::WhileStatement(while_statement) => while_statement.to_sol_vec(),
        }
    }
}

impl AstSerializer for EmitStatement {
    fn to_sol_vec(&self) -> Vec<u8> {
        EMIT_STATEMENT
            .replace(EMIT_EVENT_CALL_KEY, &self.event_call().to_sol_string())
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for ExpressionStatement {
    fn to_sol_vec(&self) -> Vec<u8> {
        self.expression().to_sol_vec()
    }
}

impl AstSerializer for ForStatement {
    fn to_sol_vec(&self) -> Vec<u8> {
        FOR_STATEMENT
            .replace(
                FOR_STATEMENT_INITIALIZATION_KEY,
                &self.initialization_expression().to_sol_string(),
            )
            .replace(
                FOR_STATEMENT_CONDITION_KEY,
                &self.condition().to_sol_string(),
            )
            .replace(
                FOR_STATEMENT_SUB_EXPRESSION_KEY,
                &self.loop_expression().to_sol_string(),
            )
            .replace(FOR_STATEMENT_BODY_KEY, &self.body().to_sol_string())
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for InitializationExpression {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            InitializationExpression::ExpressionStatement(expression_statement) => {
                expression_statement.to_sol_vec()
            }
            InitializationExpression::VariableDeclarationStatement(
                variable_declaration_statement,
            ) => variable_declaration_statement.to_sol_vec(),
        }
    }
}

impl AstSerializer for VariableDeclarationStatement {
    fn to_sol_vec(&self) -> Vec<u8> {
        VARIABLE_DECLARATION_STATEMENT
            .replace(
                VARIABLE_DECLARATION_STATEMENT_DECLARATION_KEY,
                &self
                    .declarations()
                    .to_sol_string_contexted(Context::ParameterList),
            )
            .replace(
                VARIABLE_DECLARATION_STATEMENT_INITIALIZATION_KEY,
                &self.initial_value().to_sol_string(),
            )
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for IfStatement {
    fn to_sol_vec(&self) -> Vec<u8> {
        IF_STATEMENT
            .replace(
                IF_STATEMENT_CONDITION_KEY,
                &self.condition().to_sol_string(),
            )
            .replace(
                IF_STATEMENT_TRUE_BODY_KEY,
                &self.true_body().to_sol_string(),
            )
            .replace(
                IF_STATEMENT_FALSE_BODY_KEY,
                &self.false_body().to_sol_string(),
            )
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for FalseBody {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            FalseBody::Block(block) => block.to_sol_vec(),
            FalseBody::Break(_break) => _break.to_sol_vec(),
            FalseBody::Continue(_continue) => _continue.to_sol_vec(),
            FalseBody::DoWhileStatement(do_while_statement) => do_while_statement.to_sol_vec(),
            FalseBody::EmitStatement(emit_statement) => emit_statement.to_sol_vec(),
            FalseBody::ExpressionStatement(expression_statement) => {
                expression_statement.to_sol_vec()
            }
            FalseBody::ForStatement(for_statement) => for_statement.to_sol_vec(),
            FalseBody::IfStatement(if_statement) => if_statement.to_sol_vec(),
            FalseBody::PlaceholderStatement(placeholder_statement) => {
                placeholder_statement.to_sol_vec()
            }
            FalseBody::Return(_return) => _return.to_sol_vec(),
            FalseBody::RevertStatement(revert_statement) => revert_statement.to_sol_vec(),
            FalseBody::TryStatement(try_statement) => try_statement.to_sol_vec(),
            FalseBody::UncheckedBlock(unchecked_block) => unchecked_block.to_sol_vec(),
            FalseBody::VariableDeclarationStatement(variable_declaration_statement) => {
                variable_declaration_statement.to_sol_vec()
            }
            FalseBody::WhileStatement(while_statement) => while_statement.to_sol_vec(),
        }
    }
}

impl AstSerializer for PlaceholderStatement {
    fn to_sol_vec(&self) -> Vec<u8> {
        b"_".to_vec()
    }
}

impl AstSerializer for RevertStatement {
    fn to_sol_vec(&self) -> Vec<u8> {
        REVERT_STATEMENT
            .replace(
                REVERT_STATEMENT_FUNCTION_CALL,
                &self.error_call().to_sol_string(),
            )
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for TryStatement {
    fn to_sol_vec(&self) -> Vec<u8> {
        TRY_CATCH_STATEMENT
            .replace(
                TRY_CATCH_STATEMENT_CLAUSES_KEY,
                &self.clauses().to_sol_string(),
            )
            .replace(
                TRY_CATCH_STATEMENT_EXPRESSION_KEY,
                &self.external_call().to_sol_string(),
            )
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for TryCatchClause {
    fn to_sol_vec(&self) -> Vec<u8> {
        CATCH_CLAUSE
            .replace(CATCH_CLAUSE_PARAMS_KEY, &self.parameters().to_sol_string())
            .replace(CATCH_CLAUSE_ERROR_KEY, self.error_name())
            .replace(CATCH_CLAUSE_BODY_KEY, &self.block().to_sol_string())
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for UncheckedBlock {
    fn to_sol_vec(&self) -> Vec<u8> {
        UNCHECKED_BLOCK
            .replace(
                UNCHECKED_BLOCK_BLOCK_KEY,
                &self.statements().to_sol_string(),
            )
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for WhileStatement {
    fn to_sol_vec(&self) -> Vec<u8> {
        WHILE_STATEMENT
            .replace(
                WHILE_STATEMENT_CONDITION_KEY,
                &self.condition().to_sol_string(),
            )
            .replace(WHILE_STATEMENT_BODY_KEY, &self.body().to_sol_string())
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for InlineAssembly {
    fn to_sol_vec(&self) -> Vec<u8> {
        todo!()
    }
}

impl AstSerializer for StructDefinition {
    fn to_sol_vec(&self) -> Vec<u8> {
        STRUCT_STATEMENT
            .replace(STRUCT_STATEMENT_NAME_KEY, self.name())
            .replace(STRUCT_STATEMENT_MEMBERS_KEY, &{
                let mut p = self.members().to_sol_string_contexted_and_delimited(
                    Context::ParameterList,
                    Delimiter::Terminator,
                );
                p.push(';');
                p
            })
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for UserDefinedValueTypeDefinition {
    fn to_sol_vec(&self) -> Vec<u8> {
        USER_DEFINED_TYPE_DEFINITION
            .replace(USER_DEFINED_TYPE_DEFINITION_NAME_KEY, self.name())
            .replace(
                USER_DEFINED_TYPE_DEFINITION_TYPE_KEY,
                &self.underlying_type().to_sol_string(),
            )
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for UsingForDirective {
    fn to_sol_vec(&self) -> Vec<u8> {
        USING_FOR_DIRECTIVE
            .replace(
                USING_FOR_DIRECTIVE_LIBRARY_KEY,
                &self.library_name().to_sol_string(),
            )
            .replace(
                USING_FOR_DIRECTIVE_TYPE_KEY,
                &self.type_name().to_sol_string(),
            )
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for LibraryName {
    fn to_sol_vec(&self) -> Vec<u8> {
        match self {
            LibraryName::UserDefinedTypeName(user_defined_type_name) => {
                user_defined_type_name.to_sol_vec()
            }
            LibraryName::IdentifierPath(identifier_path) => identifier_path.to_sol_vec(),
        }
    }
}

impl AstSerializer for ModifierDefinition {
    fn to_sol_vec(&self) -> Vec<u8> {
        MODIFIER
            .replace(MODIFIER_NAME_KEY, self.name())
            .replace(MODIFIER_PARAMETERS_KEY, &self.parameters().to_sol_string())
            .replace(MODIFIER_OVERRIDE_KEY, &self.overrides().to_sol_string())
            .replace(MODIFIER_VISIBILITY_KEY, &self.visibility().to_sol_string())
            .replace(MODIFIER_BODY_KEY, &self.body().to_sol_string())
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for ImportDirective {
    fn to_sol_vec(&self) -> Vec<u8> {
        IMPORT_DIRECTIVE
            .replace(
                IMPORT_DIRECTIVE_ALIASES_KEY,
                &self
                    .symbol_aliases()
                    .to_sol_string_with_delimiter(Delimiter::Comma),
            )
            .replace(IMPORT_DIRECTIVE_PATH_KEY, self.file())
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for SymbolAliases {
    fn to_sol_vec(&self) -> Vec<u8> {
        SYMBOL_ALIASES
            .replace(SYMBOL_ALIASES_FOREIGN_KEY, &self.foreign().to_sol_string())
            .replace(
                SYMBOL_ALIASES_AS_KEY,
                if self.local().is_some() { " as " } else { "" },
            )
            .replace(SYMBOL_ALIASES_LOCAL_KEY, self.local().unwrap_or_default())
            .as_bytes()
            .to_vec()
    }
}

impl AstSerializer for PragmaDirective {
    fn to_sol_vec(&self) -> Vec<u8> {
        let pragma_lits = self
            .literals()
            .iter()
            .map(|lit| {
                if lit.as_str() == "solidity" {
                    [lit, " "].concat()
                } else {
                    lit.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("");
        PRAGMA
            .replace(PRAGMA_LITERALS_KEY, &pragma_lits)
            .as_bytes()
            .to_vec()
    }
}

//
// COMMON
//
impl<T: AstSerializer> AstSerializer for Vec<T> {
    fn to_sol_vec(&self) -> Vec<u8> {
        self.as_slice().to_sol_vec()
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

#[derive(Debug, Clone, Copy)]
pub enum Delimiter {
    Comma,
    NewLine,
    DoubleLine,
    Terminator,
}

impl From<Delimiter> for &[u8] {
    fn from(value: Delimiter) -> Self {
        match value {
            Delimiter::Comma => b", ",
            Delimiter::NewLine => b"\n",
            Delimiter::DoubleLine => b"\n\n",
            Delimiter::Terminator => b";",
        }
    }
}

impl<T: AstSerializer, D: for<'a> Into<&'a [u8]> + Copy> AstSerializerDelimited<'_, D> for Vec<T> {
    fn to_sol_vec_with_delimiter(&self, d: D) -> Vec<u8> {
        self.as_slice().to_sol_vec_with_delimiter(d)
    }
}
impl<T: AstSerializer, D: for<'a> Into<&'a [u8]> + Copy> AstSerializerDelimited<'_, D> for &[T] {
    fn to_sol_vec_with_delimiter(&self, d: D) -> Vec<u8> {
        let limit = self.len();
        self.iter().enumerate().fold(Vec::new(), |mut acc, (i, t)| {
            acc.extend(t.to_sol_vec());
            if i != limit - 1 {
                acc.extend_from_slice(d.into());
            }

            acc
        })
    }
}

impl<T: AstSerializerContexted> AstSerializerContexted for Vec<T> {
    fn to_sol_vec_contexted(&self, context: Context) -> Vec<u8> {
        self.as_slice().to_sol_vec_contexted(context)
    }
}

impl<T: AstSerializerContexted> AstSerializerContexted for &[T] {
    fn to_sol_vec_contexted(&self, context: Context) -> Vec<u8> {
        self.iter().fold(Vec::new(), |mut acc, t| {
            acc.extend(t.to_sol_vec_contexted(context));

            acc
        })
    }
}

impl<T: AstSerializerContexted> AstSerializerContexted for Option<T> {
    fn to_sol_vec_contexted(&self, context: Context) -> Vec<u8> {
        match self {
            Some(t) => t.to_sol_vec_contexted(context),
            None => vec![],
        }
    }
}

impl<T: AstSerializerContexted, D: for<'a> Into<&'a [u8]> + Copy>
    AstSerializerContextedAndDelimited<'_, D> for Vec<T>
{
    fn to_sol_vec_contexted_and_delimited(&self, context: Context, d: D) -> Vec<u8> {
        self.as_slice()
            .to_sol_vec_contexted_and_delimited(context, d)
    }
}
impl<T: AstSerializerContexted, D: for<'a> Into<&'a [u8]> + Copy>
    AstSerializerContextedAndDelimited<'_, D> for &[T]
{
    fn to_sol_vec_contexted_and_delimited(&self, context: Context, d: D) -> Vec<u8> {
        let limit = self.len();
        self.iter().enumerate().fold(Vec::new(), |mut acc, (i, t)| {
            acc.extend(t.to_sol_vec_contexted(context));
            if i != limit - 1 {
                acc.extend_from_slice(d.into());
            }

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

pub trait Padded {
    fn pad_front(&self) -> String;
}

impl Padded for &str {
    fn pad_front(&self) -> String {
        " {Padding}".replace("{Padding}", self)
    }
}

impl Padded for String {
    fn pad_front(&self) -> String {
        self.as_str().pad_front()
    }
}
