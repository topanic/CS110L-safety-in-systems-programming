use std::fmt;
use std::option::Option;



struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node::<T>{value: value, next: next}
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node::<T> {
            value: self.value.clone(),
            next: self.next.clone(),
        }
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.next == other.next
    }
}



pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}


impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList::<T> {head: None, size: 0}
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        // self.get_size::<T>() == 0  // error: expected 1 type argument, found 0 type arguments
        self.get_size() == 0
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }

    pub fn push_front(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }

}


impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
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
        LinkedList::<T> {
            head: self.head.clone(),
            size: self.size,
        }
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.size == other.size
    }
}


// define trait ComputeNorm
pub trait ComputeNorm {
    fn compute_norm(&self) -> f64 {
        0.0
    }
}

impl ComputeNorm for LinkedList<f64> {
    fn compute_norm(&self) -> f64 {
        // unimplemented!()
        self.into_iter().map(|x| x * x).sum::<f64>().sqrt()
    }
}


impl<T> Iterator for LinkedList<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}


pub struct LinkedListIterator<'a, T> {
    current : &'a Option<Box<Node<T>>>,
}


impl<T: Clone> Iterator for LinkedListIterator <'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node) => {
                self.current = &node.next;
                Some(node.value.clone())
            },
            None => None,
        }
    }
}


impl<'a, T: Clone> IntoIterator for &'a LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIterator<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {current: &self.head}
    }
}





