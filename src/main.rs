use std::io::{stdin, stdout, Write};
use std::process::{Child, Command, Stdio};
mod commands;
use commands::{cd, echo};

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

            // Match the command to a predefined command or what we can pass through to process.
            match command {
                // exit the shell.
                "exit" => return,
                // cd defined just like the bash specification.
                "cd" => {
                    cd(&args);
                    previous_result = None;
                }
                // Handle the command if no match was found.
                "echo" => {
                    echo(&args);
                    previous_result = None;
                }
                command => {
                    let stdin = previous_result.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    // Check if another command comes after this one, and if so pipe it to the next, or finalize with an inherit.
                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            previous_result = Some(output);
                        }
                        Err(e) => {
                            previous_result = None;
                            eprintln!("{}", e);
                        }
                    };
                }
            }
        }

        // Wait for the commands to complete.
        if let Some(mut final_result) = previous_result {
            final_result.wait().unwrap();
        }
    }
}
