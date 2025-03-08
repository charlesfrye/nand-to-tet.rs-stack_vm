use crate::codewriter::CodeWriter;
use crate::parser::Parser;

pub fn translate(inputs: Vec<(String, String)>, do_bootstrap: bool) -> String {
    let mut result = String::new();
    let mut codewriter = CodeWriter::new();

    if do_bootstrap {
        result.push_str(&codewriter.write_bootstrap());
        result.push('\n');
    }

    for (filename, content) in inputs {
        let parser = Parser::new(&content);
        codewriter.set_file_context(filename[0..filename.len() - 3].to_string());

        for line in parser {
            let command = line.expect("Failed to parse command");
            let asm_code = codewriter.write(&command);
            result.push_str(&asm_code);
            result.push('\n');
        }
    }

    result
}
