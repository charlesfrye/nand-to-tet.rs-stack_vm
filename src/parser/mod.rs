use crate::command::{Command, MemorySegment};

pub struct Parser<'a> {
    lines: std::str::Lines<'a>,
    in_multiline_comment: bool,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            lines: input.lines(),
            in_multiline_comment: false,
        }
    }

    /// Removes comments and whitespace
    fn clean(&mut self, line: &str) -> String {
        let trimmed = line.trim();

        if self.in_multiline_comment {
            // either reach the end or return empty string
            if let Some(comment_end) = trimmed.find("*/") {
                self.in_multiline_comment = false;
                let after_comment = &trimmed[(comment_end + 2)..];
                self.clean(after_comment)
            } else {
                String::new()
            }
        } else if
        // there's a single-line comment
        let Some(comment_start) = trimmed.find("//") {
            // remove comment and any preceding whitespace, then clean remainder
            self.clean(trimmed[0..comment_start].trim())
        } else if
        // there's a multiline comment
        let Some(comment_start) = trimmed.find("/*") {
            let before_comment = &trimmed[0..comment_start];
            let after_start = &trimmed[(comment_start + 2)..];

            self.in_multiline_comment = true;
            if
            // the multiline comment ends early, clean the remainder
            let Some(end_index) = after_start.find("*/") {
                self.in_multiline_comment = false;
                let after_comment = &after_start[(end_index + 2)..];
                self.clean(&format!(
                    "{} {}",
                    before_comment.trim(),
                    after_comment.trim()
                ))
            } else {
                // return everything before the comment
                before_comment.trim().to_string()
            }
        } else {
            // nothing special, return the string
            trimmed.to_string()
        }
    }
}

pub fn parse(line: &str) -> Result<Command, String> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    match tokens.len() {
        len if len > 3 => Err(format!("too many tokens in {}", line)),
        0 => Err("empty line".to_string()),
        _ => _parse(tokens),
    }
}

fn _parse(tokens: Vec<&str>) -> Result<Command, String> {
    match tokens[0] {
        "add" => Ok(Command::Add),
        "sub" => Ok(Command::Sub),
        "neg" => Ok(Command::Neg),
        "eq" => Ok(Command::Eq),
        "gt" => Ok(Command::Gt),
        "lt" => Ok(Command::Lt),
        "and" => Ok(Command::And),
        "or" => Ok(Command::Or),
        "not" => Ok(Command::Not),
        "push" => {
            if tokens.len() < 3 {
                return Err(format!(
                    "Not enough arguments for push: {}",
                    tokens.join(" ")
                ));
            }
            let max_addr = (2 << 14) - 1;
            match tokens[1] {
                "constant" => parse_push_command(MemorySegment::Constant, tokens[2], max_addr),
                "local" => parse_push_command(MemorySegment::Local, tokens[2], max_addr),
                "argument" => parse_push_command(MemorySegment::Argument, tokens[2], max_addr),
                "this" => parse_push_command(MemorySegment::This, tokens[2], max_addr),
                "that" => parse_push_command(MemorySegment::That, tokens[2], max_addr),
                "temp" => parse_push_command(MemorySegment::Temp, tokens[2], 7),
                "pointer" => parse_push_command(MemorySegment::Pointer, tokens[2], 1),
                "static" => parse_push_command(MemorySegment::Static, tokens[2], 240),
                _ => Err(format!("unknown segment for push: {}", tokens[1])),
            }
        }
        "pop" => {
            if tokens.len() < 3 {
                return Err(format!(
                    "Not enough arguments for pop: {}",
                    tokens.join(" ")
                ));
            }
            let max_addr = (2 << 14) - 1;
            match tokens[1] {
                "local" => parse_pop_command(MemorySegment::Local, tokens[2], max_addr),
                "argument" => parse_pop_command(MemorySegment::Argument, tokens[2], max_addr),
                "this" => parse_pop_command(MemorySegment::This, tokens[2], max_addr),
                "that" => parse_pop_command(MemorySegment::That, tokens[2], max_addr),
                "temp" => parse_pop_command(MemorySegment::Temp, tokens[2], 8),
                "pointer" => parse_pop_command(MemorySegment::Pointer, tokens[2], 1),
                "static" => parse_pop_command(MemorySegment::Static, tokens[2], 240),
                _ => Err(format!("unknown segment for pop: {}", tokens[1])),
            }
        }
        "label" => {
            if tokens.len() < 2 {
                return Err(format!(
                    "Not enough arguments for label: {}",
                    tokens.join(" ")
                ));
            }
            Ok(Command::Label(tokens[1].to_string()))
        }
        "goto" => {
            if tokens.len() < 2 {
                return Err(format!(
                    "Not enough arguments for goto: {}",
                    tokens.join(" ")
                ));
            }
            Ok(Command::Goto(tokens[1].to_string()))
        }
        "if-goto" => {
            if tokens.len() < 2 {
                return Err(format!(
                    "Not enough arguments for if-goto: {}",
                    tokens.join(" ")
                ));
            }
            Ok(Command::IfGoto(tokens[1].to_string()))
        }
        "function" => {
            if tokens.len() < 3 {
                return Err(format!(
                    "Not enough arguments for function def: {}",
                    tokens.join(" ")
                ));
            }
            let name = tokens[1].to_string();
            let nargs_token = tokens[2];
            match nargs_token.parse() {
                Ok(nargs) if nargs <= ((2 << 14) - 1) => Ok(Command::Function(name, nargs)),
                Ok(value) => Err(format!("nargs exceeds maximum allowed: {}", value)),
                Err(_) => Err(format!("couldn't parse nargs: {}", nargs_token)),
            }
        }
        "call" => {
            if tokens.len() < 3 {
                return Err(format!(
                    "Not enough arguments for function call: {}",
                    tokens.join(" ")
                ));
            }
            let name = tokens[1].to_string();
            let nargs_token = tokens[2];
            match nargs_token.parse() {
                Ok(nargs) if nargs <= ((2 << 14) - 1) => Ok(Command::Call(name, nargs)),
                Ok(value) => Err(format!("nargs exceeds maximum allowed: {}", value)),
                Err(_) => Err(format!("couldn't parse nargs: {}", nargs_token)),
            }
        }
        "return" => Ok(Command::Return),
        _ => Err(format!("couldn't parse {}", tokens.join(" "))),
    }
}

fn parse_push_command(
    segment: MemorySegment,
    token: &str,
    max_value: u16,
) -> Result<Command, String> {
    match token.parse() {
        Ok(value) if value <= max_value => Ok(Command::Push(segment, value)),
        Ok(_) => Err(format!("value exceeds maximum allowed: {}", max_value)),
        Err(_) => Err(format!("couldn't parse address: {}", token)),
    }
}

fn parse_pop_command(
    segment: MemorySegment,
    token: &str,
    max_value: u16,
) -> Result<Command, String> {
    match token.parse() {
        Ok(value) if value <= max_value => Ok(Command::Pop(segment, value)),
        Ok(_) => Err(format!("value exceeds maximum allowed: {}", max_value)),
        Err(_) => Err(format!("couldn't parse address: {}", token)),
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Command, String>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(line) = self.lines.next() {
            let cleaned = self.clean(line);
            if !cleaned.is_empty() {
                return Some(parse(&cleaned));
            }
        }
        None
    }
}

#[cfg(test)]
mod clean_tests {
    use super::*;

    #[test]
    fn test_clean_basic_whitespace() {
        let mut parser = Parser::new("  test  ");
        assert_eq!(parser.clean("  test  "), "test");
    }

    #[test]
    fn test_clean_tabs_and_newlines() {
        let mut parser = Parser::new("\t\ttest\n");
        assert_eq!(parser.clean("\t\ttest\n"), "test");
    }

    #[test]
    fn test_clean_mixed_whitespace() {
        let mut parser = Parser::new(" \t test \n ");
        assert_eq!(parser.clean(" \t test \n "), "test");
    }

    #[test]
    fn test_clean_single_line_comment() {
        let mut parser = Parser::new("");
        assert_eq!(parser.clean("test // comment"), "test");
        assert_eq!(parser.clean("  test  // comment  "), "test");
    }

    #[test]
    fn test_clean_multiline_comment_same_line() {
        let mut parser = Parser::new("");
        assert_eq!(parser.clean("test /* comment */ code"), "test code");
        assert_eq!(parser.clean("  test  /* comment */  code  "), "test code");
    }

    #[test]
    fn test_clean_multiline_comment_start() {
        let mut parser = Parser::new("");
        assert_eq!(parser.clean("test /* comment"), "test");
        assert!(parser.in_multiline_comment);
    }

    #[test]
    fn test_clean_multiline_comment_end() {
        let mut parser = Parser::new("");
        parser.in_multiline_comment = true;
        assert_eq!(parser.clean("comment */ test"), "test");
        assert!(!parser.in_multiline_comment);
    }

    #[test]
    fn test_clean_multiline_comment_continuation() {
        let mut parser = Parser::new("");
        parser.in_multiline_comment = true;
        assert_eq!(parser.clean("still in comment"), "");
        assert!(parser.in_multiline_comment);
    }

