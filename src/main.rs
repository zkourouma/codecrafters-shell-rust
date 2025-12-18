use std::io::{self, Write, stdin};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .expect("Unable to read user input");
        let cmd = input.trim();
        if !cmd.is_empty() {
            print!("{cmd}: command not found");
        }
    }
}
