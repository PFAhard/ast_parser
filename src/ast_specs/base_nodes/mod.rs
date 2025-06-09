mod event_definition;
mod modifier_definition;
mod prelude;

use serde::Deserialize;

pub use prelude::*;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "nodeType")]
pub enum BaseNode {
    EnumDefinition(EnumDefinition),
    ErrorDefinition(ErrorDefinition),
    FunctionDefinition(FunctionDefinition),
    StructDefinition(StructDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
    EventDefinition(EventDefinition),
    ModifierDefinition(ModifierDefinition),
}
