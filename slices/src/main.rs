fn main() {
    let string_obj = String::from("Hello world string");
    let first_slice = first_word(&string_obj);
    println!("First word from string object: {first_slice}");
    let first_slice_on_slice = first_word(&string_obj[6..]);
    println!("First word from string object slice: {first_slice_on_slice}");
    let literal = "Hello world literal";
    let first_slice = first_word(&literal);
    println!("First word from literal: {first_slice}");
    let first_slice_on_slice = first_word(&literal[6..]);
    println!("First word from string object slice: {first_slice_on_slice}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
