use std::collections::HashMap;

pub fn vec_avg_main() {
    let items = vec![4, 3, 5, 1, 2, 19, 9, 9];
    println!("Items: {:?}", items);

    let mut item_count = HashMap::new();
    let mut total = 0;

    for i in items.iter() {
        total += i;
        let count = item_count.entry(i).or_insert(0);
        *count += 1;
    }

    let values: Vec<i32> = item_count.values().copied().collect();

    match values.iter().max() {
        Some(v) => println!("Most frequent value: {}", v),
        None => println!("Empty"),
    }
}
