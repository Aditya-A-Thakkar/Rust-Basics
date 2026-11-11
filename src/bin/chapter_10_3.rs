// Lifetimes of references
fn main() {
    // Preventing Dangling References - the main aim of lifetimes

    // Consider this unsafe program
    let r;
    {
        let x = 5;
        r = &x;
    }
    // println!("*r = {}", *r);
    // Uncomment the line above and see the compiler error
    // REASON: x has to be dropped when it goes out of scope since x is not a reference. But if r is live after line 10,
    // x would not have O permission, and hence would not be droppable. This can't be allowed, since every value must
    // be dropped when its owning place goes out of scope, in order to free any memory pointed to by fields within the value.
    // Hence, long story short, the compiler disallows a reference to a value or a part of a value to be live
    // after the place that owns the value goes out of scope. In other words,  DATA MUST OUTLIVE ALL REFERENCES TO IT.


    // Lifetime Annotations are syntactically similar to type parameters. 
    /* The annotations below serve two purposes:
        1. It tells the compiler to check after each call-site to longest that the returned reference is not used
           beyond the life of the shorter-lived data among the two data items referred to by the two actual parameters.
        2. As long as the returned reference is live after the call-site, both the actual parameter references 
           (i.e., borrows) are considered active/live.
        
        Note, the above two are enforced simply based on the signature of the function, and NOT by analyzing the code
        in the function. In other words, the signature is a declaration of the lifetime of the returned reference in
        terms of the lifetimes of the parameters.

        Separately, during the compilation of the function longest, the compiler checks that returned reference 
        has lifetime that is provably as long as at least one of the parameters tagged 'a. But the returned reference
        need not necessarily be equal to one of the parameter references. 

        A function can have both type parameters and lifetime parameters.

        There are some lifetime "elision" rules, which allow the compiler to implicitly add the lifetimes to the function
        signature in certain scenarios. We do not cover those rules.
    */
    fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {
         if x.len() < y.len() {
             y
         } else {
             x
         }
    }

    let x = String::new();
    let y = String::new();
    let r1;
    {
        let r2 = &x;
        let r3 = &y;
        r1 = longest(r2, r3);
    }
    // x.push_str(", world"); does not work, because the borrow by r2 is considered live as long as r1 is live
    // y.push_str(", yes"); does not work, for the same reason
    println!("{}", r1); // works even though r2 and r3 are not live here, because the data items underlying  them, 
                        // namely, x and y, are both in scope here.
                        // does NOT work if any of the variables x and y are declared inside the block.

    fn return_static () -> &'static str { 
        let new: &'static str = "Hello";
        new
    } // static means the returned reference can be used till the end of the program


    // Lifetime Annotations in struct definitions. 
    // The <'a> denotes the lifetime of the struct itself.
    // So we are basically saying that the data referred to by the 
    // reference placed in the field 'part' should live at least till the point(s)
    // where the fields of the struct are accessed. 
    struct ImportantExcerpt<'a> {
        part: &'a str, // lifetime specifier required for struct fields, does not compile without the specifier
    }
    
    let mut iex = ImportantExcerpt{part: "Hello"}; // ok, since "Hello" has 'static life time
    {
        let s2 = String::from("world");
        let s3 = &s2[0..2];
        iex.part = s3;
        println!("iex.part = {}, expecting wo", iex.part); // ok
    }
    // println!("iex.part = {}", iex.part); // If you uncomment this, an error is raised  above when assigning s3 to iex.part

    let mut v = vec![11,21,31];
    let rv = &v[0];
    // v.push(41);  // Not allowed, since the borrow of v[0] above is treated as a borrow of the vector itself, and that borrow
                    // is considered live as long as rv is live.
                    
                    /*  This is taken care of by the "index" function in the "Index" trait that Vec implements. 
                        Implementing this trait is necessary to use the [..] operator.
                        The signature of the index function is: fn index(&self, index: I) -> &Self::Output ,
                        which basically says that this function takes a reference to the vector itself (i.e., 
                        &self), and keeps this borrow live as long as the reference returned by the indexing
                        operation (rv in our case) is live.
                        
                        Similar to index, there is an index_mut function that is used when we write code like
                        v[0] = 5. It's signature is fn index_mut(&mut self, index: I) -> &mut Self::Output ,
                        meaning it tries to grab  a mutable reference to the vector and holds this borrow as long
                        as the assignment (via this mutable reference) is not yet completed. */

    println!("*rv = {}, expecting 11", *rv);

    // Lifetime Elision under some scenarios
    // Consider this function
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    // It compiles WITHOUT any error, without any lifetime annotations
    // It uses implicit lifetime annotation rules, such as:
    // 1. If there is exactly one reference argument, that lifetime is assigned to all output references
    // 2. If there are multiple input reference arguments, but one of them is &self or &mut self 
    //    (only methods of structs can have '&self' as a param), 
    //    the lifetime of &self is assigned to all output references.
    // Note, the 'longest' method needed explicit lifetime annotations because none of these implicit rules help there to infer the lifetime
    // of the returned reference.

    // Above, using Rule 1,  the compiler will treat the function first_word as
    // fn first_word<'a>(s: &'a str) -> &'a str {}

    // An example combining Generic Types, Trait bounds, and lifetimes
    use std::fmt::Display;
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T
    ) -> &'a str where T: Display {
        // 'where' causes requires T to have the Display trait. Can also say T:Display in the first line instead of using where.
        println!("Announcement! {}", ann);
        if x.len() > y.len() { x } else { y }
    }
}