use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::{
    env,
    io::{stdin, stdout, Write},
};

fn main() {
    loop {
        // Start the prompt.
        print!("🔥 ");
        stdout().flush().unwrap();

        // Read the input.
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // Split the input into the different commands using the "pipe".
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_result = None;

        // Handle the commands in the correct order.
        while let Some(full_command) = commands.next() {
            // Parse the commands.
            let mut parts = full_command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            // Match the command to a predefined command or what we can pass through to process.
            match command {
                // exit the shell.
                "exit" => return,
                // cd defined just like the bash specification.
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_result = None;
                }
                // Handle the command if no match was found.
                command => {
                    let stdin = previous_result.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    // Check if another command comes after this one, and if so pipe it to the next, or finalise with an inherit.
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
