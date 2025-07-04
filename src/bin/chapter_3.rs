const PQR: i32 = 1;
static MY_STR: &str = "Hi!";

fn main() {
    // Shadowing variables
    let x = 5;
    let x = x + 1;
    println!("The value of x is: {}", x);
    const ABC: i32 = 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // You cannot change the type of a variable once defined, but...
    // This code gives error, try uncommenting
    // let mut spaces = "    ";
    // spaces = spaces.len();
    // This code works
    let spaces :&str = "    ";
    let spaces :usize = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // Types in Rust
    /*
    Stored on stack:-
    The number denote the bits of space the type occupies
    Signed integers:- isize, i8, i16, i32, i64, i128
    Unsigned numbers:- usize, u8, u16, u32, u64, u128
    Float:- f32, f64
    Boolean:- bool
    Character:- char
    String slices:- &str
    etc.

    Stored on heap:-
    Strings:- String
    Vectors:- Vec<T>
    Hashmaps:- HashMap<K, V>
    etc.
     */

    // Rust can type-infer the variable declared.
    // By default, integers are type-inferred to i32, and floats as f64
    let _num = 9; // is same as `let _num :i32 = 9;`

    // Compound Types
    let mut my_tuple = (500, 6.4, 1);
    // let (x, y, z) = my_tuple;
    // Alternatively,
    let x = my_tuple.0;
    let y = my_tuple.1;
    let z = my_tuple.2;

    my_tuple.0 += 5;
    my_tuple.1 += 5.5;
    my_tuple.2 += 5;

    // Notice that x, y, z do not change, but my_tuple changes
    // Also, tuples don't implement the Display trait, hence they cannot be printed directly
    // They implement the Debug trait, so we can use `:?` to print such types
    println!("The value of tuple, x, y, z is: {:?}, {}, {}, {}", my_tuple, x, y, z);

    // [i32; 5] says that my_array is an array with 5 elements of type i32
    let my_array :[i32; 5] = [1, 2, 3, 4, 5]; // same as `let my_array  = [1, 2, 3, 4, 5]`
    let another_array = [1; 5]; // [1; 5] = [1, 1, 1, 1, 1]
    println!("The value of my_array is: {:?}", my_array);
    println!("The value of another_array is: {:?}", another_array);

    // Functions
    fn do_thing() {
        println!("Doing thing!");
    }
    do_thing();

    // If we add parameters to a function or plan to return a value, we MUST specify it's type
    // In rust, you can return a value by not writing the semi-colon at the end of an expression
    fn add_one(x:i32) -> i32 {
        x + 1
    }
    println!("The value of add_one(5) is: {:?}", add_one(5));

    // You can use if-else statements inside let (already covered)

    // Loops
    // loop { println!("Loop"); } // INFINITE LOOP!!!
    // Did I mention that loops can return values?
    let mut counter = 0;
    let loop_value = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };
    println!("The value of loop_value is: {:?}", loop_value);

    // Loop labels ACTUALLY help
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
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
    println!("End count = {count}");

    // While loops
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    // For loops
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}