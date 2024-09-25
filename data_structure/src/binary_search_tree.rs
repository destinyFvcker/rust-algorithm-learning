use std::cmp::Ordering;
use std::ops::Deref;

pub struct BinarySearchTree<T>
where
    T: Ord,
{
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T> Default for BinarySearchTree<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    /// Create a new, empty BST
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }

    /// Find a value in this tree, Returns True if value is in this
    /// tree, and false otherwise.
    pub fn search(&self, value: &T) -> bool {
        match &self.value {
            Some(key) => match key.cmp(value) {
                Ordering::Equal => true,
                Ordering::Greater => match &self.left {
                    Some(node) => node.search(value),
                    None => false,
                },
                Ordering::Less => match &self.right {
                    Some(node) => node.search(value),
                    None => false,
                },
            },
            None => false,
        }
    }

    /// Retures a new iterator which iterates over this tree in order
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        BinarySearchTreeIter::new(self)
    }

    /// Insert a value into the appropriate location in the tree
    pub fn insert(&mut self, value: T) {
        if self.value.is_none() {
            self.value = Some(value);
        } else {
            match &self.value {
                None => (),
                Some(key) => {
                    let target_node = if value < *key {
                        &mut self.left
                    } else {
                        &mut self.right
                    };
                    match target_node {
                        Some(ref mut node) => {
                            node.insert(value);
                        }
                        None => {
                            let mut node = BinarySearchTree::new();
                            node.insert(value);
                            *target_node = Some(Box::new(node));
                        }
                    }
                }
            }
        }
    }

    /// Retures the smallest value in this tree
    pub fn minimum(&self) -> Option<&T> {
        match &self.left {
            Some(node) => node.minimum(),
            None => match &self.value {
                Some(value) => Some(value),
                None => None,
            },
        }
    }

    /// Retures the largest value in this tree
    pub fn maximum(&self) -> Option<&T> {
        match &self.right {
            Some(node) => node.maximum(),
            None => match &self.value {
                Some(value) => Some(value),
                None => None,
            },
        }
    }

    /// Retures the largest value in this tree smaller than value
    pub fn floor(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(key) => match key.cmp(value) {
                Ordering::Greater => match &self.left {
                    Some(node) => node.floor(value),
                    None => None,
                },
                Ordering::Less => match &self.right {
                    Some(node) => {
                        let val = node.floor(value);
                        match val {
                            Some(_) => val,
                            None => Some(key),
                        }
                    }
                    None => Some(key),
                },
                Ordering::Equal => Some(key),
            },
            None => None,
        }
    }

    /// Returns the smallest value in this tree larger than value
    pub fn ceil(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(key) => match key.cmp(value) {
                Ordering::Less => match &self.right {
                    Some(node) => node.ceil(value),
                    None => None,
                },
                Ordering::Greater => match &self.left {
                    Some(node) => {
                        let val = node.ceil(value);
                        match val {
                            Some(_) => val,
                            None => Some(key),
                        }
                    }
                    None => Some(key),
                },
                Ordering::Equal => Some(key),
            },
            None => None,
        }
    }
}

struct BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    stack: Vec<&'a BinarySearchTree<T>>,
}

impl<'a, T> BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    pub fn new(tree: &BinarySearchTree<T>) -> BinarySearchTreeIter<T> {
        let mut iter = BinarySearchTreeIter { stack: vec![tree] };
        iter.stack_push_left();
        iter
    }

    fn stack_push_left(&mut self) {
        while let Some(child) = &self.stack.last().unwrap().left {
            self.stack.push(child);
        }
    }
}

impl<'a, T> Iterator for BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            None
        } else {
            let node = self.stack.pop().unwrap();
            if node.right.is_some() {
                self.stack.push(node.right.as_ref().unwrap().deref());
                self.stack_push_left();
            }
            node.value.as_ref()
        }
    }
}
