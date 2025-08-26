fn main() {
    let mut nums = vec![1, 2, 3, 4];
    let first_by_index = &nums[0];
    println!("First: {first_by_index}");
    let first_by_method = nums.get(0);
    match first_by_method {
        Some(first) => println!("First: {first}"),
        None => println!("Unreachable")
    }
    let not_in_vec = nums.get(10);
    match not_in_vec {
        Some(_) => println!("Unreachable"),
        None => println!("Nothing at index 10")
    }
    nums.push(5);
    println!("Nums vec: {nums:?}");
    for i in &mut nums {
        println!("{i}");
        *i += 1;
    }
    for i in &nums {
        println!("{i}");
    }
    #[allow(unused)]
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        println!("{i:?}");
    }
}