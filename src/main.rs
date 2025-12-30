use std::{
    env::{current_dir, set_current_dir, split_paths, var},
    fs,
    io::{Write, stderr, stdin, stdout},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
};

const BUILTINS: &[&str; 5] = &["cd", "echo", "exit", "pwd", "type"];

fn main() {
    loop {
        print!("$ ");
        stdout().flush().unwrap();

        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .expect("Unable to read user input");

        let (cmd, args) = parse_cmd(&input);

        match cmd {
            "cd" => try_change_dir(args.first().map(|arg| *arg)),
            "echo" => println!("{}", args.join(" ")),
            "exit" => break,
            "pwd" => {
                current_dir()
                    .map(|d| println!("{}", d.to_string_lossy()))
                    .ok();
            }
            "type" => has_type(args.first().map(|arg| *arg)),
            _ => try_cmd(cmd, args),
        }
    }
}

fn try_change_dir(arg: Option<&str>) {
    match arg {
        Some("~") => change_to_home_dir(),
        Some(dir) => {
            let path = Path::new(dir);
            if path.is_dir() {
                set_current_dir(path).ok();
            } else {
                println!("cd: {dir}: No such file or directory");
            }
        }
        None => change_to_home_dir(),
    };
}

fn change_to_home_dir() {
    let home = var("HOME").unwrap_or_default();
    let path = Path::new(home.as_str());
    set_current_dir(path).ok();
}

fn try_cmd(cmd: &str, args: Vec<&str>) {
    if find_in_path(cmd).is_some() {
        let _ = Command::new(cmd)
            .args(&args)
            .output()
            .map(|output| {
                stdout().write(&output.stdout)?;
                stderr().write(&output.stderr)
            })
            .map_err(|e| eprintln!("{}", &e));
    } else {
        println!("{cmd}: command not found");
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

fn has_type(cmd: Option<&str>) {
    if let Some(cmd) = cmd {
        if BUILTINS.contains(&cmd) {
            println!("{cmd} is a shell builtin");
        } else if let Some(executable) = find_in_path(cmd) {
            let exec_path = executable.to_str().unwrap_or_default();
            println!("{cmd} is {exec_path}");
        } else {
            println!("{cmd}: not found");
        }
    } else {
        println!("No command to type");
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
