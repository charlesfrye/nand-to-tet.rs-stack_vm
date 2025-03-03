use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum MemorySegment {
    Constant,
    Local,
    Argument,
    This,
    That,
    Temp,
    Pointer,
    Static,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Placeholder,
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
    Push(MemorySegment, u16),
    Pop(MemorySegment, u16),
    Label(String),
    Goto(String),
    IfGoto(String),
    Function(String, u16),
    Call(String, u16),
    Return,
}

impl fmt::Display for MemorySegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemorySegment::Constant => write!(f, "constant"),
            MemorySegment::Local => write!(f, "local"),
            MemorySegment::Argument => write!(f, "argument"),
            MemorySegment::This => write!(f, "this"),
            MemorySegment::That => write!(f, "that"),
            MemorySegment::Temp => write!(f, "temp"),
            MemorySegment::Pointer => write!(f, "pointer"),
            MemorySegment::Static => write!(f, "static"),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Placeholder => write!(f, "// not implemented yet"),
            Command::Add => write!(f, "add"),
            Command::Sub => write!(f, "sub"),
            Command::Neg => write!(f, "neg"),
            Command::Eq => write!(f, "eq"),
            Command::Gt => write!(f, "gt"),
            Command::Lt => write!(f, "lt"),
            Command::And => write!(f, "and"),
            Command::Or => write!(f, "or"),
            Command::Not => write!(f, "not"),
            Command::Push(segment, address) => write!(f, "push {} {}", segment, address),
            Command::Pop(segment, address) => write!(f, "pop {} {}", segment, address),
            Command::Label(value) => write!(f, "label {}", value),
            Command::Goto(value) => write!(f, "goto {}", value),
            Command::IfGoto(value) => write!(f, "if-goto {}", value),
            Command::Function(name, nargs) => write!(f, "function {} {}", name, nargs),
            Command::Call(name, nargs) => write!(f, "call {} {}", name, nargs),
            Command::Return => write!(f, "return"),
        }
    }
}
