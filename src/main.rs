use std::{
    env::{split_paths, var},
    fs,
    io::{self, Write, stdin},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
};

const BUILTINS: &[&str; 3] = &["exit", "echo", "type"];

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
            "type" => has_type(args),
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

fn has_type(cmds: Vec<&str>) {
    if cmds.len() < 1 {
        println!("command not found");
    } else if BUILTINS.contains(&cmds[0]) {
        let cmd = cmds[0];
        println!("{cmd} is a shell builtin");
    } else if let Some(executable) = find_in_path(cmds[0]) {
        let cmd = cmds[0];
        let exec_path = executable.to_str().unwrap_or_default();
        println!("{cmd} is {exec_path}");
    } else {
        let cmd = cmds[0];
        println!("{cmd}: not found");
    }
}

fn find_in_path(cmd: &str) -> Option<PathBuf> {
    var("PATH").ok().and_then(|path| {
        split_paths(&path)
            .map(|dir| dir.join(cmd))
            .find(|candidate| {
                candidate.exists()
                    && fs::metadata(candidate)
                        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
                        .unwrap_or_default()
            })
    })
}
