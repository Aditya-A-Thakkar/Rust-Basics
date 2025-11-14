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

    // A simple example of a reference to a reference
    let my_num :i32 = 5;
    let my_ref :&i32 = &my_num;
    let ref_to_ref :&&i32 = &my_ref;

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
    let mut answer = String::from("Hello");
    let ans_ref = &answer;
    // answer.push_str(" world"); // NOT ALLOWED
    println!("{}", ans_ref);

    // Reference change permissions
    // The idea behind the Borrow Checker is that any place (including variables) has 3 permissions
    // 1. Read(R), 2. Write(W), 3. Own(O), [4. Flow(F)]
    let mut v3 = vec![1, 2, 3]; // v -> {R, W, O}
    let num = &v3[2]; // v3 -> {R}, referencing removed the write and own permission from v3, you cannot change/move v3 until num dies
    println!("num: {}", num);
    // If an object is moved from one place to another (like in chapter_4.rs) , the first place loses its ownership, which in fact
    // causes a loss of all three permissions. 

    // The O permission exists to prevent a move of the struct to which (or to a sub-struct of which) another reference exists. For
    // if such a move was allowed, after the move, the target struct could even get dropped later. This could 
    // cause any raw pointers in the original struct (to which references exist) to become dangling. Lack of
    // O permission means don't allow moves. A place will get back its O permission when all references 
    // to the place reach the end of their lifetimes (and the compiler requires the lifetimes of references
    // to be a subset of the lifetime of the main struct).  Note, when a value is to be dropped,
    // the compiler will call the drop() function, which  has `self` as its argument (not &self), so it basically
    // moves the actual param value to the formal param `self'. In other words, a value can be dropped
    // only when the owning place has 'O' permission. 
    
    // Note that the compiler has to somehow conservatively track which places hold references to which other places. 
    // This is challenging when reference variables are copied to other reference variables, etc.
    // We do not discuss further how exactly this is done.
    // TODO: Aditya, please insert here the example where the compiler over-approximates the notion of who refers to who.

    // TODO: Change Debug to Display.
    #[derive(Debug)]
    struct Sample2 {fl:i32, hl:i32}
    #[derive(Debug)]
    struct Sample1 {gl:Sample2, kl:i32}

    let  mut v = Sample1{gl:Sample2{fl:0,hl:3}, kl:1};
    let rf = &mut v.gl.fl; // Prefixes are v.gl and v
    // v -> {}, v.gl -> {}, v.gl.fl -> {}, v.kl -> {R, W}
    // println!("v.gl is : {:?}", v.gl); // NOT ALLOWED
    let hf = &mut v.gl.hl;
    println!("v.kl is {:?}", v.kl);
    v.kl = 2;
    println!("v.kl is {:?}", v.kl);
    println!("rf = {rf:?}");

    // If the struct is thought of as a tree, and a mutable reference to a node is taken,
    // then you cannot take any ref either to the prefixes or to the descendants
    // (you also lose R,W permissions).

    // Explanation:- Parents aren't allowed to be referred because they can be used to
    // descend to the current node and mutate/read it. Siblings do not have any effect on the
    // current node and the current node has no effect on the siblings, and hence
    // mutation/reading can be allowed to them.


    // Similar to previous one, can think of this as a tree,
    let mut v4: Vec<i32> = vec![1, 2, 3]; // v -> {R, W, O}
    let num: &mut i32 = &mut v4[2]; // v4 -> {}, num -> {R, O}, *num -> {R, W}, referencing removed all the permissions on v4
    // taking a reference to a vector element results in calling the method 'index' or 'index_mut', which take a reference
    // to the outermost Vec struct itself.
    *num += 1;
    // A fall out of the above restriction is that if you have an immutable reference  to a vector element, the vector
    // itself becomes inaccessible for writing. The reason they do this is that by calling the push method etc. one can 
    // end up re-allocating the buffer in heap and hence updating the raw pointer inside the struct that points to this buffer. 
    // Or if you try to re-assign v4 entirely, the compiler would want to call the drop() method on the struct,
    // which would in turn want to de-allocate the heap buffer. Similarly, if we have  a mutable reference to an element of the vector,
    // no further reading of the vector is allowed while the reference is live.

    // The Borrow Checker finds permission violations
    // This code will not compile if this is uncommented
    let v5: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v5[2];
    // v5.push(4); // This line is a violation as v5 does not have the W permission
    println!("Third element is {}", *num);

    // Intro to lifetimes:- A reference has a lifetime, which is a range from its definition to its last use. 
    // NOTE: Permissions are returned to the primary place after the end of the reference's lifetime
    let mut my_string = String::from("hello world"); // my_string - {R, W, O}
    let s1 = &my_string; // my_string - {R}
    let s2 = &my_string; // my_string - {R}
    println!("s1: {}, s2: {}", s1, s2);
    // my_string - {R, W, O}
    let s3 = &mut my_string; // my_string - {}
    s3.push('!');
    println!("s3: {}", *s3);
    // my_string - {R, W, O}

    // Notice that when s3 is declared, s1 and s2 are no longer live.
    // Declare s3 above the first println statement to see the compiler error.

    // DATA MUST OUTLIVE ALL ITS REFERENCES
    let s = String::from("Hello world"); // s -> {R, O}
    let s_ref = &s; // s -> {R}
    // drop(s); // This explicit call to drop() is rejected by the borrow checker as s does not have the 'O' permission
    // Note, in its implementation, the drop() function has `self` as its argument (not &self), so it basically
    // moves the actual param value to the formal param `self'.
    // Also, for the same reason, a function cannot return a reference to a value that is owned
    // by a local variable of the function. 
    println!("{}", s_ref);
}
// Please go to chapter_10_3 after completing this, and then continue :)
