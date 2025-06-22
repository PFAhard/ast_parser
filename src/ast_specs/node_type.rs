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
                YulLiteral,
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
            YulStatement,
        },
        yul_typed_name::YulTypedName,
        ExternalReference,
    },
    statements::{
        Break, Continue, DoWhileStatement, EmitStatement, ExpressionStatement, ForStatement,
        IfStatement, PlaceholderStatement, Return, RevertStatement, TryCatchClause, TryStatement,
        UncheckedBlock, VariableDeclarationStatement, WhileStatement,
    },
    BaseNode, Body, Directive, Expression, FalseBody, InitializationExpression, LibraryName,
    ModifierName, Overrides, SourceUnit, Statement, SymbolAliases, TypeName,
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
            $variant:ident $(#[no_src: $no_src:literal])? $(#[no_id: $no_id:literal])? $(#[has_refs: $has_refs:literal])?
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

        impl<'a> NodeTypeInternalRef<'a> {
            paste::paste! {
                $(
                    pub fn [< cast_ $variant:snake >](self) -> Option<&'a $variant> {
                        match self {
                            NodeTypeInternalRef::$variant(val) => Some(val),
                            _ => None
                        }
                    }
                )*
            }
        }

        impl NodeTypeInternalRef<'_> {
            paste::paste! {
                $(
                    pub fn [< is_ $variant:snake >](self) -> bool {
                        check_node_type!(self, NodeTypeInternalRef::$variant)
                    }
                )*

                pub fn id(&self) -> isize {
                    match self {
                        $(
                            $(#[cfg(not($no_id))])?
                            NodeTypeInternalRef::$variant(val) => val.id(),
                        )*
                        _ => unreachable!("has not id field"),
                    }
                }

                #[allow(unused_doc_comments)]
                pub fn reference_id(&self) -> Option<isize> {
                    match self {
                        $(
                            $(
                                NodeTypeInternalRef::$variant(val) =>  {$has_refs;val.ref_dec_visitor()}
                            )?
                        )*
                        _ => None,
                    }
                }
            }
        }

        $(
            impl From<$variant> for NodeTypeInternal {
                fn from(value: $variant) -> Self {
                    Self::$variant(value)
                }
            }

            impl<'a> From<&'a $variant> for NodeTypeInternalRef<'a> {
                fn from(value: &'a $variant) -> Self {
                    Self::$variant(value)
                }
            }
        )*

        impl<'a> From<&'a NodeTypeInternal> for NodeTypeInternalRef<'a> {
            fn from(value: &'a NodeTypeInternal) -> Self {
                match value {
                    $(
                        NodeTypeInternal::$variant(ref_val) => NodeTypeInternalRef::$variant(ref_val),
                    )*
                }
            }
        }
    };
}

global_nodes_logic! {
    ArrayTypeName,
    Assignment,
    BinaryOperation,
    Block,
    Break,
    Conditional,
    Continue #[no_src: true],
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
    ExternalReference #[no_src: true] #[no_id: true],
    ForStatement,
    FunctionCall,
    FunctionCallOptions,
    FunctionDefinition,
    FunctionTypeName,
    Identifier #[has_refs: true],
    IdentifierPath #[has_refs: true],
    IfStatement,
    ImportDirective,
    IndexAccess,
    IndexRangeAccess,
    InheritanceSpecifier,
    InlineAssembly,
    Literal,
    Mapping,
    MemberAccess #[has_refs: true],
    ModifierDefinition,
    ModifierInvocation,
    NewExpression,
    OverrideSpecifier,
    ParameterList,
    PlaceholderStatement,
    PragmaDirective,
    Return #[no_src: true],
    RevertStatement,
    SourceUnit,
    StructDefinition,
    StructuredDocumentation,
    SymbolAliases #[no_src: true] #[no_id: true],
    TryCatchClause,
    TryStatement,
    TupleExpression,
    TypeDescriptions #[no_src: true] #[no_id: true],
    UnaryOperation,
    UncheckedBlock,
    UserDefinedTypeName #[has_refs: true],
    UserDefinedValueTypeDefinition,
    UsingForDirective,
    VariableDeclaration,
    VariableDeclarationStatement,
    WhileStatement,
    YulAssignment #[no_src: true] #[no_id: true],
    YulBlock #[no_src: true] #[no_id: true],
    YulBreak #[no_src: true] #[no_id: true],
    YulCase #[no_src: true] #[no_id: true],
    YulContinue #[no_src: true] #[no_id: true],
    YulExpressionStatement #[no_src: true] #[no_id: true],
    YulForLoop #[no_src: true] #[no_id: true],
    YulFunctionCall #[no_src: true] #[no_id: true],
    YulFunctionDefinition #[no_src: true] #[no_id: true],
    YulIdentifier #[no_src: true] #[no_id: true],
    YulIf #[no_src: true] #[no_id: true],
    YulLeave #[no_src: true] #[no_id: true],
    YulLiteralHexValue #[no_src: true] #[no_id: true],
    YulLiteralValue #[no_src: true] #[no_id: true],
    YulSwitch #[no_src: true] #[no_id: true],
    YulTypedName #[no_src: true] #[no_id: true],
    YulVariableDeclaration #[no_src: true] #[no_id: true]
}

