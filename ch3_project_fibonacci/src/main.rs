use std::io;

fn main() {
    println!("Enter how many numbers of Fibonacci sequence to print:");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n = n.trim().parse().expect("Failed to parse input");
    for i in 0..=n {
        let fib = nth_fibonacci(i);
        println!("F_{i}={fib}");
    }
}

fn nth_fibonacci(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
}
