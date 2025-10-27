use std::collections::HashMap;

fn vectors() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50; // Adds 50 to all elements. *i is the deference operator
    }

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
}

fn strings() {
    let s = String::new();
    let mut s = "initial contents".to_string(); // or String::from("initial contents");
    s.push_str(" and more");

    let s1 = String::from("Hello, ");
    let s2 = s + &s1; // s has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // format! does not take ownership of any parameter

    for c in "नमस्ते".chars() {
        println!("{c}");
    }
    for b in "नमस्ते".bytes() {
        println!("{b}");
    }
}

fn hash_maps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // copied() to get an i32 instead of &i32, unwrap_or() to handle None

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // field_name and field_value are moved here and can no longer be used

    scores.insert(String::from("Blue"), 25); // Overwrites the value for "Blue"
    scores.entry(String::from("Yellow")).or_insert(50); // Does nothing since "Yellow" is already present
    scores.entry(String::from("Green")).or_insert(50); // Inserts "Green" with value 50

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // get the counter for a word or init to 0 if first occurrence
        *count += 1; // increment the counter
    }

    println!("{:?}", map);
}

fn main() {
    vectors();
    strings();
    hash_maps();
}
