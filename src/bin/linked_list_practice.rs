struct MyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>, // For O(1) append
    len: usize,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> MyLinkedList<T> {
    // O(1) prepend
    fn prepend(&mut self, new_data: T){
        // Creating a new node
        let mut new_node = Box::new(Node{data: new_data, next: None});
        let raw_ptr: *mut Node<T> = &mut *new_node;

        if self.tail.is_null() {
            self.tail = raw_ptr;
        }

        // Setting `next` to the head node, and then setting self.head to None
        new_node.next = self.head.take();

        // Setting self.head to our new node
        self.head = Some(new_node);

        // Increment the length
        self.len += 1;
    }

    // O(1) append
    fn append(&mut self, new_data: T) {
        let mut new_node = Box::new(Node{data: new_data, next: None});
        let raw_ptr: *mut Node<T> = &mut *new_node;

        if self.tail.is_null() {
            self.head = Some(new_node);
        } else {
            unsafe {
                (*self.tail).next = Some(new_node);
            }
        }

        self.tail = raw_ptr;
        self.len += 1;
    }

    fn reverse(&mut self) {
        // Getting head node, and setting self.head to None
        let mut current_node = self.head.take();

        // There is no node before the head node, so initialized pre_node with None
        let mut prev_node = None;

        self.tail = std::ptr::null_mut();

        // while current_node is not None, let boxed_node = current_node
        while let Some(mut boxed_node) = current_node {
            // Setting next_node to the next node, and then setting next property of boxed_node to None
            let next_node = boxed_node.next.take();

            if self.tail.is_null() {
                let raw_tail: *mut Node<T> = &mut *boxed_node;
                self.tail = raw_tail;
            }

            // TO REVERSE: Change the direction of pointers
            // Setting the next field of boxed_node to previous_node
            boxed_node.next = prev_node;

            // For the next iteration, prev_node will be the current node(boxed_node) and current_node will be next_node
            prev_node = Some(boxed_node);
            current_node = next_node;
        }

        // When current_node is None, we set the node before None(previous node) as the head node.
        self.head = prev_node;
    }
}

impl<T: Clone> MyLinkedList<T> {
    fn from_vec(arr: Vec<T>) -> Self {
        // Initializing an empty Linked List
        let mut new_list = MyLinkedList { head: None, tail: std::ptr::null_mut(), len: 0 };

        // Traversing the vector from back to front
        // for i in (0..arr.len()).rev() {
        //     // Prepending element of vec to Linked List
        //     new_list.prepend(arr[i].clone());
        // }

        // Using our O(1) append
        for val in arr {
            new_list.append(val);
        }

        // Returning our new Linked List
        new_list
    }
}

// Implementing this trait so that we can print our Linked List
impl<T: std::fmt::Debug> MyLinkedList<T> {
    // NOTE: we do not want to take ownership.
    fn print(&self) {
        // Initialize current_node with head node
        let mut current_node = &self.head;

        // while current_node is not None, let node = current_node
        while let Some(node) = current_node {
            // printing "{node.data} -> "
            print!("{:?} -> ", node.data);

            // For the next iteration, set current_node to next node
            current_node = &node.next;
        }
        // Lastly, print None at the end of the Linked List
        println!("None");

        // OPTIONAL: printing the length of the Linked List
        println!("Length:- {}", self.len);
    }
}

// Optimized Drop to prevent Stack Overflow Error
impl<T> Drop for MyLinkedList<T> {
    fn drop(&mut self) {
        let mut current_node = self.head.take();
        while let Some(mut boxed_node) = current_node {
            current_node = boxed_node.next.take();
            // boxed_node dropped here
        }
    }
}

fn main() {
    let mut ll = MyLinkedList::from_vec(vec![1, 2, 3]);
    ll.print();

    ll.reverse();
    ll.print();
}