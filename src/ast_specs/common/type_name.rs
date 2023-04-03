use serde::Deserialize;

use crate::ast_parser::ast_specs::Expression;

use super::{
    identifier_path::IdentifierPath, parameter_list::ParameterList,
    type_descriptions::TypeDescriptions, StateMutability, Visibility,
};

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub(crate) enum TypeName {
    ArrayTypeName(ArrayTypeName),
    ElementaryTypeName(ElementaryTypeName),
    FunctionTypeName(FunctionTypeName),
    Mapping(Mapping),
    UserDefinedTypeName(UserDefinedTypeName),
}

impl TypeName {
    pub(crate) fn name(&self) -> String {
        match self {
            TypeName::ArrayTypeName(array_type_name) => array_type_name.name(),
            TypeName::ElementaryTypeName(elementary_type_name) => {
                elementary_type_name.name().to_owned()
            }
            _ => unimplemented!("{:?}", self),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ArrayTypeName {
    #[serde(rename = "baseType")]
    base_type: Box<TypeName>,
    id: isize,
    length: Option<Box<Expression>>,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl ArrayTypeName {
    pub(crate) fn name(&self) -> String {
        format!("{}[]", self.base_type().name())
    }

    pub(crate) fn base_type(&self) -> &TypeName {
        self.base_type.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }

    pub(crate) fn length(&self) -> Option<&Expression> {
        self.length.as_deref()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct FunctionTypeName {
    id: isize,
    #[serde(rename = "parameterTypes")]
    parameter_types: ParameterList,
    #[serde(rename = "returnParameterTypes")]
    return_parameter_types: ParameterList,
    src: String,
    #[serde(rename = "stateMutability")]
    state_mutability: StateMutability,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    visibility: Visibility,
}

impl FunctionTypeName {
    pub(crate) fn id(&self) -> isize {
        self.id
    }

    pub(crate) fn parameter_types(&self) -> &ParameterList {
        &self.parameter_types
    }

    pub(crate) fn return_parameter_types(&self) -> &ParameterList {
        &self.return_parameter_types
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Mapping {
    id: isize,
    #[serde(rename = "keyName")]
    key_name: Option<String>,
    #[serde(rename = "keyNameLocation")]
    key_name_location: Option<String>,
    #[serde(rename = "keyType")]
    key_type: Box<TypeName>,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "valueName")]
    value_name: Option<String>,
    #[serde(rename = "valueNameLocation")]
    value_name_location: Option<String>,
    #[serde(rename = "valueType")]
    value_type: Box<TypeName>,
}

impl Mapping {
    pub(crate) fn id(&self) -> isize {
        self.id
    }

    pub(crate) fn key_type(&self) -> &TypeName {
        self.key_type.as_ref()
    }

    pub(crate) fn value_type(&self) -> &TypeName {
        self.value_type.as_ref()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct UserDefinedTypeName {
    // #[serde(skip)]
    // #[serde(rename = "contractScope")]
    // contract_scope: (), // @note never seen
    id: isize,
    name: Option<String>,
    #[serde(rename = "pathNode")]
    path_node: Option<IdentifierPath>,
    #[serde(rename = "referencedDeclaration")]
    referenced_declaration: isize,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl UserDefinedTypeName {
    pub(crate) fn id(&self) -> isize {
        self.id
    }

    pub(crate) fn path_node(&self) -> Option<&IdentifierPath> {
        self.path_node.as_ref()
    }

    pub(crate) fn referenced_declaration(&self) -> isize {
        self.referenced_declaration
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ElementaryTypeName {
    id: isize,
    name: String,
    src: String,
    #[serde(rename = "stateMutability")]
    state_mutability: Option<StateMutability>,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl ElementaryTypeName {
    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}
