use std::collections::HashMap;

fn main() {
    let mut vec = vec![2, 11, 7, 13, 1, 20, 0, -9, -1];
    vec.sort();
    println!("{vec:?}");
    let median_index = vec.len() / 2;
    let median = vec.get(median_index).copied(); // median would be the average of two middle values if the vector length is even but idk how to hold either an int or float inside one Option, so I'm just going with the higher of the two
    match median {
        Some(val) => println!("Median: {val}"),
        None => println!("No median")
    }
    let mut frequency = HashMap::new();
    for i in &vec {
        let curr = frequency.entry(*i).or_insert(0);
        *curr += 1;
    }
    let mut max_key: Option<i32> = None;
    for (k, v) in &frequency {
        match max_key {
            Some(key) => {
                if let Some(max_val) = frequency.get(&key).copied()  {
                    if *v > max_val {
                        max_key = Some(*k);
                    }
                }
            },
            None => max_key = Some(*k)
        }
    }
    match max_key {
        Some(mode) => println!("Mode: {mode}"), // if the distribution is uniform then all values are modes (or maybe no values? jury's out), so this just chooses the first one
        None => println!("No mode found")
    }
}
