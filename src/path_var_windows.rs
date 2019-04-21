use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

pub fn split_path_var(path_var: &OsString) -> Vec<PathBuf> {
    let wide_chars: Vec<u16> = path_var
        .encode_wide()
        .collect();
    wide_chars
        .split(|b| *b == u16::from(b';'))
        .map(|slice| PathBuf::from(OsString::from_wide(slice)))
        .collect()
}
