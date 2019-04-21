use std::ffi::OsStr;
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

pub fn split_path_var(path_var: &OsString) -> Vec<PathBuf> {
    path_var
        .as_bytes()
        .split(|b| *b == b':')
        .map(|slice| PathBuf::from(OsStr::from_bytes(slice)))
        .collect()
}
