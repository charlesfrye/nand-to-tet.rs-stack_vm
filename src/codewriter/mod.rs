use crate::command::{Command, MemorySegment};

#[derive(Debug, Default)]
pub struct CodeWriter {
    label_counter: usize,
}

impl CodeWriter {
    pub fn new() -> Self {
        CodeWriter { label_counter: 1 }
    }

    pub fn write(&mut self, command: &Command) -> String {
        match command {
            Command::Add => self.write_add(),
            Command::Sub => self.write_sub(),
            Command::Neg => self.write_neg(),
            Command::Eq => self.write_eq(),
            Command::Gt => self.write_gt(),
            Command::Lt => self.write_lt(),
            Command::And => self.write_and(),
            Command::Or => self.write_or(),
            Command::Not => self.write_not(),
            Command::Push(segment, address) => self.write_push(segment, *address),
            _ => "// Not implemented yet".to_string(),
        }
    }

    pub fn write_add(&self) -> String {
        format!("{}\n{}", self._binary_op().join("\n"), "M=D+M")
    }

    pub fn write_sub(&self) -> String {
        format!("{}\n{}", self._binary_op().join("\n"), "M=M-D")
    }

    pub fn write_neg(&self) -> String {
        format!("{}\n{}", self._unary_op().join("\n"), "M=-D")
    }

    pub fn write_eq(&mut self) -> String {
        self._write_comparison("JEQ")
    }

    pub fn write_lt(&mut self) -> String {
        self._write_comparison("JLT")
    }

    pub fn write_gt(&mut self) -> String {
        self._write_comparison("JGT")
    }

    pub fn write_and(&self) -> String {
        format!("{}\n{}", self._binary_op().join("\n"), "M=D&M")
    }

    pub fn write_or(&self) -> String {
        format!("{}\n{}", self._binary_op().join("\n"), "M=D|M")
    }

    pub fn write_not(&self) -> String {
        format!("{}\n{}", self._unary_op().join("\n"), "M=!D")
    }

    pub fn write_push(&self, segment: &MemorySegment, address: i32) -> String {
        match segment {
            MemorySegment::Constant => [
                &format!("@{}", address), // load the constant into A
                "D=A",                    // move it to D
                "@SP",                    // point to the stack pointer
                "A=M",                    // load the stack pointer into A
                "M=D",                    // write the constant onto the stack
                "@SP",                    // increment the
                "M=M+1",                  // stack pointer
            ]
            .join("\n"),
        }
    }

    fn _write_comparison(&mut self, jump_condition: &str) -> String {
        let label_id = self._next_label_id();
        let true_label = format!("TRUE.{}", label_id);
        let out_label = format!("OUT.{}", label_id);
        format!(
            "{}\n{}",
            self._binary_op().join("\n"),
            [
                "D=M-D",                          // subtract top from bottom
                &format!("@{}", true_label),      // possibly jump to TRUE
                &format!("D;{}", jump_condition), // based on the jump_condition
                "D=0",                            // if not, result is false
                &format!("@{}", out_label),       // so jump to out_label
                "0;JMP",                          // to write to the stack
                &format!("({})", true_label),     // if we jumped here,
                "D=-1",                           // result is true (0xFFFF)
                &format!("({})", out_label),      // ready to produce output
                "@SP",                            // point to stack pointer
                "A=M-1",                          // point to the top of the stack
                "M=D"                             // write result to stack
            ]
            .join("\n")
        )
    }

    /// Loads top of the stack into D and points A at next stack element
    fn _binary_op(&self) -> [&str; 4] {
        [
            "@SP",    // point to stack pointer
            "AM=M-1", // decrement stack pointer and load it
            "D=M",    // follow stack pointer
            "A=A-1",  // point one below top of stack
        ]
    }

    /// Loads the top of the stack into D
    fn _unary_op(&self) -> [&str; 3] {
        ["@SP", "A=M-1", "D=M"]
    }

    /// Generate a unique label ID for jump operations
    fn _next_label_id(&mut self) -> usize {
        let id = self.label_counter;
        self.label_counter += 1;
        id
    }
}
