fn main() {
    // Mutable
    let mut x = 5;
    println!("The value of mutable x is: {}", x);
    x = 6;
    println!("The value of mutable x is: {}", x);

    // Immutable with shadowing
    let y = 5; // Immutable variable
    let y = y + 1;  // New immutable variable with the same name
    let y = y * 2;  // 3rd immutable variable with the same name, reads from the 2nd
    println!("The value of immutable shadowed y is: {}", y); // Print out the 3rd version of x

    // Shadowing with different types
    let spaces = "    ";  // String with 4 spaces
    let spaces = spaces.len(); // Integer that is the count of spaces
    println!("There were {} spaces.", spaces);

    // Tuples - compound variables with mixed types but a fixed length
    // Declared with ()
    let tup: (i32, f64, u8) = (500, 6.4, 1); // Type declarations optional
    // destructuring
    let (_x, _y, z) = tup; //Prefixing the variable name with '_' removes the unused variable compiler warning
    println!("The value of the z element is: {}", z);
    // accessing tuple values without destructuring
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("The values of x, y, and z are: {}, {}, {}", x, y, z);
    
    // Arrays - compound variables with the same type and are also fixed length
    // (Vectors are variable length arrays)
    // Declared with []
    let a: [i32; 5] = [1, 2, 3, 4, 5];  // Type declaration and length optional
    println!("Array a index 3 is: {}", a[3]);

    // Make a 6 entry array all initialized to 0's
    let b = [0; 6];
    println!("Array b index 5 is: {}", b[5]);

    
}
