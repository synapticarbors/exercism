use std::iter::FromIterator;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut x = 0;
        let mut curr_node = &self.head;
        while let Some(n) = curr_node {
            x += 1;
            curr_node = &n.next;
        }

        x
    }

    pub fn push(&mut self, _element: T) {
        let new_node = Box::new(Node {
            data: _element,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut x = SimpleLinkedList::new();

        let mut curr_node = &self.head;
        while let Some(node) = curr_node {
            x.push(node.data.clone());
            curr_node = &node.next;
        }

        x
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut x = SimpleLinkedList::new();
        for e in _iter {
            x.push(e);
        }

        x
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len());
        let mut head = self.head;

        while let Some(node) = head {
            out.push(node.data);
            head = node.next;
        }

        out.reverse();

        out
    }
}
