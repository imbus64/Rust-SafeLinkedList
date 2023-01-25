// "Bare minimum" doubly linked list in safe Rust
// Uses Rc and RefCell to get around the borrow checker
// Should not be used for anything besides learning, if a real linked list is needed
// use the unsafe version in the standard library (std::collections::LinkedList)

use std::cell::RefCell;
use std::rc::Rc;

/// Type alias for convenience
type Link = Rc<RefCell<Node>>;

/// Node holding data and two pointers
#[derive(Clone)]
struct Node {
    value: i32,
    prev: Option<Link>,
    next: Option<Link>,
}

/// Doubly linked list
pub struct List {
    size: usize,
    head: Option<Link>,
    tail: Option<Link>,
}

impl List {
    /// Creates a new empty LinkedList
    pub fn new() -> List {
        List {
            size: 0,
            head: None,
            tail: None,
        }
    }

    /// Push a new value onto the back of the list
    pub fn push_back(&mut self, value: i32) {
        let node = Node {
            value,
            prev: self.tail.clone(),
            next: None,
        };

        // Shadowing node
        let node = Rc::new(RefCell::new(node));

        match self.tail {
            Some(ref prev_tail) => {
                // Set prev->next to new node
                prev_tail.borrow_mut().next = Some(Rc::clone(&node));
                // Set new node->prev to prev
                node.borrow_mut().prev = Some(Rc::clone(&prev_tail));
                // Update tail
                self.tail = Some(Rc::clone(&node));
            }
            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(self.head.as_ref().unwrap()));
            }
        }
        self.size += 1;
    }

    /// Gets the data at index `index` by cloning
    /// Keep in mind that this is an O(n) operation
    pub fn get(&self, index: usize) -> Option<i32> {
        match self.get_link_at(index) {
            Some(node) => Some(node.as_ref().borrow().value.clone()),
            None => None,
        }
    }

    // Searches from the beginnnig or end of the list depending on which is closest
    /// Get the N:th node in the list, only used internally
    fn get_link_at(&self, index: usize) -> Option<Link> {
        if index >= self.len() {
            return None;
        }

        let direction_from_head = index <= self.size / 2;

        let index = if direction_from_head {
            index
        } else {
            self.size - index - 1
        };

        // Unwrapping here since we know that the list is not empty
        let mut current: Link = match direction_from_head {
            true => Rc::clone(self.head.as_ref().unwrap()),
            false => Rc::clone(self.tail.as_ref().unwrap()),
        };

        for _ in 0..index {
            current = match direction_from_head {
                true => Rc::clone(current.as_ref().borrow().next.as_ref().unwrap()),
                false => Rc::clone(current.as_ref().borrow().prev.as_ref().unwrap()),
            };
        }
        Some(current)
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const UPPER_BOUNDS: usize = 1000;

    #[test]
    fn test_push_back() {
        let mut list = List::new();
        (0..UPPER_BOUNDS).for_each(|i| list.push_back(i as i32));
    }

    #[test]
    fn test_len() {
        let mut list = List::new();
        (0..UPPER_BOUNDS).for_each(|i| list.push_back(i as i32));
        assert_eq!(list.len(), UPPER_BOUNDS);
    }

    #[test]
    fn test_get() {
        let mut list = List::new();
        (0..UPPER_BOUNDS).for_each(|i| list.push_back(i as i32));
        (0..UPPER_BOUNDS).for_each(|i| assert_eq!(list.get(i), Some(i as i32)));
    }

    #[test]
    fn test_empty_len() {
        assert_eq!(List::new().len(), 0);
    }

    #[test]
    fn test_large_data() {
        let mut list = List::new();
        (0..UPPER_BOUNDS * 10).for_each(|i| list.push_back(i as i32));
        assert_eq!(list.len(), UPPER_BOUNDS * 10);
    }

    #[test]
    fn test_getting_from_large_data() {
        let mut list: List = List::new();

        (0..10000).for_each(|i| list.push_back(i as i32));

        // Not doing exhaustive testing here since get is O(n)
        [0, 100, 2314, 3948, 231, 9489, 9999].iter().for_each(|i| {
            assert_eq!(list.get(*i as usize), Some(*i as i32));
        });
    }

    #[test]
    fn test_out_of_bounds() {
        let mut list = List::new();
        (0..20).for_each(|i| list.push_back(i as i32));
        assert_eq!(list.get(20), None);
        assert_eq!(list.get(20 * 20), None);
    }
}
