struct User {
    active: bool,
    username: String, // If we use &str, we need lifetimes
    email: String,
    sign_in_count: u64
}

// Tuple structs
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

// Unit-like structs (like unit '()')
struct AlwaysEqual;

fn main() {
    let mut user = User {
        active: true,
        username: String::from("Camilla"),
        email: String::from("camilla@bella.it"),
        sign_in_count: 47646776476
    };
    user.email = String::from("camilla07@bella.it");

    let user2 = create_user(String::from("Marco"), String::from("marco123@bella.it"));
    println!("{} - {} - {} - {}", user2.active, user2.username, user2.email, user2.sign_in_count);

    /* let user3 = User {
        active: user.active, // Value moved here
        username: user.username,
        email: String::from("camilla007@bella.it"),
        sign_in_count: user.sign_in_count
    }; */

    let _user4 = User {
        email: String::from("camilla08@bella.it"),
        ..user // Update syntax: all field not set have the same value of the struct variable
        // This moves user.username to user4.username, so user can no longer use
    };

    // Tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let Point(x, y, z) = origin;
    println!("RGB: {} {} {}", black.0, black.1, black.2);
    println!("A({x}, {y}, {z})");

    // Unit-like structs
    let _subject = AlwaysEqual;
}

fn create_user(username: String, email: String) -> User {
    User {
        active: true,
        username, // We can do this because field and param have the same name
        email,
        sign_in_count: 1
    }
}