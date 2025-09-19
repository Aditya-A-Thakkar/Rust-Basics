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
    // The function greet takes the ownership of the values of m1, and m2, hence they cannot be used after the
    // call below. However, we can use m1_again and m2_again after the call.
    let (m1_again, m2_again) = greet(m1, m2);

    // Here, m1_again and m2_again are borrowed, and they are not moved into this new function.
    // So, they remain accessible after the call.
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
    assert_eq!(x_abs1, x_abs2); // This macro panics if the two values given to it are not equal

    // TODO add  a simple example of a reference to a reference

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs(); // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len(); // Implicit reference. The "." operator always creates a reference, to pass to the method. Depending on the type of the base variable 's' and the expected type of the parameter,
    // '.' could insert ampersands or stars automatically in front of the base variable. In this case &s would be passed.
    assert_eq!(s_len1, s_len2);

    // Rust avoids simultaneous aliasing and mutation (Pointer Safety Principle)
    // In Rust, you can either have
    // 1. One mutable reference, or
    // 2. Any number of immutable references
    // But NOT both at the same time.

    // RUST avoids simultaneous aliasing and mutation
    let mut v2: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v2[2];
    // v2.push(4); // NOT ALLOWED (WHY?)
    println!("Third element is {}", *num);
    v2.push(4); // ALLOWED (WHY?)
    // TODO instead of accessing via the variable v2 (which will come later), maybe better to show simultaneous use of immutable and mutable references

    // Reference change permissions
    // The idea behind the Borrow Checker is that any place (including variables) has 3 permissions
    // 1. Read(R), 2. Write(W), 3. Own(O), [4. Flow(F)]
    let mut v3 = vec![1, 2, 3]; // v -> {R, W, O}
    let num = &v3[2]; // v -> {R}, referencing removed the write and own permission from v, you cannot change/own v until num dies
    println!("num: {}", num);

    struct Sample2 {fl:i32}
    struct Sample1 {gl:Sample2, kl:i32}

    let  mut v = Sample1{gl:Sample2{fl:0}, kl:1};
    let rf = &mut v.gl.fl;
    println!("v.kl is {}", v.kl);
    // v = Sample { fl: 5 };
    println!("{rf}");



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
    let mut my_string = String::from("hello world"); // my_string - {R, W, O}
    let s1 = &my_string; // my_string - {R}
    let s2 = &my_string; // my_string - {R}
    println!("s1: {}, s2: {}", s1, s2);
    // my_string - {R, W, O}
    let s3 = &mut my_string; // my_string - {}
    s3.push('!');
    println!("s3: {}", *s3);
    // my_string - {R, W, O}

    // Notice that when s3 is declared, s1 and s2 have already been dropped because of their lifetime
    // Declare s3 above the println statement to see the compiler error.

    // DATA MUST OUTLIVE ALL ITS REFERENCES
    let s = String::from("Hello world"); // s -> {R, O}
    let s_ref = &s; // s -> {R}
    // drop(s); // This line is rejected by the borrow checker as s does not have the 'O' permission
    println!("{}", s_ref);
}
// Please go to chapter_10_3 after completing this, and then continue :)
