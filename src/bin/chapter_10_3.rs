// Lifetimes of references
fn main() {
    // Preventing Dangling References - the main aim of lifetimes

    // Consider this unsafe program
    let r;
    {
        let x = 5;
        r = &x;
    }
    // println!("r: {}", r);
    // Uncomment the line above and see the compiler error
    // REASON: x has to be dropped when it goes out of scope since x is not a reference. But if r is live after line 10,
    // x would not have O permission, and hence would not be droppable. This can't be allowed, since every value must
    // be dropped when its owning place goes out of scope, in order to free any memory pointed to by fields within the value.
    // Hence, long story short, the compiler disallows a reference to a value or a part of a value to be live
    // after the place that owns the value goes out of scope.

    // The Borrow Checker ensures that DATA OUTLIVES ALL ITS REFERENCES

    // Fixed the code so that it does not have a dangling reference
    let x = 5;
    let r = &x;
    println!("r: {}", r);


    // Lifetime Annotation
    // LIFETIME ANNOTATIONS DO *NOT* CHANGE HOW LONG ANY OF THE REFERENCES LIVE.
    // They describe the relationship between the references.
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
         if x.len() > y.len() {
             x
         } else {
             y
         }
        

    } // WORKS!! and gives the expected output


    fn return_static () -> &'static str { 
        let new: &'static str = "Hello";
        new
    } // static means lifetime of the reference is till the end of the program

    let t1 = String::from("hello");
    let a = &t1;
    let t2 = String::from("hello");
    let b = &t2;
    let c = longest(a,b);
    // Without the lifetime annotations in the signature of longest(), the 
    // compiler would not know whether c is the same as a or b. So, it would not know
    // whether it is t1 or t2 who still needs to have no O permission. 
    // With the annotations, but t1 and t2 still don't have O permission here.
    // Will see error messages if the two lines below are uncommmented.
    // let _t3 = t1;
    // let _t4 = t2;
    // Note, the function cannot return any other reference in
    // this case, since the compiler would not allow the function's code to return references to local values. 
    println!("{c}");
   

    // WORKS here. Nothing new here compared to what is seen above.
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // WORKS here
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");
    // If you uncomment above lines it is an error, because 'result' could
    // be the same reference as string2, and string2 is dropped after its scope ends.

    // Lifetime Annotations in struct definitions. The <'a> denotes the lifetime of the struct itself.
    // So we are basically saying that the reference in the field 'part' is live as long as the struct is. 
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let _i = ImportantExcerpt {
        part: first_sentence,
    };

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
    // Compiles WITHOUT any error, without any lifetime annotations
    // WHY? The compiler uses 3 rules to infer the lifetimes of these references
    // 1. The compiler assigns a different lifetime parameter to each reference argument
    // 2. If there is exactly one reference argument, that lifetime is assigned to all output references
    // 3. If there are multiple input reference arguments, but one of them is &self or &mut self (only methods of structs can have 'self' as a param), 
    // the lifetime of self is assigned to all output references.
    // Note, the 'longest' method needed explicit lifetime annotations because none of these implicit rules help there to infer the lifetime
    // of the returned reference.

    // Now, using Rule 2,  the compiler will treat the function first_word as
    // fn first_word<'a>(s: &'a str) -> &'a str {}

    // Lifetime Annotations in Method Definitions
    // Consider our struct ImportantExcerpt
    impl<'a> ImportantExcerpt<'a> {

        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
            // the return reference will be treated as having the lifetime of self.part following Rule 3 above
        }
    }

    // The static lifetime
    // One special lifetime we need to discuss is 'static
    // It denotes that the affected reference can live for the entire duration of the program.
    // All string literals have the 'static lifetime, which we can annotate as follows:
    let _s :&'static str = "I have a static lifetime.";
    // Most of the time, an error message suggesting the 'static lifetime results from
    // attempting to create a dangling reference or a mismatch of the available lifetimes.
    // In such cases, the solution is to fix those problems, not to specify the 'static lifetime.

    // Combining Generic Types, Trait bounds, and lifetimes
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