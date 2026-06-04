use std::io::{self, Write};
use std::process::Command;
use std::env;

fn main() {
    loop {
        let current_dir = env::current_dir().unwrap();
        print!("{} $ ", current_dir.display());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        // handle cd separately
        if command == "cd" {
            let path = args.get(0).map(|s| *s).unwrap_or(".");
            if let Err(e) = env::set_current_dir(path) {
                println!("cd: {}", e);
            }
            continue;
        }

        match Command::new(command).args(&args).spawn() {
            Ok(mut child) => { child.wait().unwrap(); },
            Err(_) => println!("{}: command not found", command),
        }
    }
}