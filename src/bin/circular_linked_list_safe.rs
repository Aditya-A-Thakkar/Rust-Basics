use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
pub struct CircularList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Weak<RefCell<Node>>>, // weak pointer to avoid cycle
    len: usize,
}

impl CircularList {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push(&mut self, val: i32) {
        let new_node = Rc::new(RefCell::new(Node {
            val,
            next: None,
        }));

        match self.head.as_ref() {
            None => {
                // First node points to itself
                new_node.borrow_mut().next = Some(Rc::clone(&new_node));
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(Rc::downgrade(&new_node));
            }
            Some(head) => {
                // Get tail (upgrade from Weak to Rc)
                if let Some(tail_strong) = self.tail.as_ref().and_then(|w| w.upgrade()) {
                    tail_strong.borrow_mut().next = Some(Rc::clone(&new_node));
                    new_node.borrow_mut().next = Some(Rc::clone(&head));
                    self.tail = Some(Rc::downgrade(&new_node));
                }
            }
        }

        self.len += 1;
    }

    pub fn print(&self) {
        if self.head.is_none() {
            println!("List is empty");
            return;
        }

        let mut current = Rc::clone(self.head.as_ref().unwrap());
        for _ in 0..self.len {
            print!("{} -> ", current.borrow().val);
            let borrowed_node = current.borrow();
            let next = borrowed_node.next.as_ref().unwrap().clone();
            drop(borrowed_node);
            current = Rc::clone(&next);
        }
        println!("HEAD");
    }
}

fn main() {
    let mut clist = CircularList::new();
    clist.push(1);
    clist.push(2);
    clist.push(3);

    clist.print();
}
