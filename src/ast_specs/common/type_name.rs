use serde::Deserialize;

use crate::ast_specs::Expression;

use super::{
    identifier_path::IdentifierPath, parameter_list::ParameterList,
    type_descriptions::TypeDescriptions, StateMutability, Visibility,
};

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub enum TypeName {
    ArrayTypeName(ArrayTypeName),
    ElementaryTypeName(ElementaryTypeName),
    FunctionTypeName(FunctionTypeName),
    Mapping(Mapping),
    UserDefinedTypeName(UserDefinedTypeName),
}

impl TypeName {
    pub fn name(&self) -> String {
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
pub struct ArrayTypeName {
    #[serde(rename = "baseType")]
    base_type: Box<TypeName>,
    id: isize,
    length: Option<Box<Expression>>,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl ArrayTypeName {
    pub fn name(&self) -> String {
        format!("{}[]", self.base_type().name())
    }

    pub fn base_type(&self) -> &TypeName {
        self.base_type.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }

    pub fn length(&self) -> Option<&Expression> {
        self.length.as_deref()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct FunctionTypeName {
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
    pub fn id(&self) -> isize {
        self.id
    }

    pub fn parameter_types(&self) -> &ParameterList {
        &self.parameter_types
    }

    pub fn return_parameter_types(&self) -> &ParameterList {
        &self.return_parameter_types
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Mapping {
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
    pub fn id(&self) -> isize {
        self.id
    }

    pub fn key_type(&self) -> &TypeName {
        self.key_type.as_ref()
    }

    pub fn value_type(&self) -> &TypeName {
        self.value_type.as_ref()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserDefinedTypeName {
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
    pub fn id(&self) -> isize {
        self.id
    }

    pub fn path_node(&self) -> Option<&IdentifierPath> {
        self.path_node.as_ref()
    }

    pub fn referenced_declaration(&self) -> isize {
        self.referenced_declaration
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ElementaryTypeName {
    id: isize,
    name: String,
    src: String,
    #[serde(rename = "stateMutability")]
    state_mutability: Option<StateMutability>,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl ElementaryTypeName {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }
}
