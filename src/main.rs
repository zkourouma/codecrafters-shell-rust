use std::{
    env::args,
    io::{self, Write},
};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut args = args();
    if let Some(cmd) = args.nth(0) {
        print!("Command not found: {cmd}");
    }
}
