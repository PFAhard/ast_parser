#![warn(clippy::all)]
use crate::ast_specs::{
    inline_assembly::{
        yul_expression::{
            yul_function_call::YulFunctionCall,
            yul_identifier::YulIdentifier,
            yul_literal::{
                yul_literal_hex_value::YulLiteralHexValue, yul_literal_value::YulLiteralValue,
                YulLiteral,
            },
            YulExpression,
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
            yul_switch::{CaseValue, YulCase, YulSwitch},
            yul_variable_declaration::YulVariableDeclaration,
            YulStatement,
        },
        yul_typed_name::YulTypedName,
        ExternalReference, ExternalReferenceCompatible, InlineAssembly,
    },
    CompatabilityTypeName,
};

use super::ast_specs::{
    base_nodes::{EventDefinition, ModifierDefinition},
    common::{
        ArrayTypeName, Block, ElementaryTypeName, FunctionTypeName, IdentifierPath, LibraryName,
        Mapping, ModifierInvocation, ModifierName, OverrideSpecifier, Overrides, ParameterList,
        TypeDescriptions, TypeName, UserDefinedTypeName,
    },
    directives::{
        ContractDefinition, EnumDefinition, EnumValue, ErrorDefinition, FunctionDefinition,
        ImportDirective, PragmaDirective, StructDefinition, SymbolAliases,
        UserDefinedValueTypeDefinition, UsingForDirective, VariableDeclaration,
    },
    expressions::{
        Assignment, BinaryOperation, Conditional, ElementaryTypeNameExpression, FunctionCall,
        FunctionCallOptions, Identifier, IndexAccess, IndexRangeAccess, Literal, MemberAccess,
        NewExpression, TupleExpression, UnaryOperation,
    },
    node_type::{NodeType, NodeTypeInternal},
    statements::{
        Body, Break, Continue, DoWhileStatement, EmitStatement, ExpressionStatement, FalseBody,
        ForStatement, IfStatement, InitializationExpression, PlaceholderStatement, Return,
        RevertStatement, TryCatchClause, TryStatement, UncheckedBlock,
        VariableDeclarationStatement, WhileStatement,
    },
    BaseNode, Directive, Expression, SourceUnit, Statement,
};

pub trait AstVisitor {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal>;

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal>;

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal>;

    fn childrens_id(&self) -> Vec<isize>;

    fn references(&self) -> Vec<isize>;
}

