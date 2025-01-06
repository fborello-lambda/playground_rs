use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

#[derive(Debug)]
pub struct DoubleLinkList<T> {
    head: Link<T>,
    tail: Link<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node<T> {
    pub data: T,
    pub next: Link<T>,
    pub prev: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            data,
            prev: None,
            next: None,
        }))
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T> Default for DoubleLinkList<T> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
}

impl<T> DoubleLinkList<T>
where
    T: Clone,
{
    pub fn push_front(&mut self, data: T) {
        let new_head = Node::new(data);

        match self.head.take() {
            Some(old) => {
                old.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        let new_tail = Node::new(data);

        match self.tail.take() {
            Some(old) => {
                old.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old);
                self.tail = Some(new_tail);
            }
            None => {
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail);
            }
        }
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.data))
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.data))
    }
}

impl<T> From<&[T]> for DoubleLinkList<T>
where
    T: Clone,
{
    fn from(value: &[T]) -> Self {
        let mut list = Self::default();
        for data in value {
            list.push_front(data.clone());
        }
        list
    }
}

impl<T> DoubleLinkList<T>
where
    T: Clone,
{
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old| {
            match old.borrow_mut().next.take() {
                Some(new) => {
                    new.borrow_mut().prev.take();
                    self.head = Some(new);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old).ok().unwrap().into_inner().data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old| {
            match old.borrow_mut().prev.take() {
                Some(new) => {
                    new.borrow_mut().next.take();
                    self.tail = Some(new);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old).ok().unwrap().into_inner().data
        })
    }
}

impl<T> Drop for DoubleLinkList<T> {
    fn drop(&mut self) {
        while self.head.is_some() {
            let head = self.head.take().unwrap();
            self.head = head.borrow_mut().next.clone();
        }
        if cfg!(test) {
            println!("DoubleLinkList Dropped!");
        }
    }
}

impl<T> Iterator for DoubleLinkList<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}

impl<T> DoubleEndedIterator for DoubleLinkList<T>
where
    T: Clone,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.pop_back()
    }
}

#[test]
fn fourth_list() {
    // If it's set to "HelloHello"
    // And we run the tests with miri,
    // we will get an allocation error
    //let binding = "HelloHello".to_string();
    let binding = "Hello".to_string();
    let chars = binding.chars();

    let vec = chars.clone().collect::<Vec<_>>();
    let mut list = DoubleLinkList::<char>::from(vec.as_slice());

    assert_eq!(*list.peek_front().unwrap(), 'o');
    assert_eq!(*list.peek_back().unwrap(), 'H');

    assert_eq!(list.pop_front(), Some('o'));
    assert_eq!(list.pop_front(), Some('l'));

    list.push_back('P');

    assert_eq!(list.pop_back(), Some('P'));
    assert_eq!(list.pop_back(), Some('H'));

    assert_eq!(list.next(), Some('l'));
    assert_eq!(list.next_back(), Some('e'));
}
