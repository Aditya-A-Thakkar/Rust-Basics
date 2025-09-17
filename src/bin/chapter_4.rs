fn main() {
    /*
    Ownership Rules:-
    1. Each value in Rust has an owner [may not be really true, see "s2 = s1" example below.]
    2. There can be only one owner of a value at a time.
    3. When the owner goes out of scope, the value is dropped.
     */

    // Example of Rule 1 and 2
    // The term 'value' in the first rule refers to any data stored in the memory.
    let p = String::from('P'); // P is owner of the string "P"
    let q = p; // Due to rule 2, q becomes the owner of the string "P"
    // println!("{p}"); // The variable p was freed up in the last line and cannot be used.
    println!("{q}");

    // Example of 3.
    {
        let s = String::from("Hello");
        println!("{}", s);
    } // s goes out of scope here and is dropped. This will free up the string from the heap. 

    // Let us look at the rule number 2
    // Move:- transfer of ownership of a value from a variable to other variable / argument
    // This is called MOVE semantics.
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s2); // WORKS
    // println!("{}, world!", s1); // DOES NOT WORK
    // REASON: s1 has been moved into s2, and hence s1 is dropped.

    // String addition removes ownership from the lhs variable of the + operation.
    // (RHS of + operation is a reference, which anyway does not have ownership.)
    let l_str = String::from("Hello");
    let r_str = String::from(", world!");
    let concat_str = l_str + &r_str;
    println!("{}", concat_str);
    println!("{}", r_str);
    // println!("{}", l_str); // ERROR!!!

    // DONE: in the example above, use MyString (or MyBox), with a print statement inside the drop() method,
    // and check that at runtime the printing happens only once. This verifies that the compiler
    // does not call the drop() method on places that no longer own their value.
    #[derive(Debug)]
    struct MyBox<T>(T);
    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            println!("Dropping MyBox");
        }
    }
    let box1 = MyBox(5);
    let box2 = box1;
    println!("{:?}", box2); // MyBox(5)
    // NOTE: Dropping MyBox is printed exactly once at the end of the output.

    let num1 = 1;
    let num2 = num1;
    println!("{}", num2); // WORKS
    println!("{}", num1); // WORKS!!
    // REASON: i32 implements the Copy trait, hence instead of moving num1 into num2, it copies the value of num1 into num2

    // A struct with COPY trait.
    // Note: a struct that does not implement COPY trait defaults to MOVE semantics.
    // NOTE: Every type which implements the Copy trait must also implement the Clone trait.
    #[derive(Debug, Copy, Clone)]
    struct CopyableStruct {
        id: u8,
    }
    let copy1 = CopyableStruct { id: 1 };
    let copy2 = copy1;
    println!("{:?}", copy1);
    println!("{:?}", copy2);

    // THIS COMPILES, due to copy semantics
    let x = 1;
    let y = x;
    println!("{}", x);

    // THIS DOES NOT, due to MOVE semantics
    let a = String::from([1,2,3]);
    let b = a;
    // println!("{}", a); // ERROR!!!


    // The function `do_nothing` takes the ownership of first.
    let first = String::from("first");
    do_nothing(first);
    // println!("{}", first); // ERROR!!!
}

fn do_nothing(mut first_parameter: String) {
    println!("Noting {}", first_parameter);
    first_parameter.push_str(", second");
    println!("{}", first_parameter);
}
