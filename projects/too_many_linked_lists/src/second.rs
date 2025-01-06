#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn push(&mut self, data: T) {
        let new_node = Node {
            data,
            next: self.head.take(),
        };

        self.head = Some(Box::new(new_node));
    }

    pub fn pop_node(&mut self) -> bool {
        match self.head.take() {
            None => false,
            Some(node) => {
                self.head = node.next;
                true
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|v| &v.data)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|v| &mut v.data)
    }

    pub fn iter(&self) -> ListIter<T> {
        ListIter {
            current: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> ListIterMut<'_, T> {
        ListIterMut {
            current: self.head.as_deref_mut(),
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
    T: Copy + Clone,
{
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.head = n.next.clone();
            n.data
        })
    }
}

impl<T> From<&[T]> for List<T>
where
    T: Clone,
{
    fn from(value: &[T]) -> Self {
        let mut list = Self::default();
        for data in value {
            list.push(data.clone());
        }
        list
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<T: PartialEq> PartialEq<Link<T>> for List<T> {
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
impl<T> Iterator for ListIter<'_, T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next.as_deref();
            // Here the data is being Copied because the Copy trait,
            // that's why we don't need lifetimes
            node.data
        })
    }
}

// It consumes the structure
impl<T> Iterator for List<T>
where
    T: Copy + Clone,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

pub struct ListIterMut<'a, T> {
    current: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next.as_deref_mut();
            &mut node.data
        })
    }
}

#[test]
fn second_list() {
    let binding = "Hello".to_string();
    let chars = binding.chars();
    let vec = chars.clone().collect::<Vec<_>>();
    let mut list = List::<char>::from(vec.as_slice());
    for c in chars.clone().rev() {
        assert_eq!(list.pop(), Some(c));
    }
    assert_eq!(list, None);

    list.push('a');
    let list2 = List::<char>::from(['a'].as_slice());

    assert_eq!(list, list2.head);

    let new_data = 'b';
    if let Some(v) = list.peek_mut() {
        *v = new_data;
    }

    assert_eq!(list.pop(), Some(new_data));

    for c in chars.clone() {
        list.push(c);
    }

    let chars_rev = chars.clone().rev();
    for (list_data, char) in list.iter().zip(chars_rev) {
        assert_eq!(list_data, char)
    }

    let chars_plus_one = chars.map(|x| (x as u8 + 1) as char).collect::<Vec<_>>();

    for c in list.iter_mut() {
        *c = (*c as u8 + 1) as char
    }

    for (list_data, char) in list.iter().zip(chars_plus_one.iter().rev()) {
        assert_eq!(list_data, *char)
    }
}
