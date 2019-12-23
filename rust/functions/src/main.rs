fn main() {
    another_function(3, 4);

    let z = plus_one(11);

    if z % 4 == 0 {
        println!("z is divisible by 4")
    } else if z % 3 == 0 {
        println!("z is divisible by 3")
    } else {
        println!("The value of z, {}, isn't divisible by either 4 or 3.", z);
    }
    
}

fn another_function(x: i32, y: i32) {
    println!("The values of x and y are: {} and {}", x, y);
}

// fn that returns an i32
fn plus_one(x: i32) -> i32 {
    x + 1  //Note no semicolon, so this is an expression.  Last expression is returned.
}