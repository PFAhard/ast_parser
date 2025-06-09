#![warn(clippy::all)]
use std::{collections::HashMap, fmt::Debug};

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
    CompatabilityTypeName, NodeTypeInternalRef,
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

    fn step_back<'a, 'b, N: Into<NodeTypeInternalRef<'a>> + Copy>(
        &'b self,
        target: N,
    ) -> Option<NodeTypeInternalRef<'b>>;

    fn is_node<'a, N: Into<NodeTypeInternalRef<'a>> + Copy>(&self, target: N) -> bool;
}

macro_rules! ast_visitor {
    (
        $(
            $(#[no_id=$no_id:literal])? $(#[has_refs=$has_refs:literal])? $target:ident: [
                $(
                    $inner:ident
                ),*
            ];
        )*
    ) => {
        $(
            #[allow(unused_mut)]
            impl AstVisitor for $target {
                fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {
                    let node_type: NodeType = node_type.into();

                    let mut result = vec![];
                    $(
                        result.append(&mut self.$inner().filter_by_node_type(node_type));
                    )*
                    if node_type == NodeType::$target {
                        result.push(NodeTypeInternal::$target(self.clone()));
                    }
                    result
                }

                fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
                    let mut result = vec![];
                    $(
                        result.append(&mut self.$inner().filter_by_reference_id(id));
                    )*
                    $(
                        #[cfg($has_refs)]
                        if let Some(ref_dec) = self.ref_dec_visitor(){
                            if id == ref_dec {
                                result.push(NodeTypeInternal::$target(self.clone()));
                            }
                        }
                    )?
                    result
                }

                fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
                    let mut result = vec![];
                    $(
                        result.append(&mut self.$inner().filter_by_id(id));
                    )*
                    $(#[cfg(not($no_id))])?
                    if id == self.id() {
                        result.push(NodeTypeInternal::$target(self.clone()));
                    }
                    result
                }

                fn childrens_id(&self) -> Vec<isize> {
                    let mut result = vec![];
                    $(
                        result.append(&mut self.$inner().childrens_id());
                    )*
                    $(#[cfg(not($no_id))])?
                    result.push(self.id());
                    result
                }

                fn references(&self) -> Vec<isize> {
                    let mut result = vec![];
                    $(
                        result.append(&mut self.$inner().references());
                    )*
                    $(
                        #[cfg($has_refs)]
                        if let Some(ref_dec) = self.ref_dec_visitor(){
                            result.push(ref_dec);
                        }
                    )?
                    result
                }

                fn step_back<'a, 'b, N: Into<NodeTypeInternalRef<'a>> + Copy>(
                    &'b self,
                    target: N,
                ) -> Option<NodeTypeInternalRef<'b>> {
                    $(
                        if self.$inner().is_node(target) {
                            return Some(self.into());
                        }
                    )else*
                    $(
                        {
                            let bind = self.$inner();
                            let tmp_x = bind.step_back(target);
                            if tmp_x.is_some() {
                                // TODO: USAGE OF BLACK MAGIC IS ILLIGAL
                                let tmp_b: Option<NodeTypeInternalRef<'b>> = unsafe {
                                    std::mem::transmute::<Option<NodeTypeInternalRef<'_>>, Option<NodeTypeInternalRef<'b>>>(
                                        tmp_x,
                                    )
                                };

                                return tmp_b;
                            }
                        }
                    )*

                    None
                }

                fn is_node<'a, N: Into<NodeTypeInternalRef<'a>> + Copy>(&self, target: N) -> bool {
                    let target_node = target.into();
                    let self_node = NodeTypeInternalRef::from(self);
                    target_node == self_node
                }
            }
        )*
    };
    (
        $(
            $target:ident: (
                $(
                    $variant:ident
                ),*
            );
        )*
    ) => {
        $(
            impl AstVisitor for $target {
                fn filter_by_node_type<N: Into<NodeType>>(
                    &self,
                    node_type: N,
                ) -> Vec<NodeTypeInternal> {
                    let node_type: NodeType = node_type.into();

                    match self {
                        $(
                            $target::$variant(i) => i.filter_by_node_type(node_type),
                        )*
                    }
                }

                fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {
                    match self {
                        $(
                            $target::$variant(i) => i.filter_by_reference_id(id),
                        )*
                    }
                }

                fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {
                    match self {
                        $(
                            $target::$variant(i) => i.filter_by_id(id),
                        )*
                    }
                }

                fn childrens_id(&self) -> Vec<isize> {
                    match self {
                        $(
                            $target::$variant(i) => i.childrens_id(),
                        )*
                    }
                }

                fn references(&self) -> Vec<isize> {
                    match self {
                        $(
                            $target::$variant(i) => i.references(),
                        )*
                    }
                }

                fn step_back<'a, 'b, N: Into<NodeTypeInternalRef<'a>> + Copy>(
                    &'b self,
                    target: N,
                ) -> Option<NodeTypeInternalRef<'b>> {
                    match self {
                        $(
                            $target::$variant(i) =>  i.step_back(target),
                        )*
                    }
                }

                fn is_node<'a, N: Into<NodeTypeInternalRef<'a>> + Copy>(&self, target: N) -> bool {
                    match self {
                        $(
                            $target::$variant(i) =>  i.is_node(target),
                        )*
                    }
                }
            }
        )*
    };
    (!!!placeholder!!! $target: ident) => {
        impl AstVisitor for $target {
            fn filter_by_node_type<N: Into<NodeType>>(&self, node_type: N) -> Vec<NodeTypeInternal> {vec![]}

            fn filter_by_reference_id(&self, id: isize) -> Vec<NodeTypeInternal> {vec![]}

            fn filter_by_id(&self, id: isize) -> Vec<NodeTypeInternal> {vec![]}

            fn childrens_id(&self) -> Vec<isize> {vec![]}

            fn references(&self) -> Vec<isize> {vec![]}

            fn step_back<'a, 'b, N: Into<NodeTypeInternalRef<'a>> + Copy>(
                    &'b self,
                    target: N,
                ) -> Option<NodeTypeInternalRef<'b>> {
                None
            }

            fn is_node<'a, N: Into<NodeTypeInternalRef<'a>> + Copy>(&self, target: N) -> bool {false}
        }
    };
}

