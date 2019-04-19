use crate::path_elements::PathElements;
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;

pub struct PathElementsUnix {
    elements: Vec<Vec<u8>>,
}

impl PathElementsUnix {
    pub fn new(path_var: OsString) -> Self {
        let elements = path_var
            .as_bytes()
            .split(|b| *b == b':')
            .map(|slice| slice.to_vec())
            .collect();
        Self { elements }
    }
}

impl PathElements<u8> for PathElementsUnix {
    fn contains(&self, element: &Vec<u8>) -> bool {
        self.elements.contains(element)
    }

    fn append(&mut self, element: Vec<u8>) {
        self.elements.push(element);
    }

    fn as_bytes(&self) -> Vec<u8> {
        let sep = b':';
        self.elements.join(&sep)
    }
}
