// BTree data structure implementation placeholder
// You can implement your BTree here

pub struct BTree<T> {
    pub root : Option<Box<Node<T>>>,
}

impl <T> BTree<T> {
    pub fn new() -> Self {
        BTree { root: None }
    }
    
    
}

pub struct Node<T> {
  pub left: Option<Box<Node<T>>>,
  pub right: Option<Box<Node<T>>>,
  pub value: Option<T>, 
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            left: None,
            right: None,
            value: Some(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_node() {
        let node:   Node<i32> = Node::new(42);
        assert_eq!(node.value, Some(42));
    }
}
