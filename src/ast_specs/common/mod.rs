use std::fmt::Display;

use serde::{Deserialize, Serialize};

mod block;
mod identifier_path;
mod inheritance_specifier;
mod modifier_invocation;
mod override_specifier;
mod parameter_list;
mod prelude;
mod structured_documentation;
mod type_descriptions;
mod type_name;

pub use prelude::*;

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(tag = "nodeType")]
pub enum BaseName {
    UserDefinedTypeName(UserDefinedTypeName),
    IdentifierPath(IdentifierPath),
    #[default]
    Fallback,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub enum LibraryName {
    UserDefinedTypeName(UserDefinedTypeName),
    IdentifierPath(IdentifierPath),
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mutability {
    #[serde(rename = "mutable")]
    Mutable,
    #[serde(rename = "immutable")]
    Immutable,
    #[serde(rename = "constant")]
    Constant,
}

#[derive(Deserialize, Debug, Clone)]
pub enum StorageLocation {
    #[serde(rename = "calldata")]
    Calldata,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "memory")]
    Memory,
    #[serde(rename = "storage")]
    Storage,
}

// #[derive(Deserialize, Debug, Clone)] TODO:
// pub enum FunctionList {
//     function {}
// }

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionCallKind {
    #[serde(rename = "functionCall")]
    FunctionCall,
    #[serde(rename = "typeConversion")]
    TypeConversion,
    #[serde(rename = "structConstructorCall")]
    StructConstructorCall,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum LiteralKind {
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "hexString")]
    HexString,
    #[serde(rename = "unicodeString")]
    UnicodeString,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateMutability {
    #[serde(rename = "payable")]
    Payable,
    #[serde(rename = "pure")]
    Pure,
    #[serde(rename = "nonpayable")]
    Nonpayable,
    #[serde(rename = "view")]
    View,
}

impl std::fmt::Display for StateMutability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateMutability::Payable => write!(f, "payable"),
            StateMutability::Pure => write!(f, "pure"),
            StateMutability::Nonpayable => write!(f, "nonpayable"),
            StateMutability::View => write!(f, "view"),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    #[serde(rename = "external")]
    External,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "private")]
    Private,
}

impl Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::External => f.write_str("external"),
            Visibility::Public => f.write_str("public"),
            Visibility::Internal => f.write_str("internal"),
            Visibility::Private => f.write_str("private"),
        }
    }
}
