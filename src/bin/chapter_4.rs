fn main() {
    /*
    Ownership Rules:-
    1. Each value in Rust has an owner (which is a variable or in general other types of named memory locations)
    2. There can be only one owner of a value at a time.
    3. When the owner goes out of scope, its value is dropped by the compiler. This is the main 
       reason why the notion of ownership exists!
     */


    /*  Illusration of Rule 3. */
    {
        let  s = String::from("Hello");
        /* Recall that a String is really a struct, with two fields: (1) a pointer to the string (which is the heap),
           (2) the length of the string. This struct is the value of the variable s, and the resides in the stack
           (in the slot of s). In other words, a String is basically a fat pointer.
           The from() method allocates a string in the heap and initializes it from the
           literal (which is itself immutable and lives in the global data space. */
        println!("{}", s);
    } 
    /*  s goes out of scope here, and hence the compiler places here a call to the "drop" method of the String, by
        basically inserting a call s.drop(). The drop() method of String de-allocates the string in the heap.
        Note the co-operation between the compiler and the implementation of String::drop() (in the library)
        to clean up garbage without programmer intervention.
    */

    /* Let us look at the Rule number 2.
       Move:- transfer of ownership of a value from a variable to other variable / argument.
       This is called MOVE semantics.
    */
    let s1 = String::from("hello");
    /*  s1 owns its String (i.e., struct) */
    let s2 = s1; 
    /*  The struct is physically copied to s2's slot in the stack, but the compiler treats this logically as a MOVE, 
        not a COPY. This means the struct is now owned by s2. The content in s1 is unowned and cannot be accessed 
        any more. */
    println!("{}, world!", s2); // WORKS
    /*  When s2 goes out of scope (or when it stops being used in the code), the compiler inserts call to s2.drop().
        Compiler does NOT place a call to s1.drop() when s1 goes out of scope, because s1 does not own its content.
        And this makes sense, since two calls to drop() would result in double de-allocation of the (single) string
        in the heap! 
        Basically, the compiler keeps track of who owns what, so that each value (String struct in this case) gets
        dropped exactly once.
    */
   
    // println!("{}, world!", s1); // DOES NOT WORK
   
    /* Why should accesses to s1 be disallowed? Because if we placed a call s2.push_str(", world!") after s2 is
       initialized, the string in the heap could get re-allocated to a different space, and the pointer in the
       struct in s2 would point to the new location. Now, the old content in s1 would be a dangling pointer.
       Hence, Rust never allows a variable that does not own its value to be accessed.
     */

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

    /*  Any value that implements the copy trait will be treated as copied after a copy-statement,
        meaning the old source variable can still be accessed (and still owns its content). 
        Primitives such as integers and string slices 
        automatically implement copy trait (because their drop action needs to do nothing). 
        Note, string slices are mutable in a very limited way -- their internal characters
        can be replaced (by valid UTF-8 bytes), but their length cannot be changed. 
    
        By default structs don't implement the "copy" trait. They can only be moved.  */

    let num1 = 1;
    let num2 = num1;
    println!("{}", num2); // WORKS
    println!("{}", num1); // WORKS!!
    // REASON: i32 implements the Copy trait, hence instead of moving num1 into num2, it copies the value of num1 into num2

    // A struct with an explicit COPY trait.
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
    /*  An explicit clone() method is not required above, since the default .clone() method of a struct
        calls .clone() on each field. For primitives, the .clone method is a simple copy. 
    */

/*  If a struct has a raw pointer within it, and 
    one wishes to provide a copy trait to this struct, then in the clone() method associated with this 
    struct one should typically make a deep copy of the buffer pointed to by the raw pointer. Since this is 
    expensive, most library structs that point to potentially large data (such as String, Vector) do not 
    implement the copy trait. However, they do provide a .clone()  method, which does a deep copy, and
    which can be called explicitly. */

    // The function `do_nothing` takes the ownership of the value in first.
    let first = String::from("first");
    do_nothing(first);
    // println!("{}", first); // ERROR!!!

    // String addition removes ownership from the lhs variable of the + operation.
    // (RHS of + operation is a reference, which anyway does not have ownership.)
    let l_str = String::from("Hello");
    let r_str = String::from(", world!");
    let concat_str = l_str + &r_str;
    println!("{}", concat_str);
    println!("{}", r_str);
    // println!("{}", l_str); // ERROR!!!
}

fn do_nothing(mut first_parameter: String) {
    println!("Nothing {}", first_parameter);
    first_parameter.push_str(", second");
    println!("{}", first_parameter);
    // return first_parameter and catch this in an LHS variable at the call-site
    // if the modified string is needed after the call-site
}
