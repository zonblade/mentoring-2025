// Basic Struct

#[derive(Debug)]
enum UserRole {
    Admin,
    Guest,
}

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    is_active: bool,
    role: UserRole,
    website: Option<String>,
}

fn main() {
    let user1 = User {
        username: String::from("ali"),
        email: String::from("ali@test.com"),
        is_active: true,
        role: UserRole::Admin,
        website: Some(String::from("ali-code.id")),
    };

    let user2 = User {
        username: String::from("zkh"),
        email: String::from("zkh@test.com"),
        is_active: false,
        role: UserRole::Guest,
        website: None,
    };

    println!("User 1 Full Info: {:?}", user1);
    println!("User 2 Full Info: {:?}", user2);
}
