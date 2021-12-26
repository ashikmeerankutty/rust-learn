fn main() {

    let str1 = String::from("mrdevops");
    let str2 = String::from("ashik9591@gmail.com");

    let mut user = build_user(
        str1,
        str2
    );

    user.email = String::from("ashik5056@gmail.com");

    println!("{}, {}, {}, {}", user.email, user.active, user.username, user.sign_in_count);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
