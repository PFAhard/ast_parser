use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::Expression;

use super::{
    identifier_path::IdentifierPath, parameter_list::ParameterList,
    type_descriptions::TypeDescriptions, StateMutability, Visibility,
};

macro_rules! impl_type_conversion {
    ($variant:ident) => {
        impl From<$variant> for TypeName {
            fn from(v: $variant) -> Self {
                TypeName::$variant(v)
            }
        }

        impl TryFrom<TypeName> for $variant {
            type Error = &'static str;

            fn try_from(value: TypeName) -> Result<Self, Self::Error> {
                match value {
                    TypeName::$variant(v) => Ok(v),
                    _ => Err("Invalid variant type"),
                }
            }
        }
    };
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub enum TypeName {
    ArrayTypeName(ArrayTypeName),
    ElementaryTypeName(ElementaryTypeName),
    FunctionTypeName(FunctionTypeName),
    Mapping(Mapping),
    UserDefinedTypeName(UserDefinedTypeName),
}

impl_type_conversion!(ArrayTypeName);
impl_type_conversion!(ElementaryTypeName);
impl_type_conversion!(FunctionTypeName);
impl_type_conversion!(Mapping);
impl_type_conversion!(UserDefinedTypeName);

impl TypeName {
    pub fn name(&self) -> String {
        match self {
            TypeName::ArrayTypeName(at_name) => {
                if at_name.length().is_some() {
                    todo!();
                }
                format!("{}[]", at_name.base_type().name())
            }
            TypeName::ElementaryTypeName(elt_name) => elt_name.name().to_owned(),
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

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct FunctionTypeName {
    #[copy]
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

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct Mapping {
    #[copy]
    id: isize,
    #[serde(rename = "keyName")]
    #[use_as_deref]
    #[return_type = "Option<&str>"]
    key_name: Option<String>,
    #[serde(rename = "keyNameLocation")]
    key_name_location: Option<String>,
    #[serde(rename = "keyType")]
    #[return_type = "&TypeName"]
    key_type: Box<TypeName>,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "valueName")]
    #[use_as_deref]
    #[return_type = "Option<&str>"]
    value_name: Option<String>,
    #[serde(rename = "valueNameLocation")]
    value_name_location: Option<String>,
    #[serde(rename = "valueType")]
    #[return_type = "&TypeName"]
    value_type: Box<TypeName>,
}

impl Mapping {
    pub fn name(&self) -> String {
        format!(
            "mapping({} => {})",
            self.key_type().name(),
            self.value_type().name()
        )
    }
}

#[derive(Deserialize, Debug, Clone, Getters, Default)]
pub struct UserDefinedTypeName {
    // #[serde(skip)]
    // #[serde(rename = "contractScope")]
    // contract_scope: (), // @note never seen
    #[copy]
    id: isize,
    #[skip_getter]
    name: Option<String>,
    #[serde(rename = "pathNode")]
    #[use_as_ref]
    #[return_type = "Option<&IdentifierPath>"]
    path_node: Option<IdentifierPath>,
    #[serde(rename = "referencedDeclaration")]
    #[copy]
    referenced_declaration: isize,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl UserDefinedTypeName {
    pub fn name(&self) -> String {
        if let Some(name) = self.path_node() {
            name.name().to_owned()
        } else {
            unimplemented!()
        }
    }

    pub fn ref_dec_visitor(&self) -> Option<isize> {
        Some(self.referenced_declaration)
    }
}

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct ElementaryTypeName {
    #[copy]
    id: isize,
    #[return_type = "&str"]
    name: String,
    src: String,
    #[serde(rename = "stateMutability")]
    state_mutability: Option<StateMutability>,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}
