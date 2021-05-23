fn main() {
    let user = create_user(String::from("t"), String::from("t@t;com"));

    let user2 = User {
        name: String::from("anotherusername567"),
        email: String::from("another@example.com"),
        ..user
    };

    let black = Color(0, 0, 0);
    println!("Hello, world! {:?}", user);

    let r = Rectangle {
        width: 10,
        height: 15,
    };

    let s = Rectangle::square(12);

    println!("{:?}, {}", r, r.area());
}

fn create_user(name: String, email: String) -> User {
    User {
        name,
        email,
        active: true,
    }
}

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    active: bool,
}

struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
