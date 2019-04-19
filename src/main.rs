mod path;
mod path_elements;
#[cfg(unix)]
mod path_elements_unix;

use std::env;
use std::io::Write;
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

    let bytes = path::add(&args[2]);
    let mut stdout = std::io::stdout();
    stdout.write_all(&bytes)?;
    stdout.flush()?;
    Ok(())
}

fn usage() -> ! {
    eprintln!("usage error: pathvar add <FOLDER>");
    exit(1)
}
