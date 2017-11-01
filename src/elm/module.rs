use std::io::{Write, Result};
use super::definition::Definition;
use super::super::representation::Representation;

pub struct Module {
    name: String,
    definitions: Vec<Definition>,
}

impl Module {
    pub fn new<S>(name: S) -> Module where S: Into<String> {
        Module { name : name.into(), definitions: Vec::new() }
    }

    pub fn define(&mut self, definition: Definition) {
        self.definitions.push(definition);
    }
}

impl Representation for Module {
    fn write_representation(&self, writer: &mut Write) -> Result<()> {
        write!(writer, "module {} exposing (..)\n", self.name)?;

        for definition in &self.definitions {
            write!(writer, "\n\n")?;
            definition.write_representation(writer)?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::representation::Representation;
    use super::*;

    #[test]
    fn empty_module_should_write_it_self() {
        let mut output = Vec::new();
        let module = Module::new("Test");

        module.write_representation(&mut output).unwrap();

        assert_eq!(output[0..25], b"module Test exposing (..)"[..]);
    }
}
