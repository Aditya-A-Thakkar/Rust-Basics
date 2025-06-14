#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
    // Rust has signed (+ and -) and unsigned integer (only+) types of different sizes.
    // i8, i16, 132, іб4, i128: Signed integers.
    // u8, u16, u32, uб4, u128: Unsigned integers.
    println!("Maximum value of i32: {}", 2147483647i32);
    println!("Maximum value of i64: {}", 9223372036854775807i64);
    println!("Maximum value of u32: {}", 4294967295u32);
    println!("Maximum value of u64: {}", 18446744073709551615u64);
    println!("Maximum value of isize: {}", 2147483647isize); // size depend on computer
    println!("Maximum value of usize: {}", 4294967295usize); // a 32-bit pc would have isize=i32

    // Compound Data Types
    // Arrays, Tuples, Slices, Strings

    // Arrays
    // let name:[datatype, # of elements] = [array with # elements];
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    // let mix = [1,2, "apple", true];
    // println!("Mix Array: {:?}", mix); results in error.
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st element: {}", fruits[0]);
    println!("Fruits Array 2nd element: {}", fruits[1]);
    println!("Fruits Array 3rd element: {}", fruits[2]);

    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);
    let my_mix_tuple = ("Pirates", 23, true, [1, 2, 3, 4, 5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    // Slices: [1,2,3,4, 5]
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slice: {:?}", number_slices);
    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slice: {:?}", animal_slices);
    let book_slices: &[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("Book Slice: {:?}", book_slices);

    // Strings Vs String Slices (&str)
    // A- Strings [ growable, mutable, owned string type ]
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);
    // B- &str (String Slice)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);

    // Functions
    // Hoisting:- Define your functions anywhere in your code...
    hello_world();
    human_data("Joel", 55, 182.2);

    // Expressions and Statements
    // Expression: Anything that returns a value.
    // Statement: Anything that does not return a value.
    // Expression
    // 5
    // true & false
    // add (3,4)
    // if condition {value1} else {value2}
    // ({code})
    // while defining variables in global scope, use const and upper case.
    // Statements
    // Almost all statements in Rust end with ;
    // let y = let x = 10;
    // 1 Variable declarations: let x = 5;
    // 2 Function definitions: fn foo@ {}
    // 3 Control flow statements: if condition { /* code */ } else {/* code */ 3, while condition { /* code */ 3, etc.

    // Ownership
    // There are some memory management issues like freeing the memory more than once or not freeing it at all
    // C, C++ have solved this by using a garbage collector.
    // Garbage Collector solved this issue, but created another issue.
    // It stops(freezes) the program while collecting garbage and resumes it after completion.
    // This lead to slower performance of the languages.

    // Ownership rules in RUST
    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    // Example:Each value in Rust has a variable that's its owner.
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of {} is {}.", s1, len);

    // Example:There can be only one owner at a time.
    let s1 = String::from("RUST");
    let s2 = s1;
    // println!("{}", s1); // GIVES ERROR
    println!("{}", s2); // RUST

    // 3rd one is simple, variables can only be used in the scope in which they ae defined

    // In Rust, managing memory is crucial for ensuring both safety and performance.
    // SAFETY?
    // Safety refers to preventing a certain types of bugs and errors that commonly occur in other languages like C.
    // Things like 'null pointer dereferencing', 'dangling pointers', 'buffer overflows', 'data races', etc.

    // References and Borrowing

    // Understanding References
    // References: enable you to borrow values without taking the ownership.
    // 2 types of references:- immutable reference, and mutable reference.
    // You can either have one mutable reference or any number of immutable references.

    // Mutable Reference
    // Use '&' for creating a mutable reference.
    // Immutable reference.
    let x = 5;
    // let r = x; NOT GOOD FOR MEMORY SINCE THE MEMBERSHIP IS CHANGED
    let r = &x;
    println!("x: {}", x);
    println!("r: {}", r);

    // Mutable reference.
    let mut _x = 9;
    let _r = &mut _x;

    *_r += 1;
    // This does not work
    // println!("_x: {}", _x);
    // println!("_r: {}", _r);

    // This does work
    // println!("_r: {}", _r);
    // println!("_x: {}", _x);

    // This does not work
    // println!("_x: {}", _x);
    // *_r += 26
    // println!("_x: {}", _x);

    // This does work
    // println!("_r: {}", _r);
    // *_r += 26;
    // println!("_x: {}", _x);

    // Struct:- A data structure that allows you to group multiple fields together under one name.
    // Check line 160
    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 155.55
    };
    account.check_balance();
    account.withdraw(36.9);
    account.check_balance();

    // Variables and Mutability
    let a = 10;
    println!("The value of a is {}.", a);
    // a += 10; // DOES NOT WORK
    // println!("The value of a is {}.", a);
    let mut b = 9;
    println!("The value of b is {}.", b);
    b += 10; // WORKS
    println!("The value of b is {}.", b);
    // Constants
    let c = 1; // Variable
    const D: i32 = 2; // Constant
    // You can put "mut" with a "const".
    // Mentioning the datatype is mandatory for constants.
    // Constant variable names should be "ALL CAPS"
    // You can also declare a constant in GLOBAL scope, but not variables(using let at least)
    println!("The value of c is {}.", c);
    println!("The value of D is {}.", D);

    // Shadowing
    let abc = 5;
    println!("The value of abc is {}.", abc);
    let abc = abc + 1;
    println!("The value of abc is {}.", abc);
    {
        let abc = abc * 2;
        println!("The value of abc in inner scope is {}.", abc);
    }
    println!("The value of abc is {}.", abc);
    // Note that shadowing is not the same as assigning to an immutable variable.

    // If-Else conditions(Control Flow)
    let age: u16 = 18;
    if age >= 18 {
        println!("You can drive a car!");
    } else {
        println!("You can't drive a car!");
    }

    // Multiple statements
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if else in a let statement
    let condition = false;
    let number = if condition { 5 } else { 6 };
    // NOTE that this will give an error
    // let number = if condition { 5 } else { "six" };
    println!("The number is {}: ", number);

    // Loops

    // 'loop' keyword
    // This is an infinite loop......
    // loop {
    //     println!("Hello!")
    // }

    // loop with 'let' keyword
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // Loop Labels - to disambiguate between multiple loops
    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;
        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    // 'while' keyword
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // Looping through a collection using for loop
    let some_list = [1, 2, 3, 4, 5];
    for element in some_list {
        println!("{}", element);
    }

    // Structs
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        active: true,
        username: String::from("someusername@m.com"),
        email: String::from("someusername@m.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("another@example.com");
    println!("email is {}", user1.email);

    // you can return structs in functions
    fn create_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
    let _some_user = create_user(String::from("someusername@m.com"), String::from("someusername@m.com"));

    // Create instances from other instances
    let user2 = User {
        email: String::from("another@m.com"),
        ..user1
    }; // This will have all properties same as user1, except for the email.
    println!("email is {}", user2.email);

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black: Color = Color(0, 0, 0);
    let white: Color = Color(255, 255, 255);

    println!("Color Black: ({},{},{})", black.0, black.1, black.2);
    println!("Color White: ({},{},{})", white.0, white.1, white.2);

    // Unit-Like struct
    struct AlwaysEqual;
    let _subject = AlwaysEqual;

    // Enums
    // A versatile tool used to represent a type that can take on one of several possible variants
    // Normal ENUM:
    // enum IPAddrType {
    //     V4,
    //     V6
    // }
    // let _four = IPAddrType::V4;
    // let _six = IPAddrType::V6;
    // fn route(ip_type: IPAddrType) {}
    // struct IPAddr {
    //     kind: IPAddrType,
    //     address: String
    // }
    // let home = IPAddr{
    //     kind: IPAddrType::V4,
    //     address: String::from("127.0.0.1")
    // };
    // let loopback = IPAddr{
    //     kind: IPAddrType::V6,
    //     address: String::from("::1")
    // };
    // Enhanced ENUM:
    enum IPAddrType {
        V4,
        V6
    }
    enum IPAddr {
        V4(u8, u8, u8, u8),
        V6(String)
    }
    let _home = IPAddr::V4(127, 0, 0, 1);
    let _loopback = IPAddr::V6(String::from("::1"));

    // Error Handling Techniques
    // Approach 1
    // enum Option<T> { // Define the generic Option type
    //     Some(T), // Represents a value None,
    //     None // Represents no value
    // }
    // let result: Option<f64> = divide(10.0, 2.00);
    // match result{
    //     Some(x) => println! ("Result: {}", x),
    //     None => println! ("Cannot divide by Zero!"),
    // }

    // Approach 2
    // enum Result<T, E> { // Define the generic Result type
    //     Ok(T), // Represents a value,
    //     Err(E) // Represents an error
    // }
    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            // Option::Some(String::from("The denominator cannot be zero!"))
            Err(String::from("Divide by zero"))
        } else {
            // Option::Some(numerator / denominator)
            Ok(numerator / denominator)
        }
    }
    let result: Result<f64, String> = divide(10.0, 2.00);
    match result {
        Ok(x) => println!("Result: {}", x),
        Err(e) => println!("Error: {}", e),
    }

    // FIND THE ERROR, SOLVE IT. NOTE: There are 2 errors
    // let s = String::from("s");
    // let mut r = &s;
    // if s=="a" {
    //     let s2 = String::from("a");
    //     r = &mut s2;
    // }
    //
    // println!("r: {}", r);

    // Vectors: They allow you to store more than one value in the same data structure that puts the values next to each other in the memory
    // Vectors should be homogeneous
    let _v:Vec<i32> = Vec::new();
    println!("v: {:?}", _v);

    // Macro to create a vector of numbers
    let _the_vec:Vec<i32> = vec![1, 2, 3];
    println!("_the_vec: {:?}", _the_vec);

    let mut _the_numbers_vec: Vec<i32> = Vec::new() ;
    _the_numbers_vec.push(5);
    _the_numbers_vec.push(6);
    _the_numbers_vec.push(7);
    _the_numbers_vec.push(8);
    _the_numbers_vec.push(9);
    println!("_the_numbers_vec: {:?}", _the_numbers_vec);

    let third_element:Option<&i32> = _the_numbers_vec.get(2); // .get() method returns an <Option> type
    match third_element {
        None => println!("There is no third element."),
        Some(third) => println!("The third element is {}", third)
    }

    // UTF-8 or Strings as we say
    // 1
    let _s:String = "hello".to_string();
    // 2
    let _s:String = String::from("whatever");
    // Mutate the variable to push to it
    let mut _s:String = String::from("foo");

    // push_str() pushes a string slice whereas push() pushes a char.
    _s.push_str("bar");
    // _s.push("!"); DOES NOT WORK
    // _s.push('!'); // WORKS

    println!("the value of s is {}", _s);

    // If you want to combine strings, use the + operator
    let s1:String = String::from("Hello, ");
    let s2:String = String::from("world!");
    let _s3:String = s1 + &s2; // note s1 has been moved here and can no longer be used

    // Hash Maps
    // Takes two arguments, 1. The type of keys, and 2. The type of values.
    use std::collections::HashMap; // Keep it at the top of the file
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name:String = String::from("Blue");
    let _score:i32 = scores.get(&team_name).copied().unwrap_or(0);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn hello_world() {
    println!("Hello, World!");
}

fn human_data(name: &str, age: u32, height: f32) {
    println!("My name is {}, I am {} years old, and my height is {} cm.", name, age, height);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

struct BankAccount {
    balance: f64,
    owner: String
}

impl BankAccount {
    fn withdraw (&mut self, amount: f64) {
        println!("Withdrawing an amount of {} from the account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance (&mut self) {
        println!("{}", self.balance);
    }

}