fn main() {
    let user1 = User {
        email: String::from("email@email.com"),
        username: String::from("user123"),
        active: true,
        sign_in_count: 1,
    };
    println!("email: {}", user1.email);

    let user2 = build_user(String::from("email2"),String::from("user2"));
    println!("email: {}", user2.email);
    
    let user3 = User {
        email: String::from("email3"),
        username: String::from("user3"),
        ..user1
    };

    println!("u3: {}", user3.username);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email:String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
