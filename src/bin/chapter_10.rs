fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Instead of defining multiple function for different type, we define a generic function
// Here, T is the generic type parameter, which is forced to have the 'PartialOrd' trait because we want to compare elements.
fn largest<T : PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generic Struct Types
struct Point<T> {
    x: T,
    y: T
}

// Generic Method Definitions
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Generic Enum Types
// Widely used Option<T> and Result<T, E>

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let result = largest(&char_list);
    println!("The largest char is {result}");
}

// NOTE:- The compiler compiles generic code into that which specifies the type in each instance,
// hence, there is no runtime cost for using generic types.
