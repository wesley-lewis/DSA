struct Node<T: std::fmt::Display + Clone> {
    value: T,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

struct Tree<T: std::fmt::Display + Clone> {
    root: Node<T>,
}

impl<T: std::fmt::Display + Clone> Tree<T> {
    pub fn new(root: Node<T>) -> Self {
        Self {
            root,
        }
    }
    
    pub fn add_node(&mut self, value: T) {
        if self.root.left.is_none() {
            self.root.left = Some(Box::new(Node {
                value,
                left: None,
                right: None
            }));
        }
        else if self.root.right.is_none() {
            self.root.right = Some(Box::new(Node {
                value,
                left: None,
                right: None
            }));
        } else {
            if Self::insert(&mut self.root.left, value.clone()) {
                return;
            }
            else if Self::insert(&mut self.root.right, value) {
                return;
            }
            unreachable!()
        }
    }
    
    fn insert(node: &mut Option<Box<Node<T>>>, value: T) -> bool {
        match node {
            None => {
                *node = Some(Box::new(Node { value, left: None, right: None}));
                return true;
            }
            Some(node) => {
                if Self::insert(&mut node.left, value.clone()) { 
                    return true; 
                } else if Self::insert(&mut node.right, value) {
                    return true;
                }
            }
        }
        unreachable!()
    }

    pub fn print(&self) {
        if let Some(node) = &self.root.left {
            Self::print_node_internal(node);
        }
        if let Some(node) = &self.root.right {
            Self::print_node_internal(node);
        }
        println!("{}", self.root.value);
    }

    fn print_node_internal(node: &Node<T>) {
        if let Some(node) = &node.left {
            Self::print_node_internal(node);
        }
        if let Some(node) = &node.right {
            Self::print_node_internal(node);
        }
        println!("{}", node.value);
    }
}

fn main() {
    let node = Node { value: 10, left: None, right: None };
    let mut tree = Tree { 
        root: node,
    };
    tree.add_node(20);
    tree.add_node(30);
    tree.add_node(40);
    tree.print();
}
