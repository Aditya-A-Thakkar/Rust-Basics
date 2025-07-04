fn main() {
    let s1 = String::from("hello");

    let r1 = &s1;
    let rp1 = &s1 as *const String;

    let rp2 = rp1;
    let rp3 = rp1;
    // Ownership rule 2 broken

    let some_raw_pointer;
    {
        let my_data = 1234;
        some_raw_pointer = &my_data as *const i32;
    }

    println!("Pointer refers to:- {}", unsafe { *some_raw_pointer });
    // UB, it might print data, might print nothing, and might print something random
    // Why can I use `some_raw_pointer` when `my_data` is freed?
    // This breaks 3rd Ownership rule as well
}
