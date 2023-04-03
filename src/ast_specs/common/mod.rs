use serde::Deserialize;

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

pub(crate) use prelude::*;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub(crate) enum BaseName {
    UserDefinedTypeName(UserDefinedTypeName),
    IdentifierPath(IdentifierPath),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub(crate) enum LibraryName {
    UserDefinedTypeName(UserDefinedTypeName),
    IdentifierPath(IdentifierPath),
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) enum Mutability {
    #[serde(rename = "mutable")]
    Mutable,
    #[serde(rename = "immutable")]
    Immutable,
    #[serde(rename = "constant")]
    Constant,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) enum StorageLocation {
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
// pub(crate) enum FunctionList {
//     function {}
// }

#[derive(Deserialize, Debug, Clone)]
pub(crate) enum FunctionCallKind {
    #[serde(rename = "functionCall")]
    FunctionCall,
    #[serde(rename = "typeConversion")]
    TypeConversion,
    #[serde(rename = "structConstructorCall")]
    StructConstructorCall,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) enum LiteralKind {
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

#[derive(Deserialize, Debug, Clone)]
pub(crate) enum StateMutability {
    #[serde(rename = "payable")]
    Payable,
    #[serde(rename = "pure")]
    Pure,
    #[serde(rename = "nonpayable")]
    Nonpayable,
    #[serde(rename = "view")]
    View,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) enum Visibility {
    #[serde(rename = "external")]
    External,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "private")]
    Private,
}
