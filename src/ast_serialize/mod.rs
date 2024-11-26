use crate::ast_specs::{
    ArrayTypeName, Directive, EventDefinition, ParameterList, SourceUnit, StructuredDocumentation, TypeName, VariableDeclaration
};

pub const LICENSE: &str = "// SPDX-License-Identifier: <LICENSE>";
pub const LICENSE_KEY: &str = "<LICENSE>";

pub const EVENT: &str = "<DOCUMENTATION>event <EVENT_NAME>(<EVENT_ARGS>);";
pub const EVENT_DOCUMENTATION_KEY: &str = "<DOCUMENTATION>";
pub const EVENT_NAME_KEY: &str = "<EVENT_NAME>";
pub const EVENT_ARGS_KEY: &str = "<EVENT_ARGS>";

pub const VARIABLE: &str = "<TYPE> <INDEXED> <NAME><TERMINATOR>";
pub const VARIABLE_TYPE_KEY: &str = "<TYPE>";
pub const VARIABLE_INDEXED_KEY: &str = "<INDEXED>";
pub const VARIABLE_INDEXED_KEYWORD: &str = "indexed";
pub const VARIABLE_NAME_KEY: &str = "<NAME>";
pub const VARIABLE_TERMINATOR_KEY: &str = "<TERMINATOR>";

pub trait AstSerializer {
    fn to_sol(&self) -> Vec<u8>;
}

impl AstSerializer for SourceUnit {
    fn to_sol(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let license = LICENSE.replace(
            LICENSE_KEY,
            self.license().as_deref().unwrap_or("UNLICENSED"),
        );

        out.extend(self.nodes().to_sol());
        out.extend_from_slice(license.as_bytes());
        out
    }
}

impl AstSerializer for Directive {
    fn to_sol(&self) -> Vec<u8> {
        match self {
            Directive::EventDefinition(event_definition) => event_definition.to_sol(),
            Directive::ContractDefinition(contract_definition) => contract_definition.to_sol(),
            Directive::EnumDefinition(enum_definition) => enum_definition.to_sol(),
            Directive::ErrorDefinition(error_definition) => error_definition.to_sol(),
            Directive::FunctionDefinition(function_definition) => function_definition.to_sol(),
            Directive::ImportDirective(import_directive) => import_directive.to_sol(),
            Directive::PragmaDirective(pragma_directive) => pragma_directive.to_sol(),
            Directive::StructDefinition(struct_definition) => struct_definition.to_sol(),
            Directive::UserDefinedValueTypeDefinition(user_defined_value_type_definition) => {
                user_defined_value_type_definition.to_sol()
            }
            Directive::UsingForDirective(using_for_directive) => using_for_directive.to_sol(),
            Directive::VariableDeclaration(variable_declaration) => variable_declaration.to_sol(),
        }
    }
}

impl AstSerializer for EventDefinition {
    fn to_sol(&self) -> Vec<u8> {
        let mut event = EVENT.replace(
            EVENT_DOCUMENTATION_KEY,
            &to_string(self.documentation().to_sol()),
        );
        event = event.replace(EVENT_NAME_KEY, self.name());
        event = event.replace(EVENT_ARGS_KEY, &to_string(self.parameters().to_sol()));

        event.as_bytes().to_vec()
    }
}

impl AstSerializer for StructuredDocumentation {
    fn to_sol(&self) -> Vec<u8> {
        self.text().as_bytes().to_owned()
    }
}

impl AstSerializer for ParameterList {
    fn to_sol(&self) -> Vec<u8> {
        self.parameters().to_sol()
    }
}

impl AstSerializer for VariableDeclaration {
    fn to_sol(&self) -> Vec<u8> {
        let mut var = VARIABLE.replace(VARIABLE_TYPE_KEY, &to_string(self.type_name().to_sol()));
        var = var.replace(
            VARIABLE_INDEXED_KEY,
            match self.indexed() {
                Some(true) => VARIABLE_INDEXED_KEYWORD,
                _ => "",
            },
        );
        var = var.replace(VARIABLE_NAME_KEY, &self.name());
        var = var.replace(
            VARIABLE_TERMINATOR_KEY,
            if self.state_variable() { ";" } else { "," },
        );

        var.as_bytes().to_vec()
    }
}

impl AstSerializer for TypeName {
    fn to_sol(&self) -> Vec<u8> {
        match self {
            TypeName::ArrayTypeName(array_type_name) => array_type_name.to_sol(),
            TypeName::ElementaryTypeName(elementary_type_name) => elementary_type_name.to_sol(),
            TypeName::FunctionTypeName(function_type_name) => function_type_name.to_sol(),
            TypeName::Mapping(mapping) => mapping.to_sol(),
            TypeName::UserDefinedTypeName(user_defined_type_name) => {
                user_defined_type_name.to_sol()
            }
        }
    }
}

impl AstSerializer for ArrayTypeName {
    fn to_sol(&self) -> Vec<u8> {
        todo!()
    }
}

//
// COMMON
//
impl<T: AstSerializer> AstSerializer for Vec<T> {
    fn to_sol(&self) -> Vec<u8> {
        self.iter().fold(Vec::new(), |mut acc, t| {
            acc.extend(t.to_sol());
            acc
        })
    }
}

impl<T: AstSerializer> AstSerializer for &[T] {
    fn to_sol(&self) -> Vec<u8> {
        self.iter().fold(Vec::new(), |mut acc, t| {
            acc.extend(t.to_sol());
            acc
        })
    }
}

impl<T: AstSerializer> AstSerializer for Option<T> {
    fn to_sol(&self) -> Vec<u8> {
        match self {
            Some(t) => t.to_sol(),
            None => vec![],
        }
    }
}

fn to_string(v: Vec<u8>) -> String {
    String::from_utf8(v).unwrap()
}

#[test]
fn test_counter() {
    pub const AST: &str = "debug\\out\\Counter.sol\\Counter.json";
}
