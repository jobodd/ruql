#![allow(non_snake_case)]

use core::error;
use std::{char::MAX, collections::btree_map::Keys};

pub struct BTree<K: Ord, V> {
    root: Box<Node<K, V>>,
}

impl<K: Ord + Clone, V: Clone> BTree<K, V> {
    pub fn new() -> Self {
        BTree {
            root: Box::new(Node::<K, V>::new(true)),
        }
    }

    pub fn findNode(&mut self, key: K) -> Option<&mut Node<K, V>> {
        // special case where the root node may be empty.
        // no other nodes should ever be empty
        if self.root.nKeys == 0 {
            return Some(&mut self.root);
        }

        return self.root.findNode(key);
    }

    pub fn insert(&mut self, key: K, value: V) {
        match self.findNode(key.clone()) {
            Some(node) => {
                node.insert(key, value);
            }
            None => {
                todo!("Assert this never happens?");
            }
        }
    }

    pub fn get(&mut self, key: K) -> Option<&V> {
        match self.findNode(key.clone()) {
            Some(node) => {
                return node.get(key);
            }
            None => {
                todo!("Assert this never happens?");
            }
        }
    }
}

pub const M: usize = 2;
pub const MAX_NODE_KEYS: usize = 2 * M - 1;
pub struct Node<K: Ord, V> {
    pub isLeafNode: bool,
    pub slots: usize,
    pub nKeys: usize,
    pub keys: [Option<K>; 2 * M],
    pub values: [Option<V>; 2 * M],
    pub children: Box<[Option<Box<Node<K, V>>>; 2 * M]>,
}

impl<K: Ord + Clone, V: Clone> Node<K, V> {
    pub fn new(isLeaf: bool) -> Self {
        Node {
            isLeafNode: isLeaf,
            slots: 2 * M,
            nKeys: 0,
            keys: [const { None }; 2 * M],
            values: [const { None }; 2 * M],
            children: Box::new([const { None }; 2 * M]),
        }
    }

    pub fn findNode(&mut self, key: K) -> Option<&mut Node<K, V>> {
        if self.isLeafNode {
            return Some(self);
        } else {
            // todo: make this a binary search
            let mut childIndex = self.nKeys - 1;
            for i in 0..self.nKeys {
                if let Some(ref k) = self.keys[i] {
                    if key < *k {
                        childIndex = i;
                        break;
                    }
                }
            }

            return self.children[childIndex]
                .as_deref_mut()
                .and_then(|child| child.findNode(key));
        }
    }

    pub fn get(&mut self, key: K) -> Option<&V> {
        // todo: replace with binary search
        for i in 0..self.nKeys {
            if let Some(ref k) = self.keys[i] {
                if key == *k {
                    return self.values[i].as_ref();
                }
            }
            return None;
        }
        return None;
    }

    pub fn insert(&mut self, key: K, value: V) {
        let mut insertIndex = 0;
        // Find the position to insert the key
        for i in 0..self.nKeys {
            if let Some(ref k) = self.keys[i] {
                if key < *k {
                    insertIndex = i;
                    break;
                }
                insertIndex = i + 1;
            }
        }

        if insertIndex <= self.nKeys {
            for i in (insertIndex..self.nKeys).rev() {
                self.keys[i + 1] = self.keys[i].clone();
                self.values[i + 1] = self.values[i].clone();
            }
            self.keys[insertIndex] = Some(key);
            self.values[insertIndex] = Some(value);
        }

        self.nKeys += 1;

        // check if we need to split the node
        if self.nKeys > MAX_NODE_KEYS {
            let mut leftNode = Node::<K, V>::new(true);
            let mut rightNode = Node::<K, V>::new(true);

            for i in 0..M {
                leftNode.keys[i] = self.keys[i].take();
                leftNode.values[i] = self.values[i].take();
            }
            leftNode.nKeys = M;

            for i in M..self.nKeys {
                rightNode.keys[i - M] = self.keys[i].take();
                rightNode.values[i - M] = self.values[i].take();
            }
            rightNode.nKeys = M;

            self.isLeafNode = false;
            self.keys[0] = rightNode.keys[0].clone();
            self.children[0] = Some(Box::new(leftNode));
            self.children[1] = Some(Box::new(rightNode));
            self.nKeys = 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_empty() {
        let mut b = BTree::<i32, i32>::new();
        let actual = b.findNode(10);
        assert!(actual.is_some());
    }

    #[test]
    fn test_insert() {
        let mut b = BTree::<i32, i32>::new();
        b.insert(10, 10);
        let actual = b.get(10);
        assert_eq!(actual, Some(&10));
    }

    #[test]
    fn test_insert_unordered() {
        let mut b = BTree::<i32, i32>::new();
        b.insert(20, 20);
        b.insert(10, 10);
        let actual = b.get(10);
        assert_eq!(actual, Some(&10));
    }

    #[test]
    fn test_insert_overflow_root_node() {
        let mut b = BTree::<i32, i32>::new();
        b.insert(10, 10);
        b.insert(20, 20);
        b.insert(5, 5);
        b.insert(6, 6);
        b.insert(12, 12);
        b.insert(30, 30);
        b.insert(7, 7);
        b.insert(17, 17);

        let actual = b.get(10);
        assert_eq!(actual, Some(&100));
    }
}