macro_rules! enums_into_node_internal {
    (
        $(
            $e_name:ident: (
                $(
                    $variant:ident
                ),*
            );
        )*
    ) => {
        $(
            impl<'a> From<&'a $e_name> for NodeTypeInternalRef<'a> {
                fn from(value: &'a $e_name) -> Self {
                    match value {
                        $(
                            $e_name::$variant(ref_val) => NodeTypeInternalRef::$variant(ref_val),
                        )*
                    }
                }
            }
        )*

        $(
            impl From<$e_name> for NodeTypeInternal {
                fn from(value: $e_name) -> Self {
                    match value {
                        $(
                            $e_name::$variant(val) => NodeTypeInternal::$variant(val),
                        )*
                    }
                }
            }
        )*
    };
}

enums_into_node_internal! {
    Directive: (
        EventDefinition, ContractDefinition, EnumDefinition, ErrorDefinition, FunctionDefinition, ImportDirective, PragmaDirective, StructDefinition, UserDefinedValueTypeDefinition, UsingForDirective, VariableDeclaration
    );
    BaseNode: (
        EnumDefinition, ErrorDefinition, FunctionDefinition, StructDefinition, UserDefinedValueTypeDefinition, UsingForDirective, VariableDeclaration, EventDefinition, ModifierDefinition
    );
    Statement: (
        Block, Break, Continue, DoWhileStatement, EmitStatement, ExpressionStatement, ForStatement, IfStatement, PlaceholderStatement, Return, RevertStatement, TryStatement, UncheckedBlock, VariableDeclarationStatement, WhileStatement, InlineAssembly
    );
    ModifierName: (
        Identifier, IdentifierPath
    );
    Overrides: (
        UserDefinedTypeName, IdentifierPath
    );
    TypeName: (
        ArrayTypeName, ElementaryTypeName, FunctionTypeName, Mapping, UserDefinedTypeName
    );
    LibraryName: (
        UserDefinedTypeName, IdentifierPath
    );
    Expression: (
        Assignment, BinaryOperation, Conditional, ElementaryTypeNameExpression, FunctionCall, FunctionCallOptions, Identifier, IndexAccess, IndexRangeAccess, Literal, MemberAccess, NewExpression, TupleExpression, UnaryOperation
    );
    Body: (
        Block, Break, Continue, DoWhileStatement, EmitStatement, ExpressionStatement, ForStatement, IfStatement, InlineAssembly, PlaceholderStatement, Return, RevertStatement, TryStatement, UncheckedBlock, VariableDeclarationStatement, WhileStatement
    );
    InitializationExpression: (
        ExpressionStatement, VariableDeclarationStatement
    );
    FalseBody: (
        Block, Break, Continue, DoWhileStatement, EmitStatement, ExpressionStatement, ForStatement, IfStatement, PlaceholderStatement, Return, RevertStatement, TryStatement, UncheckedBlock, VariableDeclarationStatement, WhileStatement
    );
    YulStatement: (
        YulAssignment, YulBlock, YulBreak, YulContinue, YulExpressionStatement, YulLeave, YulForLoop, YulFunctionDefinition, YulIf, YulSwitch, YulVariableDeclaration
    );
    YulLiteral: (
        YulLiteralValue, YulLiteralHexValue
    );
}

pub type NTI = NodeTypeInternal;
pub type NTIref<'a> = NodeTypeInternalRef<'a>;

pub trait IntoNTI {
    fn into_nti(self) -> NodeTypeInternal;
    fn into_nti_ref<'a>(&'a self) -> NodeTypeInternalRef<'a>;
}

impl<T> IntoNTI for T
where
    T: Into<NodeTypeInternal>,
    for<'a> &'a T: Into<NodeTypeInternalRef<'a>>,
{
    fn into_nti(self) -> NodeTypeInternal {
        self.into()
    }

    fn into_nti_ref<'a>(&'a self) -> NodeTypeInternalRef<'a> {
        self.into()
    }
}
