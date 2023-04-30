use std::env;
use std::process::{Command, Stdio};

fn main() {
    // Check if the CHILD_PROCESS environment variable is set
    let is_child = env::var("CHILD_PROCESS").is_ok();

    if is_child {
        // This is the child process
        println!("Hello from the child process!");
    } else {
        // This is the parent process
        println!("Hello from the parent process!");

        let current_exe = env::current_exe().expect("Failed to get current executable");

        let child_process = Command::new(current_exe)
            .env("CHILD_PROCESS", "1")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn the child process");

        // Wait for the child process to finish
        let output = child_process
            .wait_with_output()
            .expect("Failed to wait for the child process");

        // Print the output of the child process
        let child_output =
            String::from_utf8(output.stdout).expect("Failed to convert child output");
        println!("Child process output:\n{}", child_output);
    }
}
