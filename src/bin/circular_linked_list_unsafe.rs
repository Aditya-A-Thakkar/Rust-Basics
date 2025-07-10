use std::ptr;
use std::fmt::Debug;

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

struct CircularLinkedList<T> {
    head: Option<*mut Node<T>>,
    tail: Option<*mut Node<T>>,
    length: usize,
}

impl<T> CircularLinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    fn push(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: ptr::null_mut() });
        let new_ptr = Box::into_raw(new_node);

        unsafe {
            match (self.head, self.tail) {
                (None, None) => {
                    // First element: points to itself
                    (*new_ptr).next = new_ptr;
                    self.head = Some(new_ptr);
                    self.tail = Some(new_ptr);
                }
                (Some(head_ptr), Some(tail_ptr)) => {
                    (*tail_ptr).next = new_ptr;
                    (*new_ptr).next = head_ptr;
                    self.tail = Some(new_ptr);
                }
                _ => unreachable!("Only valid states are both Some or both None"),
            }
        }

        self.length += 1;
    }
}

impl<T: Debug> CircularLinkedList<T> {
    fn print(&self) {
        if self.head.is_none() {
            println!("List is empty");
            return;
        }

        unsafe {
            let mut current_node = self.head.unwrap();
            for _ in 0..self.length {
                print!("{:?} -> ", (*current_node).data);
                current_node = (*current_node).next;
            }
            println!("HEAD");
        }
    }
}

impl<T> Drop for CircularLinkedList<T> {
    fn drop(&mut self) {
        unsafe {
            if self.head.is_none() {
                return;
            }

            let mut current = self.head.unwrap();
            for _ in 0..self.length {
                let next = (*current).next;
                drop(Box::from_raw(current));
                current = next;
            }
        }
    }
}

fn main() {
    let mut my_circular_linked_list = CircularLinkedList::new();
    my_circular_linked_list.push(1);
    my_circular_linked_list.push(2);
    my_circular_linked_list.push(3);

    my_circular_linked_list.print();
}
