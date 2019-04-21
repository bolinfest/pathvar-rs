mod path_var;
#[cfg(unix)]
mod path_var_unix;
#[cfg(windows)]
mod path_var_windows;

use crate::path_var::PathVar;
#[cfg(unix)]
use crate::path_var_unix::split_path_var;
#[cfg(windows)]
use crate::path_var_windows::split_path_var;
use std::env;
use std::ffi::OsString;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;

enum Command {
    Add,
    Insert,
    Usage,
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        usage();
    }

    let folder = PathBuf::from(&args[2]);
    let path_var = match parse_command(&args[1]) {
        Command::Add => {
            let mut path_var = elements_from_path_var();
            path_var.add(&folder);
            path_var
        }
        Command::Insert => {
            let mut path_var = elements_from_path_var();
            path_var.insert(&folder);
            path_var
        }
        Command::Usage => usage(),
    };

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(&path_var.to_bytes())?;
    handle.flush()?;

    Ok(())
}

fn parse_command(command: &str) -> Command {
    match command {
        "add" => Command::Add,
        "insert" => Command::Insert,
        _ => Command::Usage,
    }
}

fn elements_from_path_var() -> PathVar {
    let path_var = match std::env::var_os("PATH") {
        Some(path_var) => path_var,
        None => OsString::from(""),
    };
    let elements = split_path_var(&path_var);
    PathVar::new(elements)
}

fn usage() -> ! {
    let usage_error = r#"usage error:
    pathvar add <FOLDER>
    pathvar insert <FOLDER>"#;
    eprintln!("{}", usage_error);
    exit(1)
}
