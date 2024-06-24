struct User {
    _active: bool,
    _username: String,
    email: String,
    _sign_in_count: u64,
}

fn main() {
    let _user1 = User {
        _active: true,
        _username: String::from("user1"),
        email: String::from("user1@email.com"),
        _sign_in_count: 1,
    };

    let mut _user2 = User {
        _active: true,
        _username: String::from("user2"),
        email: String::from("user2@email.com"),
        _sign_in_count: 1,
    };
    _user2.email = String::from("user22@email.com");


}

fn _build_user(email: String, username: String) -> User {
    User {
        email,
        _username: username,
        _active: true,
        _sign_in_count: 1,
    }
}