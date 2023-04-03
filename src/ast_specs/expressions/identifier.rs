use serde::Deserialize;

use crate::ast_parser::{
    ast_specs::{common::TypeDescriptions, node_type::NodeTypeInternal, SourceUnit},
    ast_visitor::AstVisitor,
};

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Identifier {
    #[serde(rename = "argumentTypes")]
    argument_types: Option<Vec<TypeDescriptions>>,
    id: isize,
    name: String,
    #[serde(rename = "overloadedDeclarations")]
    overloaded_declarations: Vec<isize>,
    #[serde(rename = "referencedDeclaration")]
    referenced_declaration: Option<isize>,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl Identifier {
    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub(crate) fn referenced_declaration(&self) -> Option<isize> {
        self.referenced_declaration
    }

    pub(crate) fn src(&self) -> &str {
        self.src.as_ref()
    }

    pub(crate) fn argument_types(&self) -> Option<&Vec<TypeDescriptions>> {
        self.argument_types.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }

    // pub(crate) fn build_absolute_name(&self, ast: &SourceUnit) -> OccurrenceOprerable {
    //     fn loop_through(name: &mut String, ast: &SourceUnit, ref_id: isize) -> bool {
    //         let mut var_dec = ast.filter_by_id(ref_id);

    //         if var_dec.len() == 1 {
    //             match var_dec.pop().unwrap() {
    //                 NodeTypeInternal::VariableDeclaration(var_dec) => {
    //                     let ref_id = var_dec.scope();
    //                     loop_through(name, ast, ref_id);
    //                 }
    //                 NodeTypeInternal::ContractDefinition(con_def) => {
    //                     name.insert(0, '.');
    //                     name.insert_str(0, con_def.name());
    //                 }
    //                 NodeTypeInternal::FunctionDefinition(fun_def) => {
    //                     let ref_id = fun_def.scope();
    //                     loop_through(name, ast, ref_id);
    //                     // name.insert(0, '.');
    //                     // name.insert_str(0, fun_def.name()); // TODO: We do not need it. But we do need to preccess function definitions if it is first name
    //                     // let ref_id = fun_def.scope();
    //                     // loop_through(name, ast, ref_id);
    //                 }
    //                 NodeTypeInternal::StructDefinition(st_def) => {
    //                     // name.insert(0, '.');
    //                     // name.insert_str(0, st_def.name()); // TODO: check if needed
    //                     let ref_id = st_def.scope();
    //                     loop_through(name, ast, ref_id);
    //                 }
    //                 NodeTypeInternal::EventDefinition(_) => {}
    //                 NodeTypeInternal::Block(_) => {}
    //                 _d => unimplemented!("[Assumption] Unexpected node {:?}", _d),
    //             }
    //         } else if var_dec.is_empty() {
    //         } else {
    //             unimplemented!(
    //                 "[Assumption] Should be only one id, implemente if not. Target {:?}",
    //                 var_dec
    //             );
    //         }

    //         true
    //     }

    //     let mut name = self.name().to_owned();

    //     if let Some(ref_id) = self.referenced_declaration() {
    //         loop_through(&mut name, ast, ref_id);
    //     }
    //     OccurrenceOprerable::new(self.src().into(), name)
    // }
}
