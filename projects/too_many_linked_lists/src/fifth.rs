use std::ptr;

#[derive(Debug)]
pub struct Queue<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

// There is a problem when mixing Box and raw pointers
//type Link<T> = Option<Box<Node<T>>>;
type Link<T> = *mut Node<T>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

// head -> node -> ... -> old_tail -> new_tail
impl<T> Queue<T> {
    pub fn push(&mut self, data: T) {
        unsafe {
            let new = Node {
                data,
                next: ptr::null_mut(),
            };

            let raw_tail = Box::into_raw(Box::new(new));

            match self.tail.is_null() {
                false => (*self.tail).next = raw_tail,
                true => self.head = raw_tail,
            }

            self.tail = raw_tail
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                return None;
            }

            let head = Box::from_raw(self.head);
            self.head = head.next;

            if self.head.is_null() {
                self.tail = ptr::null_mut();
            }
            Some(head.data)
        }
    }

    pub fn iter(&self) -> QueueIter<'_, T> {
        unsafe {
            QueueIter {
                current: self.head.as_ref(),
            }
        }
    }

    pub fn iter_mut(&mut self) -> QueueIterMut<'_, T> {
        unsafe {
            QueueIterMut {
                current: self.head.as_mut(),
            }
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
        if cfg!(test) {
            println!("Queue Dropped!");
        }
    }
}

impl<T> From<&[T]> for Queue<T>
where
    T: Copy,
{
    fn from(value: &[T]) -> Self {
        let mut queue = Self::default();
        for data in value {
            queue.push(*data);
        }
        queue
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }
}

pub struct QueueIter<'a, T> {
    current: Option<&'a Node<T>>,
}
impl<T> Iterator for QueueIter<'_, T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.current
                .map(|node| {
                    self.current = node.next.as_ref();
                    &node.data
                })
                .copied()
        }
    }
}

pub struct QueueIterMut<'a, T> {
    current: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for QueueIterMut<'a, T>
where
    T: Copy,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.current.take().map(|node| {
                self.current = node.next.as_mut();
                &mut node.data
            })
        }
    }
}

// It consumes the structure
impl<T> Iterator for Queue<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[test]
fn fifth_list() {
    let binding = "HelloWorld".to_string();
    let chars = binding.chars();
    let vec = chars.clone().collect::<Vec<_>>();
    let mut queue = Queue::<char>::from(vec.as_slice());

    for c in chars.clone() {
        assert_eq!(queue.pop(), Some(c));
    }

    for c in chars.clone() {
        queue.push(c);
    }
    for (list_data, char) in queue.iter().zip(chars.clone()) {
        assert_eq!(list_data, char)
    }

    let chars_plus_one = chars.map(|x| (x as u8 + 1) as char).collect::<Vec<_>>();

    for c in queue.iter_mut() {
        *c = (*c as u8 + 1) as char
    }

    for (list_data, char) in queue.iter().zip(chars_plus_one.iter()) {
        assert_eq!(list_data, *char)
    }
}
