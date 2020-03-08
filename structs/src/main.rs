fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool
    };

    //all the properties must be defined
    let user1 = User {
        username: String::from("user"),
        email: String::from("asdas"),
        sign_in_count: 1,
        active: true
    };

    println!("{}", user1.username);

    let user2 = User {
        username: String::from("user2"),
        email: String::from("asdas"),
        ..user1
    };

    println!("{}", user2.username);
}

// fn build_user (email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1
//     }
// }