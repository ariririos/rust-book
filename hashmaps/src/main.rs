use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    let blue = String::from("blue");
    let yellow = String::from("yellow");
    scores.insert(blue.clone(), 10);
    let team_name = String::from("blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("blue team score: {score}");
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    scores.entry(blue).or_insert(50);
    scores.entry(yellow).or_insert(50);
    println!("{scores:?}");
    let text = "hello world wonderful world";
    let mut words = HashMap::new();
    for word in text.split_whitespace() {
        let count = words.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{words:?}");
}
