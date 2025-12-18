use std::io::{self, Write, stdin};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .expect("Unable to read user input");

        let (cmd, args) = parse_cmd(&input);

        match cmd {
            "exit" => break,
            "echo" => println!("{}", args.join(" ")),
            _ => println!("{cmd}: command not found"),
        }
    }
}

fn parse_cmd<'a>(input: &'a str) -> (&'a str, Vec<&'a str>) {
    let inputs = input.trim().split(" ").collect::<Vec<&str>>().split_off(0);

    if inputs.len() < 2 {
        (inputs[0], Vec::new())
    } else {
        (inputs[0], inputs[1..].to_vec())
    }
}
