use std::io::{stdin, stdout, Write};
use std::process::{Child, Command, Stdio};
mod commands;
use commands::*;
use custom_commands::rcat::rcat; // Assuming the custom library is named "custom"

fn handle_command(command: &str, args: &[&str], previous_result: Option<Child>) -> Option<Child> {
    match command {
        "exit" => {
            exit(args);
            None
        }
        "cd" => {
            cd(args);
            None
        }
        "echo" => {
            echo(args);
            None
        }
        "rcat" => {
            rcat(&args);

            None
        }
        _ => {
            let stdin = previous_result.map_or(Stdio::inherit(), |output: Child| {
                Stdio::from(output.stdout.unwrap())
            });

            let stdout = if args.is_empty() {
                Stdio::inherit()
            } else {
                Stdio::piped()
            };

            let output = Command::new(command)
                .args(args)
                .stdin(stdin)
                .stdout(stdout)
                .spawn();

            match output {
                Ok(output) => Some(output),
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
            }
        }
    }
}

fn main() {
    loop {
        // Start the prompt.
        print!("🔥 ");
        stdout().flush().unwrap();

        // Read the input.
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // Trim the input to remove trailing newline and leading/trailing whitespace.
        let input = input.trim();

        if input.is_empty() {
            continue; // Skip the iteration if the input is empty.
        }

        // Split the input into the different commands using the "pipe".
        let mut commands = input.split(" | ").peekable();
        let mut previous_result = None;

        // Handle the commands in the correct order.
        while let Some(full_command) = commands.next() {
            // Parse the commands.
            let mut parts = full_command.split_whitespace();
            let command = parts.next().unwrap();
            let args: Vec<_> = parts.collect();

            // Execute the command.
            previous_result = handle_command(command, &args, previous_result);
        }

        // Wait for the commands to complete.
        if let Some(mut final_result) = previous_result {
            final_result.wait().unwrap();
        }
    }
}
