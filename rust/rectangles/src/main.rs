fn main() {
    variables();
    tuples();
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