struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle{ width: size, height: size }
    }

    // Borrows a Rectangle as immutable reference
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Takes ownership of Rectangle, note that both the rectangles cannot be used after calling thus function
    fn can_hold(self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Borrows a Rectangle as mutable reference
    fn set_properties(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

fn main() {
    let mut r1 = Rectangle { width: 30, height: 50 };
    let r2 = Rectangle { width: 10, height: 40 };

    println!("{}", r1.area());
    r1.set_properties(100, 100);
    println!("{}", r1.can_hold(r2)); // r1 and r2 cannot be used beyond this line

    // println!("{}", r1.area()); // DOES NOT WORK
}