fn main() {
    /*
    Ownership Rules:-
    1. Each value in Rust has an owner
    2. There can be only one owner at a time.
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
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s2); // WORKS
    // println!("{}, world!", s1); // DOES NOT WORK
    // REASON: s1 has been moved into s2, and hence s1 is dropped.

    // Let me confuse you
    let num1 = 1;
    let num2 = num1;
    println!("{}", num2); // WORKS
    println!("{}", num1); // WORKS!!
    // REASON: i32 implements the Copy trait, hence instead of moving num1 into num2, it copies the value of num1 into num2

    // Now comes the most interesting part of the chapter
    // References and Borrowing
    // Let's discuss it in a separate file

    // THIS COMPILES
    let x = 1;
    let y = x;
    println!("{}", x);

    // THIS DOES NOT
    // let a = String::from([1,2,3]);
    // let b = a;
    // println!("{}", a);


    let mut first = String::from("first");
    do_noting(&mut first);
    println!("{}", first);
}

fn do_noting(first_parameter: &mut String) {
    println!("Noting {}", first_parameter);
    first_parameter.push_str(", second");
}
