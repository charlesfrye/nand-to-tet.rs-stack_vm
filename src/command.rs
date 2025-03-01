#[derive(Debug, PartialEq, Eq)]
pub enum MemorySegment {
    Constant,
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
    Push(MemorySegment, i32),
}
