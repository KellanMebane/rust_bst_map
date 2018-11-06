#[derive(Clone, Copy)]
pub struct GPair<K, V>(K, V);

impl<K, V> GPair<K, V> {
    pub fn new(key: K, val: V) -> GPair<K, V> {
        GPair { 0: key, 1: val }
    }
}

#[derive(Clone)]
pub struct GBST<K, V> {
    size: usize,            // number of elements currently in the tree
    capacity: usize,        // size of vector needed, this growns with the tree as it get's deeper
    current: Option<usize>, //index of current iterator node
    stack: Vec<usize>,
    first: bool,
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
            current: Some(0),
            stack: Vec::new(),
            first: true,
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

    pub fn get_key(&self, index: usize) -> Option<&K> {
        match self.vec[index].as_ref().as_ref() {
            Some(pair) => Some(&pair.0),
            None => None,
        }
    }

    fn get_val(&self, index: usize) -> Option<&V> {
        match self.vec[index].as_ref().as_ref() {
            Some(pair) => Some(&pair.1),
            None => None,
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> bool
    where
        K: PartialEq + PartialOrd,
    {
        self.internal_insert(0, GPair::new(key, val))
    }

    fn internal_insert(&mut self, index: usize, pair: GPair<K, V>) -> bool
    where
        K: PartialEq + PartialOrd,
    {
        if index > self.capacity {
            return false;
        }

        let is_empty_leaf: bool;
        {
            is_empty_leaf = self.get_key(index).is_none();
        }

        if is_empty_leaf {
            self.vec[index] = Some(pair);
            self.size += 1;
            return true;
        }

        let mut new_index = 0;
        {
            let key: &K = self.get_key(index).unwrap();

            if pair.0 == *key {
                return false;
            } else if pair.0 > *key {
                if (2 * index) + 2 <= self.capacity {
                    new_index = (2 * index) + 2;
                } else {
                    return false;
                }
            } else {
                if (2 * index) + 1 <= self.capacity {
                    new_index = (2 * index) + 1;
                } else {
                    return false;
                }
            }
        }
        return self.internal_insert(new_index, pair);
    }

    pub fn inorder(&self) -> Vec<usize> {
        let mut ordered: Vec<usize> = Vec::new();
        self.internal_inorder(&mut ordered);
        ordered
    }

    pub fn internal_inorder(&self, ordered: &mut Vec<usize>) {
        // inorder and push indeces into vec
    }

    pub fn iter(&mut self) -> Option<&K> {
        match self.next() {
            Some(x) => self.get_key(x),
            None => None,
        }
    }
}

impl<K, V> Iterator for GBST<K, V> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.current.is_some() || !self.stack.is_empty() || self.first {
            self.first = false;
            while self.current != None {
                self.stack.push(self.current.unwrap());
                let x = self.current.unwrap();
                let y = x * 2 + 1;
                if self.get_key(y).is_some() {
                    self.current = Some(y);
                } else {
                    self.current = None;
                }
            }
            self.current = self.stack.pop();

            let temp = self.current.unwrap();
            let x = self.current.unwrap();
            let y = x * 2 + 2;
            if self.get_key(y).is_some() {
                self.current = Some(y);
            } else {
                self.current = None;
            }
            return Some(temp);
        }

        None
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

    #[test]
    fn insert() {
        let mut bst: GBST<i32, i32> = GBST::new();
        assert_eq!(0, bst.size());
        bst.insert(12, 12);
        assert_eq!(1, bst.size());
    }

    #[test]
    fn iter() {
        let mut bst: GBST<&str, i32> = GBST::new();

        bst.insert("Beetle", 0);
        bst.insert("Cardinal", 0);
        bst.insert("Giraffe", 0);
        bst.insert("Kangaroo", 0);
        bst.insert("Tiger", 0);
        bst.insert("Aardvark", 0);
        bst.insert("Dog", 0);

        println!("\n\nsize is: {}", bst.size());
        
        loop {
            match bst.iter() {
                Some(x) => {
                    print!("{} ", x);
                }
                None => {
                    println!("");
                    break;
                }
            }
        }
    }
}
