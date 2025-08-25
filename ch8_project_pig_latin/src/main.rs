use std::io;

fn main() {
    println!("Input:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let words: Vec<&str> = input.split_whitespace().collect();
    let mut output = String::new();
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    for word in words {
        match word.chars().nth(0) {
            Some(ch) => {
                if vowels.contains(&ch) {
                    let word_string = word.to_string();
                    let new_word = word_string + "hay ";
                    output.push_str(&new_word);
                }
                else {
                    let remainder = word.get(1..).expect("Could not get word remainder").to_string();
                    let new_word = format!("{remainder}{ch}ay ");
                    output.push_str(&new_word);
                }
            },
            None => panic!("Could not get first character of word {word}")
        }
    }
    println!("Output: {output}");
}
