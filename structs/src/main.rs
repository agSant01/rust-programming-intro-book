#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let mut user1 = User {
        email: String::from("someone@ex.com"),
        username: String::from("someusername"),
        sign_in_count: 3,
        active: true,
    };

    user1.email = String::from("seomeai@gmail.com");

    // update syntax
    let user2 = User {
        email: String::from("someeamaile2@gmail.com"),
        username: String::from("username2"),
        ..user1
    };

    struct Point(i32, i32, i32);
    let origin = Point(0, 0, 0);
    println!("X coordinate is: {}", origin.0);

    // examples
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangle is {} square pixels",
        area(width, height)
    );

    // more solid way
    let rect = (20, 50);
    println!(
        "The area of the rectangle is {} square pixels",
        area_tuple(rect)
    );

    // area with structs
    let rect = Rectangle {
        width: 23,
        height: 89,
    };
    println!(
        "The area of the rectangle is {} square pixels",
        area_struct(&rect)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width: u32,
    height: u32,
}
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
