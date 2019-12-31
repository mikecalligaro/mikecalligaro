
// Struct definition
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // Instantiate a struct.  Note that you need to initialize all of the
    // fields, but you don't need to do them in order
    let user1 = User {
        email: String::from("somone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("email addr {} has signed in {} time(s)", user1.email, user1.sign_in_count);

    // If you want to change values in a struct, you need to instantiate it mutable
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser321"),
        active: false,
        sign_in_count: 0,
    };

    check_user(&user2);

    user2.active = true;
    user2.sign_in_count = 1;

    check_user(&user2);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("put a 0x{:02X}{:02X}{:02X} dot at ({},{},{})", black.0, black.1, black.2, origin.0, origin.1, origin.2);
}

fn check_user(user: &User) {
    if !user.active {
        println!("user {} is inactive", user.username);
    }
    else {
        println!("user {} is active and has signed in {} time(s)", user.username, user.sign_in_count);
    }
}