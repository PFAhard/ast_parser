use getters::Getters;
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
            TypeName::ArrayTypeName(at_name) => {
                if at_name.length().is_some() {
                    todo!();
                }
                format!("{}[]", at_name.base_type().name())
            }
            TypeName::ElementaryTypeName(elt_name) => elt_name.name(),
            TypeName::FunctionTypeName(ft_name) => ft_name.name(),
            TypeName::Mapping(mapping) => mapping.name(),
            TypeName::UserDefinedTypeName(udt_name) => udt_name.name(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct ArrayTypeName {
    #[serde(rename = "baseType")]
    #[use_as_ref]
    #[return_type = "&TypeName"]
    base_type: Box<TypeName>,
    #[copy]
    id: isize,
    #[skip_getter]
    length: Option<Box<Expression>>,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl ArrayTypeName {
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

    pub fn name(&self) -> String {
        let mut name = "function".to_owned();

        let params: Vec<String> = self
            .parameter_types()
            .parameters()
            .iter()
            .map(|x| x.type_name().as_ref().unwrap().name().to_string())
            .collect();

        let mut params = format!("{:?}", params);
        params.replace_range(0..1, "(");
        params.replace_range(params.len() - 1..params.len(), ")");
        name.push_str(params.as_str());

        let returns: Vec<String> = self
            .return_parameter_types()
            .parameters()
            .iter()
            .map(|x| x.type_name().as_ref().unwrap().name().to_string())
            .collect();
        if !returns.is_empty() {
            let mut returns = format!("{:?}", returns);
            returns.replace_range(0..1, "(");
            returns.replace_range(returns.len() - 1..returns.len(), ")");
            name.push_str("returns");
            name.push_str(returns.as_str());
        }

        name
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

    pub fn name(&self) -> String {
        format!(
            "mapping({} => {})",
            self.key_type().name(),
            self.value_type().name()
        )
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

    pub fn name(&self) -> String {
        if let Some(name) = self.path_node() {
            name.name().to_owned()
        } else {
            unimplemented!()
        }
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
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn id(&self) -> isize {
        self.id
    }
}
