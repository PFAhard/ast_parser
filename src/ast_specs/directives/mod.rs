mod contract_definition;
mod enum_definition;
mod error_definition;
mod function_definition;
mod import_directive;
mod pragma_directive;
mod prelude;
mod struct_definition;
mod user_defined_value_type_definition;
mod using_for_direcrive;
mod variable_declaration;

pub use prelude::*;
use serde::Deserialize;

use super::EventDefinition;

pub type Directives = Vec<Directive>;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "nodeType")]
pub enum Directive {
    EventDefinition(EventDefinition),
    ContractDefinition(ContractDefinition),
    EnumDefinition(EnumDefinition),
    ErrorDefinition(ErrorDefinition),
    FunctionDefinition(FunctionDefinition),
    ImportDirective(ImportDirective),
    PragmaDirective(PragmaDirective),
    StructDefinition(StructDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
}

macro_rules! into_enum {
    ($enum:ty; $variant:ident) => {
        impl From<$variant> for $enum {
            fn from(value: $variant) -> Self {
                Self::$variant(value)
            }
        }
    };
}

into_enum!(Directive; PragmaDirective);
into_enum!(Directive; EventDefinition);
into_enum!(Directive; ContractDefinition);
into_enum!(Directive; EnumDefinition);
into_enum!(Directive; ErrorDefinition);
into_enum!(Directive; FunctionDefinition);
into_enum!(Directive; ImportDirective);
into_enum!(Directive; StructDefinition);
into_enum!(Directive; UserDefinedValueTypeDefinition);
into_enum!(Directive; UsingForDirective);
into_enum!(Directive; VariableDeclaration);
