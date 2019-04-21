mod path;
mod path_var;
#[cfg(unix)]
mod path_var_unix;
#[cfg(windows)]
mod path_var_windows;

use std::env;
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

    let path_arg = PathBuf::from(&args[2]);
    match parse_command(&args[1]) {
        Command::Add => {
            let bytes = path::add(path_arg.as_path());
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();
            handle.write_all(&bytes)?;
            handle.flush()?;
        }
        Command::Insert => panic!("TODO"),
        Command::Usage => usage(),
    }

    Ok(())
}

fn parse_command(command: &str) -> Command {
    match command {
        "add" => Command::Add,
        "insert" => Command::Insert,
        _ => Command::Usage,
    }
}

fn usage() -> ! {
    eprintln!("usage error: pathvar add <FOLDER>");
    exit(1)
}
