fn main() {
    // References are non-owning pointers, and dereferencing it gives access to its data

    // Rust implicitly inserts dereferences and references
    let x :Box<i32> = Box::new(-1); // x is a pointer on stack to the Box on heap (all variables live in stack)
    let x_abs1 = i32::abs(*x); // Explicit dereference
    let x_abs2 = x.abs(); // Implicit deference
    assert_eq!(x_abs1, x_abs2); // This macro panics is the two values given to it are not equal

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs(); // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len(); // implicit reference
    assert_eq!(s_len1, s_len2);

    // Rust avoids simultaneous aliasing and mutation (Pointer Safety Principle)
    // In Rust, you can either have
    // 1. One mutable reference, or
    // 2. Any number of immutable references
    // But NOT both at the same time.
    //
    // Intro to lifetimes:- A reference has a lifetime, which is a range from its birth to death
    let mut my_string = String::from("hello world");
    let s1 = &my_string;
    let s2 = &my_string;
    println!("s1: {}, s2: {}", *s1, *s2);
    let s3 = &mut my_string;
    s3.push('!');
    println!("s3: {}", *s3);
    // Notice that when s3 is declared, s1 and s2 have already been dropped because of their lifetime
    // Declare s3 above the println statement to see the compiler error.

    // Reference change permissions
    // The idea behind the Borrow Checker is that any variable has 3 permissions
    // 1. Read(R), 2. Write(W), 3. Own(O), [4. Flow(F)]
    let mut v = vec![1, 2, 3]; // v = RWO
    let num = &v[2]; // v = R, referencing removed the write and own permission from v, you cannot change/own v until num dies
    println!("num: {}", num);

    // The Borrow Checker finds permission violations
    // This code will not compile if this is uncommented
    // let mut v: Vec<i32> = vec![1, 2, 3];
    // let num: &i32 = &v[2];
    // v.push(4); // This line is a violation as v does not have the W permission
    // println!("Third element is {}", *num);

    // DATA MUST OUTLIVE ALL ITS REFERENCES
    // Intro to the fourth permission, Flow
    // This permission is required whenever a function uses input references
    // Unlike the other 3 permissions, this does not change throughout the body of the function
    fn first(strings: &Vec<String>) -> &String {
        let s_ref = &strings[0]; // RF
        s_ref // RF
    }
    // This function (first) works, but the function below (first_or) doesnt, try uncommenting and see the help(ignore lifetime for now) provided in the error
    // fn first_or(strings: &Vec<String>, default: &String) -> &String {
    //     if strings.len() > 0 {
    //         &strings[0] // R
    //     } else {
    //         default  // R
    //     }
    // }
}
// Please go to chapter_10_3 after completing this, and then continue :)