ast_visitor!(!!!placeholder!!! String);
pub type TMP = HashMap<String, ExternalReference>;
ast_visitor!(!!!placeholder!!! TMP);

ast_visitor! {
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
    CompatabilityTypeName: (
        ElementaryTypeName, Name
    );
    InitializationExpression: (
        ExpressionStatement, VariableDeclarationStatement
    );
    FalseBody: (
        Block, Break, Continue, DoWhileStatement, EmitStatement, ExpressionStatement, ForStatement, IfStatement, PlaceholderStatement, Return, RevertStatement, TryStatement, UncheckedBlock, VariableDeclarationStatement, WhileStatement
    );
    ExternalReferenceCompatible: (
        ExternalReference, ExternalReferenceOld
    );
    YulStatement: (
        YulAssignment, YulBlock, YulBreak, YulContinue, YulExpressionStatement, YulLeave, YulForLoop, YulFunctionDefinition, YulIf, YulSwitch, YulVariableDeclaration
    );
    CaseValue: (
        Default, YulLiteral
    );
    YulExpression: (
        YulFunctionCall, YulIdentifier, YulLiteral
    );
    YulLiteral: (
        YulLiteralValue, YulLiteralHexValue
    );
}

ast_visitor! {
    SourceUnit: [nodes];
    ContractDefinition: [nodes];
    EnumDefinition: [members];
    ErrorDefinition: [parameters];
    FunctionDefinition: [body, modifiers, overrides, parameters, return_parameters];
    ImportDirective: [symbol_aliases];
    #[no_id=true] SymbolAliases: [foreign];
    PragmaDirective: [];
    StructDefinition: [members];
    UserDefinedValueTypeDefinition: [underlying_type];
    UsingForDirective: [function_list, library_name, type_name];
    VariableDeclaration: [value, type_name, overrides];
    EventDefinition: [parameters];
    ModifierDefinition: [body, overrides, parameters];
    EnumValue: [];
    ParameterList: [parameters];
    Block: [statements];
    Break: [];
    Continue: [];
    DoWhileStatement: [body, condition];
    EmitStatement: [event_call];
    ExpressionStatement: [expression];
    ForStatement: [body, condition, initialization_expression, loop_expression];
    IfStatement: [condition, false_body, true_body];
    PlaceholderStatement: [];
    Return: [expression];
    RevertStatement: [error_call];
    TryStatement: [clauses, external_call];
    UncheckedBlock: [statements];
    VariableDeclarationStatement: [initial_value, declarations];
    WhileStatement: [body, condition];
    ModifierInvocation: [arguments, modifier_name];
    OverrideSpecifier: [overrides];
    ArrayTypeName: [base_type, length];
    ElementaryTypeName: [];
    FunctionTypeName: [parameter_types, return_parameter_types];
    Mapping: [key_type, value_type];
    #[has_refs=true] UserDefinedTypeName: [path_node];
    #[has_refs=true] Identifier: [argument_types];
    #[no_id=true] TypeDescriptions: [];
    #[has_refs=true] IdentifierPath: [];
    Assignment: [left_hand_side, right_hand_side, argument_types];
    BinaryOperation: [left_expression, right_expression, argument_types];
    Conditional: [argument_types, condition, true_expression, false_expression];
    ElementaryTypeNameExpression: [argument_types, type_name];
    FunctionCall: [arguments, expression, argument_types];
    FunctionCallOptions: [options, expression, argument_types];
    IndexAccess: [base_expression, index_expression, argument_types];
    IndexRangeAccess: [base_expression, start_expression, end_expression, argument_types];
    Literal: [argument_types];
    MemberAccess: [argument_types, expression];
    NewExpression: [argument_types, type_name];
    TupleExpression: [argument_types, components];
    UnaryOperation: [argument_types, sub_expression];
    TryCatchClause: [block, parameters];
    InlineAssembly: [ast, external_references];
    #[no_id=true] ExternalReference: [];
    #[no_id=true] YulBlock: [statements];
    #[no_id=true] YulVariableDeclaration: [value, variables];
    #[no_id=true] YulSwitch: [cases, expression];
    #[no_id=true] YulCase: [body, value];
    #[no_id=true] YulIf: [body, condition];
    #[no_id=true] YulFunctionDefinition: [body, parameters, return_variables];
    #[no_id=true] YulForLoop: [body, condition, post, pre];
    #[no_id=true] YulLeave: [];
    #[no_id=true] YulExpressionStatement: [expression];
    #[no_id=true] YulContinue: [];
    #[no_id=true] YulBreak: [];
    #[no_id=true] YulAssignment: [value, variable_names];
    #[no_id=true] YulIdentifier: [];
    #[no_id=true] YulFunctionCall: [arguments, function_name];
    #[no_id=true] YulLiteralValue: [];
    #[no_id=true] YulLiteralHexValue: [];
    #[no_id=true] YulTypedName: [];
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

    fn step_back<'a, 'b, N: Into<NodeTypeInternalRef<'a>> + Copy>(
        &'b self,
        target: N,
    ) -> Option<NodeTypeInternalRef<'b>> {
        None
    }

    fn is_node<'a, N: Into<NodeTypeInternalRef<'a>> + Copy>(&self, target: N) -> bool {
        false
    }
}

