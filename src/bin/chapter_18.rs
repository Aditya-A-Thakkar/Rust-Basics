// Is Rust Object Oriented? Yes, but No
// Rust has abstraction, but not inheritance
// Talking of polymorphism,
/*
Rust instead uses generics to abstract over different possible types and trait bounds to impose
constraints on what those types must provide. This is sometimes called bounded parametric polymorphism.
 */

// Trait Objects

// Assume that we are trying to make a GUI for this chapter

// Defining a trait for common behaviour
trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}
// Why use this instead of generic type?
// A  generic type parameter can be substituted with only one concrete type at a time,
// whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Implementing the trait
struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button");
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw selectbox");
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ]
    };
    screen.run();
}

// NOTE: Dynamic dispatch comes at a runtime cost, because at compile-time, the compiler can’t tell which method you’re calling
// Dynamic dispatch also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations