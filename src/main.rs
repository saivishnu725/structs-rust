struct User {
    username: String,
    emial: String,
    active: bool,
    password: i8,
}
fn main() {
    let user1 = User {
        username: String::from("saivishnu725"),
        email: String::from("saivishnu725@gmail.com"),
        active: true,
        password: 1234,
    };
}
