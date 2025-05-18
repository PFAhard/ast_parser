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
    statements::{
        Break, Continue, DoWhileStatement, EmitStatement, ExpressionStatement, ForStatement,
        IfStatement, PlaceholderStatement, Return, RevertStatement, TryCatchClause, TryStatement,
        UncheckedBlock, VariableDeclarationStatement, WhileStatement,
    },
    SourceUnit, SymbolAliases,
};
use crate::ast_specs::inline_assembly::InlineAssembly;

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

impl NodeTypeInternal {
    pub fn import_directive(self) -> crate::AstParserResult<ImportDirective> {
        unwrap_node_type!(self, NodeTypeInternal::ImportDirective)
    }

    pub fn variable_declaration(self) -> crate::AstParserResult<VariableDeclaration> {
        unwrap_node_type!(self, NodeTypeInternal::VariableDeclaration)
    }

    pub fn identifier(self) -> crate::AstParserResult<Identifier> {
        unwrap_node_type!(self, NodeTypeInternal::Identifier)
    }

    pub fn block(self) -> crate::AstParserResult<Block> {
        unwrap_node_type!(self, NodeTypeInternal::Block)
    }

    pub fn modifier_definition(self) -> crate::AstParserResult<ModifierDefinition> {
        unwrap_node_type!(self, NodeTypeInternal::ModifierDefinition)
    }

    pub fn contract_definition(self) -> crate::AstParserResult<ContractDefinition> {
        unwrap_node_type!(self, NodeTypeInternal::ContractDefinition)
    }

    pub fn function_call(self) -> crate::AstParserResult<FunctionCall> {
        unwrap_node_type!(self, NodeTypeInternal::FunctionCall)
    }

    pub fn function_definition(self) -> crate::AstParserResult<FunctionDefinition> {
        unwrap_node_type!(self, NodeTypeInternal::FunctionDefinition)
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
    SymbolAliases [no_src: true]
}
