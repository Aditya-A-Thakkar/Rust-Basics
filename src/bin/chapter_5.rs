// Structs
// They are similar to tuples, the difference being that we have fields in structs.
struct Student {
    name: String,
    college: String,
    sr_no: usize,
    age: usize,
    email: String,
}

// Unit-like structs
struct Spade;
struct IAmNull;

fn add_student(name: String, age: usize) -> Student {
    Student{
        name: name.clone(),
        college: String::from("ABC"),
        sr_no: 12345,
        age,
        email: String::from(name + "@abc.com"),
    }
}

fn main() {
    // let adi :Student = Student{
    //     name: String::from("Aditya"),
    //     college: String::from("IISc"),
    //     sr_no: 12345,
    //     age: 18,
    //     email: String::from("adityaat@iisc.ac.in"),
    // };

    let adi = add_student(String::from("adi"), 18);
    let adi2 = Student{
        email: String::from("aditya@abc.com"),
        ..adi
    };// NOTE: This is a bad way because adi2 has owned the name, and college fields of adi, and hence they cannot be used anymore

    println!("Email:- {}", adi.email);
    println!("Email:- {}", adi2.email);

    println!("Sr No:- {}", adi.sr_no);
    println!("Sr No:- {}", adi2.sr_no);

    // The rules for references and borrowing also apply while borrowing the fields from structs.
}