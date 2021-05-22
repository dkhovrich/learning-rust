fn main() {
    let s = String::from("Hello");
    // take_ownership(s);
    // println!("{}", s);

    let (s2, len) = calc_length(s);
    println!("s2: {}, len: {}", s2, len);

    let x = 5;
    make_copy(x);
    println!("{}", x);
}

fn make_copy(x: i32) {
    println!("{}", x);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn calc_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
    // return (s, length);
}
