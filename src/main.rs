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

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_equal(&self, other: &Rectangle) -> bool {
        self.width == other.width && self.height == other.height
    }
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
    let a = rect.area();
    //let a = area(&rect);
    //let a = rect.width * rect.height;
    println!("Area = {}", a);

    let rect2 = Rectangle {
        width: 20,
        height: 10,
    };
    let rect3 = Rectangle {
        width: 30,
        height: 20,
    };
    println!("rect == rect2 : {}", rect.is_equal(&rect2));
    println!("rect == rect3 : {}", rect.is_equal(&rect3));
}

/* fn area(r: &Rectangle) -> u32 {
    r.width * r.height
} */
