use std::{borrow::BorrowMut, rc::Rc};

#[derive(Debug)]
pub struct PersistentList<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> PersistentList<T> {
    pub fn prepend(&self, data: T) -> Self {
        PersistentList {
            head: {
                Some(Rc::new(Node {
                    data,
                    next: self.head.clone(),
                }))
            },
        }
    }

    pub fn tail(&self) -> Self {
        PersistentList {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn iter(&self) -> ListIter<T> {
        ListIter {
            current: self.head.as_deref(),
        }
    }
}

impl<T> Drop for PersistentList<T> {
    fn drop(&mut self) {
        while self.head.is_some() {
            // Using take() sets self.head to None and returns the current head node.
            // If this was the last Rc pointing to the node, its reference count drops to 0,
            // and the memory will be freed automatically when it goes out of scope.
            let mut head = self.head.take().unwrap();
            self.head = head.borrow_mut().next.clone();
        }
        if cfg!(test) {
            println!("PersistentList Dropped!");
        }
    }
}

impl<T> From<&[T]> for PersistentList<T>
where
    T: Clone,
{
    fn from(value: &[T]) -> Self {
        let mut list = Self::default();
        for data in value {
            list = list.prepend(data.clone());
        }
        list
    }
}

impl<T> Default for PersistentList<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<T: PartialEq> PartialEq<Link<T>> for PersistentList<T> {
    fn eq(&self, other: &Link<T>) -> bool {
        match (&self.head, other) {
            (Some(a), Some(b)) => a == b,
            (None, None) => true,
            _ => false,
        }
    }
}

pub struct ListIter<'a, T> {
    current: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for ListIter<'a, T>
where
    T: Copy,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.data
        })
    }
}

#[test]
fn third_list() {
    let binding = "Hello".to_string();
    let chars = binding.chars();
    let vec = chars.clone().collect::<Vec<_>>();
    let list = PersistentList::<char>::from(vec.as_slice());

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&'o'));
    assert_eq!(iter.next(), Some(&'l'));
    assert_eq!(iter.next(), Some(&'l'));
}
