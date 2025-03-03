use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

use stack_vm::translator;

fn main() {
    let (input_path, input_name, input_files) = get_input();
    let output_filename = determine_output_path(&input_path, &input_name);

    let mut output_file = File::create(&output_filename).expect("Failed to create output file");

    let translated_code = translator::translate(input_files, true);

    writeln!(output_file, "{}", translated_code).expect("Failed to write to output file");

    println!("Translation complete: {}", output_filename);
}

/// Determines the input source (file, directory, or stdin) and returns:
/// - `input_name`: Used for naming the output file.
/// - `input_files`: A Vec of (filename, file contents) pairs.
fn get_input() -> (String, String, Vec<(String, String)>) {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(path) => {
            let path = Path::new(path);
            if path.is_dir() {
                let (name, files) = get_directory_input(path);
                (path.to_string_lossy().into_owned(), name, files)
            } else if path.is_file() {
                let (name, files) = get_file_input(path);
                (path.to_string_lossy().into_owned(), name, files)
            } else {
                panic!("Invalid input: not a file or directory.");
            }
        }
        None => {
            let (name, files) = get_stdin_input();
            (".".to_string(), name, files) // Default path for stdin
        }
    }
}

/// Reads all `.vm` files from a directory and returns their contents.
fn get_directory_input(path: &Path) -> (String, Vec<(String, String)>) {
    let files: Vec<(String, String)> = fs::read_dir(path)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_path = entry.path();
            if file_path.extension()?.to_str()? == "vm" {
                let content = fs::read_to_string(&file_path).ok()?;
                let filename = file_path.file_name()?.to_str()?.to_string();
                Some((filename, content))
            } else {
                None
            }
        })
        .collect();

    if files.is_empty() {
        panic!("No .vm files found in directory.");
    }

    let directory_name = path.file_name().unwrap().to_str().unwrap().to_string();
    (directory_name, files)
}

/// Reads a single `.vm` file and returns its contents.
fn get_file_input(path: &Path) -> (String, Vec<(String, String)>) {
    let content = fs::read_to_string(path).expect("Failed to read input file");
    let filename = path.file_name().unwrap().to_str().unwrap().to_string();
    let input_name = filename.trim_end_matches(".vm").to_string();
    (input_name, vec![(filename, content)])
}

/// Reads from stdin and treats it as a single `.vm` file.
fn get_stdin_input() -> (String, Vec<(String, String)>) {
    let mut data = String::new();
    io::stdin()
        .read_to_string(&mut data)
        .expect("Failed to read from stdin");
    ("a".to_string(), vec![("stdin".to_string(), data)])
}

/// Determines the correct output file path based on input.
fn determine_output_path(input_path: &str, input_name: &str) -> String {
    let input_path = Path::new(input_path);
    let output_filename = format!("{}.asm", input_name);

    if input_path.is_dir() {
        input_path
            .join(output_filename)
            .to_string_lossy()
            .into_owned()
    } else {
        input_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(output_filename)
            .to_string_lossy()
            .into_owned()
    }
}
