use std::{cmp::max, fmt::Display};

pub struct BST<T> {
    pub root: Option<Box<Node<T>>>,
}

pub struct Node<T> {
    val: T,
    height: usize,
    count: usize,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn from_val(val: T) -> Self {
        return Self {
            val,
            height: 1,
            count: 1,
            left: None,
            right: None,
        };
    }
    pub fn boxed_from_val(val: T) -> Box<Self> {
        return Box::new(Self::from_val(val));
    }
}

impl<T> Default for BST<T> {
    fn default() -> Self {
        return Self { root: None };
    }
}

impl<T> BST<T>
where
    T: Ord + Display,
{
    pub fn new() -> Self {
        return Default::default();
    }

    #[allow(dead_code)]
    pub fn from_val(val: T) -> Self {
        return Self {
            root: Some(Node::boxed_from_val(val)),
        };
    }

    pub fn insert_node(&mut self, new_val: T) {
        Self::push_node(Node::boxed_from_val(new_val), &mut self.root);
    }

    fn push_node(new_node: Box<Node<T>>, current_node: &mut Option<Box<Node<T>>>) {
        if let Some(node) = current_node {
            use std::cmp::Ordering;
            match node.val.cmp(&new_node.val) {
                Ordering::Greater => Self::push_node(new_node, &mut node.left),
                Ordering::Less => Self::push_node(new_node, &mut node.right),
                Ordering::Equal => node.count = node.count + 1,
            };
            node.height = max(
                node.left.as_ref().map_or(0, |a| a.as_ref().height) + 1,
                node.right.as_ref().map_or(0, |a| a.as_ref().height) + 1,
            );
        } else {
            let _ = current_node.insert(new_node);
        }
    }

    fn inorder_recursive<F>(&self, boxed_node: &Option<Box<Node<T>>>, cb: &F)
    where
        F: Fn(&Box<Node<T>>) + Clone,
    {
        if let Some(node) = boxed_node {
            self.inorder_recursive(&node.left, cb);
            let _ = cb(node);
            self.inorder_recursive(&node.right, cb);
        } else {
        }
        return;
    }

    pub fn inorder<F>(&self, cb: F)
    where
        F: Fn(&Box<Node<T>>) + Clone,
    {
        return self.inorder_recursive(&self.root, &cb);
    }
}

// struct OptionBox<'a, T>(&'a Option<Box<T>>);
//
// impl<T> Display for OptionBox<'_, T>
// where
//     T: Display,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match &self.0 {
//             Some(inner) => inner.fmt(f),
//             None => write!(f, ""),
//         }
//     }
// }
//
// fn display<T>(x: &Option<Box<T>>) -> OptionBox<T> {
//     OptionBox(x)
// }
//

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})({})", self.val, self.count, self.height)
    }
}
