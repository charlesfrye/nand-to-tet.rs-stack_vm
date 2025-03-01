use crate::command::Command;

pub struct CodeWriter;

impl CodeWriter {
    pub fn new() -> Self {
        CodeWriter
    }

    pub fn write(&self, _command: &Command) -> String {
        String::new() // Minimal implementation: returns an empty string
    }
}
