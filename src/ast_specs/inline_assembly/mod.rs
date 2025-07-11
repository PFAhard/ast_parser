pub mod yul_expression;
pub mod yul_statements;
pub mod yul_typed_name;

use std::collections::HashMap;

use getters::Getters;
use serde::{Deserialize, Serialize};
use yul_statements::yul_block::YulBlock;

#[derive(Debug, Serialize, Deserialize, Clone, Getters, PartialEq, Eq)]
pub struct InlineAssembly {
    #[serde(rename = "AST")]
    pub ast: Option<YulBlock>,
    pub documentation: Option<String>,
    #[serde(rename = "evmVersion")]
    pub evm_version: Option<EvmVersion>,
    #[serde(rename = "externalReferences")]
    pub external_references: Vec<ExternalReferenceCompatible>,
    pub flags: Option<Vec<String>>, // Assuming "memory-safe" is a string in a vector
    #[copy]
    pub id: isize,
    pub src: String,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum EvmVersion {
    #[serde(rename = "homestead")]
    Homestead,
    #[serde(rename = "tangerineWhistle")]
    TangerineWhistle,
    #[serde(rename = "spuriousDragon")]
    SpuriousDragon,
    #[serde(rename = "byzantium")]
    Byzantium,
    #[serde(rename = "constantinople")]
    Constantinople,
    #[serde(rename = "petersburg")]
    Petersburg,
    #[serde(rename = "istanbul")]
    Istanbul,
    #[serde(rename = "berlin")]
    Berlin,
    #[serde(rename = "london")]
    London,
    #[serde(rename = "paris")]
    Paris,
    #[serde(rename = "shanghai")]
    Shanghai,
    #[serde(rename = "cancun")]
    Cancun,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, PartialEq, Eq)]
pub struct ExternalReference {
    pub declaration: isize,
    #[serde(rename = "isOffset")]
    pub is_offset: bool,
    #[serde(rename = "isSlot")]
    pub is_slot: bool,
    pub src: String,
    pub suffix: Option<Suffix>,
    #[serde(rename = "valueSize")]
    pub value_size: i32,
}

pub type ExternalReferenceOld = HashMap<String, ExternalReference>;

#[derive(Debug, Serialize, Deserialize, Clone, Getters, PartialEq, Eq)]
#[serde(untagged)]
pub enum ExternalReferenceCompatible {
    ExternalReference(ExternalReference),
    ExternalReferenceOld(ExternalReferenceOld),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Suffix {
    #[serde(rename = "offset")]
    Offset,
    #[serde(rename = "slot")]
    Slot,
    #[serde(rename = "length")]
    Length,
}