impl AstVisitor for SourceUnit {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();
        let mut result: Vec<NodeTypeInternal> = self.nodes().filter_by_node_type(node_type);
        if node_type == NodeType::SourceUnit {
            result.push(NodeTypeInternal::SourceUnit(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.nodes().filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result: Vec<NodeTypeInternal> = self.nodes().filter_by_id(id);
        if self.id() == id {
            result.push(NodeTypeInternal::SourceUnit(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.nodes().childrens_id();
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        self.nodes().references()
    }
}

impl AstVisitor for Directive {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        match self {
            Directive::ContractDefinition(i) => i.filter_by_node_type(node_type),
            Directive::EnumDefinition(i) => i.filter_by_node_type(node_type),
            Directive::ErrorDefinition(i) => i.filter_by_node_type(node_type),
            Directive::FunctionDefinition(i) => i.filter_by_node_type(node_type),
            Directive::ImportDirective(i) => i.filter_by_node_type(node_type),
            Directive::PragmaDirective(i) => i.filter_by_node_type(node_type),
            Directive::StructDefinition(i) => i.filter_by_node_type(node_type),
            Directive::UserDefinedValueTypeDefinition(i) => i.filter_by_node_type(node_type),
            Directive::UsingForDirective(i) => i.filter_by_node_type(node_type),
            Directive::VariableDeclaration(i) => i.filter_by_node_type(node_type),
            Directive::EventDefinition(i) => i.filter_by_node_type(node_type),
        }
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            Directive::ContractDefinition(i) => i.filter_by_reference_id(id),
            Directive::EnumDefinition(i) => i.filter_by_reference_id(id),
            Directive::ErrorDefinition(i) => i.filter_by_reference_id(id),
            Directive::FunctionDefinition(i) => i.filter_by_reference_id(id),
            Directive::ImportDirective(i) => i.filter_by_reference_id(id),
            Directive::PragmaDirective(i) => i.filter_by_reference_id(id),
            Directive::StructDefinition(i) => i.filter_by_reference_id(id),
            Directive::UserDefinedValueTypeDefinition(i) => i.filter_by_reference_id(id),
            Directive::UsingForDirective(i) => i.filter_by_reference_id(id),
            Directive::VariableDeclaration(i) => i.filter_by_reference_id(id),
            Directive::EventDefinition(i) => i.filter_by_reference_id(id),
        }
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            Directive::ContractDefinition(i) => i.filter_by_id(id),
            Directive::EnumDefinition(i) => i.filter_by_id(id),
            Directive::ErrorDefinition(i) => i.filter_by_id(id),
            Directive::FunctionDefinition(i) => i.filter_by_id(id),
            Directive::ImportDirective(i) => i.filter_by_id(id),
            Directive::PragmaDirective(i) => i.filter_by_id(id),
            Directive::StructDefinition(i) => i.filter_by_id(id),
            Directive::UserDefinedValueTypeDefinition(i) => i.filter_by_id(id),
            Directive::UsingForDirective(i) => i.filter_by_id(id),
            Directive::VariableDeclaration(i) => i.filter_by_id(id),
            Directive::EventDefinition(i) => i.filter_by_id(id),
        }
    }

    fn childrens_id(&self) -> Vec<isize> {
        match self {
            Directive::ContractDefinition(i) => i.childrens_id(),
            Directive::EnumDefinition(i) => i.childrens_id(),
            Directive::ErrorDefinition(i) => i.childrens_id(),
            Directive::FunctionDefinition(i) => i.childrens_id(),
            Directive::ImportDirective(i) => i.childrens_id(),
            Directive::PragmaDirective(i) => i.childrens_id(),
            Directive::StructDefinition(i) => i.childrens_id(),
            Directive::UserDefinedValueTypeDefinition(i) => i.childrens_id(),
            Directive::UsingForDirective(i) => i.childrens_id(),
            Directive::VariableDeclaration(i) => i.childrens_id(),
            Directive::EventDefinition(i) => i.childrens_id(),
        }
    }

    fn references(&self) -> Vec<isize> {
        match self {
            Directive::EventDefinition(i) => i.references(),
            Directive::ContractDefinition(i) => i.references(),
            Directive::EnumDefinition(i) => i.references(),
            Directive::ErrorDefinition(i) => i.references(),
            Directive::FunctionDefinition(i) => i.references(),
            Directive::ImportDirective(i) => i.references(),
            Directive::PragmaDirective(i) => i.references(),
            Directive::StructDefinition(i) => i.references(),
            Directive::UserDefinedValueTypeDefinition(i) => i.references(),
            Directive::UsingForDirective(i) => i.references(),
            Directive::VariableDeclaration(i) => i.references(),
        }
    }
}

impl AstVisitor for ContractDefinition {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result: Vec<NodeTypeInternal> = self.nodes().filter_by_node_type(node_type);
        if node_type == NodeType::ContractDefinition {
            result.push(NodeTypeInternal::ContractDefinition(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.nodes().filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result: Vec<NodeTypeInternal> = self.nodes().filter_by_id(id);
        if id == self.id() {
            result.push(NodeTypeInternal::ContractDefinition(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.nodes().childrens_id();
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        self.nodes().references()
    }
}

impl AstVisitor for EnumDefinition {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result: Vec<NodeTypeInternal> = self.members().filter_by_node_type(node_type);
        if node_type == NodeType::EnumDefinition {
            result.push(NodeTypeInternal::EnumDefinition(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.members().filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result: Vec<NodeTypeInternal> = self.members().filter_by_id(id);
        if id == self.id() {
            result.push(NodeTypeInternal::EnumDefinition(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.members().childrens_id();
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        self.members().references()
    }
}

impl AstVisitor for ErrorDefinition {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result: Vec<NodeTypeInternal> = self.parameters().filter_by_node_type(node_type);
        if node_type == NodeType::ErrorDefinition {
            result.push(NodeTypeInternal::ErrorDefinition(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.parameters().filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result: Vec<NodeTypeInternal> = self.parameters().filter_by_id(id);
        if id == self.id() {
            result.push(NodeTypeInternal::ErrorDefinition(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.parameters().childrens_id();
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        self.parameters().references()
    }
}

impl AstVisitor for FunctionDefinition {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.body().filter_by_node_type(node_type);
        result.append(&mut self.modifiers().filter_by_node_type(node_type));
        result.append(&mut self.overrides().filter_by_node_type(node_type));
        result.append(&mut self.parameters().filter_by_node_type(node_type));
        result.append(&mut self.return_parameters().filter_by_node_type(node_type));
        if node_type == NodeType::FunctionDefinition {
            result.push(NodeTypeInternal::FunctionDefinition(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.body().filter_by_reference_id(id);
        result.append(&mut self.modifiers().filter_by_reference_id(id));
        result.append(&mut self.overrides().filter_by_reference_id(id));
        result.append(&mut self.parameters().filter_by_reference_id(id));
        result.append(&mut self.return_parameters().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.body().filter_by_id(id);
        result.append(&mut self.modifiers().filter_by_id(id));
        result.append(&mut self.overrides().filter_by_id(id));
        result.append(&mut self.parameters().filter_by_id(id));
        result.append(&mut self.return_parameters().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::FunctionDefinition(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.body().childrens_id();
        result.append(&mut self.modifiers().childrens_id());
        result.append(&mut self.overrides().childrens_id());
        result.append(&mut self.parameters().childrens_id());
        result.append(&mut self.return_parameters().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.body().references();
        result.append(&mut self.modifiers().references());
        result.append(&mut self.overrides().references());
        result.append(&mut self.parameters().references());
        result.append(&mut self.return_parameters().references());
        result
    }
}

impl AstVisitor for ImportDirective {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.symbol_aliases().filter_by_node_type(node_type);
        if node_type == NodeType::ImportDirective {
            result.push(NodeTypeInternal::ImportDirective(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.symbol_aliases().filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.symbol_aliases().filter_by_id(id);
        if id == *self.id() {
            result.push(NodeTypeInternal::ImportDirective(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.symbol_aliases().childrens_id();
        result.push(*self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        self.symbol_aliases().references()
    }
}

impl AstVisitor for SymbolAliases {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.foreign().filter_by_node_type(node_type);
        if node_type == NodeType::SymbolAliases {
            result.push(NodeTypeInternal::SymbolAliases(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.foreign().filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.foreign().filter_by_id(id)
    }

    fn childrens_id(&self) -> Vec<isize> {
        self.foreign().childrens_id()
    }

    fn references(&self) -> Vec<isize> {
        self.foreign().references()
    }
}

impl AstVisitor for PragmaDirective {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = Vec::new();
        if node_type == NodeType::PragmaDirective {
            result.push(NodeTypeInternal::PragmaDirective(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, _id: isize) -> Vec<NodeTypeInternal> {
        vec![]
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = vec![];
        if id == self.id() {
            result.push(NodeTypeInternal::PragmaDirective(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        vec![self.id()]
    }

    fn references(&self) -> Vec<isize> {
        vec![]
    }
}

impl AstVisitor for StructDefinition {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.members().filter_by_node_type(node_type);
        if node_type == NodeType::StructDefinition {
            result.push(NodeTypeInternal::StructDefinition(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.members().filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.members().filter_by_id(id);
        if id == self.id() {
            result.push(NodeTypeInternal::StructDefinition(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.members().childrens_id();
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        self.members().references()
    }
}

impl AstVisitor for UserDefinedValueTypeDefinition {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.underlying_type().filter_by_node_type(node_type);
        if node_type == NodeType::UserDefinedValueTypeDefinition {
            result.push(NodeTypeInternal::UserDefinedValueTypeDefinition(
                self.clone(),
            ));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.underlying_type().filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.underlying_type().filter_by_id(id);
        if id == self.id() {
            result.push(NodeTypeInternal::UserDefinedValueTypeDefinition(
                self.clone(),
            ));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.underlying_type().childrens_id();
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        self.underlying_type().references()
    }
}

impl AstVisitor for UsingForDirective {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.function_list().filter_by_node_type(node_type);
        result.append(&mut self.library_name().filter_by_node_type(node_type));
        result.append(&mut self.type_name().filter_by_node_type(node_type));
        if node_type == NodeType::UsingForDirective {
            result.push(NodeTypeInternal::UsingForDirective(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.function_list().filter_by_reference_id(id);
        result.append(&mut self.library_name().filter_by_reference_id(id));
        result.append(&mut self.type_name().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.function_list().filter_by_id(id);
        result.append(&mut self.library_name().filter_by_id(id));
        result.append(&mut self.type_name().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::UsingForDirective(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.function_list().childrens_id();
        result.append(&mut self.library_name().childrens_id());
        result.append(&mut self.type_name().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.function_list().references();
        result.append(&mut self.library_name().references());
        result.append(&mut self.type_name().references());
        result
    }
}

impl AstVisitor for VariableDeclaration {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.value().filter_by_node_type(node_type);
        result.append(&mut self.overrides().filter_by_node_type(node_type));
        result.append(&mut self.type_name().filter_by_node_type(node_type));
        if node_type == NodeType::VariableDeclaration {
            result.push(NodeTypeInternal::VariableDeclaration(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.value().filter_by_reference_id(id);
        result.append(&mut self.overrides().filter_by_reference_id(id));
        result.append(&mut self.type_name().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.value().filter_by_id(id);
        result.append(&mut self.overrides().filter_by_id(id));
        result.append(&mut self.type_name().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::VariableDeclaration(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.value().childrens_id();
        result.append(&mut self.overrides().childrens_id());
        result.append(&mut self.type_name().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.value().references();
        result.append(&mut self.overrides().references());
        result.append(&mut self.type_name().references());
        result
    }
}

impl AstVisitor for BaseNode {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        match self {
            BaseNode::EnumDefinition(i) => i.filter_by_node_type(node_type),
            BaseNode::ErrorDefinition(i) => i.filter_by_node_type(node_type),
            BaseNode::FunctionDefinition(i) => i.filter_by_node_type(node_type),
            BaseNode::StructDefinition(i) => i.filter_by_node_type(node_type),
            BaseNode::UserDefinedValueTypeDefinition(i) => i.filter_by_node_type(node_type),
            BaseNode::UsingForDirective(i) => i.filter_by_node_type(node_type),
            BaseNode::VariableDeclaration(i) => i.filter_by_node_type(node_type),
            BaseNode::EventDefinition(i) => i.filter_by_node_type(node_type),
            BaseNode::ModifierDefinition(i) => i.filter_by_node_type(node_type),
        }
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            BaseNode::EnumDefinition(i) => i.filter_by_reference_id(id),
            BaseNode::ErrorDefinition(i) => i.filter_by_reference_id(id),
            BaseNode::FunctionDefinition(i) => i.filter_by_reference_id(id),
            BaseNode::StructDefinition(i) => i.filter_by_reference_id(id),
            BaseNode::UserDefinedValueTypeDefinition(i) => i.filter_by_reference_id(id),
            BaseNode::UsingForDirective(i) => i.filter_by_reference_id(id),
            BaseNode::VariableDeclaration(i) => i.filter_by_reference_id(id),
            BaseNode::EventDefinition(i) => i.filter_by_reference_id(id),
            BaseNode::ModifierDefinition(i) => i.filter_by_reference_id(id),
        }
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            BaseNode::EnumDefinition(i) => i.filter_by_id(id),
            BaseNode::ErrorDefinition(i) => i.filter_by_id(id),
            BaseNode::FunctionDefinition(i) => i.filter_by_id(id),
            BaseNode::StructDefinition(i) => i.filter_by_id(id),
            BaseNode::UserDefinedValueTypeDefinition(i) => i.filter_by_id(id),
            BaseNode::UsingForDirective(i) => i.filter_by_id(id),
            BaseNode::VariableDeclaration(i) => i.filter_by_id(id),
            BaseNode::EventDefinition(i) => i.filter_by_id(id),
            BaseNode::ModifierDefinition(i) => i.filter_by_id(id),
        }
    }

    fn childrens_id(&self) -> Vec<isize> {
        match self {
            BaseNode::EnumDefinition(i) => i.childrens_id(),
            BaseNode::ErrorDefinition(i) => i.childrens_id(),
            BaseNode::FunctionDefinition(i) => i.childrens_id(),
            BaseNode::StructDefinition(i) => i.childrens_id(),
            BaseNode::UserDefinedValueTypeDefinition(i) => i.childrens_id(),
            BaseNode::UsingForDirective(i) => i.childrens_id(),
            BaseNode::VariableDeclaration(i) => i.childrens_id(),
            BaseNode::EventDefinition(i) => i.childrens_id(),
            BaseNode::ModifierDefinition(i) => i.childrens_id(),
        }
    }

    fn references(&self) -> Vec<isize> {
        match self {
            BaseNode::EnumDefinition(i) => i.references(),
            BaseNode::ErrorDefinition(i) => i.references(),
            BaseNode::FunctionDefinition(i) => i.references(),
            BaseNode::StructDefinition(i) => i.references(),
            BaseNode::UserDefinedValueTypeDefinition(i) => i.references(),
            BaseNode::UsingForDirective(i) => i.references(),
            BaseNode::VariableDeclaration(i) => i.references(),
            BaseNode::EventDefinition(i) => i.references(),
            BaseNode::ModifierDefinition(i) => i.references(),
        }
    }
}

impl AstVisitor for EventDefinition {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = vec![];
        result.append(&mut self.parameters().filter_by_node_type(node_type));
        if node_type == NodeType::EventDefinition {
            result.push(NodeTypeInternal::EventDefinition(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = vec![];
        result.append(&mut self.parameters().filter_by_reference_id(id));

        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = vec![];
        result.append(&mut self.parameters().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::EventDefinition(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = vec![];
        result.append(&mut self.parameters().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        self.parameters().references()
    }
}

impl AstVisitor for ModifierDefinition {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.body().filter_by_node_type(node_type);
        result.append(&mut self.overrides().filter_by_node_type(node_type));
        result.append(&mut self.parameters().filter_by_node_type(node_type));
        if node_type == NodeType::ModifierDefinition {
            result.push(NodeTypeInternal::ModifierDefinition(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.body().filter_by_reference_id(id);
        result.append(&mut self.overrides().filter_by_reference_id(id));
        result.append(&mut self.parameters().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.body().filter_by_id(id);
        result.append(&mut self.overrides().filter_by_id(id));
        result.append(&mut self.parameters().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::ModifierDefinition(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.body().childrens_id();
        result.append(&mut self.overrides().childrens_id());
        result.append(&mut self.parameters().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.body().references();
        result.append(&mut self.overrides().references());
        result.append(&mut self.parameters().references());
        result
    }
}

impl AstVisitor for EnumValue {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = Vec::new();
        if node_type == NodeType::EnumValue {
            result.push(NodeTypeInternal::EnumValue(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, _id: isize) -> Vec<NodeTypeInternal> {
        vec![]
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = Vec::new();
        if id == self.id() {
            result.push(NodeTypeInternal::EnumValue(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        vec![self.id()]
    }

    fn references(&self) -> Vec<isize> {
        vec![]
    }
}

impl AstVisitor for ParameterList {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.parameters().filter_by_node_type(node_type);
        if node_type == NodeType::ParameterList {
            result.push(NodeTypeInternal::ParameterList(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.parameters().filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.parameters().filter_by_id(id);
        if id == self.id() {
            result.push(NodeTypeInternal::ParameterList(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.parameters().childrens_id();
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        self.parameters().references()
    }
}

impl AstVisitor for Block {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.statements().filter_by_node_type(node_type);
        if node_type == NodeType::Block {
            result.push(NodeTypeInternal::Block(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.statements().filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.statements().filter_by_id(id);
        if id == self.id() {
            result.push(NodeTypeInternal::Block(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.statements().childrens_id();
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        self.statements().references()
    }
}

impl AstVisitor for Statement {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        match self {
            Statement::Block(i) => i.filter_by_node_type(node_type),
            Statement::Break(i) => i.filter_by_node_type(node_type),
            Statement::Continue(i) => i.filter_by_node_type(node_type),
            Statement::DoWhileStatement(i) => i.filter_by_node_type(node_type),
            Statement::EmitStatement(i) => i.filter_by_node_type(node_type),
            Statement::ExpressionStatement(i) => i.filter_by_node_type(node_type),
            Statement::ForStatement(i) => i.filter_by_node_type(node_type),
            Statement::IfStatement(i) => i.filter_by_node_type(node_type),
            Statement::PlaceholderStatement(i) => i.filter_by_node_type(node_type),
            Statement::Return(i) => i.filter_by_node_type(node_type),
            Statement::RevertStatement(i) => i.filter_by_node_type(node_type),
            Statement::TryStatement(i) => i.filter_by_node_type(node_type),
            Statement::UncheckedBlock(i) => i.filter_by_node_type(node_type),
            Statement::VariableDeclarationStatement(i) => i.filter_by_node_type(node_type),
            Statement::WhileStatement(i) => i.filter_by_node_type(node_type),
            Statement::InlineAssembly(_) => vec![],
        }
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            Statement::Block(i) => i.filter_by_reference_id(id),
            Statement::Break(i) => i.filter_by_reference_id(id),
            Statement::Continue(i) => i.filter_by_reference_id(id),
            Statement::DoWhileStatement(i) => i.filter_by_reference_id(id),
            Statement::EmitStatement(i) => i.filter_by_reference_id(id),
            Statement::ExpressionStatement(i) => i.filter_by_reference_id(id),
            Statement::ForStatement(i) => i.filter_by_reference_id(id),
            Statement::IfStatement(i) => i.filter_by_reference_id(id),
            Statement::PlaceholderStatement(i) => i.filter_by_reference_id(id),
            Statement::Return(i) => i.filter_by_reference_id(id),
            Statement::RevertStatement(i) => i.filter_by_reference_id(id),
            Statement::TryStatement(i) => i.filter_by_reference_id(id),
            Statement::UncheckedBlock(i) => i.filter_by_reference_id(id),
            Statement::VariableDeclarationStatement(i) => i.filter_by_reference_id(id),
            Statement::WhileStatement(i) => i.filter_by_reference_id(id),
            Statement::InlineAssembly(_) => vec![],
        }
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            Statement::Block(i) => i.filter_by_id(id),
            Statement::Break(i) => i.filter_by_id(id),
            Statement::Continue(i) => i.filter_by_id(id),
            Statement::DoWhileStatement(i) => i.filter_by_id(id),
            Statement::EmitStatement(i) => i.filter_by_id(id),
            Statement::ExpressionStatement(i) => i.filter_by_id(id),
            Statement::ForStatement(i) => i.filter_by_id(id),
            Statement::IfStatement(i) => i.filter_by_id(id),
            Statement::PlaceholderStatement(i) => i.filter_by_id(id),
            Statement::Return(i) => i.filter_by_id(id),
            Statement::RevertStatement(i) => i.filter_by_id(id),
            Statement::TryStatement(i) => i.filter_by_id(id),
            Statement::UncheckedBlock(i) => i.filter_by_id(id),
            Statement::VariableDeclarationStatement(i) => i.filter_by_id(id),
            Statement::WhileStatement(i) => i.filter_by_id(id),
            Statement::InlineAssembly(_) => vec![],
        }
    }

    fn childrens_id(&self) -> Vec<isize> {
        match self {
            Statement::Block(i) => i.childrens_id(),
            Statement::Break(i) => i.childrens_id(),
            Statement::Continue(i) => i.childrens_id(),
            Statement::DoWhileStatement(i) => i.childrens_id(),
            Statement::EmitStatement(i) => i.childrens_id(),
            Statement::ExpressionStatement(i) => i.childrens_id(),
            Statement::ForStatement(i) => i.childrens_id(),
            Statement::IfStatement(i) => i.childrens_id(),
            Statement::PlaceholderStatement(i) => i.childrens_id(),
            Statement::Return(i) => i.childrens_id(),
            Statement::RevertStatement(i) => i.childrens_id(),
            Statement::TryStatement(i) => i.childrens_id(),
            Statement::UncheckedBlock(i) => i.childrens_id(),
            Statement::VariableDeclarationStatement(i) => i.childrens_id(),
            Statement::WhileStatement(i) => i.childrens_id(),
            Statement::InlineAssembly(_) => vec![],
        }
    }

    fn references(&self) -> Vec<isize> {
        match self {
            Statement::Block(i) => i.references(),
            Statement::Break(i) => i.references(),
            Statement::Continue(i) => i.references(),
            Statement::DoWhileStatement(i) => i.references(),
            Statement::EmitStatement(i) => i.references(),
            Statement::ExpressionStatement(i) => i.references(),
            Statement::ForStatement(i) => i.references(),
            Statement::IfStatement(i) => i.references(),
            Statement::InlineAssembly(i) => i.references(),
            Statement::PlaceholderStatement(i) => i.references(),
            Statement::Return(i) => i.references(),
            Statement::RevertStatement(i) => i.references(),
            Statement::TryStatement(i) => i.references(),
            Statement::UncheckedBlock(i) => i.references(),
            Statement::VariableDeclarationStatement(i) => i.references(),
            Statement::WhileStatement(i) => i.references(),
        }
    }
}

impl AstVisitor for Break {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = Vec::new();
        if node_type == NodeType::Break {
            result.push(NodeTypeInternal::Break(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, _id: isize) -> Vec<NodeTypeInternal> {
        vec![]
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = Vec::new();
        if id == self.id() {
            result.push(NodeTypeInternal::Break(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        vec![self.id()]
    }

    fn references(&self) -> Vec<isize> {
        vec![]
    }
}

impl AstVisitor for Continue {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = Vec::new();
        if node_type == NodeType::Continue {
            result.push(NodeTypeInternal::Continue(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, _id: isize) -> Vec<NodeTypeInternal> {
        vec![]
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = Vec::new();
        if id == self.id() {
            result.push(NodeTypeInternal::Continue(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        vec![self.id()]
    }

    fn references(&self) -> Vec<isize> {
        vec![]
    }
}

impl AstVisitor for DoWhileStatement {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.body().filter_by_node_type(node_type);
        result.append(&mut self.condition().filter_by_node_type(node_type));
        if node_type == NodeType::DoWhileStatement {
            result.push(NodeTypeInternal::DoWhileStatement(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.body().filter_by_reference_id(id);
        result.append(&mut self.condition().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.body().filter_by_id(id);
        result.append(&mut self.condition().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::DoWhileStatement(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.body().childrens_id();
        result.append(&mut self.condition().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.body().references();
        result.append(&mut self.condition().references());
        result
    }
}

impl AstVisitor for EmitStatement {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.event_call().filter_by_node_type(node_type);
        if node_type == NodeType::EmitStatement {
            result.push(NodeTypeInternal::EmitStatement(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.event_call().filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.event_call().filter_by_id(id);
        if id == self.id() {
            result.push(NodeTypeInternal::EmitStatement(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.event_call().childrens_id();
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        self.event_call().references()
    }
}

impl AstVisitor for ExpressionStatement {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.expression().filter_by_node_type(node_type);
        if node_type == NodeType::ExpressionStatement {
            result.push(NodeTypeInternal::ExpressionStatement(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.expression().filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.expression().filter_by_id(id);
        if id == self.id() {
            result.push(NodeTypeInternal::ExpressionStatement(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.expression().childrens_id();
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        self.expression().references()
    }
}

impl AstVisitor for ForStatement {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.body().filter_by_node_type(node_type);
        result.append(&mut self.condition().filter_by_node_type(node_type));
        result.append(
            &mut self
                .initialization_expression()
                .filter_by_node_type(node_type),
        );
        result.append(&mut self.loop_expression().filter_by_node_type(node_type));
        if node_type == NodeType::ForStatement {
            result.push(NodeTypeInternal::ForStatement(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.body().filter_by_reference_id(id);
        result.append(&mut self.condition().filter_by_reference_id(id));
        result.append(&mut self.initialization_expression().filter_by_reference_id(id));
        result.append(&mut self.loop_expression().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.body().filter_by_id(id);
        result.append(&mut self.condition().filter_by_id(id));
        result.append(&mut self.initialization_expression().filter_by_id(id));
        result.append(&mut self.loop_expression().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::ForStatement(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.body().childrens_id();
        result.append(&mut self.condition().childrens_id());
        result.append(&mut self.initialization_expression().childrens_id());
        result.append(&mut self.loop_expression().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.body().references();
        result.append(&mut self.condition().references());
        result.append(&mut self.initialization_expression().references());
        result.append(&mut self.loop_expression().references());
        result
    }
}

impl AstVisitor for IfStatement {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.condition().filter_by_node_type(node_type);
        result.append(&mut self.false_body().filter_by_node_type(node_type));
        result.append(&mut self.true_body().filter_by_node_type(node_type));

        if node_type == NodeType::IfStatement {
            result.push(NodeTypeInternal::IfStatement(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.condition().filter_by_reference_id(id);
        result.append(&mut self.false_body().filter_by_reference_id(id));
        result.append(&mut self.true_body().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.condition().filter_by_id(id);
        result.append(&mut self.false_body().filter_by_id(id));
        result.append(&mut self.true_body().filter_by_id(id));

        if id == self.id() {
            result.push(NodeTypeInternal::IfStatement(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.condition().childrens_id();
        result.append(&mut self.false_body().childrens_id());
        result.append(&mut self.true_body().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.condition().references();
        result.append(&mut self.false_body().references());
        result.append(&mut self.true_body().references());
        result
    }
}

impl AstVisitor for PlaceholderStatement {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = Vec::new();
        if node_type == NodeType::PlaceholderStatement {
            result.push(NodeTypeInternal::PlaceholderStatement(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, _id: isize) -> Vec<NodeTypeInternal> {
        vec![]
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = Vec::new();
        if id == self.id() {
            result.push(NodeTypeInternal::PlaceholderStatement(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        vec![self.id()]
    }

    fn references(&self) -> Vec<isize> {
        vec![]
    }
}

impl AstVisitor for Return {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.expression().filter_by_node_type(node_type);
        if node_type == NodeType::Return {
            result.push(NodeTypeInternal::Return(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.expression().filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.expression().filter_by_id(id);
        if id == self.id() {
            result.push(NodeTypeInternal::Return(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.expression().childrens_id();
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        self.expression().references()
    }
}

impl AstVisitor for RevertStatement {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.error_call().filter_by_node_type(node_type);
        if node_type == NodeType::RevertStatement {
            result.push(NodeTypeInternal::RevertStatement(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.error_call().filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.error_call().filter_by_id(id);
        if id == self.id() {
            result.push(NodeTypeInternal::RevertStatement(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.error_call().childrens_id();
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        self.error_call().references()
    }
}

impl AstVisitor for TryStatement {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.clauses().filter_by_node_type(node_type);
        result.append(&mut self.external_call().filter_by_node_type(node_type));
        if node_type == NodeType::TryStatement {
            result.push(NodeTypeInternal::TryStatement(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.clauses().filter_by_reference_id(id);
        result.append(&mut self.external_call().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.clauses().filter_by_id(id);
        result.append(&mut self.external_call().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::TryStatement(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.clauses().childrens_id();
        result.append(&mut self.external_call().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.clauses().references();
        result.append(&mut self.external_call().references());
        result
    }
}

impl AstVisitor for UncheckedBlock {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.statements().filter_by_node_type(node_type);
        if node_type == NodeType::UncheckedBlock {
            result.push(NodeTypeInternal::UncheckedBlock(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.statements().filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.statements().filter_by_id(id);
        if id == self.id() {
            result.push(NodeTypeInternal::UncheckedBlock(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.statements().childrens_id();
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        self.statements().references()
    }
}

impl AstVisitor for VariableDeclarationStatement {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.initial_value().filter_by_node_type(node_type);
        result.append(&mut self.declarations().filter_by_node_type(node_type));
        if node_type == NodeType::VariableDeclarationStatement {
            result.push(NodeTypeInternal::VariableDeclarationStatement(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.initial_value().filter_by_reference_id(id);
        result.append(&mut self.declarations().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.initial_value().filter_by_id(id);
        result.append(&mut self.declarations().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::VariableDeclarationStatement(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.initial_value().childrens_id();
        result.append(&mut self.declarations().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.initial_value().references();
        result.append(&mut self.declarations().references());
        result
    }
}

impl AstVisitor for WhileStatement {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.body().filter_by_node_type(node_type);
        result.append(&mut self.condition().filter_by_node_type(node_type));
        if node_type == NodeType::WhileStatement {
            result.push(NodeTypeInternal::WhileStatement(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.body().filter_by_reference_id(id);
        result.append(&mut self.condition().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.body().filter_by_id(id);
        result.append(&mut self.condition().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::WhileStatement(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.body().childrens_id();
        result.append(&mut self.condition().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.body().references();
        result.append(&mut self.condition().references());
        result
    }
}

impl AstVisitor for ModifierInvocation {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.arguments().filter_by_node_type(node_type);
        result.append(&mut self.modifier_name().filter_by_node_type(node_type));
        if node_type == NodeType::ModifierInvocation {
            result.push(NodeTypeInternal::ModifierInvocation(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.arguments().filter_by_reference_id(id);
        result.append(&mut self.modifier_name().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.arguments().filter_by_id(id);
        result.append(&mut self.modifier_name().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::ModifierInvocation(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.arguments().childrens_id();
        result.append(&mut self.modifier_name().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.arguments().references();
        result.append(&mut self.modifier_name().references());
        result
    }
}

impl AstVisitor for ModifierName {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        match self {
            ModifierName::Identifier(i) => i.filter_by_node_type(node_type),
            ModifierName::IdentifierPath(i) => i.filter_by_node_type(node_type),
        }
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            ModifierName::Identifier(i) => i.filter_by_reference_id(id),
            ModifierName::IdentifierPath(i) => i.filter_by_reference_id(id),
        }
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            ModifierName::Identifier(i) => i.filter_by_id(id),
            ModifierName::IdentifierPath(i) => i.filter_by_id(id),
        }
    }

    fn childrens_id(&self) -> Vec<isize> {
        match self {
            ModifierName::Identifier(i) => i.childrens_id(),
            ModifierName::IdentifierPath(i) => i.childrens_id(),
        }
    }

    fn references(&self) -> Vec<isize> {
        match self {
            ModifierName::Identifier(i) => i.references(),
            ModifierName::IdentifierPath(i) => i.references(),
        }
    }
}

impl AstVisitor for OverrideSpecifier {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.overrides().filter_by_node_type(node_type);
        if node_type == NodeType::OverrideSpecifier {
            result.push(NodeTypeInternal::OverrideSpecifier(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.overrides().filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.overrides().filter_by_id(id);
        if id == self.id() {
            result.push(NodeTypeInternal::OverrideSpecifier(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.overrides().childrens_id();
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        self.overrides().references()
    }
}

impl AstVisitor for Overrides {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        match self {
            Overrides::UserDefinedTypeName(i) => i.filter_by_node_type(node_type),
            Overrides::IdentifierPath(i) => i.filter_by_node_type(node_type),
        }
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            Overrides::UserDefinedTypeName(i) => i.filter_by_reference_id(id),
            Overrides::IdentifierPath(i) => i.filter_by_reference_id(id),
        }
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            Overrides::UserDefinedTypeName(i) => i.filter_by_id(id),
            Overrides::IdentifierPath(i) => i.filter_by_id(id),
        }
    }

    fn childrens_id(&self) -> Vec<isize> {
        match self {
            Overrides::UserDefinedTypeName(i) => i.childrens_id(),
            Overrides::IdentifierPath(i) => i.childrens_id(),
        }
    }

    fn references(&self) -> Vec<isize> {
        match self {
            Overrides::UserDefinedTypeName(i) => i.references(),
            Overrides::IdentifierPath(i) => i.references(),
        }
    }
}

impl AstVisitor for TypeName {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        match self {
            TypeName::ArrayTypeName(i) => i.filter_by_node_type(node_type),
            TypeName::ElementaryTypeName(i) => i.filter_by_node_type(node_type),
            TypeName::FunctionTypeName(i) => i.filter_by_node_type(node_type),
            TypeName::Mapping(i) => i.filter_by_node_type(node_type),
            TypeName::UserDefinedTypeName(i) => i.filter_by_node_type(node_type),
        }
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            TypeName::ArrayTypeName(i) => i.filter_by_reference_id(id),
            TypeName::ElementaryTypeName(i) => i.filter_by_reference_id(id),
            TypeName::FunctionTypeName(i) => i.filter_by_reference_id(id),
            TypeName::Mapping(i) => i.filter_by_reference_id(id),
            TypeName::UserDefinedTypeName(i) => i.filter_by_reference_id(id),
        }
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            TypeName::ArrayTypeName(i) => i.filter_by_id(id),
            TypeName::ElementaryTypeName(i) => i.filter_by_id(id),
            TypeName::FunctionTypeName(i) => i.filter_by_id(id),
            TypeName::Mapping(i) => i.filter_by_id(id),
            TypeName::UserDefinedTypeName(i) => i.filter_by_id(id),
        }
    }

    fn childrens_id(&self) -> Vec<isize> {
        match self {
            TypeName::ArrayTypeName(i) => i.childrens_id(),
            TypeName::ElementaryTypeName(i) => i.childrens_id(),
            TypeName::FunctionTypeName(i) => i.childrens_id(),
            TypeName::Mapping(i) => i.childrens_id(),
            TypeName::UserDefinedTypeName(i) => i.childrens_id(),
        }
    }

    fn references(&self) -> Vec<isize> {
        match self {
            TypeName::ArrayTypeName(i) => i.references(),
            TypeName::ElementaryTypeName(i) => i.references(),
            TypeName::FunctionTypeName(i) => i.references(),
            TypeName::Mapping(i) => i.references(),
            TypeName::UserDefinedTypeName(i) => i.references(),
        }
    }
}

impl AstVisitor for ArrayTypeName {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.base_type().filter_by_node_type(node_type);
        result.append(&mut self.length().filter_by_node_type(node_type));
        if node_type == NodeType::ArrayTypeName {
            result.push(NodeTypeInternal::ArrayTypeName(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.base_type().filter_by_reference_id(id);
        result.append(&mut self.length().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.base_type().filter_by_id(id);
        result.append(&mut self.length().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::ArrayTypeName(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.base_type().childrens_id();
        result.append(&mut self.length().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.base_type().references();
        result.append(&mut self.length().references());
        result
    }
}

impl AstVisitor for ElementaryTypeName {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = Vec::new();
        if node_type == NodeType::ElementaryTypeName {
            result.push(NodeTypeInternal::ElementaryTypeName(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, _id: isize) -> Vec<NodeTypeInternal> {
        vec![]
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = Vec::new();
        if id == self.id() {
            result.push(NodeTypeInternal::ElementaryTypeName(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        vec![self.id()]
    }

    fn references(&self) -> Vec<isize> {
        vec![]
    }
}

impl AstVisitor for FunctionTypeName {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.return_parameter_types().filter_by_node_type(node_type);
        result.append(&mut self.parameter_types().filter_by_node_type(node_type));
        if node_type == NodeType::FunctionTypeName {
            result.push(NodeTypeInternal::FunctionTypeName(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.return_parameter_types().filter_by_reference_id(id);
        result.append(&mut self.parameter_types().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.return_parameter_types().filter_by_id(id);
        result.append(&mut self.parameter_types().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::FunctionTypeName(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.return_parameter_types().childrens_id();
        result.append(&mut self.parameter_types().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.return_parameter_types().references();
        result.append(&mut self.parameter_types().references());
        result
    }
}

impl AstVisitor for Mapping {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.key_type().filter_by_node_type(node_type);
        result.append(&mut self.value_type().filter_by_node_type(node_type));
        if node_type == NodeType::Mapping {
            result.push(NodeTypeInternal::Mapping(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.key_type().filter_by_reference_id(id);
        result.append(&mut self.value_type().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.key_type().filter_by_id(id);
        result.append(&mut self.value_type().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::Mapping(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.key_type().childrens_id();
        result.append(&mut self.value_type().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.key_type().references();
        result.append(&mut self.value_type().references());
        result
    }
}

// @note Has a referenced_declaration
impl AstVisitor for UserDefinedTypeName {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.path_node().filter_by_node_type(node_type);
        if node_type == NodeType::UserDefinedTypeName {
            result.push(NodeTypeInternal::UserDefinedTypeName(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.path_node().filter_by_reference_id(id);
        if id == self.referenced_declaration() {
            result.push(NodeTypeInternal::UserDefinedTypeName(self.clone()));
        }
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.path_node().filter_by_id(id);
        if id == self.id() {
            result.push(NodeTypeInternal::UserDefinedTypeName(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.path_node().childrens_id();
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.path_node().references();
        result.push(self.referenced_declaration());
        result
    }
}

// @note Has a referenced_declaration
impl AstVisitor for Identifier {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.argument_types().filter_by_node_type(node_type);
        if node_type == NodeType::Identifier {
            result.push(NodeTypeInternal::Identifier(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.argument_types().filter_by_reference_id(id);
        if Some(id) == self.referenced_declaration() {
            result.push(NodeTypeInternal::Identifier(self.clone()));
        }
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.argument_types().filter_by_id(id);
        if id == self.id() {
            result.push(NodeTypeInternal::Identifier(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.argument_types().childrens_id();
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.argument_types().references();
        if let Some(id) = self.referenced_declaration() {
            result.push(id);
        }
        result
    }
}

impl AstVisitor for TypeDescriptions {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = vec![];
        if node_type == NodeType::TypeDescriptions {
            result.push(NodeTypeInternal::TypeDescriptions(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, _id: isize) -> Vec<NodeTypeInternal> {
        vec![]
    }

    fn filter_by_id(&self, _id: isize) -> Vec<NodeTypeInternal> {
        vec![]
    }

    fn childrens_id(&self) -> Vec<isize> {
        vec![]
    }

    fn references(&self) -> Vec<isize> {
        vec![]
    }
}

impl AstVisitor for LibraryName {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        match self {
            LibraryName::UserDefinedTypeName(i) => i.filter_by_node_type(node_type),
            LibraryName::IdentifierPath(i) => i.filter_by_node_type(node_type),
        }
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            LibraryName::UserDefinedTypeName(i) => i.filter_by_reference_id(id),
            LibraryName::IdentifierPath(i) => i.filter_by_reference_id(id),
        }
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            LibraryName::UserDefinedTypeName(i) => i.filter_by_id(id),
            LibraryName::IdentifierPath(i) => i.filter_by_id(id),
        }
    }

    fn childrens_id(&self) -> Vec<isize> {
        match self {
            LibraryName::UserDefinedTypeName(i) => i.childrens_id(),
            LibraryName::IdentifierPath(i) => i.childrens_id(),
        }
    }

    fn references(&self) -> Vec<isize> {
        match self {
            LibraryName::UserDefinedTypeName(i) => i.references(),
            LibraryName::IdentifierPath(i) => i.references(),
        }
    }
}

// @note Has referenced declarations
impl AstVisitor for IdentifierPath {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = Vec::new();
        if node_type == NodeType::IdentifierPath {
            result.push(NodeTypeInternal::IdentifierPath(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = Vec::new();
        if id == self.referenced_declaration() {
            result.push(NodeTypeInternal::IdentifierPath(self.clone()));
        }
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = Vec::new();
        if id == self.id() {
            result.push(NodeTypeInternal::IdentifierPath(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        vec![self.id()]
    }

    fn references(&self) -> Vec<isize> {
        vec![self.referenced_declaration()]
    }
}

impl AstVisitor for Expression {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        match self {
            Expression::Assignment(i) => i.filter_by_node_type(node_type),
            Expression::BinaryOperation(i) => i.filter_by_node_type(node_type),
            Expression::Conditional(i) => i.filter_by_node_type(node_type),
            Expression::ElementaryTypeNameExpression(i) => i.filter_by_node_type(node_type),
            Expression::FunctionCall(i) => i.filter_by_node_type(node_type),
            Expression::FunctionCallOptions(i) => i.filter_by_node_type(node_type),
            Expression::Identifier(i) => i.filter_by_node_type(node_type),
            Expression::IndexAccess(i) => i.filter_by_node_type(node_type),
            Expression::IndexRangeAccess(i) => i.filter_by_node_type(node_type),
            Expression::Literal(i) => i.filter_by_node_type(node_type),
            Expression::MemberAccess(i) => i.filter_by_node_type(node_type),
            Expression::NewExpression(i) => i.filter_by_node_type(node_type),
            Expression::TupleExpression(i) => i.filter_by_node_type(node_type),
            Expression::UnaryOperation(i) => i.filter_by_node_type(node_type),
        }
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            Expression::Assignment(i) => i.filter_by_reference_id(id),
            Expression::BinaryOperation(i) => i.filter_by_reference_id(id),
            Expression::Conditional(i) => i.filter_by_reference_id(id),
            Expression::ElementaryTypeNameExpression(i) => i.filter_by_reference_id(id),
            Expression::FunctionCall(i) => i.filter_by_reference_id(id),
            Expression::FunctionCallOptions(i) => i.filter_by_reference_id(id),
            Expression::Identifier(i) => i.filter_by_reference_id(id),
            Expression::IndexAccess(i) => i.filter_by_reference_id(id),
            Expression::IndexRangeAccess(i) => i.filter_by_reference_id(id),
            Expression::Literal(i) => i.filter_by_reference_id(id),
            Expression::MemberAccess(i) => i.filter_by_reference_id(id),
            Expression::NewExpression(i) => i.filter_by_reference_id(id),
            Expression::TupleExpression(i) => i.filter_by_reference_id(id),
            Expression::UnaryOperation(i) => i.filter_by_reference_id(id),
        }
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            Expression::Assignment(i) => i.filter_by_id(id),
            Expression::BinaryOperation(i) => i.filter_by_id(id),
            Expression::Conditional(i) => i.filter_by_id(id),
            Expression::ElementaryTypeNameExpression(i) => i.filter_by_id(id),
            Expression::FunctionCall(i) => i.filter_by_id(id),
            Expression::FunctionCallOptions(i) => i.filter_by_id(id),
            Expression::Identifier(i) => i.filter_by_id(id),
            Expression::IndexAccess(i) => i.filter_by_id(id),
            Expression::IndexRangeAccess(i) => i.filter_by_id(id),
            Expression::Literal(i) => i.filter_by_id(id),
            Expression::MemberAccess(i) => i.filter_by_id(id),
            Expression::NewExpression(i) => i.filter_by_id(id),
            Expression::TupleExpression(i) => i.filter_by_id(id),
            Expression::UnaryOperation(i) => i.filter_by_id(id),
        }
    }

    fn childrens_id(&self) -> Vec<isize> {
        match self {
            Expression::Assignment(i) => i.childrens_id(),
            Expression::BinaryOperation(i) => i.childrens_id(),
            Expression::Conditional(i) => i.childrens_id(),
            Expression::ElementaryTypeNameExpression(i) => i.childrens_id(),
            Expression::FunctionCall(i) => i.childrens_id(),
            Expression::FunctionCallOptions(i) => i.childrens_id(),
            Expression::Identifier(i) => i.childrens_id(),
            Expression::IndexAccess(i) => i.childrens_id(),
            Expression::IndexRangeAccess(i) => i.childrens_id(),
            Expression::Literal(i) => i.childrens_id(),
            Expression::MemberAccess(i) => i.childrens_id(),
            Expression::NewExpression(i) => i.childrens_id(),
            Expression::TupleExpression(i) => i.childrens_id(),
            Expression::UnaryOperation(i) => i.childrens_id(),
        }
    }

    fn references(&self) -> Vec<isize> {
        match self {
            Expression::Assignment(i) => i.references(),
            Expression::BinaryOperation(i) => i.references(),
            Expression::Conditional(i) => i.references(),
            Expression::ElementaryTypeNameExpression(i) => i.references(),
            Expression::FunctionCall(i) => i.references(),
            Expression::FunctionCallOptions(i) => i.references(),
            Expression::Identifier(i) => i.references(),
            Expression::IndexAccess(i) => i.references(),
            Expression::IndexRangeAccess(i) => i.references(),
            Expression::Literal(i) => i.references(),
            Expression::MemberAccess(i) => i.references(),
            Expression::NewExpression(i) => i.references(),
            Expression::TupleExpression(i) => i.references(),
            Expression::UnaryOperation(i) => i.references(),
        }
    }
}

impl AstVisitor for Body {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        match self {
            Body::Block(i) => i.filter_by_node_type(node_type),
            Body::Break(i) => i.filter_by_node_type(node_type),
            Body::Continue(i) => i.filter_by_node_type(node_type),
            Body::DoWhileStatement(i) => i.filter_by_node_type(node_type),
            Body::EmitStatement(i) => i.filter_by_node_type(node_type),
            Body::ExpressionStatement(i) => i.filter_by_node_type(node_type),
            Body::ForStatement(i) => i.filter_by_node_type(node_type),
            Body::IfStatement(i) => i.filter_by_node_type(node_type),
            Body::InlineAssembly(i) => i.filter_by_node_type(node_type),
            Body::PlaceholderStatement(i) => i.filter_by_node_type(node_type),
            Body::Return(i) => i.filter_by_node_type(node_type),
            Body::RevertStatement(i) => i.filter_by_node_type(node_type),
            Body::TryStatement(i) => i.filter_by_node_type(node_type),
            Body::UncheckedBlock(i) => i.filter_by_node_type(node_type),
            Body::VariableDeclarationStatement(i) => i.filter_by_node_type(node_type),
            Body::WhileStatement(i) => i.filter_by_node_type(node_type),
        }
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            Body::Block(i) => i.filter_by_reference_id(id),
            Body::Break(i) => i.filter_by_reference_id(id),
            Body::Continue(i) => i.filter_by_reference_id(id),
            Body::DoWhileStatement(i) => i.filter_by_reference_id(id),
            Body::EmitStatement(i) => i.filter_by_reference_id(id),
            Body::ExpressionStatement(i) => i.filter_by_reference_id(id),
            Body::ForStatement(i) => i.filter_by_reference_id(id),
            Body::IfStatement(i) => i.filter_by_reference_id(id),
            Body::InlineAssembly(i) => i.filter_by_reference_id(id),
            Body::PlaceholderStatement(i) => i.filter_by_reference_id(id),
            Body::Return(i) => i.filter_by_reference_id(id),
            Body::RevertStatement(i) => i.filter_by_reference_id(id),
            Body::TryStatement(i) => i.filter_by_reference_id(id),
            Body::UncheckedBlock(i) => i.filter_by_reference_id(id),
            Body::VariableDeclarationStatement(i) => i.filter_by_reference_id(id),
            Body::WhileStatement(i) => i.filter_by_reference_id(id),
        }
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            Body::Block(i) => i.filter_by_id(id),
            Body::Break(i) => i.filter_by_id(id),
            Body::Continue(i) => i.filter_by_id(id),
            Body::DoWhileStatement(i) => i.filter_by_id(id),
            Body::EmitStatement(i) => i.filter_by_id(id),
            Body::ExpressionStatement(i) => i.filter_by_id(id),
            Body::ForStatement(i) => i.filter_by_id(id),
            Body::IfStatement(i) => i.filter_by_id(id),
            Body::InlineAssembly(i) => i.filter_by_id(id),
            Body::PlaceholderStatement(i) => i.filter_by_id(id),
            Body::Return(i) => i.filter_by_id(id),
            Body::RevertStatement(i) => i.filter_by_id(id),
            Body::TryStatement(i) => i.filter_by_id(id),
            Body::UncheckedBlock(i) => i.filter_by_id(id),
            Body::VariableDeclarationStatement(i) => i.filter_by_id(id),
            Body::WhileStatement(i) => i.filter_by_id(id),
        }
    }

    fn childrens_id(&self) -> Vec<isize> {
        match self {
            Body::Block(i) => i.childrens_id(),
            Body::Break(i) => i.childrens_id(),
            Body::Continue(i) => i.childrens_id(),
            Body::DoWhileStatement(i) => i.childrens_id(),
            Body::EmitStatement(i) => i.childrens_id(),
            Body::ExpressionStatement(i) => i.childrens_id(),
            Body::ForStatement(i) => i.childrens_id(),
            Body::IfStatement(i) => i.childrens_id(),
            Body::InlineAssembly(i) => i.childrens_id(),
            Body::PlaceholderStatement(i) => i.childrens_id(),
            Body::Return(i) => i.childrens_id(),
            Body::RevertStatement(i) => i.childrens_id(),
            Body::TryStatement(i) => i.childrens_id(),
            Body::UncheckedBlock(i) => i.childrens_id(),
            Body::VariableDeclarationStatement(i) => i.childrens_id(),
            Body::WhileStatement(i) => i.childrens_id(),
        }
    }

    fn references(&self) -> Vec<isize> {
        match self {
            Body::Block(i) => i.references(),
            Body::Break(i) => i.references(),
            Body::Continue(i) => i.references(),
            Body::DoWhileStatement(i) => i.references(),
            Body::EmitStatement(i) => i.references(),
            Body::ExpressionStatement(i) => i.references(),
            Body::ForStatement(i) => i.references(),
            Body::IfStatement(i) => i.references(),
            Body::InlineAssembly(i) => i.references(),
            Body::PlaceholderStatement(i) => i.references(),
            Body::Return(i) => i.references(),
            Body::RevertStatement(i) => i.references(),
            Body::TryStatement(i) => i.references(),
            Body::UncheckedBlock(i) => i.references(),
            Body::VariableDeclarationStatement(i) => i.references(),
            Body::WhileStatement(i) => i.references(),
        }
    }
}

impl AstVisitor for Assignment {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.left_hand_side().filter_by_node_type(node_type);
        result.append(&mut self.right_hand_side().filter_by_node_type(node_type));
        result.append(&mut self.argument_types().filter_by_node_type(node_type));

        if node_type == NodeType::Assignment {
            result.push(NodeTypeInternal::Assignment(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.left_hand_side().filter_by_reference_id(id);
        result.append(&mut self.right_hand_side().filter_by_reference_id(id));
        result.append(&mut self.argument_types().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.left_hand_side().filter_by_id(id);
        result.append(&mut self.right_hand_side().filter_by_id(id));
        result.append(&mut self.argument_types().filter_by_id(id));

        if id == self.id() {
            result.push(NodeTypeInternal::Assignment(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.left_hand_side().childrens_id();
        result.append(&mut self.right_hand_side().childrens_id());
        result.append(&mut self.argument_types().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.left_hand_side().references();
        result.append(&mut self.right_hand_side().references());
        result.append(&mut self.argument_types().references());
        result
    }
}

impl AstVisitor for BinaryOperation {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.left_expression().filter_by_node_type(node_type);
        result.append(&mut self.right_expression().filter_by_node_type(node_type));
        result.append(&mut self.argument_types().filter_by_node_type(node_type));

        if node_type == NodeType::BinaryOperation {
            result.push(NodeTypeInternal::BinaryOperation(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.left_expression().filter_by_reference_id(id);
        result.append(&mut self.right_expression().filter_by_reference_id(id));
        result.append(&mut self.argument_types().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.left_expression().filter_by_id(id);
        result.append(&mut self.right_expression().filter_by_id(id));
        result.append(&mut self.argument_types().filter_by_id(id));

        if id == self.id() {
            result.push(NodeTypeInternal::BinaryOperation(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.left_expression().childrens_id();
        result.append(&mut self.right_expression().childrens_id());
        result.append(&mut self.argument_types().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.left_expression().references();
        result.append(&mut self.right_expression().references());
        result.append(&mut self.argument_types().references());
        result
    }
}

impl AstVisitor for Conditional {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.argument_types().filter_by_node_type(node_type);
        result.append(&mut self.condition().filter_by_node_type(node_type));
        result.append(&mut self.true_expression().filter_by_node_type(node_type));
        result.append(&mut self.false_expression().filter_by_node_type(node_type));

        if node_type == NodeType::Conditional {
            result.push(NodeTypeInternal::Conditional(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.argument_types().filter_by_reference_id(id);
        result.append(&mut self.condition().filter_by_reference_id(id));
        result.append(&mut self.true_expression().filter_by_reference_id(id));
        result.append(&mut self.false_expression().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.argument_types().filter_by_id(id);
        result.append(&mut self.condition().filter_by_id(id));
        result.append(&mut self.true_expression().filter_by_id(id));
        result.append(&mut self.false_expression().filter_by_id(id));

        if id == self.id() {
            result.push(NodeTypeInternal::Conditional(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.argument_types().childrens_id();
        result.append(&mut self.condition().childrens_id());
        result.append(&mut self.true_expression().childrens_id());
        result.append(&mut self.false_expression().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.argument_types().references();
        result.append(&mut self.condition().references());
        result.append(&mut self.true_expression().references());
        result.append(&mut self.false_expression().references());
        result
    }
}

impl AstVisitor for CompatabilityTypeName {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        match self {
            CompatabilityTypeName::ElementaryTypeName(etn) => etn.filter_by_node_type(node_type),
            CompatabilityTypeName::Name(_) => Vec::new(),
        }
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            CompatabilityTypeName::ElementaryTypeName(etn) => etn.filter_by_reference_id(id),
            CompatabilityTypeName::Name(_) => Vec::new(),
        }
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            CompatabilityTypeName::ElementaryTypeName(etn) => etn.filter_by_id(id),
            CompatabilityTypeName::Name(_) => Vec::new(),
        }
    }

    fn childrens_id(&self) -> Vec<isize> {
        match self {
            CompatabilityTypeName::ElementaryTypeName(etn) => etn.childrens_id(),
            CompatabilityTypeName::Name(_) => Vec::new(),
        }
    }

    fn references(&self) -> Vec<isize> {
        match self {
            CompatabilityTypeName::ElementaryTypeName(etn) => etn.references(),
            CompatabilityTypeName::Name(_) => Vec::new(),
        }
    }
}

impl AstVisitor for ElementaryTypeNameExpression {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.argument_types().filter_by_node_type(node_type);
        result.append(&mut self.type_name().filter_by_node_type(node_type));
        if node_type == NodeType::ElementaryTypeNameExpression {
            result.push(NodeTypeInternal::ElementaryTypeNameExpression(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.argument_types().filter_by_reference_id(id);
        result.append(&mut self.type_name().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.argument_types().filter_by_id(id);
        result.append(&mut self.type_name().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::ElementaryTypeNameExpression(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.argument_types().childrens_id();
        result.append(&mut self.type_name().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.argument_types().references();
        result.append(&mut self.type_name().references());
        result
    }
}

impl AstVisitor for FunctionCall {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.arguments().filter_by_node_type(node_type);
        result.append(&mut self.expression().filter_by_node_type(node_type));
        result.append(&mut self.argument_types().filter_by_node_type(node_type));

        if node_type == NodeType::FunctionCall {
            result.push(NodeTypeInternal::FunctionCall(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.arguments().filter_by_reference_id(id);
        result.append(&mut self.expression().filter_by_reference_id(id));
        result.append(&mut self.argument_types().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.arguments().filter_by_id(id);
        result.append(&mut self.expression().filter_by_id(id));
        result.append(&mut self.argument_types().filter_by_id(id));

        if id == self.id() {
            result.push(NodeTypeInternal::FunctionCall(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.arguments().childrens_id();
        result.append(&mut self.expression().childrens_id());
        result.append(&mut self.argument_types().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.arguments().references();
        result.append(&mut self.expression().references());
        result.append(&mut self.argument_types().references());
        result
    }
}

impl AstVisitor for FunctionCallOptions {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.options().filter_by_node_type(node_type);
        result.append(&mut self.expression().filter_by_node_type(node_type));
        result.append(&mut self.argument_types().filter_by_node_type(node_type));

        if node_type == NodeType::FunctionCallOptions {
            result.push(NodeTypeInternal::FunctionCallOptions(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.options().filter_by_reference_id(id);
        result.append(&mut self.expression().filter_by_reference_id(id));
        result.append(&mut self.argument_types().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.options().filter_by_id(id);
        result.append(&mut self.expression().filter_by_id(id));
        result.append(&mut self.argument_types().filter_by_id(id));

        if id == self.id() {
            result.push(NodeTypeInternal::FunctionCallOptions(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.options().childrens_id();
        result.append(&mut self.expression().childrens_id());
        result.append(&mut self.argument_types().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.options().references();
        result.append(&mut self.expression().references());
        result.append(&mut self.argument_types().references());
        result
    }
}

impl AstVisitor for IndexAccess {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.base_expression().filter_by_node_type(node_type);
        result.append(&mut self.index_expression().filter_by_node_type(node_type));
        result.append(&mut self.argument_types().filter_by_node_type(node_type));

        if node_type == NodeType::IndexAccess {
            result.push(NodeTypeInternal::IndexAccess(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.base_expression().filter_by_reference_id(id);
        result.append(&mut self.index_expression().filter_by_reference_id(id));
        result.append(&mut self.argument_types().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.base_expression().filter_by_id(id);
        result.append(&mut self.index_expression().filter_by_id(id));
        result.append(&mut self.argument_types().filter_by_id(id));

        if id == self.id() {
            result.push(NodeTypeInternal::IndexAccess(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.base_expression().childrens_id();
        result.append(&mut self.index_expression().childrens_id());
        result.append(&mut self.argument_types().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.base_expression().references();
        result.append(&mut self.index_expression().references());
        result.append(&mut self.argument_types().references());
        result
    }
}

impl AstVisitor for IndexRangeAccess {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.base_expression().filter_by_node_type(node_type);
        result.append(&mut self.start_expression().filter_by_node_type(node_type));
        result.append(&mut self.end_expression().filter_by_node_type(node_type));
        result.append(&mut self.argument_types().filter_by_node_type(node_type));

        if node_type == NodeType::IndexRangeAccess {
            result.push(NodeTypeInternal::IndexRangeAccess(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.base_expression().filter_by_reference_id(id);
        result.append(&mut self.start_expression().filter_by_reference_id(id));
        result.append(&mut self.end_expression().filter_by_reference_id(id));
        result.append(&mut self.argument_types().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.base_expression().filter_by_id(id);
        result.append(&mut self.start_expression().filter_by_id(id));
        result.append(&mut self.end_expression().filter_by_id(id));
        result.append(&mut self.argument_types().filter_by_id(id));

        if id == self.id() {
            result.push(NodeTypeInternal::IndexRangeAccess(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.base_expression().childrens_id();
        result.append(&mut self.start_expression().childrens_id());
        result.append(&mut self.end_expression().childrens_id());
        result.append(&mut self.argument_types().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.base_expression().references();
        result.append(&mut self.start_expression().references());
        result.append(&mut self.end_expression().references());
        result.append(&mut self.argument_types().references());
        result
    }
}

impl AstVisitor for Literal {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.argument_types().filter_by_node_type(node_type);
        if node_type == NodeType::Literal {
            result.push(NodeTypeInternal::Literal(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.argument_types().filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.argument_types().filter_by_id(id);
        if id == self.id() {
            result.push(NodeTypeInternal::Literal(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.argument_types().childrens_id();
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        self.argument_types().references()
    }
}

impl AstVisitor for MemberAccess {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.argument_types().filter_by_node_type(node_type);
        result.append(&mut self.expression().filter_by_node_type(node_type));
        if node_type == NodeType::MemberAccess {
            result.push(NodeTypeInternal::MemberAccess(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.argument_types().filter_by_reference_id(id);
        result.append(&mut self.expression().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.argument_types().filter_by_id(id);
        result.append(&mut self.expression().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::MemberAccess(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.argument_types().childrens_id();
        result.append(&mut self.expression().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.argument_types().references();
        result.append(&mut self.expression().references());
        if let Some(id) = self.referenced_declaration() {
            result.push(id);
        }
        result
    }
}

impl AstVisitor for NewExpression {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.argument_types().filter_by_node_type(node_type);
        result.append(&mut self.type_name().filter_by_node_type(node_type));
        if node_type == NodeType::NewExpression {
            result.push(NodeTypeInternal::NewExpression(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.argument_types().filter_by_reference_id(id);
        result.append(&mut self.type_name().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.argument_types().filter_by_id(id);
        result.append(&mut self.type_name().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::NewExpression(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.argument_types().childrens_id();
        result.append(&mut self.type_name().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.argument_types().references();
        result.append(&mut self.type_name().references());
        result
    }
}

impl AstVisitor for TupleExpression {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.argument_types().filter_by_node_type(node_type);
        result.append(&mut self.components().filter_by_node_type(node_type));
        if node_type == NodeType::TupleExpression {
            result.push(NodeTypeInternal::TupleExpression(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.argument_types().filter_by_reference_id(id);
        result.append(&mut self.components().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.argument_types().filter_by_id(id);
        result.append(&mut self.components().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::TupleExpression(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.argument_types().childrens_id();
        result.append(&mut self.components().childrens_id());
        result.push(self.id());
        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.argument_types().references();
        result.append(&mut self.components().references());
        result
    }
}

impl AstVisitor for UnaryOperation {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.argument_types().filter_by_node_type(node_type);
        result.append(&mut self.sub_expression().filter_by_node_type(node_type));
        if node_type == NodeType::UnaryOperation {
            result.push(NodeTypeInternal::UnaryOperation(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.argument_types().filter_by_reference_id(id);
        result.append(&mut self.sub_expression().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.argument_types().filter_by_id(id);
        result.append(&mut self.sub_expression().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::UnaryOperation(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.argument_types().childrens_id();
        result.append(&mut self.sub_expression().childrens_id());

        result.push(self.id());

        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.argument_types().references();
        result.append(&mut self.sub_expression().references());
        result
    }
}

impl AstVisitor for InitializationExpression {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        match self {
            InitializationExpression::ExpressionStatement(i) => i.filter_by_node_type(node_type),
            InitializationExpression::VariableDeclarationStatement(i) => {
                i.filter_by_node_type(node_type)
            }
        }
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            InitializationExpression::ExpressionStatement(i) => i.filter_by_reference_id(id),
            InitializationExpression::VariableDeclarationStatement(i) => {
                i.filter_by_reference_id(id)
            }
        }
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            InitializationExpression::ExpressionStatement(i) => i.filter_by_id(id),
            InitializationExpression::VariableDeclarationStatement(i) => i.filter_by_id(id),
        }
    }

    fn childrens_id(&self) -> Vec<isize> {
        match self {
            InitializationExpression::ExpressionStatement(i) => i.childrens_id(),
            InitializationExpression::VariableDeclarationStatement(i) => i.childrens_id(),
        }
    }

    fn references(&self) -> Vec<isize> {
        match self {
            InitializationExpression::ExpressionStatement(i) => i.references(),
            InitializationExpression::VariableDeclarationStatement(i) => i.references(),
        }
    }
}

impl AstVisitor for FalseBody {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        match self {
            FalseBody::Block(i) => i.filter_by_node_type(node_type),
            FalseBody::Break(i) => i.filter_by_node_type(node_type),
            FalseBody::Continue(i) => i.filter_by_node_type(node_type),
            FalseBody::DoWhileStatement(i) => i.filter_by_node_type(node_type),
            FalseBody::EmitStatement(i) => i.filter_by_node_type(node_type),
            FalseBody::ExpressionStatement(i) => i.filter_by_node_type(node_type),
            FalseBody::ForStatement(i) => i.filter_by_node_type(node_type),
            FalseBody::IfStatement(i) => i.filter_by_node_type(node_type),
            FalseBody::PlaceholderStatement(i) => i.filter_by_node_type(node_type),
            FalseBody::Return(i) => i.filter_by_node_type(node_type),
            FalseBody::RevertStatement(i) => i.filter_by_node_type(node_type),
            FalseBody::TryStatement(i) => i.filter_by_node_type(node_type),
            FalseBody::UncheckedBlock(i) => i.filter_by_node_type(node_type),
            FalseBody::VariableDeclarationStatement(i) => i.filter_by_node_type(node_type),
            FalseBody::WhileStatement(i) => i.filter_by_node_type(node_type),
        }
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            FalseBody::Block(i) => i.filter_by_reference_id(id),
            FalseBody::Break(i) => i.filter_by_reference_id(id),
            FalseBody::Continue(i) => i.filter_by_reference_id(id),
            FalseBody::DoWhileStatement(i) => i.filter_by_reference_id(id),
            FalseBody::EmitStatement(i) => i.filter_by_reference_id(id),
            FalseBody::ExpressionStatement(i) => i.filter_by_reference_id(id),
            FalseBody::ForStatement(i) => i.filter_by_reference_id(id),
            FalseBody::IfStatement(i) => i.filter_by_reference_id(id),
            FalseBody::PlaceholderStatement(i) => i.filter_by_reference_id(id),
            FalseBody::Return(i) => i.filter_by_reference_id(id),
            FalseBody::RevertStatement(i) => i.filter_by_reference_id(id),
            FalseBody::TryStatement(i) => i.filter_by_reference_id(id),
            FalseBody::UncheckedBlock(i) => i.filter_by_reference_id(id),
            FalseBody::VariableDeclarationStatement(i) => i.filter_by_reference_id(id),
            FalseBody::WhileStatement(i) => i.filter_by_reference_id(id),
        }
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            FalseBody::Block(i) => i.filter_by_id(id),
            FalseBody::Break(i) => i.filter_by_id(id),
            FalseBody::Continue(i) => i.filter_by_id(id),
            FalseBody::DoWhileStatement(i) => i.filter_by_id(id),
            FalseBody::EmitStatement(i) => i.filter_by_id(id),
            FalseBody::ExpressionStatement(i) => i.filter_by_id(id),
            FalseBody::ForStatement(i) => i.filter_by_id(id),
            FalseBody::IfStatement(i) => i.filter_by_id(id),
            FalseBody::PlaceholderStatement(i) => i.filter_by_id(id),
            FalseBody::Return(i) => i.filter_by_id(id),
            FalseBody::RevertStatement(i) => i.filter_by_id(id),
            FalseBody::TryStatement(i) => i.filter_by_id(id),
            FalseBody::UncheckedBlock(i) => i.filter_by_id(id),
            FalseBody::VariableDeclarationStatement(i) => i.filter_by_id(id),
            FalseBody::WhileStatement(i) => i.filter_by_id(id),
        }
    }

    fn childrens_id(&self) -> Vec<isize> {
        match self {
            FalseBody::Block(i) => i.childrens_id(),
            FalseBody::Break(i) => i.childrens_id(),
            FalseBody::Continue(i) => i.childrens_id(),
            FalseBody::DoWhileStatement(i) => i.childrens_id(),
            FalseBody::EmitStatement(i) => i.childrens_id(),
            FalseBody::ExpressionStatement(i) => i.childrens_id(),
            FalseBody::ForStatement(i) => i.childrens_id(),
            FalseBody::IfStatement(i) => i.childrens_id(),
            FalseBody::PlaceholderStatement(i) => i.childrens_id(),
            FalseBody::Return(i) => i.childrens_id(),
            FalseBody::RevertStatement(i) => i.childrens_id(),
            FalseBody::TryStatement(i) => i.childrens_id(),
            FalseBody::UncheckedBlock(i) => i.childrens_id(),
            FalseBody::VariableDeclarationStatement(i) => i.childrens_id(),
            FalseBody::WhileStatement(i) => i.childrens_id(),
        }
    }

    fn references(&self) -> Vec<isize> {
        match self {
            FalseBody::Block(i) => i.references(),
            FalseBody::Break(i) => i.references(),
            FalseBody::Continue(i) => i.references(),
            FalseBody::DoWhileStatement(i) => i.references(),
            FalseBody::EmitStatement(i) => i.references(),
            FalseBody::ExpressionStatement(i) => i.references(),
            FalseBody::ForStatement(i) => i.references(),
            FalseBody::IfStatement(i) => i.references(),
            FalseBody::PlaceholderStatement(i) => i.references(),
            FalseBody::Return(i) => i.references(),
            FalseBody::RevertStatement(i) => i.references(),
            FalseBody::TryStatement(i) => i.references(),
            FalseBody::UncheckedBlock(i) => i.references(),
            FalseBody::VariableDeclarationStatement(i) => i.references(),
            FalseBody::WhileStatement(i) => i.references(),
        }
    }
}

impl AstVisitor for TryCatchClause {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        let mut result = self.block().filter_by_node_type(node_type);
        result.append(&mut self.parameters().filter_by_node_type(node_type));
        if node_type == NodeType::TryCatchClause {
            result.push(NodeTypeInternal::TryCatchClause(self.clone()));
        }
        result
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.block().filter_by_reference_id(id);
        result.append(&mut self.parameters().filter_by_reference_id(id));
        result
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        let mut result = self.block().filter_by_id(id);
        result.append(&mut self.parameters().filter_by_id(id));
        if id == self.id() {
            result.push(NodeTypeInternal::TryCatchClause(self.clone()));
        }
        result
    }

    fn childrens_id(&self) -> Vec<isize> {
        let mut result = self.block().childrens_id();
        result.append(&mut self.parameters().childrens_id());

        result.push(self.id());

        result
    }

    fn references(&self) -> Vec<isize> {
        let mut result = self.block().references();
        result.append(&mut self.parameters().references());
        result
    }
}

impl AstVisitor for InlineAssembly {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        [self.ast.references(), self.external_references.references()].concat()
    }
}

impl AstVisitor for ExternalReferenceCompatible {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        match self {
            ExternalReferenceCompatible::ExternalReference(external_reference) => {
                external_reference.references()
            }
            ExternalReferenceCompatible::ExternalReferenceOld(hash_map) => {
                hash_map.values().flat_map(AstVisitor::references).collect()
            }
        }
    }
}

impl AstVisitor for ExternalReference {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        vec![self.declaration]
    }
}

impl AstVisitor for YulBlock {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        self.statements.references()
    }
}

impl AstVisitor for YulStatement {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        match self {
            YulStatement::YulAssignment(i) => i.references(),
            YulStatement::YulBlock(i) => i.references(),
            YulStatement::YulBreak(i) => i.references(),
            YulStatement::YulContinue(i) => i.references(),
            YulStatement::YulExpressionStatement(i) => i.references(),
            YulStatement::YulLeave(i) => i.references(),
            YulStatement::YulForLoop(i) => i.references(),
            YulStatement::YulFunctionDefinition(i) => i.references(),
            YulStatement::YulIf(i) => i.references(),
            YulStatement::YulSwitch(i) => i.references(),
            YulStatement::YulVariableDeclaration(i) => i.references(),
        }
    }
}

impl AstVisitor for YulVariableDeclaration {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        [self.value.references(), self.variables.references()].concat()
    }
}

impl AstVisitor for YulSwitch {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        [self.cases.references(), self.expression.references()].concat()
    }
}

impl AstVisitor for YulCase {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        [self.body.references(), self.value.references()].concat()
    }
}

impl AstVisitor for CaseValue {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        match self {
            CaseValue::Default(_) => todo!(),
            CaseValue::YulLiteral(i) => i.references(),
        }
    }
}

impl AstVisitor for YulIf {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        [self.body.references(), self.condition.references()].concat()
    }
}

impl AstVisitor for YulFunctionDefinition {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        [
            self.body.references(),
            self.parameters.references(),
            self.return_variables.references(),
        ]
        .concat()
    }
}

impl AstVisitor for YulForLoop {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        [
            self.body.references(),
            self.condition.references(),
            self.post.references(),
            self.pre.references(),
        ]
        .concat()
    }
}

impl AstVisitor for YulLeave {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        vec![]
    }
}

impl AstVisitor for YulExpressionStatement {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        self.expression.references()
    }
}

impl AstVisitor for YulContinue {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        vec![]
    }
}

impl AstVisitor for YulBreak {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        vec![]
    }
}

impl AstVisitor for YulAssignment {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        [self.value.references(), self.variable_names.references()].concat()
    }
}

impl AstVisitor for YulExpression {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        match self {
            YulExpression::YulFunctionCall(i) => i.references(),
            YulExpression::YulIdentifier(i) => i.references(),
            YulExpression::YulLiteral(i) => i.references(),
        }
    }
}

impl AstVisitor for YulIdentifier {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        vec![]
    }
}

impl AstVisitor for YulFunctionCall {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        [self.arguments.references(), self.function_name.references()].concat()
    }
}

impl AstVisitor for YulLiteral {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        match self {
            YulLiteral::YulLiteralValue(i) => i.references(),
            YulLiteral::YulLiteralHexValue(i) => i.references(),
        }
    }
}

impl AstVisitor for YulLiteralValue {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        vec![]
    }
}

impl AstVisitor for YulLiteralHexValue {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        vec![]
    }
}

impl AstVisitor for YulTypedName {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        todo!()
    }

    fn childrens_id(&self) -> Vec<isize> {
        todo!()
    }

    fn references(&self) -> Vec<isize> {
        vec![]
    }
}

/**
 *
 * @note Json Value is usually a placeholder
 *
 */
impl AstVisitor for serde_json::Value {
    fn filter_by_node_type<N: Into<NodeType>>(&self, _node_type: N) -> Vec<NodeTypeInternal> {
        vec![]
    }

    fn filter_by_reference_id(&self, _id: isize) -> Vec<NodeTypeInternal> {
        vec![]
    }

    fn filter_by_id(&self, _id: isize) -> Vec<NodeTypeInternal> {
        vec![]
    }

    fn childrens_id(&self) -> Vec<isize> {
        vec![]
    }

    fn references(&self) -> Vec<isize> {
        vec![]
    }
}

impl<T: AstVisitor> AstVisitor for Option<T> {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        match self {
            Some(t) => t.filter_by_node_type(node_type),
            None => vec![],
        }
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            Some(t) => t.filter_by_reference_id(id),
            None => vec![],
        }
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        match self {
            Some(t) => t.filter_by_id(id),
            None => vec![],
        }
    }

    fn childrens_id(&self) -> Vec<isize> {
        match self {
            Some(t) => t.childrens_id(),
            None => vec![],
        }
    }

    fn references(&self) -> Vec<isize> {
        match self {
            Some(t) => t.references(),
            None => vec![],
        }
    }
}

impl<T: AstVisitor> AstVisitor for Vec<T> {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        self.iter()
            .flat_map(|node| node.filter_by_node_type(node_type))
            .collect()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.iter()
            .flat_map(|node| node.filter_by_reference_id(id))
            .collect()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.iter().flat_map(|node| node.filter_by_id(id)).collect()
    }

    fn childrens_id(&self) -> Vec<isize> {
        self.iter().flat_map(|node| node.childrens_id()).collect()
    }

    fn references(&self) -> Vec<isize> {
        self.iter().flat_map(|node| node.references()).collect()
    }
}

impl<T: AstVisitor> AstVisitor for &[T] {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        self.iter()
            .flat_map(|node| node.filter_by_node_type(node_type))
            .collect()
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.iter()
            .flat_map(|node| node.filter_by_reference_id(id))
            .collect()
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.iter().flat_map(|node| node.filter_by_id(id)).collect()
    }

    fn childrens_id(&self) -> Vec<isize> {
        self.iter().flat_map(|node| node.childrens_id()).collect()
    }

    fn references(&self) -> Vec<isize> {
        self.iter().flat_map(|node| node.references()).collect()
    }
}

impl<T: AstVisitor> AstVisitor for &T {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        (*self).filter_by_node_type(node_type)
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        (*self).filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        (*self).filter_by_id(id)
    }

    fn childrens_id(&self) -> Vec<isize> {
        (*self).childrens_id()
    }

    fn references(&self) -> Vec<isize> {
        (*self).references()
    }
}

impl<T: AstVisitor> AstVisitor for Box<T> {
    fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
        let node_type: NodeType = node_type.into();

        self.as_ref().filter_by_node_type(node_type)
    }

    fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.as_ref().filter_by_reference_id(id)
    }

    fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
        self.as_ref().filter_by_id(id)
    }

    fn childrens_id(&self) -> Vec<isize> {
        self.as_ref().childrens_id()
    }

    fn references(&self) -> Vec<isize> {
        self.as_ref().references()
    }
}

//TODO: Check that all Option, Vec and &[] use bind instead of repeating the code
