use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineAssembly {
    pub AST: YulBlock,
    pub documentation: Option<String>,
    pub evmVersion: EvmVersion,
    pub externalReferences: Vec<ExternalReference>,
    pub flags: Option<Vec<String>>, // Assuming "memory-safe" is a string in a vector
    pub id: isize,
    pub src: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EvmVersion {
    Homestead,
    TangerineWhistle,
    SpuriousDragon,
    Byzantium,
    Constantinople,
    Petersburg,
    Istanbul,
    Berlin,
    London,
    Paris,
    Shanghai,
    Cancun,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalReference {
    pub declaration: isize,
    pub isOffset: bool,
    pub isSlot: bool,
    pub src: String,
    pub suffix: Option<Suffix>,
    pub valueSize: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Suffix {
    Offset,
    Slot,
}
