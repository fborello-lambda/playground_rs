use std::mem;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

#[derive(Debug, Clone)]
enum Link<T> {
    Nil,
    Cons(Box<Node<T>>),
}

#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn push(&mut self, data: T) {
        let new_node = Node {
            data,
            next: mem::replace(&mut self.head, Link::Nil),
        };

        self.head = Link::Cons(Box::new(new_node));
    }

    pub fn pop_node(&mut self) -> bool {
        match mem::replace(&mut self.head, Link::Nil) {
            Link::Nil => false,
            Link::Cons(node) => {
                self.head = node.next;
                true
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_node() {}
        if cfg!(test) {
            println!("List Dropped!");
        }
    }
}

impl<T> List<T>
where
    T: Copy,
{
    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, Link::Nil) {
            Link::Nil => None,
            Link::Cons(node) => {
                self.head = node.next.clone();
                Some(node.data)
            }
        }
    }
}

impl<T> From<Vec<T>> for List<T> {
    fn from(value: Vec<T>) -> Self {
        let mut list = Self::default();
        for data in value {
            list.push(data);
        }
        list
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self { head: Link::Nil }
    }
}

impl<T: PartialEq> PartialEq<Link<T>> for List<T> {
    fn eq(&self, other: &Link<T>) -> bool {
        &self.head == other
    }
}

impl<T: PartialEq> PartialEq for Link<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Link::Nil, Link::Nil) => true,
            (Link::Cons(node_a), Link::Cons(node_b)) => {
                node_a.data == node_b.data && node_a.next == node_b.next
            }
            _ => false,
        }
    }
}

#[test]
fn first_list() {
    let binding = "Hello".to_string();
    let chars = binding.chars();
    let mut list: List<char> = List::from(chars.clone().collect::<Vec<_>>());
    for c in chars.rev() {
        assert_eq!(list.pop(), Some(c));
    }
    assert_eq!(list, Link::Nil);
}
