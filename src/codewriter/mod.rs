use crate::command::{Command, MemorySegment};
use std::fmt;

#[derive(Debug, Default)]
pub struct CodeWriter {
    label_counter: usize,
    context: Context,
}

#[derive(Debug, Default)]
struct Context {
    file: String,
    function: String,
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.function.is_empty() {
            write!(f, "{}", self.file)
        } else {
            write!(f, "{}", self.function)
        }
    }
}

impl CodeWriter {
    pub fn new() -> Self {
        CodeWriter {
            label_counter: 1,
            context: Context::default(),
        }
    }

    pub fn set_file_context(&mut self, filename: String) {
        self.context.file = filename;
    }

    fn _set_function_context(&mut self, name: String) {
        self.context.function = name;
    }

    pub fn write_bootstrap(&mut self) -> String {
        [
            // initialize stack pointer
            "@256",
            "D=A",
            "@SP",
            "M=D",
            // start executing entrypoint
            &self.write_call("Sys.init", 0),
        ]
        .join("\n")
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
            Command::Label(value) => self.write_label(value),
            Command::Goto(value) => self.write_goto(value),
            Command::IfGoto(value) => self.write_ifgoto(value),
            Command::Function(name, nargs) => self.write_function(name, *nargs),
            Command::Call(name, nargs) => self.write_call(name, *nargs),
            Command::Return => self.write_return(),
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
                self.context.file,
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
                self.context.file,
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

    pub fn write_label(&self, label: &str) -> String {
        format!("({}${})", self.context, label)
    }

    pub fn write_goto(&self, label: &str) -> String {
        format!("@{}${}\n0;JMP", self.context, label)
    }

    pub fn write_ifgoto(&self, label: &str) -> String {
        [
            // pop the stack into D
            "@SP",
            "AM=M-1",
            "D=M",
            // load the label into A
            &format!("@{}${}", self.context, label),
            // jump there if D != 0
            "D;JNE",
        ]
        .join("\n")
    }

    pub fn write_function(&mut self, name: &str, nlocals: u16) -> String {
        self._set_function_context(name.to_string());
        format!(
            "({}){}",
            self.context,
            format!("\nD=0\n{}", self._push().join("\n")).repeat(nlocals as usize)
        )
    }

    pub fn write_call(&mut self, name: &str, nargs: u16) -> String {
        let call_label = format!("__RET_{}", self._next_label_id());
        [
            // push return-address
            &format!("@{}\nD=A", call_label),
            &self._push().join("\n"),
            // store LCL, ARG, THIS, and THAT on stack
            &self._push_segment("LCL"),
            &self._push_segment("ARG"),
            &self._push_segment("THIS"),
            &self._push_segment("THAT"),
            // reposition ARG
            &format!("@{}", nargs + 5),
            "D=A",
            "@SP",
            "D=M-D",
            "@ARG",
            "M=D",
            // reposition LCL
            "@SP",
            "D=M",
            "@LCL",
            "M=D",
            // transfer control
            &format!("@{}\n0;JMP", name),
            // provide return address
            &format!("({})", call_label),
        ]
        .join("\n")
    }

    pub fn write_return(&mut self) -> String {
        self._set_function_context("".to_string());
        [
            // stash stack frame pointer in a general-purpose register
            "@LCL",
            "D=M",
            "@R14",
            "M=D",
            // store return address in another register
            "@5",
            "A=D-A",
            "D=M",
            "@R15",
            "M=D",
            // pop return value into position (same as base of argument segment)
            "@SP",
            "AM=M-1",
            "D=M", // D set to return value
            "@ARG",
            "A=M",
            "M=D", // ARG[0] = D
            // restore SP -- just above return value
            "D=A+1",
            "@SP",
            "M=D",
            // restore THAT, THIS, ARG, LCL
            &self._restore_segment("THAT", 1),
            &self._restore_segment("THIS", 2),
            &self._restore_segment("ARG", 3),
            &self._restore_segment("LCL", 4),
            // relinquish control
            "@R15",
            "A=M",
            "0;JMP",
        ]
        .join("\n")
    }

    pub fn _push_segment(&self, segment_pointer: &str) -> String {
        [
            &format!("@{}", segment_pointer),
            "D=M",
            &self._push().join("\n"),
        ]
        .join("\n")
    }

    pub fn _restore_segment(&self, segment_pointer: &str, frame_offset: u16) -> String {
        [
            // load frame top into D
            "@R14",
            "D=M",
            // subtract offset
            &format!("@{}", frame_offset),
            "A=D-A",
            // load contents
            "D=M",
            // restore into segment pointer
            &format!("@{}", segment_pointer),
            "M=D",
        ]
        .join("\n")
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
            format!("{}\nA=M", segment_well_known_addr)
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
