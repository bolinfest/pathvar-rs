pub trait PathElements<WIDTH> {
    fn contains(&self, element: &Vec<WIDTH>) -> bool;
    fn append(&mut self, element: Vec<WIDTH>);
    fn as_bytes(&self) -> Vec<u8>;
}
