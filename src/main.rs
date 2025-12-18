use std::io::{self, Write, stdin};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .expect("Unable to read user input");

        let (cmd, args) = input.trim().split_once(char::is_whitespace).unwrap();

        match cmd {
            "exit" => break,
            "echo" => println!("{}", args),
            _ => println!("{cmd}: command not found"),
        }
    }
}
