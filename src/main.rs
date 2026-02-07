use std::env;
use std::error;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use subprocess::{Popen, PopenConfig, Redirection};

#[derive(Default)]
struct TestVector {
    pub inputs: String,
    pub outputs: String,
}

fn run_single_test(
    asm_path: &String,
    test: &TestVector,
) -> Result<(bool, String), Box<dyn error::Error>> {
    // RARS is assumed to be in the same directory where the riscv-harness is being called from for now.
    let rars_path: String = format!("rars1_6.jar");
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
            stdout: Redirection::Pipe,
            detached: true,
            ..Default::default()
        },
    )?;

    let (out, _err) = p.communicate(Some(test.inputs.as_str()))?;
    let test_result: bool = out == Some(test.outputs.clone());

    Ok((test_result, Option::expect(out, "")))
}

fn collect_and_run_tests(
    asm_path: &String,
    test_vector_path: &String,
) -> Result<(u64, u64), Box<dyn std::error::Error>> {
    let mut num_tests: u64 = 0;
    let mut num_successful_tests: u64 = 0;

    // Collect + run tests
    for entry in fs::read_dir(test_vector_path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if !entry_path.is_file() {
            continue;
        }

        let test_name = &entry_path.file_stem().unwrap();

        // Read test vector.
        let test_file = fs::File::open(&entry_path)?;
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

        let test = TestVector {
            inputs: inputs,
            outputs: outputs,
            ..Default::default()
        };

        let (is_test_pass, test_output) = run_single_test(asm_path, &test)?;
        if is_test_pass {
            println!("{} -- PASSED", test_name.display());
            num_successful_tests = num_successful_tests + 1;
        } else {
            println!(
                "{} -- FAILED\nExpected:\n'{}'\nReceived:\n'{}'\n",
                test_name.display(),
                &test.outputs,
                test_output
            )
        }

        num_tests = num_tests + 1;
    }

    Ok((num_successful_tests, num_tests))
}

fn main() -> Result<(), Box<dyn error::Error>> {
    // Inputs:
    // riscv-harness [riscv binary] [test vector directory]
    let args: Vec<String> = env::args().collect();

    // These need to be relative paths from the location where riscv-harness is being called for now.
    let riscv_asm_path: &String = &args[1];
    let test_vector_path: &String = &args[2];

    let (num_success, num_tests) = collect_and_run_tests(riscv_asm_path, test_vector_path)?;
    println!(
        "\n\tTest Results: {}/{} of tests passed",
        num_success, num_tests
    );

    Ok(())
}
