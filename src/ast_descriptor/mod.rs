use super::ast_specs::directives::{FunctionDefinition, VariableDeclaration};
use std::fmt::Write;

pub trait AstDescriptor {
    fn describe(&self, desc: DescType) -> String;
}

#[derive(Debug, Clone, Copy)]
pub enum DescType {
    Action,
    Entity,
}

impl AstDescriptor for FunctionDefinition {
    fn describe(&self, desc: DescType) -> String {
        let mut description = String::new();

        writeln!(description, "Function {}:", self.name());
        if Some(true) == self.parameters().map(|x| !x.is_empty()) {
            writeln!(
                description,
                "\tParameters: {}",
                self.parameters().describe(desc)
            );
        }
        if Some(true) == self.return_parameters().map(|x| !x.is_empty()) {
            writeln!(description, "\tReturns: {}", self.return_parameters().describe(desc));
        }
        description
    }
}

impl AstDescriptor for VariableDeclaration {
    fn describe(&self, desc: DescType) -> String {
        let mut description = String::new();

        write!(description, "{}", self.name());

        description
    }
}

//
//
//
//
//

impl<T: AstDescriptor> AstDescriptor for Option<T> {
    fn describe(&self, desc: DescType) -> String {
        match self {
            Some(x) => x.describe(desc),
            None => String::new(),
        }
    }
}

impl<T: AstDescriptor> AstDescriptor for &[T] {
    fn describe(&self, desc: DescType) -> String {
        let mut description = String::new();

        let last = self.len().saturating_sub(1);
        let mut self_iter = self.iter();

        write!(description, "{}", self_iter.next().describe(desc));
        for i in 1..last {
            write!(description, ", {}", self_iter.next().describe(desc));
        }
        if last != 0 {
            write!(description, " and {}", self_iter.next().describe(desc));
        }

        description
    }
}

impl<T: AstDescriptor> AstDescriptor for &T {
    fn describe(&self, desc: DescType) -> String {
        (*self).describe(desc)
    }
}
