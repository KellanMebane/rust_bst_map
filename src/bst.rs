// array based BST (SEQUENTIAL)

#[derive(Clone)]
pub struct Pair(char, String);

#[allow(dead_code)]
impl Pair {
    pub fn new(c: char, s: String) -> Pair {
        Pair { 0: c, 1: s }
    }
}

pub struct BST {
    size: usize,
    capacity: usize,
    vec: Vec<Option<Pair>>, // vector of optional indeces containing
                            // either none or some pair of character and string
}

// parent(index) = [(index - 1) / 2] if r != 0
// left(index) = 2(index) + 1 if 2(index) + 1 <= size
// right(index) = 2(index) + 2 if 2(index + 2) <= size
#[allow(dead_code)]
impl BST {
    pub fn new() -> BST {
        BST {
            size: 0,
            capacity: 100,
            vec: vec![None; 100],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn insert(&mut self, item: Pair) -> bool {
        self.internal_insert(0, item)
    }

    // basically checks if the index is used
    // returns the char at the index or null char
    fn get_char(&self, index: usize) -> char {
        match self.vec[index].as_ref().as_ref() {
            Some(value) => value.0,
            None => 0 as char, // null I think lmao
        }
    }

    fn get_string(&self, index: usize) -> String {
        match self.vec[index].as_ref().as_ref() {
            Some(value) => {
                let x = (&value.1).to_string();
                //"cool".to_string()
                //println!("{}", x);
                x
            }
            None => "".to_string(), // null I think lmao
        }
    }

    fn internal_insert(&mut self, index: usize, item: Pair) -> bool {
        let key = self.get_char(index);

        // this means we have an unused index
        // insert here
        if key == 0 as char {
            self.vec[index] = Some(item);
            self.size += 1;
            return true;
        }

        if item.0 == key {
            return false;
        } else if item.0 > key {
            let mut new_index = 0;
            if (2 * index) + 2 <= self.capacity {
                new_index = (2 * index) + 2;
            }
            return self.internal_insert(new_index, item);
        } else {
            let mut new_index = 0;
            if (2 * index) + 1 <= self.capacity {
                new_index = (2 * index) + 1;
            }
            return self.internal_insert(new_index, item);
        }
    }

    pub fn inorder(&self) {
        self.internal_inorder(0);
    }

    fn internal_inorder(&self, index: usize) {
        let key = self.get_char(index);

        if key == 0 as char {
            return;
        }

        self.internal_inorder((2 * index) + 1);
        self.internal_inorder((2 * index) + 2);
    }

    pub fn search(&self, letter: char) -> String {
        self.internal_search(0, letter)
    }

    fn internal_search(&self, index: usize, letter: char) -> String {
        let key = self.get_char(index);
        let x = self.get_string(index);

        let item = Pair::new(letter, "".to_string());

        // we can't find it
        // return empty string
        if key == 0 as char {
            return item.1;
        }

        if item.0 == key {
            return x;
        } else if item.0 > key {
            let mut new_index = 0;
            if (2 * index) + 2 <= self.capacity {
                new_index = (2 * index) + 2;
            }
            return self.internal_search(new_index, letter);
        } else {
            let mut new_index = 0;
            if (2 * index) + 1 <= self.capacity {
                new_index = (2 * index) + 1;
            }
            return self.internal_search(new_index, letter);
        }
    }
}

#[cfg(test)]
mod test {
    use super::BST;
    use super::Pair;

    #[test]
    fn is_empty() {
        let mut bst = BST::new();
        assert_eq!(true, bst.is_empty());
        bst.insert(Pair::new('A', "hello".to_string()));
        assert_eq!(false, bst.is_empty());
    }
}
