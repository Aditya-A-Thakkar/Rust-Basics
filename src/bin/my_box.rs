use std::alloc::{alloc, dealloc, Layout, handle_alloc_error};
use std::ptr::{write, drop_in_place};
use std::ops::{Deref, DerefMut};

// let x :[u8; 3] = [1, 2, 3]; // This is sized
// let y :[u8] = [1, 2, 3]; // Compiler will reject this as it is unsized and need be stored on heap
// let z :Box<[u8]> = [1, 2, 3]; // WORKS

struct MyBox<T> {
    ptr: *mut T,
    label: String,
}

impl<T> MyBox<T> {
    fn my_new(x: T, label: String) -> MyBox<T> {
        unsafe {
            let layout = Layout::new::<T>();
            // Layout describes memory layout of a Type.
            // Tells the allocator two things
            // 1. How many bytes to allocate
            // 2. How much alignment is required.


            let raw_ptr = alloc(layout) as *mut T;
            // This allocates uninitialized heap memory


            if raw_ptr.is_null() { handle_alloc_error(layout); }
            // Checking if the raw pointer is null (maybe due to failed alloc or invalid layout)
            // `handle_alloc_error` panics upon allocation failure

            write(raw_ptr, x); // allocate(write) data to heap
            // Moves x into the memory at raw_ptr without reading/dropping what's there

            MyBox { ptr: raw_ptr, label }
        }
    }

    fn get_label(&self) -> &str {
        &self.label
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            drop_in_place(self.ptr); // calling destructor of T

            dealloc(self.ptr as *mut u8, Layout::new::<T>()); // deallocate(free) data from heap
        }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

fn main() {
    let mut my_box = MyBox::my_new(10, "Hello".to_string());

    println!("Label:- {}", my_box.get_label());
    println!("Value:- {}", *my_box);

    *my_box += 1;

    println!("Value:- {}", *my_box);
}