#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SinglyLinkedList<T> {
    fn new() -> Self {
        SinglyLinkedList { head: None }
    }

    fn append(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: None });
        
        // Use a mutable reference to traverse the list
        let mut current = &mut self.head;
        
        while let Some(ref mut node) = current {
            current = &mut node.next;
        }
        
        // Reached the end (None), place the new node there
        *current = Some(new_node);
    }

    fn get_head(&self) -> &Option<Box<Node<T>>> {
        &self.head
    }
}



fn main() {
    let mut list = SinglyLinkedList::new();
    list.append(1);
    list.append(2);
    list.append(3);
    
    //println!("{:?}", list);
}