#[allow(unused)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
#[allow(unused)]
#[derive(Debug)]
struct Color(i32, i32, i32);
#[allow(unused)]
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let user1 = build_user(String::from("Test"), String::from("test@test.com"));
    println!("{user1:?}");
    let user2 = User {
        email: String::from("test2@test.com"),
        ..user1
    };
    println!("{user2:?}");
    println!("{}", user1.email);
    let color = Color(0, 0, 0);
    let point = Point(0, 0, 0);
    println!("{color:?}");
    println!("{point:?}");
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}
