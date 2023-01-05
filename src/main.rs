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

        // Input
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_result = None;

        while let Some(full_command) = commands.next() {
            let mut parts = full_command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_result = None;
                }
                "exit" => return,
                command => {
                    let stdin = previous_result.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        // there is another command piped behind this one
                        // prepare to send output to the next command
                        Stdio::piped()
                    } else {
                        // there are no more commands piped behind this one
                        // send output to shell stdout
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

        if let Some(mut final_result) = previous_result {
            // block until the final command has finished
            final_result.wait().unwrap();
        }
    }
}
