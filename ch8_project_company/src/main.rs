use std::io;
use std::collections::HashMap;

fn main() {
    let mut company: HashMap<String, String> = HashMap::new();
    println!("Allowed commands:");
    println!("Add [name] to [department]");
    println!("Print [department]");
    println!("Print (will print entire company)");
    println!("Quit");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let words: Vec<String> = input.trim().split_whitespace().map(str::to_string).collect();
        match words.get(0) {
            Some(first) => {
                match &first[..] {
                    "Add" => {
                        let second = words.get(1);
                        let third = words.get(2);
                        let fourth = words.get(3);
                        if let (Some(name), Some(to), Some(dept)) = (second, third, fourth) {
                             if *to == "to" {
                                company.insert(name.clone(), dept.clone());
                             }
                        }
                        else {
                            println!("Unknown command {input}");
                        }
                    },
                    "Print" => {
                        let second = words.get(1);
                        if let Some(dept_chosen) = second {
                            for (name, dept_name) in &company {
                                if *dept_name == *dept_chosen {
                                    println!("{name}");
                                }
                            }
                        }
                        else {
                            for (name, dept) in &company {
                                println!("{name} in {dept}");
                            }
                        }
                    },
                    "Quit" => break,
                    _ => println!("Unknown command {first}")
                }
            },
            None => println!("No command")
        }
    }
}