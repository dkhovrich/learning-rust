fn main() {
    let v: Vec<u32> = Vec::new();
    let mut v = vec![1, 2, 3];

    v.push(4);

    let a = &v[2];

    println!("vector: {:?}, a: {}", v, a);

    if let Some(b) = v.get(2) {
        println!("b: {}", b);
    }

    match v.get(2) {
        Some(val) => println!("Value is: {}", { val }),
        _ => {}
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("i:{}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Row:{:?}", row);
}
