use crate::command::Command;

pub struct Parser<'a> {
    lines: std::str::Lines<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            lines: input.lines(),
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Command, String>;

    fn next(&mut self) -> Option<Self::Item> {
        None // TODO
    }
}
