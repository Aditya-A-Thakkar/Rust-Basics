fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");

    fn greet(g1: String, g2: String) -> (String, String) {
        println!("{} {}!", g1, g2);
        (g1, g2)
    }

    // Variables cannot be used after being moved.
    // The function greet takes the ownership of the values of m1, and m2, hence they cannot be used after the
    // call below. However, we can use m1_again and m2_again after the call.
    let (m1_again, m2_again) = greet(m1, m2);
    // println!("{}", m1);  reference to m1 above not allowed since m1 was moved to g1
    println!("{}", m1_again);
    /* Copying values to and fro like this is both inconvenient and expensive */

    /*  References are basically type-safe pointers.
    A reference variable owns its (pointer) value, but not the value being pointed to. */
    fn greet_with_references(g1: &String, g2: &String) {
        // Note the '&'
        println!("{} {}!", g1, g2); // technically should have been *g1, *g2, but Rust implicitly adds *'s where necessary
                                    /* g1.drop() and g2.drop() do get called here, but the drop methods
                                    of references do nothing. */
    }
    greet_with_references(&m1_again, &m2_again); // Note the '&'. Rust does not add '&' implicitly.

    /* Mutable references:- They provide 'unique', 'non-owning' access to data */
    let mut v1: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v1[2];
    // println!("{}", v1[0]);
    /* Above line does not work, because taking a mutable reference blocks any access to the underlying data without
    going through this reference. */
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v1); // The reference num is no longer live

    /*
      Rust avoids simultaneous aliasing and mutation (Pointer Safety Principle)
      In Rust, you can either have
        1. One mutable reference, or
        2. Any number of immutable references
      But NOT both at the same time. The underlying variable name (e.g., answer1 below) is also treated
      as a reference for the purposes  of the rule mentioned above.
    */

    // RUST avoids simultaneous aliasing and mutation
    let mut answer1 = String::from("Hello");
    let ans_ref1 = &answer1;
    // answer1.push_str(" world"); // NOT ALLOWED
    println!("Expecting Hello, got: {}", ans_ref1);
    /* Cannot mutate underlying data while a reference to it is live. In this case the mutation can't actually cause a problem, unless the reference is passed to another thread (in which case data races can occur between the write (i.e., push_str) in the main thread and the read via the reference in the other thread).  */

    let mut answer2 = String::from("Hello");
    let ans_ref2 = &answer2;
    let substr2 = &ans_ref2[0..2];
    // answer2.push_str(" world"); // NOT ALLOWED
    println!("Expecting He, got: {}", substr2);
    /* In this example, the string-slice substr2 could become a dangling pointer in case the push_str relocates
    the underlying string in the heap. */

    let mut answer3 = String::from("Hello");
    let substr3 = &answer3[0..2];
    let mut ans_ref3 = &mut answer3;
    ans_ref3.push_str(" world");
    // println!("Expecting He, got: {}", substr3); NOT ALLOWED, unless we comment out both lines above
    /* In this example, the mutation via the mutable reference is not allowed if we uncomment the println above,
    since, a mutable reference and an immutable reference are live together. Note, if it was allowed, substr3 could
    become a dangling pointer due to the mutation. */

    /*
      A "place" is anything that can occur on the LHS of an assignment statement. In other words, it is an l-value.
      Examples: v, *v, v[1], etc.
      The idea behind how the Borrow Checker works is that any place has 3 *permissions*
      1. Read(R), 2. Write(W), 3. Own(O), [4. Flow(F)]
      Taking references changes the permissions.
    */
    let mut v3 = vec![10, 20, 30]; // v3 -> {R, W, O}. That is, v3 has all permissions immediately upon declaration.
    let num = &v3[2]; 
        /* Permissions at this point: v3 -> {R}, num -> {R,O}, *num -> {R} */

    // v3.push(40);   NOT ALLOWED, since v3 does not have W permission
    // let _v3a = v3; NOT ALLOWED, since v3 does not have O permission

    /*
        Conceptually, taking a reference to a vector element results in calling the method 'index' on the vector, which  takes a reference to the outermost Vec struct itself and keeps this "borrow" of the vector alive as long as the returned reference (i.e., num) is live. This borrow causes removal of W and O permissions on the vector. 
                    
        Note, one needs W permission on v3 to be able to mutate any element of v3 (how this permission check happens (via the index_mut method) will be discussed later). The reason W permission is removed is that additions to the vector can move the entire vector in heap, which can make num a dangling pointer. Or if you try to re-assign v3 entirely to a new vector, the compiler would want to call the drop() method on the old value, which would in turn want to de-allocate the heap buffer. 

        The O permission exists to prevent a move of the struct to which (or to a sub-struct of which) another reference exists. For if such a move was allowed, after the move, the target struct (i.e., the target of the move) could get mutated or even get dropped later. This could cause any  pointers in the original struct (to which references exist) to become dangling. Lack of O permission means don't allow to be moved to another place. A place will get back its O permission when all references to the place (and to other locations reachable from the place) reach the end of their lifetimes (and the compiler requires the lifetimes of references to be a subset of the lifetime of the main struct).  Note, when a value is to be dropped, the compiler will call the drop() function, which  has `self` as its argument (not &self), so it basically moves the actual param value to the formal param `self'. In other words, a value can be dropped only when the owning place has 'O' permission.

        Note, *num has no O permission, and this means don't drop the value in  *num when num goes out of scope. Any reference is basically a non-owning pointer. 
    
        Note that the compiler has to somehow conservatively track which places hold references to which other places, and till what point the borrows are live. This is challenging when reference variables are copied to other reference variables, etc. We do not discuss further how exactly this is done.
    */
    println!("Expecting 20, got: {}", v3[1]); // ok, since v3 still has R permission
    println!("Expecting 30, got: {}", *num);  // ok, since num and *num have R permission
                                              // here, the permissions are v3 -> {R,W,O}, since num is no longer live
    v3[1] = 19;

    /* 
      A note about programming practice: Why save a reference to v3[2] in num (and pass this reference to wherever it will be dereferenced), rather than just storing the index 2 in num and referring v3[num] later? Well, holding on to the reference prevents (unintended) mutations to v3 that might insert additional elements before position 2 or truncate the vector to lose position 2 onwards. 
    */
    /*  
        Vec has a method split_at_mut, which returns two mutable array slices representing two disjoint partitions of the vector. This allows concurrent readers and writes, or even two concurrent writers, but to disjoint parts of the vector. 
    */


    let mut v4: Vec<i32> = vec![11, 21, 31]; // v -> {R, W, O}
    let num: &mut i32 = &mut v4[2]; // v4 -> {}, num -> {R, W, O}, *num -> {R, W}       
    /*
        Conceptually, taking a mutable reference to a vector element results in calling the method 'index_mut' on the vector, which takes a mutable reference to the outermost Vec struct itself and keeps that "borrow" of the vector alive as long as the returned reference (i.e., num) is live.
    */
    // let _t4 = &v4[0];  NOT ALLOWED
    *num += 1;
    println!("Expecting 32, got: {}", v4[2]); // v3 -> {R,W,O}, since num is no longer live


    let v6: Vec<i32> = vec![1, 2, 3]; // v -> {R, O}
    let mut num = &v6[2]; // v4 -> {R}, num -> {R, W, O}, *num -> {R}
    // can re-assign num, but cannot write to *num (because v6 is not marked mutable)
    num = &v6[2];

    /* A more detailed note about lifetimes:- Any reference has a *lifetime*, which is a range of lines of code from its definition to its last use. The reference (and the borrow it represents) is said to  be *live* or *active* over this lifetime. Permissions are returned to the primary place (from which the reference has borrowed) after the end of the reference's lifetime. 

    The range of lines can in general be non-contiguous also, which is called non-lexical lifetimes. See fn ascii_capitalize in Chap. 4.2 in the Brown university copy of the Rust book.
    */

    // DATA MUST OUTLIVE ALL ITS REFERENCES
    let r7;
    {
        let t7 = 0;
        r7 = &t7;
        let _u7 = *r7;
    }
    //let s7 = *r7; NOT ALLOWED. The reference's lifetime is not allowed to go beyond the scope of the data it refers to.

    // We now move onto understanding references into structures
    struct Sample2 {
        fl: i32,
        hl: i32,
    }
    struct Sample1 {
        gl: Sample2,
        kl: i32,
    }

    let mut v = Sample1 {
        gl: Sample2 { fl: 0, hl: 3 },
        kl: 1,
    };
    let rf = &mut v.gl.fl; // Prefixes are v.gl and v
                           // v -> {}, v.gl -> {}, v.gl.fl -> {}, v.gl.hl -> {O, W, R}, v.kl -> {O, R, W}
    // println!("v.gl is : {:?}", v.gl); // NOT ALLOWED
    let hf = &mut v.gl.hl;
    /*
    let w = &v; // not allowed
    println!("w.kl is {:?}", (*w).kl);
    */
    println!("v.kl is expected to be 1, got: {}", v.kl); // this is NOT considered a read of v followed by a read of v.kl.
                                                         // One has to somehow consider it to be a direct read of the place v.kl.
                                                         // Note, we have no read permission to v itself.
    v.kl = 2;
    println!("v.kl is expected to be 2, got: {}", v.kl);
    println!("*rf is expected to be 0, got: {}", *rf);

    // If the struct is thought of as a tree, and a mutable reference to a node is taken,
    // then you cannot take any ref either to the prefixes or to the descendants
    // (you also lose R,W permissions to those places).

    // Explanation:- Parents aren't allowed to be referred because they can be used to
    // descend to the current node and mutate/read it. Siblings do not have any effect on the
    // current node and the current node has no effect on the siblings, and hence
    // mutation/reading can be allowed to them.

      /*  A note on Boxes.

        Similar to vectors, a Box normally owns the object in the heap that it points to. Hence,  when a Box is dropped, the heap object gets de-allocated, and additionally, recursive calls to .drop() are made corresponding to other Boxes or other structs that are declared as fields within the heap object. However, the compiler treats Box alone specially even though it is just like a user-defined type. The compiler allows the data in the heap object pointed to by a Box to be MOVEd out by an assignment statement, such as:

          let v = *b;   // entire heap object is moved out
          let w = b.f;  // value in f field of the heap object  alone is moved out

        So, when a box is dropped, the value in the heap object (or in a field of the heap object) is dropped by the compiler only if this value is still being owned by the Box. This tracking is taken care of specially by the compiler, as opposed to the drop() method in the Box taking care of this, as the drop() cannot know which parts of the object have been moved out and which ones not. Intuitively, the compiler, using special-casing, treats the struct in the heap to which the Box is pointing as if that struct is a local variable residing in the stack itself. 
        
        The same is not true about Vectors (and not true about any other user-defined type that has implemented the Deref trait). If v is a vector, then:
          let t = v[0];
        is not allowed unless the elements of v implement the copy trait. It is allowed to say:
          let t = &v[0];
          let t1 = &v[m..n];
    */

    // Following are examples how to use references syntactically. 
    // This is kind of orthogonal to the concept behind references and permissions.
    // Rust implicitly inserts dereferences and references
    let x: Box<i32> = Box::new(-1); // x is a pointer on stack to the Box on heap (all variables live in stack)
    let x_abs1 = i32::abs(*x); // Explicit dereference
    let x_abs2 = x.abs(); // Implicit deference
    assert_eq!(x_abs1, x_abs2); // This macro panics if the two values given to it are not equal

    // A simple example of a reference to a reference
    let my_num: i32 = 5;
    let my_ref: &i32 = &my_num;
    let ref_to_ref: &&i32 = &my_ref;
    println!("Expecting 5, got {}", **ref_to_ref); // the *'s can be omitted, they wil be implicitly added

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs(); // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len(); // Implicit reference. The "." operator always creates a reference, to pass to the method.
                          // Depending on the type of the base variable 's' and the expected type of the parameter,
                          // '.' could insert ampersands or stars automatically in front of the base variable. In this case &s would be passed.
    assert_eq!(s_len1, s_len2);
}
// Please go to chapter_10_3 after completing this, and then continue :)
