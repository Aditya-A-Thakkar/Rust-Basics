fn main() {
    // References are non-owning pointers, and dereferencing it gives access to its data
    let m1 = String::from("Hello");
    let m2 = String::from("world");

    fn greet(g1: String, g2: String) -> (String, String) {
        println!("{} {}!", g1, g2);
        (g1, g2)
    }

    fn greet_with_references(g1: &String, g2: &String) { // Note the '&'
        println!("{} {}!", g1, g2);
    }

    // Variables cannot be used after being moved.
    // The function greet takes the ownership of the values of m1, and m2, hence they cannot be used.
    let (m1_again, m2_again) = greet(m1, m2);

    // Here, m1_again and m2_again are borrowed, and they are not moved into this new function
    greet_with_references(&m1_again, &m2_again); // Note the '&'

    // Mutable references:- They provide 'unique', 'non-owning' access to data
    let mut v1: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v1[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v1);


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

    // RUST avoids simultaneous aliasing and mutation
    let mut v2: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v2[2];
    // v.push(4); // NOT ALLOWED (WHY?)
    println!("Third element is {}", *num);
    v2.push(4); // ALLOWED (WHY?)

    // Reference change permissions
    // The idea behind the Borrow Checker is that any variable has 3 permissions
    // 1. Read(R), 2. Write(W), 3. Own(O), [4. Flow(F)]
    let mut v3 = vec![1, 2, 3]; // v -> {R, W, O}
    let num = &v3[2]; // v -> {R}, referencing removed the write and own permission from v, you cannot change/own v until num dies
    println!("num: {}", num);

    let mut v4: Vec<i32> = vec![1, 2, 3]; // v -> {R, W, O}
    let num: &mut i32 = &mut v4[2]; // v -> {}, num -> {R, O}, *num -> {R, W}, referencing removed all the permissions on v
    *num += 1;

    // The Borrow Checker finds permission violations
    // This code will not compile if this is uncommented
    let v5: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v5[2];
    // v5.push(4); // This line is a violation as v does not have the W permission
    println!("Third element is {}", *num);

    // Intro to lifetimes:- A reference has a lifetime, which is a range from its birth to death
    // NOTE: Permissions are returned after the end of a reference's lifetime
    let mut my_string = String::from("hello world");
    let s1 = &my_string;
    let s2 = &my_string;
    println!("s1: {}, s2: {}", *s1, *s2);
    let s3 = &mut my_string;
    s3.push('!');
    println!("s3: {}", *s3);
    // Notice that when s3 is declared, s1 and s2 have already been dropped because of their lifetime
    // Declare s3 above the println statement to see the compiler error.

    // DATA MUST OUTLIVE ALL ITS REFERENCES
    let s = String::from("Hello world"); // s -> {R, O}
    let s_ref = &s; // s -> {R}
    // drop(s); // This line is rejected by the borrow checker as s does not have the 'O' permission
    println!("{}", s_ref);


    // Intro to the fourth permission, Flow
    // This permission is required whenever a function uses input references, or returns an output ref.
    // Unlike the other 3 permissions, this does not change throughout the body of the function
    fn first(strings: &Vec<String>) -> &String {
        let s_ref = &strings[0]; // s_ref -> {R, F}
        s_ref // s_ref -> {R, F}
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
