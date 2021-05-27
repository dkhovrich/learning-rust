#[derive(Debug)]
enum IpAddress {
    v4(u32, u32, u32, u32),
    v6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message: {:?}", &self);
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn coin_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State: {:?}", state);
            25
        }
    }
}

fn plus_one(v: Option<u32>) -> Option<u32> {
    match v {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn main() {
    let v4 = IpAddress::v4(127, 0, 0, 1);
    let v6 = IpAddress::v6(String::from("::1"));

    let m = Message::Write(String::from("HW"));
    m.call();

    println!("{:?}, {:?}", v4, v6);

    let coin = Coin::Quarter(UsState::Alabama);
    println!("Coin: {}", coin_in_cents(&coin));

    let v = plus_one(Some(4));
    println!("Plus one: {:?}", v);

    let some_num = Some(8);

    match some_num {
        Some(8) => {
            println!("8");
        }
        _ => {}
    }

    if let Some(8) = some_num {
        println!("It's 8");
    }

    if let Some(x) = some_num {
        println!("It's Some: {}", x);
    }
}
