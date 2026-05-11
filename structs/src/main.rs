/**
 * Structs
 * structs are used to name and package related values similar to tuples.
 */

fn main() {
    // tuples
    let rect = (200, 500);

    // struct
    struct Book {
        title: String,
        author: String,
        pages: i32,
        available: bool,
    };

    struct User {
        active: bool,
        name: String,
        email: String,
        sign_in_count: u64,
    };

    let mut user1 = User {
        active: true,
        name: String::from("Jhon"),
        email: String::from("jhon@email.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("doe@email.com");
    print!("User email is: {}", user1.email);

    fn build_user(email: String, name: String) -> User {
        User {
            active: true,
            email,
            name,
            sign_in_count: 1,
        }
    }

    let user2 = User {
        email: String::from("doe@email.com"),
        ..user1
    };

    // struct tuples
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    // unit-like struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}
