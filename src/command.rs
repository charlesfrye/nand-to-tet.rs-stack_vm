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
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
            _ => write!(f, "// not implemented yet"),
        }
    }
}
