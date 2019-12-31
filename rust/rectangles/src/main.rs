fn main() {
    variables();
    tuples();
    structs();

    methods();
}

fn variables() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "Variables: The area of the rectangle is {} square pixels.",
        area_var(width1, height1)
    );
}

fn area_var(width: u32, height: u32) -> u32 {
    width * height
}

fn tuples() {
    let rect1 = (30, 50);

    println!(
        "Tuples: The area of the rectangle is {} square pixels.",
        area_tup(rect1)
    );
}

fn area_tup(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }   
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn structs() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "Structs: The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}


fn methods() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}