impl<T: AstVisitor + Debug> AstVisitor for Option<T> {
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

    fn step_back<'a, 'b, N: Into<NodeTypeInternalRef<'a>> + Copy>(
        &'b self,
        target: N,
    ) -> Option<NodeTypeInternalRef<'b>> {
        match self {
            Some(t) => t.step_back(target),
            None => None,
        }
    }

    fn is_node<'a, N: Into<NodeTypeInternalRef<'a>> + Copy>(&self, target: N) -> bool {
        match self {
            Some(t) => t.is_node(target),
            None => false,
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

    fn step_back<'a, 'b, N: Into<NodeTypeInternalRef<'a>> + Copy>(
        &'b self,
        target: N,
    ) -> Option<NodeTypeInternalRef<'b>> {
        self.iter().find_map(|node| node.step_back(target))
    }

    fn is_node<'a, N: Into<NodeTypeInternalRef<'a>> + Copy>(&self, target: N) -> bool {
        self.iter().any(|node| node.is_node(target))
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

    fn step_back<'a, 'b, N: Into<NodeTypeInternalRef<'a>> + Copy>(
        &'b self,
        target: N,
    ) -> Option<NodeTypeInternalRef<'b>> {
        self.iter().find_map(|node| node.step_back(target))
    }

    fn is_node<'a, N: Into<NodeTypeInternalRef<'a>> + Copy>(&self, target: N) -> bool {
        self.iter().any(|node| node.is_node(target))
    }
}

impl<T: AstVisitor> AstVisitor for [T] {
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

    fn step_back<'a, 'b, N: Into<NodeTypeInternalRef<'a>> + Copy>(
        &'b self,
        target: N,
    ) -> Option<NodeTypeInternalRef<'b>> {
        self.iter().find_map(|node| node.step_back(target))
    }

    fn is_node<'a, N: Into<NodeTypeInternalRef<'a>> + Copy>(&self, target: N) -> bool {
        self.iter().any(|node| node.is_node(target))
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

    fn step_back<'a, 'b, N: Into<NodeTypeInternalRef<'a>> + Copy>(
        &'b self,
        target: N,
    ) -> Option<NodeTypeInternalRef<'b>> {
        (*self).step_back(target)
    }

    fn is_node<'a, N: Into<NodeTypeInternalRef<'a>> + Copy>(&self, target: N) -> bool {
        (*self).is_node(target)
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

    fn step_back<'a, 'b, N: Into<NodeTypeInternalRef<'a>> + Copy>(
        &'b self,
        target: N,
    ) -> Option<NodeTypeInternalRef<'b>> {
        self.as_ref().step_back(target)
    }

    fn is_node<'a, N: Into<NodeTypeInternalRef<'a>> + Copy>(&self, target: N) -> bool {
        self.as_ref().is_node(target)
    }
}

//TODO: Check that all Option, Vec and &[] use bind instead of repeating the code
