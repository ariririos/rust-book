type Kilometers = i32;

#[allow(unused)]
fn bar() -> ! {
    print!("forever ");
    loop {
        println!("and ever");
    }
}

fn generic<T: ?Sized + std::fmt::Display>(t: &T) {
    println!("{}", t)
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
}

#[derive(Debug)]
#[allow(dead_code)]
enum Status {
    Value(u32),
    Stop,
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    generic("test");
    generic(&*"test2");

    let answer = do_twice(add_one, 5);
    println!("add_one(add_one(5)) = {answer}");

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{list_of_statuses:?}");

    let closures = [returns_closure(), returns_initialized_closure(1)];
    for closure in closures {
        let output = closure(1);
        println!("{output}");
    }
}
