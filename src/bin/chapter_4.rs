fn main() {
    /*
    Ownership Rules:-
    1. Each value in Rust has an owner
    2. There can be only one owner at a time.
    3. When the owner goes out of scope, the value is dropped.
     */

    // One is straightforward

    // Example of 3.
    // Did I mention you can leave the variable uninitialized, but can't use them until initialized?
    let mut s_out;
    {
        let s = 1;
        s_out = &s;
        println!("{s}");
    } // s goes out of scope here and is dropped

    // Compiler rejects the below line:-
    // println!("{s_out}")
    // PROBLEM: s is dropped, and s_out points to freed memory, which is undefined behaviour

    // Let us look at the rule number 2
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
}