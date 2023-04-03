mod event_definition;
mod modifier_definition;
mod prelude;

use serde::Deserialize;

pub(crate) use prelude::*;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub(crate) enum BaseNode {
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
