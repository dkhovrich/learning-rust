fn main() {
    let c = "Привет!";

    let s = &c[0..4];

    println!("{}", s);

    for c in "hello, world".chars() {
        println!("{}", c)
    }

    for c in "hello, world".bytes() {
        println!("{}", c)
    }
}
