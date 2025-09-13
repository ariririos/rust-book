use adder::add;
mod common;

#[test]
fn two_plus_two() {
    let result = add(2, 2);
    common::print_helper("result: ", &*result.to_string()); // won't print anything normally
    assert_eq!(result, 4, "result: {result}");
}