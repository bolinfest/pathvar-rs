use std::path::Path;
use std::path::PathBuf;

pub struct PathVar {
    elements: Vec<PathBuf>,
}

impl PathVar {
    pub fn new(elements: Vec<PathBuf>) -> Self {
        Self { elements }
    }

    pub fn contains(&self, element: &Path) -> bool {
        for path_buf in &self.elements {
            if path_buf.as_path() == element {
                return true;
            }
        }
        false
    }

    pub fn append(&mut self, element: &Path) {
        self.elements.push(element.to_path_buf());
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        #[cfg(unix)]
        let sep = { b':' };
        #[cfg(windows)]
        let sep = { b';' };
        let elements: Vec<&[u8]> = self
            .elements
            .iter()
            .map(|path| path_to_bytes(path))
            .collect();
        elements.join(&sep)
    }
}

#[cfg(unix)]
fn path_to_bytes(path: &Path) -> &[u8] {
    // Prefer the use of raw bytes in case one of the PathBufs contains
    // non-UTF-8 characters.
    use std::os::unix::ffi::OsStrExt;
    path.as_os_str().as_bytes()
}

#[cfg(windows)]
fn path_to_bytes(path: &Path) -> &[u8] {
    // Note that a Path is backed by an OsString, which on Windows, is UTF-16
    // encoded. If it cannot be encoded as UTF-8, we will not be able to
    // properly reproduce the original %PATH% with this implementation.
    //
    // To fix this, we probably need to use encode_wide() from
    // std::os::windows::ffi::OsStrExt.
    path.to_str().unwrap().as_bytes()
}