    #[test]
    fn test_clean_empty_after_comment_removal() {
        let mut parser = Parser::new("");
        assert_eq!(parser.clean(" // only a comment"), "");
        assert_eq!(parser.clean("/* just another comment */ "), "");
    }

    #[test]
    fn test_clean_multiple_comments() {
        let mut parser = Parser::new("");
        assert_eq!(
            parser.clean("test /* comment */ more // end comment"),
            "test more"
        );
    }

    #[test]
    fn test_parser_iterator() {
        let input = "
            // Comment line
            test1  // Inline comment
            /* Multiline
               comment */ test2
               
            test3 /* comment */ test4
        ";
        let parser = Parser::new(input);
        let commands: Vec<Result<Command, String>> = parser.collect();

        assert_eq!(commands.len(), 3);
    }
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    // Test arithmetic commands
    #[test]
    fn test_parse_arithmetic() {
        assert_eq!(parse("add"), Ok(Command::Add));
        assert_eq!(parse("sub"), Ok(Command::Sub));
        assert_eq!(parse("neg"), Ok(Command::Neg));
    }

    // Test comparison commands
    #[test]
    fn test_parse_comparison() {
        assert_eq!(parse("eq"), Ok(Command::Eq));
        assert_eq!(parse("gt"), Ok(Command::Gt));
        assert_eq!(parse("lt"), Ok(Command::Lt));
    }

    // Test logical commands
    #[test]
    fn test_parse_logical() {
        assert_eq!(parse("and"), Ok(Command::And));
        assert_eq!(parse("or"), Ok(Command::Or));
        assert_eq!(parse("not"), Ok(Command::Not));
    }

    // Test push command with constants
    #[test]
    fn test_parse_push_constant() {
        assert_eq!(
            parse("push constant 42"),
            Ok(Command::Push(MemorySegment::Constant, 42))
        );
        assert_eq!(
            parse("push constant 0"),
            Ok(Command::Push(MemorySegment::Constant, 0))
        );
        assert_eq!(
            parse("push constant 9999"),
            Ok(Command::Push(MemorySegment::Constant, 9999))
        );
    }

    // Test general push command
    #[test]
    fn test_parse_push_general() {
        assert_eq!(
            parse("push local 4"),
            Ok(Command::Push(MemorySegment::Local, 4))
        );
        assert_eq!(
            parse("push argument 0"),
            Ok(Command::Push(MemorySegment::Argument, 0))
        );
        assert_eq!(
            parse("push this 9999"),
            Ok(Command::Push(MemorySegment::This, 9999))
        );
        assert_eq!(
            parse("push that 37"),
            Ok(Command::Push(MemorySegment::That, 37))
        );
        assert_eq!(
            parse("push temp 5"),
            Ok(Command::Push(MemorySegment::Temp, 5))
        );
    }

    // Test pop commands
    #[test]
    fn test_parse_pop() {
        assert_eq!(
            parse("pop local 4"),
            Ok(Command::Pop(MemorySegment::Local, 4))
        );
        assert_eq!(
            parse("pop argument 0"),
            Ok(Command::Pop(MemorySegment::Argument, 0))
        );
        assert_eq!(
            parse("pop this 9999"),
            Ok(Command::Pop(MemorySegment::This, 9999))
        );
        assert_eq!(
            parse("pop that 37"),
            Ok(Command::Pop(MemorySegment::That, 37))
        );
        assert_eq!(
            parse("pop temp 5"),
            Ok(Command::Pop(MemorySegment::Temp, 5))
        );
    }

    #[test]
    fn test_parse_errors() {
        assert!(parse("").is_err());

        assert!(parse("push constant 0 extra stuff").is_err());

        assert!(parse("invalid_command").is_err());

        assert!(parse("push invalid_segment 59").is_err());

        assert!(parse("push constant abc").is_err());

        assert!(parse("push").is_err());
        assert!(parse("push constant").is_err());
    }

    #[test]
    fn test_parse_with_extra_whitespace() {
        assert_eq!(parse("  add  "), Ok(Command::Add));
        assert_eq!(
            parse("push   constant   117"),
            Ok(Command::Push(MemorySegment::Constant, 117))
        );
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_parse_with_comments() {
        let mut parser = Parser::new("");

        // Clean the line, then parse the result
        let cleaned = parser.clean("add // This is a comment");
        assert_eq!(parse(&cleaned), Ok(Command::Add));

        let cleaned = parser.clean("push constant 42 /* Comment */");
        assert_eq!(
            parse(&cleaned),
            Ok(Command::Push(MemorySegment::Constant, 42))
        );

        // Multi-line comment
        parser.in_multiline_comment = true;
        let cleaned = parser.clean("* still comment */ add");
        assert_eq!(parse(&cleaned), Ok(Command::Add));
    }
}
