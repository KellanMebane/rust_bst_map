#[derive(Clone, Copy)]
pub struct GPair<K, V>(K, V);

#[derive(Clone)]
pub struct GBST<K, V> {
    size: usize,                   // number of elements currently in the tree
    capacity: usize, // size of vector needed, this growns with the tree as it get's deeper
    vec: Vec<Option<GPair<K, V>>>, // vector of optional indeces containing either none or some pair of character and string
}

#[allow(dead_code)]
impl<K, V> GBST<K, V> {
    pub fn new() -> GBST<K, V>
    where
        K: Clone,
        V: Clone,
    {
        GBST {
            size: 0,
            capacity: 127, // enough for depth of 6 initially
            vec: vec![None; 127],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod test {
    use super::GPair;
    use super::GBST;

    #[test]
    fn is_empty() {
        let mut bst: GBST<i32, i32> = GBST::new();
        assert_eq!(true, bst.is_empty());
        //bst.insert(Pair::new('A', "hello".to_string()));
    }

    #[test]
    fn size() {
        let mut bst: GBST<i32, i32> = GBST::new();
        assert_eq!(0, bst.size());
    }
}
