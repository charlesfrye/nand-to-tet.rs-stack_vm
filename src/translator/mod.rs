use crate::codewriter::CodeWriter;
use crate::parser::Parser;

pub fn translate(inputs: Vec<(String, String)>) -> String {
    let mut result = String::new();
    let mut codewriter = CodeWriter::new();

    for (_filename, content) in inputs {
        let parser = Parser::new(&content);

        for line in parser {
            let command = line.expect("Failed to parse command");
            let asm_code = codewriter.write(&command);
            result.push_str(&asm_code);
            result.push('\n');
        }
    }

    result
}
