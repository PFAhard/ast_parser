pub mod yul_block;
pub mod yul_statement;
pub mod yul_assignment;
pub mod yul_break;
pub mod yul_continue;
pub mod yul_expression_statement;
pub mod yul_leave;
pub mod yul_for_loop;
pub mod yul_function_definition;
pub mod yul_if;
pub mod yul_switch;
pub mod yul_variable_declaration;

use serde::{Deserialize, Serialize};
use yul_block::YulBlock;

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineAssembly {
    #[serde(rename = "AST")]
    pub ast: YulBlock,
    pub documentation: Option<String>,
    #[serde(rename = "evmVersion")]
    pub evm_version: EvmVersion,
    #[serde(rename = "externalReferences")]
    pub external_references: Vec<ExternalReference>,
    pub flags: Option<Vec<String>>, // Assuming "memory-safe" is a string in a vector
    pub id: isize,
    pub src: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum Suffix {
    #[serde(rename = "offset")]
    Offset,
    #[serde(rename = "slot")]
    Slot,
}
