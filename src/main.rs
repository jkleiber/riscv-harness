use std::env;
use std::error;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use subprocess::{Popen, PopenConfig, Redirection};

#[derive(Default)]
struct TestVector {
    pub inputs: String,
    pub outputs: String,
}

fn run_single_test(asm_path: &String, test: TestVector) -> Result<(), Box<dyn error::Error>> {
    // RARS is assumed to be in the same directory where the riscv-harness is being called from for now.
    let rars_path: String = format!("rars1_6.jar");

    let rars_run_cmd: String = format!("java -jar {rars_path} nc {asm_path}");
    println!("{rars_run_cmd}");

    let mut p = Popen::create(
        &[
            "java",
            "-jar",
            rars_path.to_string().as_str(),
            "nc",
            asm_path,
        ],
        PopenConfig {
            stdin: Redirection::Pipe,
            detached: true,
            ..Default::default()
        },
    )?;

    let _result = p.communicate(Some(test.inputs.as_str()));

    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    // Inputs:
    // riscv-harness [riscv binary] [test vector directory]
    let args: Vec<String> = env::args().collect();

    // These need to be relative paths from the location where riscv-harness is being called for now.
    let riscv_asm_path: &String = &args[1];
    let test_vector_path: &String = &args[2];

    let harness_path: PathBuf = env::current_dir()?;
    println!("The current directory is {}", harness_path.display());

    // Collect tests
    for entry in fs::read_dir(test_vector_path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_file() {
            println!("{}", entry_path.display());
        }

        // Read test vector.
        let test_file = fs::File::open(entry_path)?;
        let test_reader = BufReader::new(test_file);

        let mut inputs = String::new();
        let mut outputs = String::new();
        let mut is_reading_input: bool = true;
        let mut is_first_line: bool = true;
        for test_line in test_reader.lines() {
            let test_line = test_line?;

            // Handle the separation between input and output.
            if test_line == "---" {
                is_reading_input = false;
                is_first_line = true;
                continue;
            } else if !is_first_line && is_reading_input {
                inputs = inputs + "\n"
            } else if !is_first_line && !is_reading_input {
                outputs = outputs + "\n";
            }

            if is_reading_input {
                inputs = inputs + test_line.as_str();
            } else {
                outputs = outputs + test_line.as_str();
            }

            is_first_line = false;
        }

        println!("inputs:\n{inputs}\n---\n{outputs}");

        let test = TestVector {
            inputs: inputs,
            ..Default::default()
        };

        let _ = run_single_test(riscv_asm_path, test);
    }

    Ok(())
}
