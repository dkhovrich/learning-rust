fn main() {
    let mut s = String::from("hello world");
    // let word = first_word(&s);
    //
    // s.clear();
    // println!("{}, {}", s, word);

    let hello = &s[..5];
    let world = &s[6..];

    println!("{} {}", hello, world);

    let word = first_word_slice("hello world");
    println!("{}", word);

    s.clear();
    // println!("{}", word); error

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    println!("{:?}", slice);
}

fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
