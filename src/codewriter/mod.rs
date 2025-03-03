use crate::command::{Command, MemorySegment};

#[derive(Debug, Default)]
pub struct CodeWriter {
    label_counter: usize,
    namespace: String,
}

impl CodeWriter {
    pub fn new() -> Self {
        CodeWriter {
            label_counter: 1,
            namespace: "STATIC".to_string(),
        }
    }

    pub fn set_namespace(&mut self, ns: String) {
        self.namespace = ns;
    }

    pub fn write(&mut self, command: &Command) -> String {
        let debug_comment = format!("// {}", command);

        let assembly = match command {
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
            Command::Pop(segment, address) => self.write_pop(segment, *address),
            _ => "// Not implemented yet".to_string(),
        };

        format!("{}\n{}", debug_comment, assembly)
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

    pub fn write_push(&self, segment: &MemorySegment, argument: u16) -> String {
        if *segment == MemorySegment::Constant {
            format!(
                "{}\n{}",
                [
                    &format!("@{}", argument), // load the constant into A
                    "D=A",                     // move it to D
                ]
                .join("\n"),
                self._push().join("\n"), // push D
            )
        } else if *segment == MemorySegment::Static {
            format!(
                "@{}.{}\nD=M\n{}",
                self.namespace,
                argument,
                self._push().join("\n")
            )
        } else {
            format!(
                "{}\n{}",
                [
                    // load the base address into D
                    &self._get_base_address(segment),
                    "D=A",
                    // load the index into A
                    &format!("@{}", argument),
                    // index into segment with A
                    "A=D+A",
                    // load value into D
                    "D=M"
                ]
                .join("\n"),
                self._push().join("\n")
            )
        }
    }

    pub fn write_pop(&self, segment: &MemorySegment, argument: u16) -> String {
        if *segment == MemorySegment::Static {
            format!(
                "D=0\n@{}.{}\n{}",
                self.namespace,
                argument,
                self._pop().join("\n")
            )
        } else {
            let get_base_address = self._get_base_address(segment);

            format!(
                "{}\n{}",
                [
                    // load the base address into D
                    &get_base_address,
                    "D=A",
                    // load the index into A
                    &format!("@{}", argument)
                ]
                .join("\n"),
                self._pop().join("\n"), // pop stack into D+A
            )
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

    /// Map each segment to its 'well-known' address -- which may contain a pointer to its base
    fn _get_segment_well_known_addr(&self, segment: &MemorySegment) -> String {
        match segment {
            MemorySegment::Local => "@LCL",
            MemorySegment::Argument => "@ARG",
            MemorySegment::This => "@THIS",
            MemorySegment::That => "@THAT",
            MemorySegment::Temp => "@5",
            MemorySegment::Pointer => "@3",
            _ => panic!("Invalid segment for address calculation"),
        }
        .to_string()
    }

    /// Loads the base address of a segment into A
    fn _get_base_address(&self, segment: &MemorySegment) -> String {
        let segment_well_known_addr = self._get_segment_well_known_addr(segment);
        let is_pointer = self._is_pointed_segment(segment);

        if is_pointer {
            // chase pointer
            format!("{}\nA=M\n", segment_well_known_addr)
        } else {
            segment_well_known_addr
        }
    }

    fn _is_pointed_segment(&self, segment: &MemorySegment) -> bool {
        matches!(
            segment,
            MemorySegment::Local
                | MemorySegment::Argument
                | MemorySegment::This
                | MemorySegment::That
        )
    }

    /// Pushes D onto the top of the stack
    fn _push(&self) -> [&str; 5] {
        [
            "@SP",   // point to the stack pointer
            "A=M",   // load the stack pointer into A
            "M=D",   // write the value onto the stack
            "@SP",   // increment the stack pointer
            "M=M+1", //
        ]
    }

    /// Pops top of stack into D+A, via R13
    fn _pop(&self) -> [&str; 9] {
        [
            // store D+A in general-purpose register
            "D=D+A", "@R13", "M=D", // pop stack into D and decrement
            "@SP", "AM=M-1", "D=M", // store D into *R13
            "@R13", "A=M", "M=D",
        ]
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
