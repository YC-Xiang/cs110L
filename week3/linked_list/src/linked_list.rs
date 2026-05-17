use std::fmt;
use std::option::Option;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {
            value: value,
            next: next,
        }
    }
}
pub trait ComputeNorm {
    fn compute_norm(&self) -> f64 {
        0.0
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }

    pub fn push_front(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}

impl<T: std::fmt::Debug> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {:?}", result, node.value);
                    current = &node.next;
                }
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        Self {
            head: self.head.as_ref().map(|node| Box::new(clone_node(node))),
            size: self.size,
        }
    }
}

fn clone_node<T: Clone>(node: &Node<T>) -> Node<T> {
    Node {
        value: node.value.clone(),
        next: node.next.as_ref().map(|n| Box::new(clone_node(n))),
    }
}

impl<T: std::cmp::PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &LinkedList<T>) -> bool {
        if self.get_size() != other.get_size() {
            return false;
        }

        let mut curr = &self.head;
        let mut other_curr = &other.head;

        loop {
            match (curr, other_curr) {
                (Some(a), Some(b)) => {
                    if a.value != b.value {
                        return false;
                    }
                    curr = &a.next;
                    other_curr = &b.next;
                }
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}

impl<T> ComputeNorm for LinkedList<T>
where
    T: Into<f64> + Copy,
{
    fn compute_norm(&self) -> f64 {
        let mut ret = 0.0;
        let mut current = &self.head;

        while let Some(node) = current {
            let val: f64 = node.value.into();
            ret += val * val;
            current = &node.next;
        }

        ret.sqrt()
    }
}
