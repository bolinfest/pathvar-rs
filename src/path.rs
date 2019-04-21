use crate::path_var::PathVar;
use std::ffi::OsString;
#[cfg(unix)]
use crate::path_var_unix::split_path_var;
#[cfg(windows)]
use crate::path_var_windows::split_path_var;
use std::path::Path;

pub fn add(folder: &Path) -> Vec<u8> {
    let mut elements = elements_from_path_var();
    if !elements.contains(folder) {
        elements.append(folder)
    }
    elements.to_bytes()
}

fn elements_from_path_var() -> PathVar {
    let path_var = match std::env::var_os("PATH") {
        Some(path_var) => path_var,
        None => OsString::from(""),
    };
    let elements = split_path_var(&path_var);
    PathVar::new(elements)
}
