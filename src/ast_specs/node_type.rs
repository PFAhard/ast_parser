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
    SourceUnit,
};

use crate::AstParserError;

#[macro_export]
macro_rules! unwrap_node_type {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = $target {
            // #1
            Ok(a)
        } else {
            AstParserError::result_node_type_internal_cast()
            // panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
        }
    }};
}

#[macro_export]
macro_rules! cast_node_type {
    ($target: expr, $pat: path) => {{
        $target.filter_by_node_type($pat)
            .into_iter()
            .map(|v| unwrap_node_type!(v, $pat))
            .collect()
    }};
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    ArrayTypeName,
    Assignment,
    BinaryOperation,
    Block,
    Break,
    Conditional,
    Continue,
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
    Return,
    RevertStatement,
    SourceUnit,
    StructDefinition,
    StructuredDocumentation,
    TryCatchClause,
    TryStatement,
    TupleExpression,
    TypeDescriptions,
    UnaryOperation,
    UncheckedBlock,
    UserDefinedTypeName,
    UserDefinedValueTypeDefinition,
    UsingForDirective,
    VariableDeclaration,
    VariableDeclarationStatement,
    WhileStatement,
}

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

impl From<&str> for NodeType {
    fn from(value: &str) -> Self {
        match value {
            "ArrayTypeName" => NodeType::ArrayTypeName,
            "Assignment" => NodeType::Assignment,
            "BinaryOperation" => NodeType::BinaryOperation,
            "Block" => NodeType::Block,
            "Break" => NodeType::Break,
            "Conditional" => NodeType::Conditional,
            "Continue" => NodeType::Continue,
            "ContractDefinition" => NodeType::ContractDefinition,
            "DoWhileStatement" => NodeType::DoWhileStatement,
            "ElementaryTypeName" => NodeType::ElementaryTypeName,
            "ElementaryTypeNameExpression" => NodeType::ElementaryTypeNameExpression,
            "EmitStatement" => NodeType::EmitStatement,
            "EnumDefinition" => NodeType::EnumDefinition,
            "EnumValue" => NodeType::EnumValue,
            "ErrorDefinition" => NodeType::ErrorDefinition,
            "EventDefinition" => NodeType::EventDefinition,
            "ExpressionStatement" => NodeType::ExpressionStatement,
            "ForStatement" => NodeType::ForStatement,
            "FunctionCall" => NodeType::FunctionCall,
            "FunctionCallOptions" => NodeType::FunctionCallOptions,
            "FunctionDefinition" => NodeType::FunctionDefinition,
            "FunctionTypeName" => NodeType::FunctionTypeName,
            "Identifier" => NodeType::Identifier,
            "IdentifierPath" => NodeType::IdentifierPath,
            "IfStatement" => NodeType::IfStatement,
            "ImportDirective" => NodeType::ImportDirective,
            "IndexAccess" => NodeType::IndexAccess,
            "IndexRangeAccess" => NodeType::IndexRangeAccess,
            "InheritanceSpecifier" => NodeType::InheritanceSpecifier,
            "InlineAssembly" => NodeType::InlineAssembly,
            "Literal" => NodeType::Literal,
            "Mapping" => NodeType::Mapping,
            "MemberAccess" => NodeType::MemberAccess,
            "ModifierDefinition" => NodeType::ModifierDefinition,
            "ModifierInvocation" => NodeType::ModifierInvocation,
            "NewExpression" => NodeType::NewExpression,
            "OverrideSpecifier" => NodeType::OverrideSpecifier,
            "ParameterList" => NodeType::ParameterList,
            "PlaceholderStatement" => NodeType::PlaceholderStatement,
            "PragmaDirective" => NodeType::PragmaDirective,
            "Return" => NodeType::Return,
            "RevertStatement" => NodeType::RevertStatement,
            "SourceUnit" => NodeType::SourceUnit,
            "StructDefinition" => NodeType::StructDefinition,
            "StructuredDocumentation" => NodeType::StructuredDocumentation,
            "TryCatchClause" => NodeType::TryCatchClause,
            "TryStatement" => NodeType::TryStatement,
            "TupleExpression" => NodeType::TupleExpression,
            "TypeDescriptions" => NodeType::TypeDescriptions,
            "UnaryOperation" => NodeType::UnaryOperation,
            "UncheckedBlock" => NodeType::UncheckedBlock,
            "UserDefinedTypeName" => NodeType::UserDefinedTypeName,
            "UserDefinedValueTypeDefinition" => NodeType::UserDefinedValueTypeDefinition,
            "UsingForDirective" => NodeType::UsingForDirective,
            "VariableDeclaration" => NodeType::VariableDeclaration,
            "VariableDeclarationStatement" => NodeType::VariableDeclarationStatement,
            "WhileStatement" => NodeType::WhileStatement,
            _ => unreachable!("Should be valid node type, but: {}", value),
        }
    }
}

#[derive(Debug)]
pub enum NodeTypeInternal {
    ArrayTypeName(ArrayTypeName),
    Assignment(Assignment),
    BinaryOperation(BinaryOperation),
    Block(Block),
    Break(Break),
    Conditional(Conditional),
    Continue(Continue),
    ContractDefinition(ContractDefinition),
    DoWhileStatement(DoWhileStatement),
    ElementaryTypeName(ElementaryTypeName),
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
    EmitStatement(EmitStatement),
    EnumDefinition(EnumDefinition),
    EnumValue(EnumValue),
    ErrorDefinition(ErrorDefinition),
    EventDefinition(EventDefinition),
    ExpressionStatement(ExpressionStatement),
    ForStatement(ForStatement),
    FunctionCall(FunctionCall),
    FunctionCallOptions(FunctionCallOptions),
    FunctionDefinition(FunctionDefinition),
    FunctionTypeName(FunctionTypeName),
    Identifier(Identifier),
    IdentifierPath(IdentifierPath),
    IfStatement(IfStatement),
    ImportDirective(ImportDirective),
    IndexAccess(IndexAccess),
    IndexRangeAccess(IndexRangeAccess),
    InheritanceSpecifier(InheritanceSpecifier),
    Literal(Literal),
    Mapping(Mapping),
    MemberAccess(MemberAccess),
    ModifierDefinition(ModifierDefinition),
    ModifierInvocation(ModifierInvocation),
    NewExpression(NewExpression),
    OverrideSpecifier(OverrideSpecifier),
    ParameterList(ParameterList),
    PlaceholderStatement(PlaceholderStatement),
    PragmaDirective(PragmaDirective),
    Return(Return),
    RevertStatement(RevertStatement),
    SourceUnit(SourceUnit),
    StructDefinition(StructDefinition),
    StructuredDocumentation(StructuredDocumentation),
    TryCatchClause(TryCatchClause),
    TryStatement(TryStatement),
    TupleExpression(TupleExpression),
    TypeDescriptions(TypeDescriptions),
    UnaryOperation(UnaryOperation),
    UncheckedBlock(UncheckedBlock),
    UserDefinedTypeName(UserDefinedTypeName),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
    VariableDeclarationStatement(VariableDeclarationStatement),
    WhileStatement(WhileStatement),
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
