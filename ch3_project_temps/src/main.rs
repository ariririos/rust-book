use std::io;

fn main() {
    println!("Enter temperature in Celsius:");
    let mut c_temp = String::new();
    io::stdin()
        .read_line(&mut c_temp)
        .expect("Failed to read line");
    let c_temp = c_temp.trim().parse().expect("Failed to parse input");
    let f_temp = c_to_f(c_temp);
    println!("in Farhenheit: {f_temp}");
}

fn c_to_f(c: f64) -> f64 {
    c * 1.8 + 32.0
}
