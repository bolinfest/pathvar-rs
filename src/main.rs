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

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        usage();
    }
    // TODO: Support other subcommands.
    if &args[1] != "add" {
        usage();
    }

    let path_arg = PathBuf::from(&args[2]);
    let bytes = path::add(path_arg.as_path());
    let mut stdout = std::io::stdout();
    stdout.write_all(&bytes)?;
    stdout.flush()?;
    Ok(())
}

fn usage() -> ! {
    eprintln!("usage error: pathvar add <FOLDER>");
    exit(1)
}
