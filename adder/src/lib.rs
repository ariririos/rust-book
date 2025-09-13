#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    #[allow(dead_code)]
    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn add_two(a: u64) -> u64 {
    a + 2
}

pub struct Guess {
    #[allow(dead_code)]
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess must be greater than or equal to 1, got {value}");
        }
        else if value > 100 {
            panic!("Guess must be less than or equal to 100, got {value}");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;

    #[test]
    fn largest_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle {
            width: 5,
            height: 1
        };
        
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle {
            width: 5,
            height: 1
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[should_panic]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 2, "result: {result}");
    }

    #[test]
    #[should_panic(expected = "less than or equal to")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        }
        else {
            Err(String::from("2+2!=4"))
        }
    }

    #[test]
    #[ignore]
    fn takes_long_time() {
        sleep(Duration::from_secs(5));
    }
}
