use crate::path_elements::PathElements;
use std::ffi::OsString;

pub fn add(folder: &str) -> Vec<u8> {
    let mut elements = elements_from_path_var();
    if !elements.contains(&folder.as_bytes().to_vec()) {
        elements.append(folder.as_bytes().to_vec())
    }
    elements.as_bytes()
}

fn elements_from_path_var() -> Box<PathElements<u8>> {
    let path_var = match std::env::var_os("PATH") {
        Some(path_var) => path_var,
        None => OsString::from(""),
    };
    #[cfg(unix)]
    {
        use crate::path_elements_unix::PathElementsUnix;
        Box::new(PathElementsUnix::new(path_var))
    }
    #[cfg(windows)]
    {
        panic!("Not implemented!")
    }
}
