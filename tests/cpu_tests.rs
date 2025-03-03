use stack_vm::translator;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn test_simple_add() {
    assert!(run_cpu_emulator_test(
        &(get_base_test_dir() + "/StackArithmetic/SimpleAdd")
    ));
}

#[test]
fn test_stack_test() {
    assert!(run_cpu_emulator_test(
        &(get_base_test_dir() + "/StackArithmetic/StackTest")
    ));
}

#[test]
fn test_basic_test() {
    assert!(run_cpu_emulator_test(
        &(get_base_test_dir() + "/MemoryAccess/BasicTest")
    ));
}

fn run_cpu_emulator_test(test_dir: &str) -> bool {
    let dir_path = Path::new(test_dir);

    let vm_files = gather_vm_files(dir_path);
    if vm_files.is_empty() {
        println!("No .vm files found in {}", test_dir);
        return false;
    }

    let test_name = dir_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let asm_output = PathBuf::from(test_dir).join(format!("{}.asm", test_name));

    let translated_code = translator::translate(vm_files);
    fs::write(&asm_output, translated_code).expect("Failed to write assembly output");

    let test_script = PathBuf::from(test_dir).join(format!("{}.tst", test_name));
    if !test_script.exists() {
        println!("Test script not found: {:?}", test_script);
        return false;
    }

    let output = Command::new(get_cpu_emulator_path())
        .arg(test_script)
        .output()
        .expect("Failed to execute CPUEmulator");

    if !output.status.success() {
        println!("CPUEmulator execution failed");
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    let out_file = PathBuf::from(test_dir).join(format!("{}.out", test_name));
    let cmp_file = PathBuf::from(test_dir).join(format!("{}.cmp", test_name));

    if out_file.exists() && cmp_file.exists() {
        let out_content = fs::read_to_string(out_file)
            .expect("Failed to read output file")
            .trim()
            .to_string();
        let cmp_content = fs::read_to_string(cmp_file)
            .expect("Failed to read comparison file")
            .replace("\r\n", "\n")
            .trim()
            .to_string();

        if out_content != cmp_content {
            println!("TARGET:\n{}", cmp_content);
            println!("GOT:\n{}", out_content);
            return false;
        }
    }

    true
}

fn gather_vm_files(dir_path: &Path) -> Vec<(String, String)> {
    fs::read_dir(dir_path)
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
        .collect()
}

fn get_cpu_emulator_path() -> String {
    fn default_path() -> String {
        let current_exe = std::env::current_exe().expect("Failed to get current executable path");
        let project_dir = current_exe
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap();

        // Navigate up one more level to the 7 folder
        let chapter_dir = project_dir.parent().unwrap();

        // Navigate up one more level to the 'projects' folder
        let projects_dir = chapter_dir.parent().unwrap();

        // Navigate up to the nand2tetris root and then to tools
        let nand2tetris_dir = projects_dir.parent().unwrap();

        nand2tetris_dir
            .join("tools")
            .join("CPUEmulator.sh")
            .to_string_lossy()
            .to_string()
    }

    std::env::var("CPU_EMULATOR_PATH").unwrap_or(default_path())
}

fn get_base_test_dir() -> String {
    fn default_dir() -> String {
        let current_exe = std::env::current_exe().expect("Failed to get current executable path");
        let project_dir = current_exe
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap();

        let test_data_dir = project_dir.join("tests").join("test_data");

        test_data_dir.to_string_lossy().to_string()
    }

    std::env::var("STACK_VM_TEST_DIR").unwrap_or(default_dir())
}
