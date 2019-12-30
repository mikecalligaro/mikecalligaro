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
    // add stuff here

    let s1 = String::from("Hello");  // Allocates memory on the heap
    takes_ownership(s1);
    // s1 got freed in takes_ownership, so we can't use it anymore
    // println!("This gives a compiler error: {}", s1);

    // passing by refrence doesn't take ownership
    let s2 = String::from("refrence");
    does_not_take_ownership(&s2);
    
    // Note that pass by refrence is immutable
    // If you want to change the string, you need to explicitly make it mutable
    let mut s3 = String::from("mutable");
    changes_string(&mut s3);
    println!("{}", s3);

    // Can have one mutable refrence in a scope.
    let m1 = &mut s3;
    println!("{}", m1);

    // But you can't have two
    let m2 = &mut s3;
    println!("{}", m2);

    // This would fail because m2 took ownership from m1 and m1 is no longer valid
    // println!("{} {}", m1, m2);

}

fn another_function(x: i32, y: i32) {
    println!("The values of x and y are: {} and {}", x, y);
}

// fn that returns an i32
fn plus_one(x: i32) -> i32 {
    x + 1  //Note no semicolon, so this is an expression.  Last expression is returned.
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
    // some_string goes out of scope and its memory is freed here, even though
    // it was allocated elsewhere.
}

fn does_not_take_ownership(some_string: &String) {
    println!("{}", *some_string);
}

fn changes_string(some_string: &mut String) {
    some_string.push_str(" string");
}