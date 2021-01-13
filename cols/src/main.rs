fn main() {
    // complete way
    let mut v: Vec<i32> = Vec::new();
    // macro
    let v2 = vec![1,2,3];

    // add to vector
    v.push(5);
    v.push(6);
    v.push(7);

    // read from vec
    let third: &i32 = &v[2];

    for i in &v {
        println!("{}", i);
    }

    // to store different values, use enum!
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.1),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}
