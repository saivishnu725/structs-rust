struct User {
    username: String,
    email: String,
    active: bool,
    password: i32,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let user1 = User {
        username: String::from("saivishnu725"),
        email: String::from("saivishnu725@gmail.com"),
        active: true,
        password: 1234,
    };
    //println!("{}", user1.username);
    let user2 = User {
        active: false,
        ..user1
    };
    //println!(" username : {}", user2.username);
    let rect = Rectangle {
        width: 20,
        height: 10,
    };
    println!("rect: {:#?}", rect);
    let a = area(&rect);
    //let a = rect.width * rect.height;
    println!("Area = {}", a);
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.height
}
