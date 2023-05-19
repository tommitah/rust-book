struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}

fn main() {
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let user1 = User {
        active: true,
        username: String::from("Johnny"),
        email: String::from("johnny69@gmail.com"),
        sign_in_count: 0,
    };
    if user1.active {
        println!("{}", user1.username);
    }

    let mut user2 = User {
        active: true,
        username: String::from("Janet"),
        email: String::from("janet420@gmail.com"),
        sign_in_count: 0,
    };
    user2.email = String::from("new_janet@gmail.com");

    let user3 = build_user("julian11@gmail.com".to_owned(), "Julian".to_owned());
    println!("{}", user3.username);

    let user4 = User {
        email: String::from("new_email_for_janet@gmail.com"),
        ..user2
    };
    println!(
        "user: {}, email: {}, sign-ins: {}",
        user4.username, user4.email, user4.sign_in_count
    );
}
