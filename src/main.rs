use std::{
    env,
    fs::read_dir,
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
    let path_cmd = format!("{cmd}");
    env::var_os("PATH")
        .and_then(|path_var| path_var.into_string().ok())
        .and_then(|path| {
            let greedy_path = path.split(":").collect::<Vec<&str>>().into_iter().rev();
            for dir in greedy_path {
                if let Ok(dir_entries) = read_dir(dir) {
                    for dir_entry in dir_entries.flat_map(|d| d) {
                        if dir_entry.path().ends_with(&path_cmd) {
                            if let Ok(metadata) = dir_entry.metadata() {
                                if metadata.permissions().mode() & 0x111 != 0 {
                                    return Some(dir_entry.path());
                                }
                            }
                        }
                    }
                }
            }
            None
        })
}
