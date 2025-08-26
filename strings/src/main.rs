#[allow(unused)]
fn main() {
    let data = "initial";
    let s1 = data.to_string();
    let s2 = "initial".to_string();
    let mut s3 = String::from("initial");
    println!("Initial strings: {s1} {s2} {s3}");
    s3.push_str("string");
    println!("s3: {s3}");
    let mut s4 = String::new();
    s4 = s2 + " string";
    println!("s4: {s4}");
    let mut s5 = s4; // move
    let s6 = " footer";
    s5 += s6; 
    println!("s5: {s5}");
    let mut s7 = s5; // move
    let s8 = String::from(" appendage");
    s7 += &s8;
    println!("s7: {s7}");
    for c in data.chars() {
        println!("{c}");
    }
    for c in data.bytes() {
        println!("{c}");
    }
}
