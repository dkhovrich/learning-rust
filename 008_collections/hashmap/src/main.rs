use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores.entry(String::from("Green")).or_insert(70);
    scores.entry(String::from("Green")).or_insert(80);

    println!("{:?}", scores);

    let team_name = String::from("Blue");
    // let score = scores.get(&team_name);

    if let Some(score) = scores.get(&team_name) {
        println!("Score: {}", score);
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{},{}", field_name, field_value);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
