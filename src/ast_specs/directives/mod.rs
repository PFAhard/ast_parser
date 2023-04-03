mod contract_definition;
mod enum_definition;
mod error_definition;
mod function_definition;
mod import_directive;
mod prelude;
mod pragma_directive;
mod struct_definition;
mod user_defined_value_type_definition;
mod using_for_direcrive;
mod variable_declaration;

use serde::Deserialize;
pub use prelude::*;


pub type Directives = Vec<Directive>;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub enum Directive {
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