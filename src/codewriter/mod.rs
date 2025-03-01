use crate::command::{Command, MemorySegment};

#[derive(Debug, Default)]
pub struct CodeWriter;

impl CodeWriter {
    pub fn new() -> Self {
        CodeWriter
    }

    pub fn write(&self, command: &Command) -> String {
        match command {
            Command::Add => self.write_add(),
            //Command::Sub => self.write_sub(),
            //Command::Neg => self.write_neg(),
            //Command::Eq => self.write_eq(),
            //Command::Gt => self.write_gt(),
            //Command::Lt => self.write_lt(),
            //Command::And => self.write_and(),
            //Command::Or => self.write_or(),
            //Command::Not => self.write_not(),
            Command::Push(segment, address) => self.write_push(segment, *address),
            _ => "// Not implemented yet".to_string(),
        }
    }

    pub fn write_add(&self) -> String {
        [
            "@SP",    // point to stack pointer
            "AM=M-1", // decrement stack pointer and load it
            "D=M",    // follow stack pointer
            "A=A-1",  // point one below top of stack
            "M=D+M",  // add D to the value and over-write it
        ]
        .join("\n")
    }

    pub fn write_push(&self, segment: &MemorySegment, address: i32) -> String {
        match segment {
            MemorySegment::Constant => [
                &format!("@{}", address), // load the constant into A
                "D=A",                    // move it to D
                "@SP",                    // point to the stack pointer
                "A=M",                    // load the stack pointer into A
                "M=D",                    // write the constant onto the stack
                "@SP",                    // increment the stack pointer
                "M=M+1",
            ]
            .join("\n"),
        }
    }
}
