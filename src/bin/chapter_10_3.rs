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
    // REASON: x goes out of scope and is dropped, then r becomes a pointer to freed memory which is undefined behaviour

    // The Borrow Checker ensures that DATA OUTLIVES ALL ITS REFERENCES

    // Fixed the code so that it does not have a dangling reference
    let x = 5;
    let r = &x;
    println!("r: {}", r);

    // Now consider this code below
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() { x } else { y }
    // }
    // Uncomment this code and see the help given in the error
    //  help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
    //
    // Rust cannot tell whether the reference being returned refers to x or y

    // Lifetime Annotation
    // LIFETIME ANNOTATIONS DO *NOT* CHANGE HOW LONG ANY OF THE REFERENCES LIVE.
    // They describe the relationship between the references.
    fn longest<'a, 'b>(x: &'a str, y: &'a str) -> &'b str {
        // if x.len() > y.len() {
        //     x
        // } else {
        //     y
        // }
        let new: &'b str = "Hello";
        new
    } // WORKS!! and gives the expected output

    // WORKS here
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

    // NOT HERE
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");
    // WHY?: DATA DOES NOT OUTLIVE ALL ITS REFERENCES
    // string2 should be valid when result is used, but it is dropped.

    // Lifetime Annotations in struct definitions
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let _i = ImportantExcerpt {
        part: first_sentence,
    };

    // Lifetime Elision
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
    // 1. The compiler assigns a different lifetime parameter to each lifetime in each input type
    // 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
    // 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters

    // Now, using rule 2, we know that the compiler sees the function first_word as
    // fn first_word<'a>(s: &'a str) -> &'a str {}

    // Lifetime Annotations in Method Definitions
    // Consider our struct ImportantExcerpt
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }

        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
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
        println!("Announcement! {}", ann);
        if x.len() > y.len() { x } else { y }
    }
}