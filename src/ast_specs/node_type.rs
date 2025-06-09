use serde::Deserialize;

use super::{
    base_nodes::{EventDefinition, ModifierDefinition},
    common::{
        ArrayTypeName, Block, ElementaryTypeName, FunctionTypeName, IdentifierPath,
        InheritanceSpecifier, Mapping, ModifierInvocation, OverrideSpecifier, ParameterList,
        StructuredDocumentation, TypeDescriptions, UserDefinedTypeName,
    },
    directives::{
        ContractDefinition, EnumDefinition, EnumValue, ErrorDefinition, FunctionDefinition,
        ImportDirective, PragmaDirective, StructDefinition, UserDefinedValueTypeDefinition,
        UsingForDirective, VariableDeclaration,
    },
    expressions::{
        Assignment, BinaryOperation, Conditional, ElementaryTypeNameExpression, FunctionCall,
        FunctionCallOptions, Identifier, IndexAccess, IndexRangeAccess, Literal, MemberAccess,
        NewExpression, TupleExpression, UnaryOperation,
    },
    inline_assembly::{
        yul_expression::{
            yul_function_call::YulFunctionCall,
            yul_identifier::YulIdentifier,
            yul_literal::{
                yul_literal_hex_value::YulLiteralHexValue, yul_literal_value::YulLiteralValue,
            },
        },
        yul_statements::{
            yul_assignment::YulAssignment,
            yul_block::YulBlock,
            yul_break::YulBreak,
            yul_continue::YulContinue,
            yul_expression_statement::YulExpressionStatement,
            yul_for_loop::YulForLoop,
            yul_function_definition::YulFunctionDefinition,
            yul_if::YulIf,
            yul_leave::YulLeave,
            yul_switch::{YulCase, YulSwitch},
            yul_variable_declaration::YulVariableDeclaration,
        },
        yul_typed_name::YulTypedName,
        ExternalReference,
    },
    statements::{
        Break, Continue, DoWhileStatement, EmitStatement, ExpressionStatement, ForStatement,
        IfStatement, PlaceholderStatement, Return, RevertStatement, TryCatchClause, TryStatement,
        UncheckedBlock, VariableDeclarationStatement, WhileStatement,
    },
    SourceUnit, SymbolAliases,
};
use crate::{ast_specs::inline_assembly::InlineAssembly, check_node_type};

use crate::{unwrap_node_type, AstParserError};

impl From<&NodeType> for NodeType {
    fn from(value: &NodeType) -> Self {
        *value
    }
}

impl From<String> for NodeType {
    fn from(value: String) -> Self {
        NodeType::from(value.as_str())
    }
}

macro_rules! global_nodes_logic {
    (
        $(
            $variant:ident $([no_src: $no_src:literal])?
        ),*
    ) => {
        #[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
        pub enum NodeType {
            $(
                $variant,
            )*
        }

        impl From<&str> for NodeType {
            fn from(value: &str) -> Self {
                match value {
                    $(
                        stringify!($variant) => NodeType::$variant,
                    )*
                    _ => unreachable!("Should be valid node type, but: {}", value),
                }
            }
        }

        #[derive(Debug)]
        pub enum NodeTypeInternal {
            $(
                $variant($variant),
            )*
        }

        impl NodeTypeInternal {
            pub fn src(&self) -> &str {
                match self {
                    $(
                        $(#[cfg(not($no_src))])?
                        NodeTypeInternal::$variant(v) => &v.src(),
                    )*
                    _ => unreachable!("This Node type do not have an src"),
                }
            }

            paste::paste! {
                $(
                    pub fn [< cast_ $variant:snake >](self) -> $crate::AstParserResult<$variant> {
                        unwrap_node_type!(self, NodeTypeInternal::$variant)
                    }
                )*
            }
        }

        impl PartialEq for NodeTypeInternal {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    $(
                        (Self::$variant(l0), Self::$variant(r0)) => l0 == r0,
                    )*
                    _ => false,
                }
            }
        }

        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub enum NodeTypeInternalRef<'a> {
            $(
                $variant(&'a $variant),
            )*
        }

        impl NodeTypeInternalRef<'_> {
            paste::paste! {
                $(
                    pub fn [< is_ $variant:snake >](self) -> bool {
                        check_node_type!(self, NodeTypeInternalRef::$variant)
                    }

                    pub fn [< cast_ $variant:snake >](&self) -> Option<&$variant> {
                        match self {
                            NodeTypeInternalRef::$variant(val) => Some(val),
                            _ => None
                        }
                    }
                )*
            }
        }

        $(
            impl<'a> From<&'a $variant> for NodeTypeInternalRef<'a> {
                fn from(value: &'a $variant) -> Self {
                    Self::$variant(value)
                }
            }
        )*
    };
}

global_nodes_logic! {
    ArrayTypeName,
    Assignment,
    BinaryOperation,
    Block,
    Break,
    Conditional,
    Continue [no_src: true],
    ContractDefinition,
    DoWhileStatement,
    ElementaryTypeName,
    ElementaryTypeNameExpression,
    EmitStatement,
    EnumDefinition,
    EnumValue,
    ErrorDefinition,
    EventDefinition,
    ExpressionStatement,
    ForStatement,
    FunctionCall,
    FunctionCallOptions,
    FunctionDefinition,
    FunctionTypeName,
    Identifier,
    IdentifierPath,
    IfStatement,
    ImportDirective,
    IndexAccess,
    IndexRangeAccess,
    InheritanceSpecifier,
    InlineAssembly,
    Literal,
    Mapping,
    MemberAccess,
    ModifierDefinition,
    ModifierInvocation,
    NewExpression,
    OverrideSpecifier,
    ParameterList,
    PlaceholderStatement,
    PragmaDirective,
    Return [no_src: true],
    RevertStatement,
    SourceUnit,
    StructDefinition,
    StructuredDocumentation,
    TryCatchClause,
    TryStatement,
    TupleExpression,
    TypeDescriptions [no_src: true],
    UnaryOperation,
    UncheckedBlock,
    UserDefinedTypeName,
    UserDefinedValueTypeDefinition,
    UsingForDirective,
    VariableDeclaration,
    VariableDeclarationStatement,
    WhileStatement,
    SymbolAliases [no_src: true],
    ExternalReference [no_src: true],
    YulBlock [no_src: true],
    YulVariableDeclaration [no_src: true],
    YulCase [no_src: true],
    YulIf [no_src: true],
    YulFunctionDefinition [no_src: true],
    YulForLoop [no_src: true],
    YulLeave [no_src: true],
    YulExpressionStatement [no_src: true],
    YulContinue [no_src: true],
    YulBreak [no_src: true],
    YulAssignment [no_src: true],
    YulIdentifier [no_src: true],
    YulFunctionCall [no_src: true],
    YulLiteralValue [no_src: true],
    YulLiteralHexValue [no_src: true],
    YulTypedName [no_src: true],
    YulSwitch [no_src: true]
}
