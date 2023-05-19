#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // NOTE: these are 'methods' since they take a reference to the instance of the object
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // NOTE: this on the other hand can be called with the syntax "Rectangle::square()"
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // NOTE: you could use this for a constructor 'new'
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 10,
    };

    println!("The are of the rectangle is {} square pixels.", rect.area());
    if rect.can_hold(&rect2) {
        println!("Can hold!")
    }

    let _three_squared = Rectangle::square(3);
    let _rect3 = Rectangle::new(2, 3);
}
